use std::collections::HashSet;

use super::{digit::Digit, pos::Cell};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum CellContent {
    Digit(Digit, bool),
    Notes(HashSet<Digit>),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CellData {
    pub id: Cell,
    pub content: CellContent,
}

impl CellData {
    pub fn new(id: Cell) -> CellData {
        CellData {
            id,
            content: CellContent::Notes(HashSet::from_iter(Digit::list())),
        }
    }

    pub fn new_given(id: Cell, digit: Digit) -> CellData {
        CellData {
            id,
            content: CellContent::Digit(digit, true),
        }
    }
}
