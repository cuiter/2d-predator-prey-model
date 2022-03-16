use crate::util::{time_ns, Size};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::collections::HashMap;

pub mod params;
pub use params::{ModelParams, ModelType};

pub mod grid;
pub use grid::{Cell, Grid, Quadrant};

mod simple_model;
use simple_model::SimpleModel;

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

pub mod utils {
    use super::*;

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
    pub fn get_neighbor_predators(
        cell: &Cell,
        neighbors: &Vec<Cell>,
        params: &ModelParams,
    ) -> (u32, u32) {
        match cell {
            &Cell::Animal(specie_id) => {
                let predating_neighbors: Vec<&Cell> = neighbors
                    .iter()
                    .filter(|neighbor| match neighbor {
                        Cell::Animal(neighbor_specie_id) => {
                            params.is_specie_predator_for(*neighbor_specie_id, specie_id)
                        }
                        Cell::Empty => false,
                    })
                    .collect();
                let n_predators = predating_neighbors.len() as u32;

                let (_, dominant_predator_id) = most_occurring_neighbor(&predating_neighbors);

                (n_predators, dominant_predator_id)
            }
            &Cell::Empty => (0, 0),
        }
    }

    /// Returns the amount of same-specie herbivores in the neighborhood and the specie id of the most prevalent herbivore.
    pub fn get_neighbor_herbivores(neighbors: &Vec<Cell>, params: &ModelParams) -> (u32, u32) {
        let herbivore_neighbors: Vec<&Cell> = neighbors
            .iter()
            .filter(|neighbor| match neighbor {
                Cell::Animal(neighbor_specie_id) => params.is_specie_herbivore(*neighbor_specie_id),
                Cell::Empty => false,
            })
            .collect();

        most_occurring_neighbor(&herbivore_neighbors)
    }
}
pub use utils::*;
