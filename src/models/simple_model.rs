use crate::models::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashMap;

pub struct SimpleModel {
    grid: Grid,
    params: ModelParams,
    rng: Pcg32,
}

impl SimpleModel {
    pub fn new(params: ModelParams) -> SimpleModel {
        let grid = Grid::new(params.grid_size);
        let rng = Pcg32::seed_from_u64(params.random_seed.unwrap_or(time_ns() as u64));

        SimpleModel { grid, params, rng }
    }

    /// Determines the next state of the given cell, given the current state and the cell's surrounding neighbors.
    fn next_cell_state(&mut self, cell: &Cell, neighbors: &Vec<Cell>) -> Cell {
        let (n_predators, dominant_predator_id) =
            get_neighbor_predators(cell, neighbors, &self.params);

        match cell {
            &Cell::Animal(specie_id) => {
                let specie_params = self.params.get_specie_by_id(specie_id);
                let cell_birth_rate = specie_params.birth_rate;
                let cell_death_rate = specie_params.death_rate;
                let specie_is_herbivore = self.params.is_specie_herbivore(specie_id);

                if specie_is_herbivore || n_predators > 0 {
                    // Cell is prey
                    let random_1 = self.rng.gen::<f32>();

                    if random_1 < (1.0f32 - cell_death_rate).powf(n_predators as f32) {
                        // Hunt failed.
                    } else {
                        let predator_birth_rate = self
                            .params
                            .get_specie_by_id(dominant_predator_id)
                            .birth_rate;
                        let random_2 = self.rng.gen::<f32>();
                        if random_2 < predator_birth_rate {
                            // Cell becomes predator by breeding
                            return Cell::Animal(dominant_predator_id);
                        }
                    }
                }
                if !specie_is_herbivore {
                    // Cell is a predator
                    let random = self.rng.gen::<f32>();
                    if random < cell_death_rate {
                        // Cell becomes empty due to predator death
                        Cell::Empty
                    } else {
                        // Cell remains predator
                        Cell::Animal(specie_id)
                    }
                } else {
                    // Cell is a herbivore, remains the same.
                    Cell::Animal(specie_id)
                }
            }
            &Cell::Empty => {
                let (n_same_herbivores, dominant_herbivore_id) =
                    get_neighbor_herbivores(neighbors, &self.params);

                if n_same_herbivores == 0 || n_predators > 0 {
                    // Cell remains empty
                    Cell::Empty
                } else {
                    // Cell may become the neighborhood's most common herbivore by breeding
                    let random = self.rng.gen::<f32>();
                    let cell_birth_rate = self
                        .params
                        .get_specie_by_id(dominant_herbivore_id)
                        .birth_rate;

                    if random < (1.0f32 - cell_birth_rate).powf(n_same_herbivores as f32) {
                        // Cell becomes herbivore by breeding
                        Cell::Animal(dominant_herbivore_id)
                    } else {
                        Cell::Empty
                    }
                }
            }
        }
    }
}

impl Model for SimpleModel {
    fn populate(&mut self) {
        self.grid.populate(&self.params);
    }

    fn tick(&mut self) {
        let grid_size = self.grid.get_size();
        let mut new_cells = Grid::new(grid_size);

        for x in 0..grid_size.w {
            for y in 0..grid_size.h {
                let cell = self.grid.get_cell_at(x, y).clone();
                let mut neighbors: Vec<Cell> = vec![];

                {
                    neighbors = self
                        .grid
                        .moore_neighborhood(x, y, 1, None)
                        .iter()
                        .map(|cell| (*cell).clone())
                        .collect();
                }

                let new_cell = self.next_cell_state(&cell, &neighbors);
                new_cells.set_cell_at(x, y, new_cell);
            }
        }

        println!("tick");

        self.grid = new_cells;
    }

    fn get_grid(&self) -> &Grid {
        &self.grid
    }

    fn get_params(&self) -> &ModelParams {
        &self.params
    }
}
