mod gfx;
mod gui;
mod models;
mod util;

extern crate sdl2;

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();
    if arguments.len() < 2 {
        println!("Usage: {} <config file>", arguments[0]);
        std::process::exit(1);
    }
    let config_path = &arguments[1];

    let params = models::params::params_from_file(config_path).expect("Failed to load parameters");

    println!("\nsimulation controls:\n  R: restart\n  ,/.: decrease/increase speed\n  scroll wheel: decrease/increase scale\n  space: toggle pause/resume\n");
    gui::main_loop(params);
}
