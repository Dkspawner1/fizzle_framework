use crate::game::game::Game;
use crate::threading::job_system::JobSystem;
use num_cpus;

pub struct Game1 {
    game: Game,
    job_system: JobSystem,
}

impl Game1 {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let thread_count = num_cpus::get();

        Ok(Game1 {
            game: Game::new()?,
            job_system: JobSystem::new(thread_count),
        })
    }
    pub fn run(&mut self) {
        self.initialize();
        self.game.run();
    }
    fn initialize(&mut self) {
        self.game.initialize();
    }
}
