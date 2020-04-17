/// This file contains world's logic



use crate::random::*;



#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub fn init (&self) {
        //
    }
}

impl PartialEq for Cell {
    fn eq (&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}



pub struct World {
    pub cells: Vec<Cell>,
    pub zoom: f32,
}

impl World {
    pub fn set_cells (&mut self, _cells: Vec<Cell>) {
        self.cells = _cells;
    }

    // pub fn get_cells (&self) -> Vec<Cell> {
    //     self.cells
    // }

    pub fn init_random (&mut self, bounds: u32, amount_of_cells: u32) {
        let mut new_cells: Vec<Cell> = vec![];

        let mut rnd_x: i32;
        let mut rnd_y: i32;

        for _ in 0..amount_of_cells {
            rnd_x = random_i32( -(bounds as i32), bounds as i32 );
            rnd_y = random_i32( -(bounds as i32), bounds as i32 );
            // println!("rnd_x = {}, rnd_y = {}", rnd_x, rnd_y);

            new_cells.push(
                Cell{
                    x: rnd_x,
                    y: rnd_y,
                }
            );
        }

        self.set_cells(new_cells);
    }

    pub fn process_old (&mut self) {
        for cell in &mut self.cells {
            cell.x = cell.x + 1;
        }
    }

    pub fn process (&mut self) {
        let mut new_cells: Vec<Cell> = vec![];
        let mut cells_to_check: Vec<Cell> = vec![];

        let check_bounds: i32 = 1;
        let mut tmp_cell: Cell;

        // println!("{:#?}", self.cells);

        for cell in &self.cells {
            for x in cell.x-check_bounds..=cell.x+check_bounds {
                // println!("checking x = {}", x);
                for y in cell.y-check_bounds..=cell.y+check_bounds {
                    // println!("    checking y = {}", y);
                    // println!("checking x={}, y={}", x, y);
                    
                    tmp_cell = Cell{x: x, y: y};

                    if !cells_to_check.contains(&tmp_cell) {
                        // println!("adding x={}, y={}", x, y);
                        // println!("{:?}", cells_to_check);
                        cells_to_check.push(tmp_cell);
                        // println!("{:?}\n", cells_to_check);
                    }

                    // println!("cell = {:?}, x = {}, y = {}", cell, x, y);
                }
            }
        }

        // println!("cells_to_check generated!");
        // println!("Checkinhg this cells: {:#?}", cells_to_check);

        let mut amount_of_neighbor: u8;

        for cell in &cells_to_check {
            amount_of_neighbor = 0;

            for cell_another in &self.cells {
                if (cell.x-cell_another.x).abs() <= 1 && (cell.y-cell_another.y).abs() <= 1 
                        && !(cell.x == cell_another.x && cell.y == cell_another.y) {
                    amount_of_neighbor += 1;
                    // if cell.x == cell_another.x && cell.y == cell_another.y { } else { }
                }
            }

            // println!("for x={}, y={}, amount_of_neighbor = {}", cell.x, cell.y, amount_of_neighbor);

            match amount_of_neighbor {
                0 | 1 => {
                    // to few -> die, so dont add to new_cells
                },
                2 => {      // 2 -> if alive, must be added
                    if self.cells.contains(cell) {
                        new_cells.push(Cell{x: cell.x, y: cell.y});
                    }
                },
                3 => {      // 3 -> must be added
                    // new_cells.push(*cell);
                    new_cells.push(Cell{x: cell.x, y: cell.y});
                },
                _ => {
                    // to many -> so die, so dont add to new_cells
                }
            }
        }

        self.set_cells(new_cells);
    }

    pub fn get_vec_vertices (&self, dx: &f32, dy: &f32, zoom: &f32) -> Vec<f32> {
        let mut vertices: Vec<f32> = vec![];

        let mut cell_x_minus_zoom: f32;
        let mut cell_y_minus_zoom: f32;
        let mut cell_x_plus_zoom: f32;
        let mut cell_y_plus_zoom: f32;

        for cell in &self.cells {
            cell_x_minus_zoom = (cell.x as f32 - 0.5)*zoom + dx;
            cell_y_minus_zoom = (cell.y as f32 - 0.5)*zoom - dy;

            cell_x_plus_zoom = (cell.x as f32 + 0.5)*zoom + dx;
            cell_y_plus_zoom = (cell.y as f32 + 0.5)*zoom - dy;

            let new_vertices: Vec<f32> = vec![
            //       X                  Y              Z      R    G    B
                cell_x_minus_zoom, cell_y_minus_zoom, 0.0,   1.0, 1.0, 1.0,
                cell_x_minus_zoom, cell_y_plus_zoom , 0.0,   1.0, 1.0, 1.0,
                cell_x_plus_zoom , cell_y_plus_zoom , 0.0,   1.0, 1.0, 1.0,
                cell_x_plus_zoom , cell_y_plus_zoom , 0.0,   1.0, 1.0, 1.0,
                cell_x_plus_zoom , cell_y_minus_zoom, 0.0,   1.0, 1.0, 1.0,
                cell_x_minus_zoom, cell_y_minus_zoom, 0.0,   1.0, 1.0, 1.0,
            ];

            for item in &new_vertices {
                vertices.push(*item);
            }
        }

        vertices
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_eq () {
        for x in -10..=10 {
            for y in -10..=10 {
                assert_eq!(
                    Cell{x: x, y: y},
                    Cell{x: x, y: y}
                );
            }
        }
    }

}



