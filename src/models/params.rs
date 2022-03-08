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
    // Returns the mapping from specie name -> specie id
    pub fn specie_ids(&self) -> BiMap<String, u32> {
        let mut specie_ids = BiMap::new();
        for (index, specie_name) in self.species.keys().enumerate() {
            specie_ids.insert(specie_name.clone(), index as u32 + 1);
        }

        specie_ids
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
