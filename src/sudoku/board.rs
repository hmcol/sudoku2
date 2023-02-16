use log::error;

use crate::bitset::{Element, Set};

use super::{Candidate, Cell, Digit};

// =============================================================================

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CellData {
    Digit(Digit),
    Notes(Set<Digit>),
}

impl Default for CellData {
    fn default() -> CellData {
        CellData::Notes(Set::full())
    }
}

// =============================================================================

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    cell_data: [CellData; 81],
    givens: Set<Cell>,
}

impl Board {
    // constructors ------------------------------------------------------------

    pub fn new() -> Board {
        let mut cell_data_vec = Vec::with_capacity(81);

        for _ in Cell::list() {
            cell_data_vec.push(CellData::default());
        }

        let cell_data = cell_data_vec.try_into().unwrap_or_else(|_| {
            panic!("Could not convert `Vec` to `[CellData; 81]` while creating new board.")
        });

        Board {
            cell_data,
            givens: Set::new(),
        }
    }

    pub fn from_string(string: &str) -> Board {
        let mut board = Board::new();

        for cell in Cell::list() {
            let i = cell.index();
            let digit = string.get(i..(i + 1)).and_then(|s| s.parse().ok());

            if digit.is_some() {
                board.givens.insert(cell);
            }

            let data2 = match digit {
                Some(digit) => CellData::Digit(digit),
                None => CellData::default(),
            };

            board.cell_data[i] = data2;
        }

        board
    }

    // cell getters ------------------------------------------------------------

    pub fn get_data(&self, cell: &Cell) -> &CellData {
        self.cell_data
            .get(cell.index())
            .unwrap_or_else(|| panic!("Cell {cell} not found in board"))
    }

    fn get_data_mut(&mut self, cell: &Cell) -> &mut CellData {
        self.cell_data
            .get_mut(cell.index())
            .unwrap_or_else(|| panic!("Cell {cell} not found in board"))
    }

    pub fn get_digit(&self, cell: &Cell) -> Option<Digit> {
        match self.get_data(cell) {
            CellData::Digit(digit) => Some(*digit),
            CellData::Notes(_) => None,
        }
    }

    pub fn get_notes(&self, cell: &Cell) -> Option<&Set<Digit>> {
        match self.get_data(cell) {
            CellData::Digit(_) => None,
            CellData::Notes(notes) => Some(notes),
        }
    }

    fn get_notes_mut(&mut self, cell: &Cell) -> Option<&mut Set<Digit>> {
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
        self.givens.contains(*cell)
    }

    pub fn is_notes(&self, cell: &Cell) -> bool {
        matches!(self.get_data(cell), CellData::Notes(_))
    }

    pub fn has_note(&self, cell: &Cell, digit: Digit) -> bool {
        match self.get_notes(cell) {
            Some(notes) => notes.contains(digit),
            None => false,
        }
    }

    // mutators ----------------------------------------------------------------

    pub fn reset(&mut self) {
        for ref cell in Cell::list() {
            if self.givens.contains(*cell) {
                continue;
            }

            *self.get_data_mut(cell) = CellData::default();
        }
    }

    pub fn input_solution(&mut self, candidate: Candidate) {
        let (cell, digit) = candidate.as_tuple();

        *self.get_data_mut(&cell) = CellData::Digit(digit);
    }

    pub fn input_elimination(&mut self, candidate: Candidate) {
        let (cell, digit) = candidate.as_tuple();

        let Some(notes) = self.get_notes_mut(&cell) else {
            error!("Cell {cell} is not a notes cell");
            return;
        };

        if !notes.contains(digit) {
            error!("Cell {cell} does not contain note {digit}");
        }

        notes.remove(digit);
    }

    // iterators ---------------------------------------------------------------

    pub fn iter_unsolved_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        Cell::list().filter(|cell| self.is_notes(cell))
    }
}
