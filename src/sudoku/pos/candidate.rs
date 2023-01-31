use std::fmt;

use super::{Cell, Digit};

// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Candidate(u16);

super::macros::impl_bounded_int_newtype! { Candidate = u16 < 729 }

impl Candidate {
    pub fn from_cell_and_digit(cell: Cell, digit: Digit) -> Self {
        Self::from_index_unchecked(cell.as_index() * 9 + digit.as_index())
    }

    pub fn cell(self) -> Cell {
        Cell::from_index_unchecked(self.as_index() / 9)
    }

    pub fn digit(self) -> Digit {
        Digit::from_index_unchecked(self.as_index() % 9)
    }
}

impl fmt::Display for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}#{}", self.cell(), self.digit())
    }
}

impl fmt::Debug for Candidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Candidate({}#{})", self.cell(), self.digit())
    }
}
