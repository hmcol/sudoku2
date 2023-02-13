use std::collections::HashSet;

use itertools::Itertools;

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

impl From<Row> for Line {
    fn from(row: Row) -> Line {
        Line(row.0)
    }
}

impl From<Col> for Line {
    fn from(col: Col) -> Line {
        Line(col.0 + 9)
    }
}

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

impl From<Row> for Unit {
    fn from(row: Row) -> Unit {
        Unit(row.0)
    }
}

impl From<Col> for Unit {
    fn from(col: Col) -> Unit {
        Unit(col.0 + 9)
    }
}

impl From<Line> for Unit {
    fn from(line: Line) -> Unit {
        match line.line_type() {
            LineType::Row(row) => Unit::from(row),
            LineType::Col(col) => Unit::from(col),
        }
    }
}

impl From<Block> for Unit {
    fn from(block: Block) -> Unit {
        Unit(block.0 + 18)
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
}

// =============================================================================

impl Row {
    pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
        Col::list().map(move |col| Cell::from_row_and_col(self, col))
    }

    pub fn cells_set(self) -> HashSet<Cell> {
        self.cells_iter().collect()
    }
}

impl Col {
    pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
        Row::list().map(move |row| Cell::from_row_and_col(row, self))
    }

    pub fn cells_set(self) -> HashSet<Cell> {
        self.cells_iter().collect()
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
    pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
        let row_start = self.as_index() / 3 * 3;
        let col_start = self.as_index() % 3 * 3;

        (row_start..row_start + 3)
            .cartesian_product(col_start..col_start + 3)
            .map(|(row, col)| {
                Cell::from_row_and_col(
                    Row::from_index_unchecked(row),
                    Col::from_index_unchecked(col),
                )
            })
    }

    pub fn cells_set(self) -> HashSet<Cell> {
        self.cells_iter().collect()
    }
}

impl Unit {
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

pub trait UnitClass: Copy + Sized {
    fn all_vec() -> Vec<Self>;
    fn cells_set(self) -> HashSet<Cell>;
    fn cells_vec(self) -> Vec<Cell>;
}

impl UnitClass for Row {
    fn all_vec() -> Vec<Self> {
        Row::list().collect_vec()
    }
    fn cells_set(self) -> HashSet<Cell> {
        self.cells_iter().collect()
    }
    fn cells_vec(self) -> Vec<Cell> {
        self.cells_iter().collect_vec()
    }
}

impl UnitClass for Col {
    fn all_vec() -> Vec<Self> {
        Col::list().collect_vec()
    }
    fn cells_set(self) -> HashSet<Cell> {
        self.cells_iter().collect()
    }
    fn cells_vec(self) -> Vec<Cell> {
        self.cells_iter().collect_vec()
    }
}

impl UnitClass for Line {
    fn all_vec() -> Vec<Self> {
        Line::list().collect_vec()
    }
    fn cells_set(self) -> HashSet<Cell> {
        self.cells_iter().collect()
    }
    fn cells_vec(self) -> Vec<Cell> {
        self.cells_iter().collect_vec()
    }
}

impl UnitClass for Block {
    fn all_vec() -> Vec<Self> {
        Block::list().collect_vec()
    }
    fn cells_set(self) -> HashSet<Cell> {
        self.cells_iter().collect()
    }
    fn cells_vec(self) -> Vec<Cell> {
        self.cells_iter().collect_vec()
    }
}

impl UnitClass for Unit {
    fn all_vec() -> Vec<Self> {
        Unit::list().collect_vec()
    }
    fn cells_set(self) -> HashSet<Cell> {
        self.cells_iter().collect()
    }
    fn cells_vec(self) -> Vec<Cell> {
        self.cells_iter().collect_vec()
    }
}

// pub enum UnitClass {
//     Row,
//     Col,
//     Line,
//     Block,
//     Unit,
// }

// impl UnitClass {
//     pub fn list(self) -> impl Iterator<Item = Unit> {
//         let vec = match self {
//             Self::Row => Row::list().map(Unit::from).collect_vec(),
//             Self::Col => Col::list().map(Unit::from).collect_vec(),
//             Self::Line => Line::list().map(Unit::from).collect_vec(),
//             Self::Block => Block::list().map(Unit::from).collect_vec(),
//             Self::Unit => Unit::list().map(Unit::from).collect_vec(),
//         };
//         vec.into_iter()
//     }

//     pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
//         self.list().flat_map(Unit::cells_iter)
//     }
// }

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
