use crate::core::game_time::GameTime;
use crate::graphics::renderer::Renderer;
use crate::scenes::scene::Scene;
use crate::threading::job::JobPtr;
use crate::threading::job_system::JobSystem;

pub struct Menu {
    // Add any menu-specific fields here
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            // Initialize menu-specific fields
        }
    }
}

impl Scene for Menu {
    fn initialize(&mut self, _job_system: &JobSystem) {
        // Initialize menu
    }

    fn load_content(&mut self, _job_system: &JobSystem) -> Vec<JobPtr> {
        // Load menu assets
        Vec::new() // Return an empty vector if no async loading is needed
    }

    fn update(&mut self, _game_time: &GameTime) {
        // Update menu logic
    }

    fn draw(&self, renderer: &Renderer) {
        // Draw menu
    }


}
