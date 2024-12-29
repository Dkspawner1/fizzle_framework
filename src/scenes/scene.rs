use crate::core::game_time::GameTime;
use crate::graphics::renderer::Renderer;
use crate::threading::job_system::JobSystem;
use crate::threading::job::JobPtr;

pub trait Scene {
    fn initialize(&mut self, job_system: &JobSystem);
    fn load_content(&mut self, job_system: &JobSystem) -> Vec<JobPtr>;
    fn update(&mut self, game_time: &GameTime);
    fn draw(&self, renderer: &Renderer);
}
