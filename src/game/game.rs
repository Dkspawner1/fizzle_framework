use glfw::Key::Menu;
use crate::core::game_time::GameTime;
use crate::graphics::renderer::Renderer;
use crate::graphics::window::Window;
use crate::managers::scene_manager::SceneManager;


pub struct Game {
    window: Window,
    renderer: Renderer,
    game_time: GameTime,
    scene_manager: SceneManager,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut window = Window::new(1600, 900, "Fizzle Framework")?;
        let renderer = Renderer::new(|s| window.get_proc_address(s))?;
        let thread_count = num_cpus::get();

        Ok(Game {
            window,
            renderer,
            game_time: GameTime::new(),
            scene_manager: SceneManager::new(thread_count),
        })
    }


    pub fn run(&mut self) {
        self.initialize();

        while !self.window.should_close() {
            self.game_time.update();
            self.handle_events();
            self.scene_manager.update(&self.game_time);
            self.renderer.clear();
            self.scene_manager.draw(&self.renderer);
            self.renderer.render();
            self.window.swap_buffers();
            self.game_time.sleep_to_sync();
        }
    }


    pub fn initialize(&mut self) {
        self.window.set_size_callback(|width, height| {
            println!("Window resized to {}x{}", width, height);
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        });
        self.scene_manager.add_scene("menu".to_string(), Box::new(Menu::new()));
    }

    pub fn load_content(&self) {
        // Implement content loading here
    }
    pub fn handle_events(&mut self) {
        self.window.update();
        // Handle any game-specific events here
    }
}