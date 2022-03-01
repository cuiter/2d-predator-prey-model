use serde::{Deserialize, Serialize};
use crate::util::Size;

#[derive(Clone, Serialize, Deserialize)]
pub enum ModelType
{
    Simple,
    PPPE,
    DSAM,
    Custom
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpecieParams
{
    name: String,
    color: Option<String>, // (optional) Cell color of the specie, in hex format (e.g. FF0000)
    initial_population: f32, // Initial population on the grid divided by the total amount of cells
    birth_rate: f32, // Probability of birth in a given time step (depending on neighboring cells)
    death_rate: f32, // Probability of death in a given time step (depending on neighboring cells)
    energy_sources: Option<Vec<String>> // (optional) Other species that may be used as an energy source (predator-prey relationship)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModelParams {
    model_type: ModelType,
    species: Vec<SpecieParams>,

    grid_size: Size,
    random_seed: Option<u64>,
}