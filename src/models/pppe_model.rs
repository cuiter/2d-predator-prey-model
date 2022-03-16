use crate::models::*;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::BTreeMap;

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

    fn feeding_phase_next_cell_state(
        &mut self,
        cell: &Cell,
        neighbors: &Vec<Cell>,
    ) -> (Cell, bool) {
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
                    let (n_prey, most_occurring_prey_id) =
                        get_neighbor_prey(cell, neighbors, &self.params);
                    let prey_death_rate = if n_prey == 0 {
                        0.0
                    } else {
                        self.params
                            .get_specie_by_id(most_occurring_prey_id)
                            .death_rate
                    };

                    if random < (1.0f32 - prey_death_rate).powf(n_prey as f32) {
                        // Hunt fails, predator stays unfed.
                        (Cell::Animal(*specie_id), false)
                    } else {
                        // Hunt succeeded, predator gets fed.
                        (Cell::Animal(*specie_id), true)
                    }
                }
            }
            Cell::Empty => (Cell::Empty, false),
        }
    }

    fn feeding_phase(&mut self) -> (Grid, Vec<bool>) {
        let grid_size = self.grid.get_size();
        let mut new_cells = Grid::new(grid_size);
        let mut cells_fed_or_killed = vec![];

        for x in 0..grid_size.w {
            for y in 0..grid_size.h {
                let cell = self.grid.get_cell_at(x, y).clone();
                let neighbors = self.grid.von_neumann_neighborhood_r1(x, y);
                let (new_cell, fed_or_killed) =
                    self.feeding_phase_next_cell_state(&cell, &neighbors);
                new_cells.set_cell_at(x, y, new_cell);
                cells_fed_or_killed.push(fed_or_killed);
            }
        }

        (new_cells, cells_fed_or_killed)
    }

    fn reproduction_phase_next_cell_state(
        &mut self,
        cell: &Cell,
        fed_or_killed: bool,
        neighbors: &Vec<Cell>,
        neighbor_cells_fed_or_killed: &Vec<bool>,
    ) -> Cell {
        match cell {
            Cell::Animal(specie_id) => {
                let (n_predators, _) = get_neighbor_predators(cell, neighbors, &self.params);
                if self.params.is_specie_herbivore(*specie_id) || n_predators > 0 {
                    // Cell is herbivore, stays herbivore
                    Cell::Animal(*specie_id)
                } else {
                    // Cell is a predator
                    let random = self.rng.gen::<f32>();
                    let death_rate = self.params.get_specie_by_id(*specie_id).death_rate;

                    if random < death_rate {
                        // The predator dies, the cell is now empty.
                        Cell::Empty
                    } else {
                        // The predator lives.
                        Cell::Animal(*specie_id)
                    }
                }
            }
            Cell::Empty => {
                if !fed_or_killed {
                    // Cell was already empty
                    let (n_herbivores, most_occurring_herbivore_id) =
                        get_neighbor_herbivores(neighbors, &self.params);
                    let (n_predators, most_occurring_predator_id) =
                        get_neighbor_predators(cell, neighbors, &self.params);
                    if n_herbivores == 0 || n_predators > 0 {
                        // Cell remains empty
                        Cell::Empty
                    } else {
                        let prey_birth_rate = self
                            .params
                            .get_specie_by_id(most_occurring_herbivore_id)
                            .birth_rate;
                        let random = self.rng.gen::<f32>();
                        if random < (1.0f32 - prey_birth_rate).powf(n_herbivores as f32) {
                            // Cell becomes prey by breeding
                            Cell::Animal(most_occurring_herbivore_id)
                        } else {
                            // Cell remains empty
                            Cell::Empty
                        }
                    }
                } else {
                    // Cell was empty due to a kill
                    let mut fed_or_killed_neighbors = vec![];
                    for i in 0..neighbor_cells_fed_or_killed.len() {
                        if neighbor_cells_fed_or_killed[i] {
                            fed_or_killed_neighbors.push(neighbors[i].clone());
                        }
                    }

                    let (n_fed_predators, most_occurring_predator_id) =
                        get_neighbor_predators(cell, &fed_or_killed_neighbors, &self.params);
                    let predator_birth_rate = if n_fed_predators == 0 {
                        0.0
                    } else {
                        self.params
                            .get_specie_by_id(most_occurring_predator_id)
                            .birth_rate
                    };

                    let random = self.rng.gen::<f32>();

                    if random < (1.0f32 - predator_birth_rate).powf(n_fed_predators as f32) {
                        // No reproduction occurs, cell remains empty
                        Cell::Empty
                    } else {
                        // Cell becomes predator
                        Cell::Animal(most_occurring_predator_id)
                    }
                }
            }
        }
    }

    fn reproduction_phase(&mut self, fed_cells: &Grid, cells_fed_or_killed: &Vec<bool>) -> Grid {
        let grid_size = fed_cells.get_size();
        let mut new_cells = Grid::new(grid_size);

        for x in 0..grid_size.w {
            for y in 0..grid_size.h {
                let cell = fed_cells.get_cell_at(x, y).clone();
                let fed_or_killed = cells_fed_or_killed[(y * grid_size.w + x) as usize];
                let neighbors = self.grid.von_neumann_neighborhood_r1(x, y);

                let mut neighbor_cells_fed_or_killed: Vec<bool> = vec![];
                if x > 0 {
                    neighbor_cells_fed_or_killed
                        .push(cells_fed_or_killed[(y * grid_size.w + x - 1) as usize]);
                }
                if y > 0 {
                    neighbor_cells_fed_or_killed
                        .push(cells_fed_or_killed[((y - 1) * grid_size.w + x) as usize]);
                }
                if x < grid_size.w - 1 {
                    neighbor_cells_fed_or_killed
                        .push(cells_fed_or_killed[(y * grid_size.w + x + 1) as usize]);
                }
                if y < grid_size.h - 1 {
                    neighbor_cells_fed_or_killed
                        .push(cells_fed_or_killed[((y + 1) * grid_size.w + x) as usize]);
                }

                let new_cell = self.reproduction_phase_next_cell_state(
                    &cell,
                    fed_or_killed,
                    &neighbors,
                    &neighbor_cells_fed_or_killed,
                );
                new_cells.set_cell_at(x, y, new_cell);
            }
        }

        new_cells
    }

    fn movement_phase(&mut self, cells: &Grid) -> Grid {
        let grid_size = self.grid.get_size();
        let quadrants = vec![
            Quadrant::East,
            Quadrant::North,
            Quadrant::West,
            Quadrant::South,
        ];

        let mut competition_list: Vec<(u32, u32, u32, u32)> = vec![];

        for x in 0..grid_size.w {
            for y in 0..grid_size.h {
                let cell = self.grid.get_cell_at(x, y).clone();

                let neighbors = self
                    .grid
                    .moore_neighborhood(x, y, self.params.sense_radius, None);
                let neighbors_by_quatrant: BTreeMap<Quadrant, Vec<Cell>> = quadrants
                    .iter()
                    .map(|quadrant| {
                        (
                            *quadrant,
                            self.grid.moore_neighborhood(
                                x,
                                y,
                                self.params.sense_radius,
                                Some(*quadrant),
                            ),
                        )
                    })
                    .filter(|(_, neighbors)| neighbors.len() > 0) // Do not consider quadrants that contain no cells
                    .collect();
                let possible_quadrants: Vec<Quadrant> = neighbors_by_quatrant
                    .keys()
                    .map(|quadrant| *quadrant)
                    .collect();

                match cell {
                    Cell::Animal(specie_id) => {
                        let (n_predators, most_occurring_predator_id) =
                            get_neighbor_predators(&cell, &neighbors, &self.params);
                        let (n_prey, most_occurring_prey_id) =
                            get_neighbor_prey(&cell, &neighbors, &self.params);

                        let mut intent = None;

                        if n_predators > 0 {
                            // Cell is prey
                            let mut n_predators_by_quadrant: Vec<(Quadrant, u32)> =
                                possible_quadrants
                                    .iter()
                                    .map(|quadrant| {
                                        (
                                            *quadrant,
                                            get_neighbor_predators(
                                                &cell,
                                                &neighbors_by_quatrant[quadrant],
                                                &self.params,
                                            )
                                            .0,
                                        )
                                    })
                                    .collect();
                            n_predators_by_quadrant.sort_by(
                                |(_, a_n_predators), (_, b_n_predators)| {
                                    a_n_predators.partial_cmp(b_n_predators).unwrap()
                                },
                            );

                            if n_predators_by_quadrant.len() > 0 {
                                // Intent towards quadrant with least amount of predators
                                // Note: may be improved by choosing randomly between the optimal quadrants, but is not necessary
                                intent = Some(n_predators_by_quadrant[0].0);
                            }
                        } else if !self.params.is_specie_herbivore(specie_id) {
                            // Cell is predator
                            let mut n_prey_by_quadrant: Vec<(Quadrant, u32)> = possible_quadrants
                                .iter()
                                .map(|quadrant| {
                                    (
                                        *quadrant,
                                        get_neighbor_prey(
                                            &cell,
                                            &neighbors_by_quatrant[quadrant],
                                            &self.params,
                                        )
                                        .0,
                                    )
                                })
                                .collect();
                            n_prey_by_quadrant.sort_by(|(_, a_n_prey), (_, b_n_prey)| {
                                a_n_prey.partial_cmp(b_n_prey).unwrap()
                            });

                            if n_prey_by_quadrant.len() > 0 {
                                // Intent towards quadrant with most amount of prey
                                // Note: may be improved by choosing randomly between the optimal quadrants, but is not necessary
                                intent = Some(n_prey_by_quadrant.last().unwrap().0);
                            } else {
                                // Choose random direction
                                intent = Some(
                                    possible_quadrants
                                        [self.rng.gen_range(0, possible_quadrants.len())],
                                );
                            }
                        } else {
                            // Cell is prey, remains stationary
                        }

                        match intent {
                            Some(Quadrant::East) => {
                                competition_list.push((x, y, x + 1, y));
                            }
                            Some(Quadrant::North) => {
                                competition_list.push((x, y, x, y - 1));
                            }
                            Some(Quadrant::West) => {
                                competition_list.push((x, y, x - 1, y));
                            }
                            Some(Quadrant::South) => {
                                competition_list.push((x, y, x, y + 1));
                            }
                            None => {}
                        }
                    }
                    Cell::Empty => {}
                }
            }
        }

        let mut competition_map: BTreeMap<(u32, u32), Vec<(u32, u32)>> = BTreeMap::new();

        for (x_from, y_from, x_to, y_to) in competition_list.iter() {
            if cells.get_cell_at(*x_to, *y_to) == &Cell::Empty {
                if !competition_map.contains_key(&(*x_to, *y_to)) {
                    competition_map.insert((*x_to, *y_to), vec![]);
                }
                competition_map
                    .get_mut(&(*x_to, *y_to))
                    .unwrap()
                    .push((*x_from, *y_from));
            }
        }

        let mut new_cells = cells.clone();

        for ((x_to, y_to), candidates) in competition_map.iter() {
            let random = self.rng.gen_range(0, candidates.len());
            let (x_from, y_from) = candidates[random];
            let cell = new_cells.get_cell_at(x_from, y_from);
            new_cells.set_cell_at(*x_to, *y_to, cell.clone());
            new_cells.set_cell_at(x_from, y_from, Cell::Empty);
        }

        new_cells
    }
}

impl Model for PPPEModel {
    fn populate(&mut self) {
        self.grid.populate(&self.params, &mut self.rng);
    }

    fn tick(&mut self) {
        let grid_size = self.grid.get_size();

        // Feeding phase
        let (cells_after_feed, cells_fed_or_killed) = self.feeding_phase();
        let cells_after_reproduction =
            self.reproduction_phase(&cells_after_feed, &cells_fed_or_killed);
        let cells_after_movement = self.movement_phase(&cells_after_reproduction);

        println!("tick");

        self.grid = cells_after_movement;
    }

    fn get_grid(&self) -> &Grid {
        &self.grid
    }

    fn get_params(&self) -> &ModelParams {
        &self.params
    }
}
