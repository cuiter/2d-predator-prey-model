use crate::model::{Model, Cell};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

pub struct View
{
    pub midpoint: Point,
    pub scale: u32
}

const BACKGROUND_COLOR: Color = Color::RGBA(100, 100, 100, 255);
const CELL_EMPTY_COLOR: Color = Color::RGBA(220, 220, 220, 255);
const CELL_PLANT_COLOR: Color = Color::RGBA(20, 180, 20, 255);
const CELL_HERBIVORE_COLOR: Color = Color::RGBA(100, 40, 30, 255);
const CELL_CARNIVORE_COLOR: Color = Color::RGBA(200, 90, 10, 255);
const GRID_DIVIDER_COLOR: Color = Color::RGBA(0, 0, 0, 255);


pub fn draw_model(canvas: &mut Canvas<Window>, model: &Model, view: &View)
{
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    let grid_size = model.get_grid_size();
    let (canvas_width, canvas_height) = canvas.output_size().unwrap();

    let mut prev_color = Color::RGBA(0, 0, 0, 0);
    for x in 0..grid_size.w {
        for y in 0..grid_size.h {
            let draw_x = (canvas_width / 2) as i32 + (x as i32 - view.midpoint.x) * view.scale as i32;
            let draw_y = (canvas_height / 2) as i32 + (y as i32 - view.midpoint.y) * view.scale as i32;
            let draw_rect = Rect::new(draw_x, draw_y, view.scale, view.scale);

            let color = match model.get_cell_at(x, y) {
                Cell::Empty => CELL_EMPTY_COLOR,
                Cell::Plant => CELL_PLANT_COLOR,
                Cell::Herbivore(_) => CELL_HERBIVORE_COLOR,
                Cell::Carnivore(_) => CELL_CARNIVORE_COLOR
            };

            if color != prev_color {
                canvas.set_draw_color(color);
                prev_color = color;
            }

            canvas.fill_rect(draw_rect).unwrap();
        }
    }
}