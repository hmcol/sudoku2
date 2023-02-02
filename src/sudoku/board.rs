use std::collections::{HashMap, HashSet};

use log::error;

use super::{Candidate, Cell, Digit};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CellData {
    Digit(Digit),
    Notes(HashSet<Digit>),
}

impl Default for CellData {
    fn default() -> CellData {
        CellData::Notes(HashSet::from_iter(Digit::list()))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    cell_data: HashMap<Cell, CellData>,
    givens: HashSet<Cell>,
}

impl Board {
    // constructors ------------------------------------------------------------

    pub fn new() -> Board {
        let cell_data = Cell::list()
            .map(|cell| (cell, CellData::default()))
            .collect();

        Board {
            cell_data,
            givens: HashSet::new(),
        }
    }

    pub fn from_string(string: &str) -> Board {
        let mut cell_data = HashMap::new();
        let mut givens = HashSet::new();

        for (i, cell) in Cell::list().enumerate() {
            let digit = string.get(i..(i + 1)).and_then(|s| s.parse().ok());

            if digit.is_some() {
                givens.insert(cell);
            }

            let data2 = match digit {
                Some(digit) => CellData::Digit(digit),
                None => CellData::default(),
            };

            cell_data.insert(cell, data2);
        }

        Board { cell_data, givens }
    }

    // cell getters ------------------------------------------------------------

    pub fn get_data(&self, cell: &Cell) -> &CellData {
        let Some(data) = self.cell_data.get(cell) else {
            panic!("Cell {cell} not found in board");
        };

        data
    }

    pub fn get_data_mut(&mut self, cell: &Cell) -> &mut CellData {
        let Some(data) = self.cell_data.get_mut(cell) else {
            panic!("Cell {cell} not found in board");
        };

        data
    }

    pub fn get_digit(&self, cell: &Cell) -> Option<Digit> {
        match self.get_data(cell) {
            CellData::Digit(digit) => Some(*digit),
            CellData::Notes(_) => None,
        }
    }

    pub fn get_notes(&self, cell: &Cell) -> Option<&HashSet<Digit>> {
        match self.get_data(cell) {
            CellData::Digit(_) => None,
            CellData::Notes(notes) => Some(notes),
        }
    }

    pub fn get_notes_mut(&mut self, cell: &Cell) -> Option<&mut HashSet<Digit>> {
        match self.get_data_mut(cell) {
            CellData::Digit(_) => None,
            CellData::Notes(notes) => Some(notes),
        }
    }

    // cell checkers -----------------------------------------------------------

    pub fn is_digit(&self, cell: &Cell) -> bool {
        matches!(self.get_data(cell), CellData::Digit(_))
    }

    pub fn is_given(&self, cell: &Cell) -> bool {
        self.givens.contains(cell)
    }

    pub fn is_notes(&self, cell: &Cell) -> bool {
        matches!(self.get_data(cell), CellData::Notes(_))
    }

    pub fn has_note(&self, cell: &Cell, digit: Digit) -> bool {
        match self.get_notes(cell) {
            Some(notes) => notes.contains(&digit),
            None => false,
        }
    }

    // mutators ----------------------------------------------------------------

    pub fn reset(&mut self) {
        for (cell, data) in self.cell_data.iter_mut() {
            if self.givens.contains(cell) {
                continue;
            }

            *data = CellData::default();
        }
    }

    pub fn input_solution(&mut self, candidate: Candidate) {
        let (cell, digit) = candidate.as_tuple();

        self.cell_data.insert(cell, CellData::Digit(digit));
    }

    pub fn input_elimination(&mut self, candidate: Candidate) {
        let (cell, digit) = candidate.as_tuple();

        let Some(notes) = self.get_notes_mut(&cell) else {
            error!("Cell {cell} is not a notes cell");
            return;
        };

        if !notes.remove(&digit) {
            error!("Cell {cell} does not contain note {digit}");
        }
    }

    // iterators ---------------------------------------------------------------

    pub fn iter_unsolved_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        Cell::list().filter(|cell| matches!(self.get_data(cell), CellData::Notes(_)))
    }
}
