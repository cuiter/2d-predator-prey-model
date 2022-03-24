use crate::gfx::{draw_model, View};
use crate::models::{create_model, params::params_from_file, Model, ModelParams};
use crate::stats::Stats;
use crate::util::{time_ns, Size};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;

/// Controls the time aspect of the simulation, e.g. how fast the simulation should run, whether the simulation is paused or not.
struct TimeController {
    ticks_per_second: f32,
    running: bool,
    leftover_seconds: f32,
}

impl TimeController {
    pub fn default() -> TimeController {
        TimeController {
            ticks_per_second: 1.0,
            running: true,
            leftover_seconds: 0.0,
        }
    }

    /// Updates the time and returns the number of times model.tick() should be called in this frame.
    pub fn tick(&mut self, seconds_elapsed: f32) -> u32 {
        if self.running {
            self.leftover_seconds += seconds_elapsed;

            let mut ticks = 0u32;
            while self.leftover_seconds >= (1.0 / self.ticks_per_second) {
                ticks += 1;
                self.leftover_seconds -= 1.0 / self.ticks_per_second;
            }

            ticks
        } else {
            0
        }
    }

    pub fn toggle_paused(&mut self) {
        self.running = !self.running;

        if self.running {
            println!("model resumed");
        } else {
            println!("model paused");
        }
    }

    pub fn increase_speed(&mut self) {
        self.ticks_per_second *= 2.0;
    }

    pub fn decrease_speed(&mut self) {
        self.ticks_per_second /= 2.0;
    }
}

const ENABLE_VSYNC: bool = true;
const WINDOW_SIZE: Size = Size::new(800, 600);
// Number of seconds (target) for processing model behavior per frame, before continuing on.
const MODEL_TIME_PER_FRAME_THRESHOLD_SEC: f32 = 0.025;

/// The main (GUI) loop of the program.
/// Creates an SDL2 window and runs an event loop.
pub fn main_loop(config_path: &str, stats_path: Option<&str>) {
    let model_params = params_from_file(config_path).expect("Failed to load parameters");
    let mut model: Box<dyn Model> = create_model(model_params);
    model.populate();
    let mut stats = stats_path.map(|path| Stats::new(path));
    let mut ticks_elapsed = 0;

    let mut time_controller = TimeController::default();
    let mut view = View::default(model.get_grid().get_size());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Onderzoeksmethoden", WINDOW_SIZE.w, WINDOW_SIZE.h)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas;
    {
        let mut canvasbuilder = window.into_canvas();
        if ENABLE_VSYNC {
            canvasbuilder = canvasbuilder.present_vsync();
        }
        canvas = canvasbuilder.build().unwrap();
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut prev_nano_time = time_ns();

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'event_loop;
                }

                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::SizeChanged(_width, _height) => {
                        // Note: Window size may be used for later optimizations when using large grid sizes.
                    }
                    _ => {}
                },

                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => {
                    if scancode == Scancode::R {
                        // Reload parameters from file and repopulate model
                        match params_from_file(config_path) {
                            Ok(params) => {
                                model = create_model(params.clone());
                                model.populate();

                                if let Some(stats) = &mut stats {
                                    stats.reset();
                                }
                                ticks_elapsed = 0;
                            }
                            Err(error) => {
                                println!("Failed to load parameters: {}", error);
                            }
                        }
                    } else if scancode == Scancode::Space {
                        time_controller.toggle_paused();
                    } else if scancode == Scancode::Comma {
                        time_controller.decrease_speed();
                    } else if scancode == Scancode::Period {
                        time_controller.increase_speed();
                    }
                }

                Event::MouseWheel { y, .. } => {
                    if y < 0 {
                        view.decrease_scale();
                    } else if y > 0 {
                        view.increase_scale();
                    }
                }

                _ => {}
            }
        }

        let cur_nano_time = time_ns();
        let raw_seconds_elapsed: f32 = (cur_nano_time - prev_nano_time) as f32 / 1e9f32;
        prev_nano_time = cur_nano_time;
        // Clamp the elapsed time in this frame between 1 nanosecond and 1 second to prevent divide by zero and runaway.
        let seconds_elapsed: f32 = raw_seconds_elapsed.clamp(1e-9f32, 1.0);

        let ticks = time_controller.tick(seconds_elapsed);
        for _ in 0..ticks {
            model.tick();
            ticks_elapsed += 1;
            if let Some(stats) = &mut stats {
                stats.collect(ticks_elapsed, model.get_grid(), model.get_params());
            }

            if (time_ns() - cur_nano_time) as f32 / 1e9f32 > MODEL_TIME_PER_FRAME_THRESHOLD_SEC {
                // Skip processing any more ticks, continue.
                break;
            }
        }

        draw_model(&mut canvas, &model, &view);
        canvas.present();
    }
}
