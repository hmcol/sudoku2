use std::fmt;

use super::{macros::impl_bounded_int_newtype, Cell, Col};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Row(u8);

impl_bounded_int_newtype! { Row = u8 < 9 }

const ROW_CHARS: [char; 9] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];

impl From<Row> for char {
    fn from(row: Row) -> Self {
        ROW_CHARS[row.as_index()]
    }
}

impl TryFrom<char> for Row {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        ROW_CHARS
            .iter()
            .position(|&c| c == value)
            .map(Self::from_index_unchecked)
            .ok_or(())
    }
}

impl Row {
    pub fn iter_cells(self) -> impl Iterator<Item = Cell> {
        Col::list().map(move |col| Cell::from_row_and_col(self, col))
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Row({self})")
    }
}
