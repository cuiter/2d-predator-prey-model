use std::time::{SystemTime, UNIX_EPOCH};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use crate::util::Size;

struct View
{
    pub midpoint: Point,
    pub scale: u32
}

struct TimeControl
{
    pub ticks_per_second: f32,
    pub running: bool
}

impl TimeControl
{
    pub fn default() -> TimeControl {
        TimeControl {
            ticks_per_second: 1.0,
            running: true
        }
    }
}

/// Get the current Unix time in nanoseconds.
fn time_ns() -> u128 {
    let time_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    time_since_epoch.as_nanos()
}

const ENABLE_VSYNC: bool = true;
const WINDOW_SIZE: Size = Size::new(800, 600);
const BACKGROUND_COLOR: Color = Color::RGBA(0, 0, 0, 255);

/// The main (GUI) loop of the program.
/// Creates an SDL2 window and runs an event loop.
pub fn main_loop() {
    //let mut world = World::new(params);
    let mut time_control = TimeControl::default();
    let mut view = View { midpoint: Point::new(0, 0), scale: 1 };

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
                    WindowEvent::SizeChanged(width, height) => {
                        //view.window_size = Size::new(width as u32, height as u32);
                    }
                    _ => {}
                },

                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => {
                    if scancode == Scancode::R {
                        //world = World::new(&params);
                    } else if scancode == Scancode::T {
                        //time_controller.goto_prompt(params, &mut world);
                    } else {
                        //view.key_down(scancode);
                    }
                }

                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => {
                    //view.key_up(scancode);
                }

                Event::MouseWheel { y, .. } => {
                    //view.change_zoom(y as f32);
                }

                _ => {}
            }
        }

        let cur_nano_time = time_ns();
        let raw_d_time: f32 = (cur_nano_time - prev_nano_time) as f32 / 1e9f32;
        // Clamp d_time between 1 nanosecond and 1 second to prevent divide by zero and runaway.
        let d_time: f32 = raw_d_time.clamp(1e-9f32, 1.0);

        //view.tick(d_time);

        /*if !view.paused {
            time_controller.tick(params, &mut world, d_time * view.time_factor);
        }*/

        //draw_world(&mut canvas, &mut assets, &view, &world);
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();
        canvas.present();

        prev_nano_time = cur_nano_time;
    }
}
