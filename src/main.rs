use crate::graphics::window;
use crate::graphics::window::Window;

mod graphics;
mod threading;

fn main() {
    let window = Window::new(1600, 900, "Test!");

    println!("Hello, world!");
}
