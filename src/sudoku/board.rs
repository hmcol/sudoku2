use log::error;

use crate::bitset::{Element, Set};

use super::{Candidate, Cell, Digit};

// =============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
pub struct Board([CellData; 81]);

impl Board {
    // constructors ------------------------------------------------------------

    pub fn new() -> Board {
        Board([CellData::default(); 81])
    }

    // cell getters ------------------------------------------------------------

    pub fn get_data(&self, cell: &Cell) -> &CellData {
        self.0
            .get(cell.index())
            .unwrap_or_else(|| panic!("Cell {cell} not found in board"))
    }

    fn get_data_mut(&mut self, cell: &Cell) -> &mut CellData {
        self.0
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

    pub fn is_notes(&self, cell: &Cell) -> bool {
        matches!(self.get_data(cell), CellData::Notes(_))
    }

    pub fn has_note(&self, cell: &Cell, digit: Digit) -> bool {
        match self.get_notes(cell) {
            Some(notes) => notes.contains(digit),
            None => false,
        }
    }

    pub fn count_notes(&self, cell: &Cell) -> usize {
        match self.get_notes(cell) {
            Some(notes) => notes.len(),
            None => 0,
        }
    }

    // mutators ----------------------------------------------------------------

    pub fn clear(&mut self) {
        for ref cell in Cell::list() {
            *self.get_data_mut(cell) = CellData::default();
        }
    }

    pub fn set_digit(&mut self, cell: Cell, digit: Digit) {
        *self.get_data_mut(&cell) = CellData::Digit(digit);
    }

    pub fn input_solution(&mut self, candidate: Candidate) {
        let (cell, digit) = candidate.as_tuple();
        self.set_digit(cell, digit);
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

    pub fn iter_solved(&self) -> impl Iterator<Item = Cell> + '_ {
        Cell::list().filter(|cell| self.is_digit(cell))
    }

    pub fn iter_unsolved(&self) -> impl Iterator<Item = Cell> + '_ {
        Cell::list().filter(|cell| self.is_notes(cell))
    }

    pub fn iter_with_digit(&self, digit: Digit) -> impl Iterator<Item = Cell> + '_ {
        Cell::list().filter(move |cell| self.get_digit(cell) == Some(digit))
    }

    pub fn iter_with_note(&self, digit: Digit) -> impl Iterator<Item = Cell> + '_ {
        Cell::list().filter(move |cell| self.has_note(cell, digit))
    }

    // cell sets ---------------------------------------------------------------

    pub fn cells_unsolved(&self) -> Set<Cell> {
        self.iter_unsolved().collect()
    }

    pub fn cells_with_digit(&self, digit: Digit) -> Set<Cell> {
        self.iter_with_digit(digit).collect()
    }

    pub fn cells_with_note(&self, digit: Digit) -> Set<Cell> {
        self.iter_with_note(digit).collect()
    }
}
