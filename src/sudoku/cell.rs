use std::collections::HashSet;

use super::pos::Digit;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum CellContent {
    Digit(Digit, bool),
    Notes(HashSet<Digit>),
}

impl Default for CellContent {
    fn default() -> CellContent {
        CellContent::Notes(HashSet::from_iter(Digit::list()))
    }
}

impl CellContent {
    pub fn new_given(digit: Digit) -> CellContent {
        CellContent::Digit(digit, true)
    }

    pub fn new_digit(digit: Digit) -> CellContent {
        CellContent::Digit(digit, false)
    }

    pub fn is_digit(&self) -> bool {
        matches!(self, CellContent::Digit(_, _))
    }

    pub fn is_given(&self) -> bool {
        matches!(self, CellContent::Digit(_, true))
    }

    pub fn is_notes(&self) -> bool {
        matches!(self, CellContent::Notes(_))
    }

    pub fn has_digit(&self, digit: Digit) -> bool {
        match self {
            CellContent::Digit(d, _) => *d == digit,
            _ => false,
        }
    }

    pub fn get_digit(&self) -> Option<Digit> {
        match self {
            CellContent::Digit(d, _) => Some(*d),
            _ => None,
        }
    }

    pub fn has_note(&self, digit: Digit) -> bool {
        match self {
            CellContent::Notes(notes) => notes.contains(&digit),
            CellContent::Digit(_, _) => false,
        }
    }

    pub fn get_notes(&self) -> Option<&HashSet<Digit>> {
        match self {
            CellContent::Notes(notes) => Some(notes),
            CellContent::Digit(_, _) => None,
        }
    }
}
