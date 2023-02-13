use std::collections::HashSet;

use super::{macros::impl_bounded_int_newtype, Cell};

// =============================================================================

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Row(u8);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Col(u8);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Block(u8);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Line(u8);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Unit(u8);

impl_bounded_int_newtype! { Row = u8 < 9 }
impl_bounded_int_newtype! { Col = u8 < 9 }
impl_bounded_int_newtype! { Block = u8 < 9 }
impl_bounded_int_newtype! { Line = u8 < 18 }
impl_bounded_int_newtype! { Unit = u8 < 27 }

// =============================================================================

enum LineType {
    Row(Row),
    Col(Col),
}

impl Line {
    fn line_type(self) -> LineType {
        match self.0 {
            0..=8 => LineType::Row(Row(self.0)),
            _ => LineType::Col(Col(self.0 - 9)),
        }
    }
}

// =============================================================================

impl Row {
    pub fn cells_set(self) -> HashSet<Cell> {
        // Row::list().map(move |row| Cell::from_row_and_col(row, self)).collect()
        unimplemented!()
    }

    pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
        self.cells_set().into_iter()
    }
}

impl Col {
    pub fn cells_set(self) -> HashSet<Cell> {
        // Col::list().map(move |col| Cell::from_row_and_col(self, col)).collect()
        unimplemented!()
    }

    pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
        self.cells_set().into_iter()
    }
}

impl Line {
    pub fn cells_set(self) -> HashSet<Cell> {
        match self.line_type() {
            LineType::Row(row) => row.cells_set(),
            LineType::Col(col) => col.cells_set(),
        }
    }

    pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
        self.cells_set().into_iter()
    }
}

impl Block {
    pub fn cells_set(self) -> HashSet<Cell> {
        // Block::list().map(move |block| Cell::from_block_and_index(self, block)).collect()
        unimplemented!()
    }

    pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
        self.cells_set().into_iter()
    }
}

pub enum UnitType {
    Row(Row),
    Col(Col),
    Block(Block),
}

impl Unit {
    pub fn unit_type(self) -> UnitType {
        match self.0 {
            0..=8 => UnitType::Row(Row(self.0)),
            9..=17 => UnitType::Col(Col(self.0 - 9)),
            _ => UnitType::Block(Block(self.0 - 18)),
        }
    }

    pub fn cells_set(self) -> HashSet<Cell> {
        match self.unit_type() {
            UnitType::Row(row) => row.cells_set(),
            UnitType::Col(col) => col.cells_set(),
            UnitType::Block(block) => block.cells_set(),
        }
    }

    pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
        self.cells_set().into_iter()
    }
}

// =============================================================================

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

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Row({self})")
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

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Col({self})")
    }
}
