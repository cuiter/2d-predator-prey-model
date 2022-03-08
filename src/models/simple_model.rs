use crate::models::*;

pub struct SimpleModel {
    grid: Grid,
    params: ModelParams,
}

impl SimpleModel {
    pub fn new(params: ModelParams) -> SimpleModel {
        let grid = Grid::new(params.grid_size);

        SimpleModel { grid, params }
    }
}

impl Model for SimpleModel {
    fn populate(&mut self) {
        self.grid.populate(&self.params);
    }

    fn tick(&mut self) {
        // Model code goes here
        println!("tick");
    }

    fn get_grid(&self) -> &Grid {
        &self.grid
    }

    fn get_params(&self) -> &ModelParams {
        &self.params
    }
}
