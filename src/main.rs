mod sim;
mod stats;
mod gui;
mod gfx;
mod util;

extern crate sdl2;

fn main() {
    println!("Hello, world!");
    gui::main_loop();
}
