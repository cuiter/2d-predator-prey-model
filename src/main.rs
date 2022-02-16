mod gfx;
mod gui;
mod model;
mod stats;
mod util;

extern crate sdl2;

fn main() {
    gui::main_loop(model::ModelParams::default());
}
