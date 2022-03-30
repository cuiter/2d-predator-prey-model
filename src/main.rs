mod gfx;
mod gui;
mod models;
mod stats;
mod util;

extern crate sdl2;

const DEFAULT_CONFIG_PATH: &str = "sample-configs/simple-fish.json";

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();
    if arguments.len() < 1 {
        println!(
            "Usage: {} [path/to/config.json] [path/to/stats.csv]",
            arguments[0]
        );
        std::process::exit(1);
    }
    let config_path = if arguments.len() >= 2 { &arguments[1] } else { DEFAULT_CONFIG_PATH };
    let stats_path = if arguments.len() >= 3 {
        Some(arguments[2].as_str())
    } else {
        None
    };

    println!("\nsimulation controls:\n  R: restart\n  ,/.: decrease/increase speed\n  scroll wheel: decrease/increase scale\n  space: toggle pause/resume\n");
    gui::main_loop(config_path, stats_path);
}
