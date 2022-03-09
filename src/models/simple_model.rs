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

    /// Returns the count and specie ID of the most occurring neighboring species.
    fn most_occurring_neighbor(neighbors: &Vec<&Cell>) -> (u32, u32) {
        if neighbors.len() > 0 {
            let mut count_by_specie = HashMap::new();
            for neighbor in neighbors {
                match neighbor {
                    Cell::Animal(neighbor_specie_id) => {
                        if (!count_by_specie.contains_key(neighbor_specie_id)) {
                            count_by_specie.insert(neighbor_specie_id, 1u32);
                        } else {
                            count_by_specie.insert(
                                neighbor_specie_id,
                                count_by_specie[neighbor_specie_id] + 1u32,
                            );
                        }
                    }
                    Cell::Empty => {}
                }
            }
            let mut count_by_specie_vec: Vec<(&&u32, &u32)> = count_by_specie.iter().collect();
            count_by_specie_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); // Sort by count descending

            let specie_id = **count_by_specie_vec[0].0;
            let count = *count_by_specie_vec[0].1;

            (count, specie_id)
        } else {
            (0, 0)
        }
    }

    /// Returns the amount of predators in the neighborhood and the specie id of the most prevalent predator.
    fn get_neighbor_predators(&self, cell: &Cell, neighbors: &Vec<Cell>) -> (u32, u32) {
        match cell {
            &Cell::Animal(specie_id) => {
                let predating_neighbors: Vec<&Cell> = neighbors
                    .iter()
                    .filter(|neighbor| match neighbor {
                        Cell::Animal(neighbor_specie_id) => self
                            .params
                            .is_specie_predator_for(*neighbor_specie_id, specie_id),
                        Cell::Empty => false,
                    })
                    .collect();
                let n_predators = predating_neighbors.len() as u32;

                let (_, dominant_predator_id) =
                    SimpleModel::most_occurring_neighbor(&predating_neighbors);

                (n_predators, dominant_predator_id)
            }
            &Cell::Empty => (0, 0),
        }
    }

    /// Returns the amount of same-specie herbivores in the neighborhood and the specie id of the most prevalent herbivore.
    fn get_neighbor_herbivores(&self, neighbors: &Vec<Cell>) -> (u32, u32) {
        let herbivore_neighbors: Vec<&Cell> = neighbors
            .iter()
            .filter(|neighbor| match neighbor {
                Cell::Animal(neighbor_specie_id) => {
                    self.params.is_specie_herbivore(*neighbor_specie_id)
                }
                Cell::Empty => false,
            })
            .collect();

        SimpleModel::most_occurring_neighbor(&herbivore_neighbors)
    }

    /// Determines the next state of the given cell, given the current state and the cell's surrounding neighbors.
    fn next_cell_state(&mut self, cell: &Cell, neighbors: &Vec<Cell>) -> Cell {
        let (n_predators, dominant_predator_id) = self.get_neighbor_predators(cell, neighbors);

        match cell {
            &Cell::Animal(specie_id) => {
                /*let specie_params = self.params.get_specie_by_id(specie_id);
                let cell_birth_rate = specie_params.birth_rate;
                let cell_death_rate = specie_params.death_rate;

                if self.params.is_specie_herbivore(specie_id) || n_predators > 0 {
                    // Cell is prey

                    let random_1 = rng.gen::<f32>();

                    Cell::Empty
                    /*if random_1 < (1.0f32 - cell_death_rate).pow(n_predators.count()) {
                        // Hunt failed. Note: if cell is predator, should die if not the case
                    } else {
                        let random_2 = self.rng.gen::<f32>();
                        //    cell becomes predator by breeding
                    }*/
                } else {
                    // Cell is a predator
                    let random = self.rng.gen::<f32>();
                    if random < cell_death_rate {
                        // Cell becomes empty due to predator death
                        Cell::Empty
                    } else {
                        // Cell remains predator
                        cell.clone()
                    }
                }*/
                Cell::Animal(specie_id)
            }
            &Cell::Empty => {
                let (n_same_herbivores, dominant_herbivore_id) =
                    self.get_neighbor_herbivores(neighbors);

                if n_same_herbivores == 0 || n_predators > 0 {
                    // Cell remains empty
                    Cell::Empty
                } else {
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
                        .moore_neighborhood(x, y, 1)
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
