use crate::models::{Cell, Model};
use crate::util::Size;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use std::collections::HashMap;

/// Represents a viewport for drawing the model's cells onto a canvas
pub struct View {
    pub midpoint: Point, // position of the cell that is drawn in the middle of the screen
    pub scale: u32,      // pixels per cell
}
impl View {
    pub fn default(grid_size: Size) -> View {
        View {
            midpoint: Point::new(grid_size.w as i32 / 2, grid_size.h as i32 / 2),
            scale: 8,
        }
    }

    pub fn increase_scale(&mut self) {
        self.scale *= 2;
    }

    pub fn decrease_scale(&mut self) {
        if self.scale > 1 {
            self.scale /= 2;
        }
    }
}

const BACKGROUND_COLOR: Color = Color::RGBA(100, 100, 100, 255);
const CELL_EMPTY_COLOR: Color = Color::RGBA(220, 220, 220, 255);
const CELL_ANIMAL_DEFAULT_COLOR: Color = Color::RGBA(200, 90, 10, 255);
const GRID_DIVIDER_COLOR: Color = Color::RGBA(140, 140, 140, 255);
const MIN_SCALE_FOR_DRAWING_GRID: u32 = 8;

fn model_to_canvas_coord(model_coord: Point, canvas_size: Size, view: &View) -> Point {
    let draw_x = (canvas_size.w / 2) as i32 + (model_coord.x - view.midpoint.x) * view.scale as i32;
    let draw_y = (canvas_size.h / 2) as i32 + (model_coord.y - view.midpoint.y) * view.scale as i32;

    Point::new(draw_x, draw_y)
}

pub fn draw_model(canvas: &mut Canvas<Window>, model: &Box<dyn Model>, view: &View) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    // Draw cells

    let grid = model.get_grid();
    let grid_size = grid.get_size();
    let params = model.get_params();
    let specie_ids = params.specie_ids();
    let (canvas_width, canvas_height) = canvas.output_size().unwrap();
    let canvas_size = Size::new(canvas_width, canvas_height);

    let mut color_cache = HashMap::new();

    let mut prev_color = Color::RGBA(0, 0, 0, 0);
    for x in 0..grid_size.w {
        for y in 0..grid_size.h {
            let draw_point =
                model_to_canvas_coord(Point::new(x as i32, y as i32), canvas_size, view);
            let draw_rect = Rect::new(draw_point.x, draw_point.y, view.scale, view.scale);

            let color = match grid.get_cell_at(x, y) {
                Cell::Empty => CELL_EMPTY_COLOR,
                Cell::Animal(specie_id) => {
                    if !color_cache.contains_key(specie_id) {
                        let color_str =
                            &params.species[specie_ids.get_by_right(specie_id).unwrap()].color;
                        let color = match color_str {
                            Some(specie_color) => {
                                let hex_color = u32::from_str_radix(specie_color, 16).unwrap();

                                Color::RGB(
                                    (hex_color >> 16) as u8,
                                    (hex_color >> 8) as u8,
                                    hex_color as u8,
                                )
                            }
                            None => CELL_ANIMAL_DEFAULT_COLOR,
                        };

                        color_cache.insert(specie_id, color);
                    }

                    color_cache[specie_id]
                }
            };

            if color != prev_color {
                canvas.set_draw_color(color);
                prev_color = color;
            }

            canvas.fill_rect(draw_rect).unwrap();
        }
    }

    draw_grid(canvas, canvas_size, grid_size, view);
}

pub fn draw_grid(canvas: &mut Canvas<Window>, canvas_size: Size, grid_size: Size, view: &View) {
    if view.scale >= MIN_SCALE_FOR_DRAWING_GRID {
        canvas.set_draw_color(GRID_DIVIDER_COLOR);

        // Draw horizontal lines
        for y in 0..grid_size.h {
            let start_point = model_to_canvas_coord(Point::new(0, y as i32), canvas_size, view);
            let end_point =
                model_to_canvas_coord(Point::new(grid_size.w as i32, y as i32), canvas_size, view);

            let draw_rect = Rect::new(
                start_point.x,
                start_point.y,
                (end_point.x - start_point.x) as u32,
                1,
            );

            canvas.fill_rect(draw_rect).unwrap();
        }

        // Draw vertical lines
        for x in 0..grid_size.w {
            let start_point = model_to_canvas_coord(Point::new(x as i32, 0), canvas_size, view);
            let end_point =
                model_to_canvas_coord(Point::new(x as i32, grid_size.h as i32), canvas_size, view);

            let draw_rect = Rect::new(
                start_point.x,
                start_point.y,
                1,
                (end_point.y - start_point.y) as u32,
            );

            canvas.fill_rect(draw_rect).unwrap();
        }
    }
}
