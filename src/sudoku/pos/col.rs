use std::fmt;

use super::{Cell, Row};

// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Col(u8);

super::macros::impl_bounded_int_newtype! { Col = u8 < 9 }

impl Col {
    pub fn iter_cells(self) -> impl Iterator<Item = Cell> {
        Row::list().map(move |row| Cell::from_row_and_col(row, self))
    }
}

impl fmt::Display for Col {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

impl fmt::Debug for Col {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Col({self})")
    }
}
