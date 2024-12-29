use std::collections::HashMap;
use crate::core::game_time::GameTime;
use crate::graphics::renderer::Renderer;
use crate::scenes::loading_scene::LoadingScene;
use crate::scenes::scene::Scene;
use crate::threading::job::JobPtr;
use crate::threading::job_system::JobSystem;

pub struct SceneManager
{
    scenes: HashMap<String, Box<dyn Scene>>,
    current_scene: Option<String>,
    loading_scene: LoadingScene,
    next_scene: Option<String>,
    job_system: JobSystem,
    loading_jobs: Vec<JobPtr>,
}

impl SceneManager
{
    pub fn new(thread_count: usize) -> Self
    {
        SceneManager{
            scenes: HashMap::new(),
            current_scene: None,
            loading_scene: LoadingScene::new(),
            next_scene: None,
            job_system: JobSystem::new(thread_count),
            loading_jobs: Vec::new(),
        }
    }
    pub fn add_scene(&mut self, name: String, mut scene: Box<dyn Scene>)
    {
        scene.initialize(&self.job_system);
        self.scenes.insert(name,scene);
    }
    pub fn set_scene(&mut self, name: String) {
        if self.scenes.contains_key(&name) {
            self.next_scene = Some(name.clone());
            if let Some(scene) = self.scenes.get_mut(&name) {
                self.loading_jobs = scene.load_content(&self.job_system);
                for job in &self.loading_jobs {
                    self.job_system.add_job(job.clone());
                }
            }
        }
    }

    pub fn update(&mut self, game_time: &GameTime) {
        if let Some(scene_name) = &self.current_scene {
            if let Some(scene) = self.scenes.get_mut(scene_name) {
                scene.update(game_time);
            }
        }

        if !self.loading_jobs.is_empty() {
            let total_jobs = self.loading_jobs.len();
            let completed_jobs = total_jobs - *self.job_system.active_jobs.lock().unwrap();
            let progress = completed_jobs as f32 / total_jobs as f32;
            self.loading_scene.set_progress(progress);

            if completed_jobs == total_jobs {
                self.loading_jobs.clear();
                self.current_scene = self.next_scene.take();
            }
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        if !self.loading_jobs.is_empty() {
            self.loading_scene.draw(renderer);
        } else if let Some(scene_name) = &self.current_scene {
            if let Some(scene) = self.scenes.get(scene_name) {
                scene.draw(renderer);
            }
        }
    }
}
