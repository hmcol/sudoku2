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

macro_rules! unit {
    [$($index:literal),+ $(,)?] => {
        [
            $(
                Cell::new_unchecked($index),
            )+
        ]
    };
}

type UnitArray = [Cell; 9];

pub const UNITS: &[UnitArray] = &[
    // rows
    unit![0, 1, 2, 3, 4, 5, 6, 7, 8],
    unit![9, 10, 11, 12, 13, 14, 15, 16, 17],
    unit![18, 19, 20, 21, 22, 23, 24, 25, 26],
    unit![27, 28, 29, 30, 31, 32, 33, 34, 35],
    unit![36, 37, 38, 39, 40, 41, 42, 43, 44],
    unit![45, 46, 47, 48, 49, 50, 51, 52, 53],
    unit![54, 55, 56, 57, 58, 59, 60, 61, 62],
    unit![63, 64, 65, 66, 67, 68, 69, 70, 71],
    unit![72, 73, 74, 75, 76, 77, 78, 79, 80],
    // cols
    unit![0, 9, 18, 27, 36, 45, 54, 63, 72],
    unit![1, 10, 19, 28, 37, 46, 55, 64, 73],
    unit![2, 11, 20, 29, 38, 47, 56, 65, 74],
    unit![3, 12, 21, 30, 39, 48, 57, 66, 75],
    unit![4, 13, 22, 31, 40, 49, 58, 67, 76],
    unit![5, 14, 23, 32, 41, 50, 59, 68, 77],
    unit![6, 15, 24, 33, 42, 51, 60, 69, 78],
    unit![7, 16, 25, 34, 43, 52, 61, 70, 79],
    unit![8, 17, 26, 35, 44, 53, 62, 71, 80],
    // boxes
    unit![0, 1, 2, 9, 10, 11, 18, 19, 20],
    unit![3, 4, 5, 12, 13, 14, 21, 22, 23],
    unit![6, 7, 8, 15, 16, 17, 24, 25, 26],
    unit![27, 28, 29, 36, 37, 38, 45, 46, 47],
    unit![30, 31, 32, 39, 40, 41, 48, 49, 50],
    unit![33, 34, 35, 42, 43, 44, 51, 52, 53],
    unit![54, 55, 56, 63, 64, 65, 72, 73, 74],
    unit![57, 58, 59, 66, 67, 68, 75, 76, 77],
    unit![60, 61, 62, 69, 70, 71, 78, 79, 80],
];

pub const ROWS: &[UnitArray] = &[
    UNITS[0], UNITS[1], UNITS[2], UNITS[3], UNITS[4], UNITS[5], UNITS[6], UNITS[7], UNITS[8],
];

pub const COLS: &[UnitArray] = &[
    UNITS[9], UNITS[10], UNITS[11], UNITS[12], UNITS[13], UNITS[14], UNITS[15], UNITS[16],
    UNITS[17],
];

pub const BLOCKS: &[UnitArray] = &[
    UNITS[18], UNITS[19], UNITS[20], UNITS[21], UNITS[22], UNITS[23], UNITS[24], UNITS[25],
    UNITS[26],
];

pub const LINES: &[UnitArray] = &[
    // rows
    UNITS[0], UNITS[1], UNITS[2], UNITS[3], UNITS[4], UNITS[5], UNITS[6], UNITS[7], UNITS[8],
    // cols
    UNITS[9], UNITS[10], UNITS[11], UNITS[12], UNITS[13], UNITS[14], UNITS[15], UNITS[16],
    UNITS[17],
];

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

macro_rules! impl_cells_iter {
    ($name:ty) => {
        impl $name {
            pub fn cells_iter(self) -> impl Iterator<Item = Cell> {
                self.array().iter().copied()
            }
        }
    };
}

impl_cells_iter! { Row }
impl_cells_iter! { Col }
impl_cells_iter! { Block }
impl_cells_iter! { Line }
impl_cells_iter! { Unit }

// =============================================================================

pub trait UnitClass: Copy + Sized {
    fn all_vec() -> Vec<Self>;
    fn array(self) -> &'static UnitArray;
    fn cells_set(self) -> HashSet<Cell> {
        self.array().iter().copied().collect()
    }
    #[deprecated]
    fn cells_vec(self) -> Vec<Cell> {
        self.array().to_vec()
    }
}

impl UnitClass for Row {
    fn array(self) -> &'static UnitArray {
        &ROWS[self.as_index()]
    }

    fn all_vec() -> Vec<Self> {
        Row::list().collect_vec()
    }
}

impl UnitClass for Col {
    fn array(self) -> &'static UnitArray {
        &COLS[self.as_index()]
    }

    fn all_vec() -> Vec<Self> {
        Col::list().collect_vec()
    }
}

impl UnitClass for Line {
    fn array(self) -> &'static UnitArray {
        &LINES[self.as_index()]
    }

    fn all_vec() -> Vec<Self> {
        Line::list().collect_vec()
    }
}

impl UnitClass for Block {
    fn array(self) -> &'static UnitArray {
        &BLOCKS[self.as_index()]
    }

    fn all_vec() -> Vec<Self> {
        Block::list().collect_vec()
    }
}

impl UnitClass for Unit {
    fn array(self) -> &'static UnitArray {
        &UNITS[self.as_index()]
    }

    fn all_vec() -> Vec<Self> {
        Unit::list().collect_vec()
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
