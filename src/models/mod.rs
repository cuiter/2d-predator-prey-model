use crate::util::{time_ns, Size};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

pub mod params;
pub use params::{ModelParams, ModelType};

mod simple_model;
use simple_model::SimpleModel;

#[derive(Clone, PartialEq)]
pub enum Cell {
    Empty,
    Animal(u32),
}

pub struct Grid {
    size: Size,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(size: Size) -> Grid {
        Grid {
            size: size,
            cells: vec![Cell::Empty; size.w as usize * size.h as usize],
        }
    }

    pub fn populate(&mut self, params: &ModelParams) {
        let mut rng = Pcg32::seed_from_u64(params.random_seed.unwrap_or(time_ns() as u64));
        let specie_ids = params.specie_ids();

        for (specie_name, specie_params) in params.species.iter() {
            let specie_id = specie_ids.get_by_left(&specie_name).unwrap();
            let target_population =
                (specie_params.initial_population * self.size.w as f32 * self.size.h as f32) as u32;
            let mut population = 0;
            while population < target_population {
                let new_x = rng.gen_range(0, self.size.w);
                let new_y = rng.gen_range(0, self.size.h);

                if self.get_cell_at(new_x, new_y) == &Cell::Empty {
                    self.set_cell_at(new_x, new_y, Cell::Animal(*specie_id));
                    population += 1;
                }
            }
        }
    }

    #[inline]
    pub fn get_cell_at(&self, x: u32, y: u32) -> &Cell {
        &self.cells[x as usize + y as usize * self.size.w as usize]
    }

    #[inline]
    fn set_cell_at(&mut self, x: u32, y: u32, cell: Cell) {
        self.cells[x as usize + y as usize * self.size.w as usize] = cell;
    }

    pub const fn get_size(&self) -> Size {
        self.size
    }
}

pub trait Model {
    fn populate(&mut self);
    fn tick(&mut self);
    fn get_grid(&self) -> &Grid;
    fn get_params(&self) -> &ModelParams;
}

pub fn create_model(params: ModelParams) -> Box<dyn Model> {
    match &params.model {
        &ModelType::Simple => Box::new(SimpleModel::new(params)),
        _ => {
            panic!("Model {:?} not implemented", &params.model)
        }
    }
}
