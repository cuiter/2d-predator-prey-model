use crate::util::{time_ns, Size};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::models::params::{ModelParams, ModelType};

#[derive(Clone, PartialEq)]
pub enum Cell {
    Empty,
    Animal(u32),
}

#[derive(Clone)]
pub struct Grid {
    size: Size,
    cells: Vec<Cell>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Quadrant {
    West,
    North,
    East,
    South,
}

impl Grid {
    pub fn new(size: Size) -> Grid {
        Grid {
            size: size,
            cells: vec![Cell::Empty; size.w as usize * size.h as usize],
        }
    }

    pub fn populate(&mut self, params: &ModelParams, rng: &mut Pcg32) {
        let specie_ids = params.specie_ids();

        for (specie_name, specie_params) in params.species.iter() {
            let specie_id = specie_ids.get_by_left(&specie_name).unwrap();
            let target_population =
                (specie_params.initial_population * self.size.w as f32 * self.size.h as f32) as u32;
            let mut population = 0;
            while population < target_population {
                let new_x = rng.gen_range(0, self.size.w as i32);
                let new_y = rng.gen_range(0, self.size.h as i32);

                if self.get_cell_at(new_x, new_y, false) == &Cell::Empty {
                    self.set_cell_at(new_x, new_y, Cell::Animal(*specie_id), false);
                    population += 1;
                }
            }
        }
    }

    /// Calculates the Moore neighborhood M around the cell at (x, y).
    /// This is also known as the surrounding cells in the square with the given (border) radius.
    /// A radius of 1 means a square shape of size (3, 3), excluding the middle cell.
    ///
    /// Optionally filters the neighbors down to a specific quadrant (north, east, west or south).
    /// Note that the bordering cells between quadrants may be included in multiple quadrants.
    pub fn moore_neighborhood(
        &self,
        x: u32,
        y: u32,
        radius: u32,
        quadrant: Option<Quadrant>,
        wrap_edges: bool
    ) -> Vec<Cell> {
        let mut neighbors = Vec::with_capacity((radius * radius - 1) as usize);

        let i_radius = radius as i32;

        for i in -i_radius..(i_radius + 1) {
            for j in -i_radius..(i_radius + 1) {
                if i == 0 && j == 0 {
                    continue;
                }

                if wrap_edges ||
                       ((x as i32 + i) >= 0
                     && (x as i32 + i) < self.size.w as i32
                     && (y as i32 + j) >= 0
                     && (y as i32 + j) < self.size.h as i32)
                {
                    let inside_quadrant = match quadrant {
                        None => true,
                        Some(Quadrant::East) => i > 0 && j >= -i && j <= i,
                        Some(Quadrant::North) => j < 0 && i >= j && i <= -j,
                        Some(Quadrant::West) => i < 0 && j >= i && j <= -i,
                        Some(Quadrant::South) => j > 0 && i >= -j && i <= j,
                    };

                    if inside_quadrant {
                        neighbors.push(
                            self.get_cell_at(x as i32 + i, y as i32 + j, wrap_edges)
                                .clone(),
                        );
                    }
                }
            }
        }

        neighbors
    }

    /// Calculates the Von Neumann neighborhood around the cell at (x, y) with radius 1.
    pub fn von_neumann_neighborhood_r1(&self, x: u32, y: u32, wrap_edges: bool) -> Vec<Cell> {
        let mut neighbors = Vec::with_capacity(4);

        if wrap_edges || x > 0 {
            neighbors.push(self.get_cell_at(x as i32 - 1, y as i32, wrap_edges).clone());
        }
        if wrap_edges || y > 0 {
            neighbors.push(self.get_cell_at(x as i32, y as i32 - 1, wrap_edges).clone());
        }
        if wrap_edges || x < self.size.w - 1 {
            neighbors.push(self.get_cell_at(x as i32 + 1, y as i32, wrap_edges).clone());
        }
        if wrap_edges || y < self.size.h - 1 {
            neighbors.push(self.get_cell_at(x as i32, y as i32 + 1, wrap_edges).clone());
        }

        neighbors
    }

    #[inline]
    pub fn get_cell_at(&self, x: u32, y: u32, wrap_edges: bool) -> &Cell {
        if wrap_edges {
            let size_w = self.size.w as i32;
            let size_h = self.size.h as i32;
            let wrapped_x = ((x % size_w) + size_w) % size_w;
            let wrapped_y = ((y % size_h) + size_h) % size_h;

            &self.cells[wrapped_x as usize + wrapped_y as usize * self.size.w as usize]
        } else {
            &self.cells[x as usize + y as usize * self.size.w as usize]
        }
    }

    #[inline]
    pub fn set_cell_at(&mut self, x: u32, y: u32, cell: Cell, wrap_edges: bool) {
        if wrap_edges {
            let size_w = self.size.w as i32;
            let size_h = self.size.h as i32;
            let wrapped_x = ((x % size_w) + size_w) % size_w;
            let wrapped_y = ((y % size_h) + size_h) % size_h;

            self.cells[wrapped_x as usize + wrapped_y as usize * self.size.w as usize] = cell;
        } else {
            self.cells[x as usize + y as usize * self.size.w as usize] = cell;
        }
    }

    pub fn get_cell_specie_ids(&self) -> Vec<u32> {
        self.cells
            .iter()
            .map(|cell| match cell {
                Cell::Empty => 0,
                Cell::Animal(specie_id) => *specie_id,
            })
            .collect()
    }

    pub const fn get_size(&self) -> Size {
        self.size
    }
}
