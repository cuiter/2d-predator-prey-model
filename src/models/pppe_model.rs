use crate::models::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashMap;

pub struct PPPEModel {
    grid: Grid,
    params: ModelParams,
    rng: Pcg32,
}

impl PPPEModel {
    pub fn new(params: ModelParams) -> PPPEModel {
        let grid = Grid::new(params.grid_size);
        let rng = Pcg32::seed_from_u64(params.random_seed.unwrap_or(time_ns() as u64));

        PPPEModel { grid, params, rng }
    }

    fn feeding_phase_next_cell_state(&mut self, cell: &Cell, neighbors: &Vec<Cell>) -> (Cell, bool) {
        match cell {
            Cell::Animal(specie_id) => {
                let (n_predators, _) = get_neighbor_predators(cell, neighbors, &self.params);
                if self.params.is_specie_herbivore(*specie_id) || n_predators > 0 {
                    // Cell is prey
                    let random = self.rng.gen::<f32>();
                    let prey_death_rate = self.params.get_specie_by_id(*specie_id).death_rate;
                    
                    if random < (1.0f32 - prey_death_rate).powf(n_predators as f32) {
                        // Hunt failed/no predators, cell stays prey
                        (Cell::Animal(*specie_id), false)
                    } else {
                        // Cell becomes empty due to kill
                        (Cell::Empty, true)
                    }
                } else {
                    // Cell is predator
                    let random = self.rng.gen::<f32>();
                    let (n_prey, most_occurring_prey_id) = get_neighbor_prey(cell, neighbors, &self.params);
                    let prey_death_rate = if n_prey == 0 { 0.0 } else { self.params.get_specie_by_id(most_occurring_prey_id).death_rate };

                    if random < (1.0f32 - prey_death_rate).powf(n_prey as f32) {
                        // Hunt fails, predator stays unfed.
                        (Cell::Animal(*specie_id), false)
                    } else {
                        // Hunt succeeded, predator gets fed.
                        (Cell::Animal(*specie_id), true)
                    }
                }
            },
            Cell::Empty => { (Cell::Empty, false) }
        }
    }

    fn feeding_phase(&mut self) -> (Grid, Vec<bool>) {
        let grid_size = self.grid.get_size();
        let mut new_cells = Grid::new(grid_size);
        let mut cells_fed_or_killed = vec![];

        for x in 0..grid_size.w {
            for y in 0..grid_size.h {
                let cell = self.grid.get_cell_at(x, y).clone();
                let mut neighbors: Vec<Cell> = vec![];

                {
                    neighbors = self
                        .grid
                        .von_neumann_neighborhood_r1(x, y)
                        .iter()
                        .map(|cell| (*cell).clone())
                        .collect();
                }

                let (new_cell, fed_or_killed) = self.feeding_phase_next_cell_state(&cell, &neighbors);
                new_cells.set_cell_at(x, y, new_cell);
                cells_fed_or_killed.push(fed_or_killed);
            }
        }

        (new_cells, cells_fed_or_killed)
    }

    fn reproduction_phase(&mut self, cells: &Grid, cells_fed_or_killed: &Vec<bool>) -> Grid {
        todo!();
    }

    fn movement_phase(&mut self, cells: &Grid) -> Grid {
        todo!();
    }
}

impl Model for PPPEModel {
    fn populate(&mut self) {
        self.grid.populate(&self.params);
    }

    fn tick(&mut self) {
        let grid_size = self.grid.get_size();

        // Feeding phase
        let (cells_after_feed, cells_fed_or_killed) = self.feeding_phase();
        //let cells_after_reproduction = self.reproduction_phase(&cells_after_feed, &cells_fed_or_killed);
        //let cells_after_movement = self.movement_phase(&cells_after_reproduction);

        println!("tick");

        self.grid = cells_after_feed;
    }

    fn get_grid(&self) -> &Grid {
        &self.grid
    }

    fn get_params(&self) -> &ModelParams {
        &self.params
    }
}
