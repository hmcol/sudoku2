use std::collections::HashSet;

use super::{Cell, Row};

// =============================================================================

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Col(pub(super) u8);

super::macros::impl_bounded_int_newtype! { Col = u8 < 9 }

impl Col {
    pub fn iter_cells(self) -> impl Iterator<Item = Cell> {
        Row::list().map(move |row| Cell::from_row_and_col(row, self))
    }

    pub fn cells_set(self) -> HashSet<Cell> {
        self.iter_cells().collect()
    }
}

impl std::fmt::Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

impl std::fmt::Debug for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Col({self})")
    }
}
