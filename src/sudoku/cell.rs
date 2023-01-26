use std::collections::HashSet;

use super::id::{Digit, CellId, DIGITS};

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
    pub fn new(id: CellId) -> Cell {
        let notes = HashSet::from_iter(DIGITS);

        Cell {
            id,
            content: CellContent::Notes(notes),
        }
    }

    pub fn new_given(id: CellId, digit: Digit) -> Cell {
        Cell {
            id,
            content: CellContent::Digit(digit, true),
        }
    }
}
