// mod content;
mod core;
mod game;
mod graphics;
// mod managers;
mod threading;
mod scenes;
mod managers;

use crate::core::game1::Game1;

fn main() {
    let mut game= Game1::new().expect("Failed to create game");
    game.run();

    println!("Hello, world!");
}
