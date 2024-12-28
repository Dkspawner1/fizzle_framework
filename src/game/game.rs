use crate::core::game_time::GameTime;
use crate::graphics::renderer::Renderer;
use crate::graphics::window::Window;
// use crate::managers::scene_manager::SceneManager;
// use crate::scenes::game_scene::GameScene;
// use crate::scenes::menu_scene::MenuScene;

pub struct Game {
    // pub(crate) scene_manager: SceneManager,
    window: Window,
    renderer: Renderer,
    game_time: GameTime,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let mut window = Window::new(1600, 900, "Fizzle Framework")?;
        let renderer = Renderer::new(|s| window.get_proc_address(s))?;

        Ok(Game {
            // scene_manager: SceneManager::new(),
            window,
            renderer,
            game_time: GameTime::new(),
        })
    }

    pub fn run(&mut self) {
        self.initialize();
        self.load_content();

        self.game_time.set_target_fps(60);

        while !self.window.should_close() {
            self.game_time.update();
            self.window.update();
            self.update();
            self.draw();
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

        // self.scene_manager
        //     .add_scene("menu".to_string(), Box::new(MenuScene::new()));
        // self.scene_manager
        //     .add_scene("game".to_string(), Box::new(GameScene::new()));
        // self.scene_manager.set_scene("menu".to_string());
    }

    pub fn load_content(&self) {
        // Implement content loading here
    }

    pub fn update(&mut self) {
        // self.scene_manager.update(self.game_time.delta_time());
    }

    pub fn draw(&self) {
        self.renderer.render();
        // self.scene_manager.draw();
    }
}