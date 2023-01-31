use std::{fmt, str::FromStr};

use super::{Block, Col, Row, CELLS_BY_BLOCK};

// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Cell(u8);

super::macros::impl_bounded_int_newtype! { Cell = u8 < 81 }

impl Cell {
    pub fn from_row_and_col(row: Row, col: Col) -> Self {
        Self::from_index_unchecked(row.as_index() * 9 + col.as_index())
    }

    pub fn row(self) -> Row {
        Row::from_index_unchecked(self.as_index() / 9)
    }

    pub fn col(self) -> Col {
        Col::from_index_unchecked(self.as_index() % 9)
    }

    pub fn block(self) -> Block {
        Block::from_index_unchecked((self.row().as_index() / 3) * 3 + (self.col().as_index() / 3))
    }

    pub fn iter_neighbors(self) -> impl Iterator<Item = Self> {
        let block = CELLS_BY_BLOCK[self.block().as_index()];

        let row_iter = self.col().iter_cells().filter(move |c| !block.contains(c));
        let col_iter = self.row().iter_cells().filter(move |c| !block.contains(c));

        self.block()
            .iter_cells()
            .filter(move |c| *c != self)
            .chain(row_iter)
            .chain(col_iter)
    }
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let row = chars.next().ok_or(()).and_then(Row::try_from)?;

        let col = chars
            .next()
            .and_then(|c| c.to_digit(10))
            .and_then(|d| Col::from_index(d as usize - 1))
            .ok_or(())?;

        Ok(Cell::from_row_and_col(row, col))
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.row(), self.col())
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cell({self})")
    }
}
