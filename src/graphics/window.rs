extern crate glfw;
use glfw::{fail_on_errors, Action, Context, Glfw, GlfwReceiver, Key, WindowEvent};

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
    size_callback: Option<Box<dyn Fn(i32, i32)>>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, String> {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let monitor = glfw
            .with_primary_monitor(|_, m| m.unwrap().get_video_mode())
            .unwrap();
        let screen_width = monitor.width;
        let screen_height = monitor.height;
        let x_position = (screen_width - width) / 2;
        let y_position = (screen_height - height) / 2;

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or("Failed to create GLFW window.")?;

        window.set_pos(x_position as i32, y_position as i32);
        window.make_current();
        window.set_key_polling(true);
        window.set_size_polling(true);

        Ok(Self {
            glfw,
            window,
            events,
            size_callback: None,
        })
    }
    pub fn get_proc_address(&mut self, name: &str) -> *const std::os::raw::c_void {
        self.window.get_proc_address(name)
    }
    pub fn set_size_callback<F: Fn(i32, i32) + 'static>(&mut self, callback: F) {
        self.size_callback = Some(Box::new(callback));
    }


    pub fn update(&mut self) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            println!("{:?}", event);
            match event {
                WindowEvent::Size(width, height) => {
                    if let Some(ref callback) = self.size_callback {
                        callback(width, height);
                    }
                }
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}
