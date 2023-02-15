use std::{fmt, str::FromStr};

use crate::bitset::{impl_element_for_int_newtype, Element, Set};

use super::{macros::impl_bounded_int_newtype, Block, Col, Row, UnitClass};

// =============================================================================

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Cell(u8);

impl_bounded_int_newtype! { Cell = u8 < 81 }

impl_element_for_int_newtype! { Cell = u8 < 81 in u128 }

impl Cell {
    pub fn from_row_and_col(row: Row, col: Col) -> Self {
        Self::from_index(row.index() * 9 + col.index())
    }

    pub fn row(self) -> Row {
        Row::from_index(self.index() / 9)
    }

    pub fn col(self) -> Col {
        Col::from_index(self.index() % 9)
    }

    pub fn block(self) -> Block {
        Block::from_index((self.row().index() / 3) * 3 + (self.col().index() / 3))
    }

    pub fn iter_neighbors(self) -> impl Iterator<Item = Self> {
        let mut neighbors = Set::new();

        for cell in self.block().array() {
            neighbors.insert(*cell);
        }

        for cell in self.row().array() {
            neighbors.insert(*cell);
        }

        for cell in self.col().array() {
            neighbors.insert(*cell);
        }

        neighbors.iter()
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
            .and_then(|d| Col::new(d as u8 - 1))
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
