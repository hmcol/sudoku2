use std::fmt;

use super::{Cell, Col, Row};

// =============================================================================

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Block(u8);

super::macros::impl_bounded_int_newtype! { Block = u8 < 9 }

impl Block {
    pub fn iter_cells(self) -> impl Iterator<Item = Cell> {
        let row_index = self.as_index() / 3 * 3;
        let col_index = self.as_index() % 3 * 3;

        (0..9).map(move |i| {
            Cell::from_row_and_col(
                Row::from_index_unchecked(row_index + i / 3),
                Col::from_index_unchecked(col_index + i % 3),
            )
        })
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block({self})")
    }
}
