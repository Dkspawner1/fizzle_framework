use crate::core::game_time::GameTime;
use crate::graphics::renderer::Renderer;
use crate::scenes::scene::Scene;
use crate::threading::job::JobPtr;
use crate::threading::job_system::JobSystem;

pub struct LoadingScene {
    progress: f32,
}

impl LoadingScene {
    pub fn new() -> Self {
        LoadingScene { progress: 0.0 }
    }
    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress;
    }
}
impl Scene for LoadingScene {
    fn initialize(&mut self, _job_system: &JobSystem) {}

    fn load_content(&mut self, _job_system: &JobSystem) -> Vec<JobPtr> {
        Vec::new()
    }

    fn update(&mut self, _game_time: &GameTime) {}

    fn draw(&self, renderer: &Renderer) {
        // Draw loading bar using self.progress
        // renderer.draw_loading_bar(self.progress);
    }

}