use crate::models::{Grid, ModelParams};
use std::fs::File;
use std::io::*;

// Statistics writer. Collects statistics from a model and writes them to a CSV file.
pub struct Stats {
    file_path: String,
    file_handle: Option<File>
}

impl Stats {
    pub fn new(file_path: &str) -> Stats {
        Stats { file_path: file_path.to_string(), file_handle: None }
    }

    pub fn reset(&mut self) {
        self.file_handle = None;
    }

    pub fn collect(&mut self, ticks_elapsed: usize, grid: &Grid, params: &ModelParams) -> Result<()> {
        if self.file_handle.is_none() {
            let mut file_handle = std::fs::File::create(&self.file_path)?;

            let mut line = String::from("Time");

            for specie_name in params.species.keys() {
                line.push_str(",");
                line.push_str(specie_name);
            }

            // Construct CSV header
            writeln!(file_handle, "{}", line);

            self.file_handle = Some(file_handle);
        }

        match &mut self.file_handle {
            Some( file_handle) => {
                let mut line = String::from(ticks_elapsed.to_string());

                let cell_specie_ids = grid.get_cell_specie_ids();

                for specie_name in params.species.keys() {
                    let specie_id = params.specie_id_from_name(specie_name);
                    
                    let specie_count = cell_specie_ids.iter().filter(|id| **id == specie_id).count();

                    line.push_str(",");
                    line.push_str(&specie_count.to_string());
                }

                // Write CSV row
                writeln!(file_handle, "{}", line);

                Ok(())
            },
            None => {Ok(())}
        }
    }
}