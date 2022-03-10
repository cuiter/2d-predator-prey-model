mod gfx;
mod gui;
mod models;
mod stats;
mod util;

extern crate sdl2;

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();
    if arguments.len() < 2 {
        println!("Usage: {} <path/to/config.json> [path/to/stats.csv]", arguments[0]);
        std::process::exit(1);
    }
    let config_path = &arguments[1];
    let stats_path = if arguments.len() >= 3 { Some(arguments[2].as_str()) } else { None };

    println!("\nsimulation controls:\n  R: restart\n  ,/.: decrease/increase speed\n  scroll wheel: decrease/increase scale\n  space: toggle pause/resume\n");
    gui::main_loop(config_path, stats_path);
}
