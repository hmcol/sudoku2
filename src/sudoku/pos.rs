use std::{hash::Hash, str::FromStr};

use super::digit::Digit;

macro_rules! impl_bounded_int_newtype {
    ($name:ident = $repr:ident < $bound:literal) => {
        impl $name {
            const fn new_unchecked(value: $repr) -> Self {
                Self(value)
            }

            pub fn new_checked(value: $repr) -> Option<Self> {
                (value < $bound).then(|| Self::new_unchecked(value))
            }

            pub fn list() -> impl Iterator<Item = Self> {
                (0..$bound).map(Self::new_unchecked)
            }

            pub fn as_index(self) -> usize {
                self.0 as usize
            }

            fn from_index_unchecked(index: usize) -> Self {
                Self::new_unchecked(index as $repr)
            }

            pub fn from_index_checked(index: usize) -> Option<Self> {
                index.try_into().ok().and_then(Self::new_checked)
            }
        }
    };
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
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

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Col(u8);

impl_bounded_int_newtype! { Col = u8 < 9 }

impl Col {
    pub fn iter_cells(self) -> impl Iterator<Item = Cell> {
        Row::list().map(move |row| Cell::from_row_and_col(row, self))
    }
}

impl std::fmt::Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Block(u8);

impl_bounded_int_newtype! { Block = u8 < 9 }

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

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Cell(u8);

impl_bounded_int_newtype! { Cell = u8 < 81 }

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
            .and_then(|d| Col::from_index_checked(d as usize - 1))
            .ok_or(())?;

        Ok(Cell::from_row_and_col(row, col))
    }
}

macro_rules! wrap_with {
    ($name:ident for $value:literal) => {
        $name($value)
    };
    ($name:ident for [ $($t:tt),+ $(,)?]) => {
        [
            $(
                wrap_with! { $name for $t }
            ),+
        ]
    };
}

macro_rules! into_cells {
    ($value:literal) => {
        Cell($value)
    };
    ([ $($t:tt),+ $(,)?]) => {
        [
            $(
                into_cells! { $t }
            ),+
        ]
    };
}

pub const CELLS_BY_UNIT: [[Cell; 9]; 27] = into_cells! {
    [
        // rows
        [ 0,  1,  2,  3,  4,  5,  6,  7,  8],
        [ 9, 10, 11, 12, 13, 14, 15, 16, 17],
        [18, 19, 20, 21, 22, 23, 24, 25, 26],
        [27, 28, 29, 30, 31, 32, 33, 34, 35],
        [36, 37, 38, 39, 40, 41, 42, 43, 44],
        [45, 46, 47, 48, 49, 50, 51, 52, 53],
        [54, 55, 56, 57, 58, 59, 60, 61, 62],
        [63, 64, 65, 66, 67, 68, 69, 70, 71],
        [72, 73, 74, 75, 76, 77, 78, 79, 80],
        // cols
        [ 0,  9, 18, 27, 36, 45, 54, 63, 72],
        [ 1, 10, 19, 28, 37, 46, 55, 64, 73],
        [ 2, 11, 20, 29, 38, 47, 56, 65, 74],
        [ 3, 12, 21, 30, 39, 48, 57, 66, 75],
        [ 4, 13, 22, 31, 40, 49, 58, 67, 76],
        [ 5, 14, 23, 32, 41, 50, 59, 68, 77],
        [ 6, 15, 24, 33, 42, 51, 60, 69, 78],
        [ 7, 16, 25, 34, 43, 52, 61, 70, 79],
        [ 8, 17, 26, 35, 44, 53, 62, 71, 80],
        // blocks
        [ 0,  1,  2,  9, 10, 11, 18, 19, 20],
        [ 3,  4,  5, 12, 13, 14, 21, 22, 23],
        [ 6,  7,  8, 15, 16, 17, 24, 25, 26],
        [27, 28, 29, 36, 37, 38, 45, 46, 47],
        [30, 31, 32, 39, 40, 41, 48, 49, 50],
        [33, 34, 35, 42, 43, 44, 51, 52, 53],
        [54, 55, 56, 63, 64, 65, 72, 73, 74],
        [57, 58, 59, 66, 67, 68, 75, 76, 77],
        [60, 61, 62, 69, 70, 71, 78, 79, 80],
    ]
};

pub const CELLS_BY_ROW: [[Cell; 9]; 9] = into_cells! {
    [
        [ 0,  1,  2,  3,  4,  5,  6,  7,  8],
        [ 9, 10, 11, 12, 13, 14, 15, 16, 17],
        [18, 19, 20, 21, 22, 23, 24, 25, 26],
        [27, 28, 29, 30, 31, 32, 33, 34, 35],
        [36, 37, 38, 39, 40, 41, 42, 43, 44],
        [45, 46, 47, 48, 49, 50, 51, 52, 53],
        [54, 55, 56, 57, 58, 59, 60, 61, 62],
        [63, 64, 65, 66, 67, 68, 69, 70, 71],
        [72, 73, 74, 75, 76, 77, 78, 79, 80],
    ]
};

pub const CELLS_BY_COL: [[Cell; 9]; 9] = into_cells! {
    [
        [ 0,  9, 18, 27, 36, 45, 54, 63, 72],
        [ 1, 10, 19, 28, 37, 46, 55, 64, 73],
        [ 2, 11, 20, 29, 38, 47, 56, 65, 74],
        [ 3, 12, 21, 30, 39, 48, 57, 66, 75],
        [ 4, 13, 22, 31, 40, 49, 58, 67, 76],
        [ 5, 14, 23, 32, 41, 50, 59, 68, 77],
        [ 6, 15, 24, 33, 42, 51, 60, 69, 78],
        [ 7, 16, 25, 34, 43, 52, 61, 70, 79],
        [ 8, 17, 26, 35, 44, 53, 62, 71, 80],
    ]
};

pub const CELLS_BY_BLOCK: [[Cell; 9]; 9] = into_cells! {
    [
        [ 0,  1,  2,  9, 10, 11, 18, 19, 20],
        [ 3,  4,  5, 12, 13, 14, 21, 22, 23],
        [ 6,  7,  8, 15, 16, 17, 24, 25, 26],
        [27, 28, 29, 36, 37, 38, 45, 46, 47],
        [30, 31, 32, 39, 40, 41, 48, 49, 50],
        [33, 34, 35, 42, 43, 44, 51, 52, 53],
        [54, 55, 56, 63, 64, 65, 72, 73, 74],
        [57, 58, 59, 66, 67, 68, 75, 76, 77],
        [60, 61, 62, 69, 70, 71, 78, 79, 80],
    ]
};

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.row(), self.col())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Candidate(u16);

impl_bounded_int_newtype! { Candidate = u16 < 729 }

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

#[test]
fn make_cell_ids() {
    for row in Row::list() {
        for col in Col::list() {
            let cell = Cell::from_row_and_col(row, col);

            let name = cell.to_string();

            println!("const {name}: CellId = CellId::new(Row::{row:?}, Col::{col:?});");
        }
    }
}
