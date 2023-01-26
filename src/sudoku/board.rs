use std::collections::{HashMap, HashSet};

use super::id::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum CellContent {
    Digit(Digit, bool),
    Notes(HashSet<Digit>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Cell {
    pub id: CellId,
    pub content: CellContent,
}

impl Cell {
    fn new(id: CellId) -> Cell {
        let notes = HashSet::from_iter(DIGITS);

        Cell {
            id,
            content: CellContent::Notes(notes),
        }
    }

    fn new_given(id: CellId, digit: Digit) -> Cell {
        Cell {
            id,
            content: CellContent::Digit(digit, true),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Board {
    cells: HashMap<CellId, Cell>,
}

impl Board {
    pub fn new() -> Board {
        let mut cells = HashMap::new();

        for id in CELLS {
            cells.insert(id, Cell::new(id));
        }

        Board { cells }
    }

    pub fn from_string(string: &str) -> Board {
        let mut cells = HashMap::new();

        for (i, &id) in CELLS.iter().enumerate() {
            let digit = string.chars().nth(i).and_then(|c| Digit::try_from(c).ok());

            let cell = match digit {
                Some(digit) => Cell::new_given(id, digit),
                None => Cell::new(id),
            };

            cells.insert(id, cell);
        }

        Board { cells }
    }

    pub fn get_cell(&self, id: CellId) -> &Cell {
        self.cells.get(&id).unwrap()
    }

    pub fn cells(&self) -> [&Cell; 81] {
        CELLS.map(|id| self.get_cell(id))
    }

    pub fn boxes(&self) -> [[&Cell; 9]; 9] {
        BOXES.map(|id_box| id_box.map(|id| self.get_cell(id)))
    }
}
