use super::{
    cell::CellData,
    pos::{Cell, CELLS_BY_BOX, CELLS_BY_COL, CELLS_BY_ROW},
};
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Board {
    cells: HashMap<Cell, CellData>,
}

impl Board {
    pub fn new() -> Board {
        let mut cells = HashMap::new();

        for id in Cell::list() {
            cells.insert(id, CellData::new(id));
        }

        Board { cells }
    }

    pub fn from_string(string: &str) -> Board {
        let mut cells = HashMap::new();

        for cell in Cell::list() {
            let index = cell.as_index();

            let digit = string.get(index..(index + 1)).and_then(|s| s.parse().ok());

            let data = match digit {
                Some(digit) => CellData::new_given(cell, digit),
                None => CellData::new(cell),
            };

            cells.insert(cell, data);
        }

        Board { cells }
    }

    pub fn get_data(&self, cell: Cell) -> &CellData {
        self.cells
            .get(&cell)
            .unwrap_or_else(|| panic!("Cell {cell} not found"))
    }

    pub fn cells(&self) -> impl Iterator<Item = &CellData> {
        Cell::list().map(|id| self.get_data(id))
    }

    pub fn rows(&self) -> [[&CellData; 9]; 9] {
        CELLS_BY_ROW.map(|id_row| id_row.map(|id| self.get_data(id)))
    }

    pub fn cols(&self) -> [[&CellData; 9]; 9] {
        CELLS_BY_COL.map(|id_col| id_col.map(|id| self.get_data(id)))
    }

    pub fn boxes(&self) -> [[&CellData; 9]; 9] {
        CELLS_BY_BOX.map(|id_box| id_box.map(|id| self.get_data(id)))
    }
}
