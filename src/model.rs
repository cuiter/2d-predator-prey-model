use crate::util::{Size, time_ns};
use rand_pcg::Pcg32;
use rand::{Rng, SeedableRng};

#[derive(Clone, PartialEq)]
pub enum Cell
{
    Empty,
    Plant,
    Herbivore(AnimalState),
    Carnivore(AnimalState)
}

#[derive(Clone, PartialEq)]
pub struct AnimalState
{
    pub age: u32, // number of ticks since birth
    pub direction: u8, // 8-way direction, counter-clockwise, starting at 0 = right
    pub energy: u8, // energy
}

impl AnimalState
{
    pub fn new(rng: &mut Pcg32, initial_energy: u8) -> AnimalState {
        AnimalState {
            age: 0,
            direction: rng.gen_range(0, 8),
            energy: initial_energy
        }
    }
}

pub struct Model
{
    cells: Vec<Cell>,
    params: ModelParams
}

pub struct ModelParams
{
    pub grid_size: Size,
    pub n_plants: u32,
    pub n_herbivores: u32,
    pub n_carnivores: u32,
    pub ticks_to_reproduce: u32,
    pub birth_energy_units: u8,
    pub food_energy_units: u8,
    pub max_energy_units: u8,
    pub random_seed: u64,
}

impl ModelParams
{
    pub fn default() -> ModelParams {
        ModelParams
        {
            grid_size: Size::new(40, 40),
            n_plants: 40,
            n_herbivores: 60,
            n_carnivores: 60,
            ticks_to_reproduce: 20,
            birth_energy_units: 10,
            food_energy_units: 8,
            max_energy_units: 8,
            random_seed: time_ns() as u64
        }
    }
}

impl Model {
    pub fn new(params: ModelParams) -> Model {
        let cells = vec![Cell::Empty; params.grid_size.w as usize * params.grid_size.h as usize];

        let mut model = Model {
            cells,
            params
        };

        model.populate();

        model
    }

    fn populate(&mut self) {
        let mut rng = Pcg32::seed_from_u64(self.params.random_seed);

        let mut n_plants = 0;
        while n_plants < self.params.n_plants {
            let new_x = rng.gen_range(0, self.get_grid_size().w);
            let new_y = rng.gen_range(0, self.get_grid_size().h);

            if self.get_cell_at(new_x, new_y) == &Cell::Empty {
                self.set_cell_at(new_x, new_y, Cell::Plant);
                n_plants += 1;
            }
        }

        let mut n_carnivores = 0;
        while n_carnivores < self.params.n_carnivores {
            let new_x = rng.gen_range(0, self.get_grid_size().w);
            let new_y = rng.gen_range(0, self.get_grid_size().h);

            if self.get_cell_at(new_x, new_y) == &Cell::Empty {
                self.set_cell_at(new_x, new_y, Cell::Carnivore(AnimalState::new(&mut rng, self.params.birth_energy_units)));
                n_carnivores += 1;
            }
        }

        let mut n_herbivores = 0;
        while n_herbivores < self.params.n_herbivores {
            let new_x = rng.gen_range(0, self.get_grid_size().w);
            let new_y = rng.gen_range(0, self.get_grid_size().h);

            if self.get_cell_at(new_x, new_y) == &Cell::Empty {
                self.set_cell_at(new_x, new_y, Cell::Herbivore(AnimalState::new(&mut rng, self.params.birth_energy_units)));
                n_herbivores += 1;
            }
        }
    }

    pub fn tick() {

    }

    #[inline]
    pub fn get_cell_at(&self, x: u32, y: u32) -> &Cell {
        &self.cells[x as usize + y as usize * self.params.grid_size.w as usize]
    }

    #[inline]
    fn set_cell_at(&mut self, x: u32, y: u32, cell: Cell) {
        self.cells[x as usize + y as usize * self.params.grid_size.w as usize] = cell;
    }

    pub const fn get_grid_size(&self) -> Size {
        self.params.grid_size
    }
}