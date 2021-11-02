mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: cualquier celda con menos de 2 vecinos
                    // vivos muere, por la regla de subpoblacion
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Cualquier celda con 2 o 3 vecinos vivos
                    // vive en la siguiente generacion
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Cualquier celda con mas de 3 vecinos vivos
                    // muere por sobrepoblacion
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Cualquier celda muerta con exactamente 3
                    // vecinos vivos, revive en la siguiente generacion
                    (Cell::Dead, 3) => Cell::Alive,
                    // todas las demas celdas se mantienen igual
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width - 1, 0, 1].iter().cloned() {
                if delta_column == 0 && delta_row == 0 {
                    // si estan muertos, continua
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_column);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 32;
        
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}