mod gfx;
mod gui;
mod model;
mod util;

extern crate sdl2;

fn main() {
    println!("\nsimulation controls:\n  R: restart\n  ,/.: decrease/increase speed\n  scroll wheel: decrease/increase scale\n  space: toggle pause/resume\n");
    gui::main_loop(&model::ModelParams::default());
}
