use crate::model::{Cell, Model};
use crate::util::Size;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

pub struct View {
    pub midpoint: Point,
    pub scale: u32,
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
const CELL_PLANT_COLOR: Color = Color::RGBA(20, 180, 20, 255);
const CELL_HERBIVORE_COLOR: Color = Color::RGBA(100, 40, 30, 255);
const CELL_CARNIVORE_COLOR: Color = Color::RGBA(200, 90, 10, 255);
const GRID_DIVIDER_COLOR: Color = Color::RGBA(140, 140, 140, 255);

const MIN_SCALE_FOR_GRID_DIVIDER: u32 = 8;

fn model_to_canvas_coord(model_coord: Point, canvas_size: Size, view: &View) -> Point {
    let draw_x = (canvas_size.w / 2) as i32 + (model_coord.x - view.midpoint.x) * view.scale as i32;
    let draw_y = (canvas_size.h / 2) as i32 + (model_coord.y - view.midpoint.y) * view.scale as i32;

    Point::new(draw_x, draw_y)
}

pub fn draw_model(canvas: &mut Canvas<Window>, model: &Model, view: &View) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    // Draw model cells

    let grid_size = model.get_grid_size();
    let (canvas_width, canvas_height) = canvas.output_size().unwrap();
    let canvas_size = Size::new(canvas_width, canvas_height);

    let mut prev_color = Color::RGBA(0, 0, 0, 0);
    for x in 0..grid_size.w {
        for y in 0..grid_size.h {
            let draw_point =
                model_to_canvas_coord(Point::new(x as i32, y as i32), canvas_size, view);
            let draw_rect = Rect::new(draw_point.x, draw_point.y, view.scale, view.scale);

            let color = match model.get_cell_at(x, y) {
                Cell::Empty => CELL_EMPTY_COLOR,
                Cell::Plant => CELL_PLANT_COLOR,
                Cell::Herbivore(_) => CELL_HERBIVORE_COLOR,
                Cell::Carnivore(_) => CELL_CARNIVORE_COLOR,
            };

            if color != prev_color {
                canvas.set_draw_color(color);
                prev_color = color;
            }

            canvas.fill_rect(draw_rect).unwrap();
        }
    }

    if view.scale >= MIN_SCALE_FOR_GRID_DIVIDER {
        canvas.set_draw_color(GRID_DIVIDER_COLOR);

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
    }
}
