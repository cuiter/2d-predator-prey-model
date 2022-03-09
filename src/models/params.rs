use crate::util::Size;
use bimap::BiMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ModelType {
    Simple,
    PPPE,
    DSAM,
    Custom,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpecieParams {
    pub color: Option<String>, // (optional) Cell color of the specie, in hex format (e.g. FF0000)
    pub initial_population: f32, // Initial population on the grid divided by the total amount of cells
    pub birth_rate: f32, // Probability of birth in a given time step (depending on neighboring cells)
    pub death_rate: f32, // Probability of death in a given time step (depending on neighboring cells)
    pub energy_sources: Option<Vec<String>>, // (optional) Other species that may be used as an energy source (predator-prey relationship)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModelParams {
    pub model: ModelType,
    // Mapping from specie name -> specie params
    pub species: HashMap<String, SpecieParams>,

    pub grid_size: Size,
    pub random_seed: Option<u64>,
}

impl ModelParams {
    /// Returns the mapping from specie name -> specie id
    pub fn specie_ids(&self) -> BiMap<String, u32> {
        let mut specie_ids = BiMap::new();
        for (index, specie_name) in self.species.keys().enumerate() {
            specie_ids.insert(specie_name.clone(), index as u32 + 1);
        }

        specie_ids
    }

    pub fn specie_id_from_name(&self, specie_name: &str) -> u32 {
        for (index, name) in self.species.keys().enumerate() {
            if name == specie_name {
                return index as u32 + 1;
            }
        }

        panic!("Could not find specie {}", specie_name)
    }

    pub fn specie_name_from_id(&self, specie_id: u32) -> &str {
        for (index, specie_name) in self.species.keys().enumerate() {
            if (index as u32 + 1) == specie_id {
                return specie_name;
            }
        }

        panic!("Could not find specie with id {}", specie_id)
    }

    pub fn get_specie_by_id(&self, specie_id: u32) -> &SpecieParams {
        &self.species[self.specie_name_from_id(specie_id)]
    }

    /// Returns whether the given specie is a herbivore, i.e. does not eat any other species.
    pub fn is_specie_herbivore(&self, specie_id: u32) -> bool {
        self.species[self.specie_name_from_id(specie_id)]
            .energy_sources
            .as_ref()
            .map(|es| es.len())
            .unwrap_or(0)
            == 0
    }

    /// Returns whether the given specie is a predator for the other given specie.
    pub fn is_specie_predator_for(&self, specie_id: u32, other_specie_id: u32) -> bool {
        self.species[self.specie_name_from_id(specie_id)]
            .energy_sources
            .as_ref()
            .map(|es| es.contains(&String::from(self.specie_name_from_id(other_specie_id))))
            .unwrap_or(false)
    }
}

// Loads model parameters from a file. Returns an error if the file could not be read or if the contents are invalid.
pub fn params_from_file(file_path: &str) -> Result<ModelParams, Box<dyn Error>> {
    let file_contents = fs::read_to_string(file_path)?;

    let params: ModelParams = serde_json::from_str(&file_contents)?;

    for (specie_name, specie_params) in params.species.iter() {
        for energy_source in specie_params
            .energy_sources
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
        {
            if !params.species.contains_key(energy_source) {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Energy source {} for species {} does not exist",
                        energy_source, specie_name
                    ),
                )));
            }
        }
    }

    Ok(params)
}
