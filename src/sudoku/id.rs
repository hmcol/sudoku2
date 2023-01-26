macro_rules! TypeSet {
    ($ty:ident $ls:ident = $($x:ident)+) => {
        #[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
        pub enum $ty {
            $($x),+
        }

        pub const $ls: [$ty; 9] = [
            $($ty::$x),+
        ];
    };
}

TypeSet!(Digit DIGITS = One Two Three Four Five Six Seven Eight Nine);

impl ToString for Digit {
    fn to_string(&self) -> String {
        match self {
            Digit::One => "1",
            Digit::Two => "2",
            Digit::Three => "3",
            Digit::Four => "4",
            Digit::Five => "5",
            Digit::Six => "6",
            Digit::Seven => "7",
            Digit::Eight => "8",
            Digit::Nine => "9",
        }
        .to_string()
    }
}

impl TryFrom<char> for Digit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Digit::One),
            '2' => Ok(Digit::Two),
            '3' => Ok(Digit::Three),
            '4' => Ok(Digit::Four),
            '5' => Ok(Digit::Five),
            '6' => Ok(Digit::Six),
            '7' => Ok(Digit::Seven),
            '8' => Ok(Digit::Eight),
            '9' => Ok(Digit::Nine),
            _ => Err(()),
        }
    }
}

TypeSet!(Row ROWS = A B C D E F G H I);

impl ToString for Row {
    fn to_string(&self) -> String {
        match self {
            Row::A => "A",
            Row::B => "B",
            Row::C => "C",
            Row::D => "D",
            Row::E => "E",
            Row::F => "F",
            Row::G => "G",
            Row::H => "H",
            Row::I => "I",
        }
        .to_string()
    }
}

TypeSet!(Col COLS = One Two Three Four Five Six Seven Eight Nine);

impl ToString for Col {
    fn to_string(&self) -> String {
        match self {
            Col::One => "1",
            Col::Two => "2",
            Col::Three => "3",
            Col::Four => "4",
            Col::Five => "5",
            Col::Six => "6",
            Col::Seven => "7",
            Col::Eight => "8",
            Col::Nine => "9",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct CellId {
    row: Row,
    col: Col,
}

impl ToString for CellId {
    fn to_string(&self) -> String {
        format!("{}{}", self.row.to_string(), self.col.to_string())
    }
}

impl CellId {
    pub const fn new(row: Row, col: Col) -> CellId {
        CellId { row, col }
    }
}

#[test]
fn make_cell_ids() {
    for row in ROWS {
        for col in COLS {
            let id = CellId::new(row, col);

            let name = id.to_string();

            println!("const {name}: CellId = CellId::new(Row::{row:?}, Col::{col:?});");
        }
    }
}

const A1: CellId = CellId::new(Row::A, Col::One);
const A2: CellId = CellId::new(Row::A, Col::Two);
const A3: CellId = CellId::new(Row::A, Col::Three);
const A4: CellId = CellId::new(Row::A, Col::Four);
const A5: CellId = CellId::new(Row::A, Col::Five);
const A6: CellId = CellId::new(Row::A, Col::Six);
const A7: CellId = CellId::new(Row::A, Col::Seven);
const A8: CellId = CellId::new(Row::A, Col::Eight);
const A9: CellId = CellId::new(Row::A, Col::Nine);
const B1: CellId = CellId::new(Row::B, Col::One);
const B2: CellId = CellId::new(Row::B, Col::Two);
const B3: CellId = CellId::new(Row::B, Col::Three);
const B4: CellId = CellId::new(Row::B, Col::Four);
const B5: CellId = CellId::new(Row::B, Col::Five);
const B6: CellId = CellId::new(Row::B, Col::Six);
const B7: CellId = CellId::new(Row::B, Col::Seven);
const B8: CellId = CellId::new(Row::B, Col::Eight);
const B9: CellId = CellId::new(Row::B, Col::Nine);
const C1: CellId = CellId::new(Row::C, Col::One);
const C2: CellId = CellId::new(Row::C, Col::Two);
const C3: CellId = CellId::new(Row::C, Col::Three);
const C4: CellId = CellId::new(Row::C, Col::Four);
const C5: CellId = CellId::new(Row::C, Col::Five);
const C6: CellId = CellId::new(Row::C, Col::Six);
const C7: CellId = CellId::new(Row::C, Col::Seven);
const C8: CellId = CellId::new(Row::C, Col::Eight);
const C9: CellId = CellId::new(Row::C, Col::Nine);
const D1: CellId = CellId::new(Row::D, Col::One);
const D2: CellId = CellId::new(Row::D, Col::Two);
const D3: CellId = CellId::new(Row::D, Col::Three);
const D4: CellId = CellId::new(Row::D, Col::Four);
const D5: CellId = CellId::new(Row::D, Col::Five);
const D6: CellId = CellId::new(Row::D, Col::Six);
const D7: CellId = CellId::new(Row::D, Col::Seven);
const D8: CellId = CellId::new(Row::D, Col::Eight);
const D9: CellId = CellId::new(Row::D, Col::Nine);
const E1: CellId = CellId::new(Row::E, Col::One);
const E2: CellId = CellId::new(Row::E, Col::Two);
const E3: CellId = CellId::new(Row::E, Col::Three);
const E4: CellId = CellId::new(Row::E, Col::Four);
const E5: CellId = CellId::new(Row::E, Col::Five);
const E6: CellId = CellId::new(Row::E, Col::Six);
const E7: CellId = CellId::new(Row::E, Col::Seven);
const E8: CellId = CellId::new(Row::E, Col::Eight);
const E9: CellId = CellId::new(Row::E, Col::Nine);
const F1: CellId = CellId::new(Row::F, Col::One);
const F2: CellId = CellId::new(Row::F, Col::Two);
const F3: CellId = CellId::new(Row::F, Col::Three);
const F4: CellId = CellId::new(Row::F, Col::Four);
const F5: CellId = CellId::new(Row::F, Col::Five);
const F6: CellId = CellId::new(Row::F, Col::Six);
const F7: CellId = CellId::new(Row::F, Col::Seven);
const F8: CellId = CellId::new(Row::F, Col::Eight);
const F9: CellId = CellId::new(Row::F, Col::Nine);
const G1: CellId = CellId::new(Row::G, Col::One);
const G2: CellId = CellId::new(Row::G, Col::Two);
const G3: CellId = CellId::new(Row::G, Col::Three);
const G4: CellId = CellId::new(Row::G, Col::Four);
const G5: CellId = CellId::new(Row::G, Col::Five);
const G6: CellId = CellId::new(Row::G, Col::Six);
const G7: CellId = CellId::new(Row::G, Col::Seven);
const G8: CellId = CellId::new(Row::G, Col::Eight);
const G9: CellId = CellId::new(Row::G, Col::Nine);
const H1: CellId = CellId::new(Row::H, Col::One);
const H2: CellId = CellId::new(Row::H, Col::Two);
const H3: CellId = CellId::new(Row::H, Col::Three);
const H4: CellId = CellId::new(Row::H, Col::Four);
const H5: CellId = CellId::new(Row::H, Col::Five);
const H6: CellId = CellId::new(Row::H, Col::Six);
const H7: CellId = CellId::new(Row::H, Col::Seven);
const H8: CellId = CellId::new(Row::H, Col::Eight);
const H9: CellId = CellId::new(Row::H, Col::Nine);
const I1: CellId = CellId::new(Row::I, Col::One);
const I2: CellId = CellId::new(Row::I, Col::Two);
const I3: CellId = CellId::new(Row::I, Col::Three);
const I4: CellId = CellId::new(Row::I, Col::Four);
const I5: CellId = CellId::new(Row::I, Col::Five);
const I6: CellId = CellId::new(Row::I, Col::Six);
const I7: CellId = CellId::new(Row::I, Col::Seven);
const I8: CellId = CellId::new(Row::I, Col::Eight);
const I9: CellId = CellId::new(Row::I, Col::Nine);


pub const CELLS: [CellId; 81] = [
    A1, A2, A3, A4, A5, A6, A7, A8, A9,
    B1, B2, B3, B4, B5, B6, B7, B8, B9,
    C1, C2, C3, C4, C5, C6, C7, C8, C9,
    D1, D2, D3, D4, D5, D6, D7, D8, D9,
    E1, E2, E3, E4, E5, E6, E7, E8, E9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9,
    G1, G2, G3, G4, G5, G6, G7, G8, G9,
    H1, H2, H3, H4, H5, H6, H7, H8, H9,
    I1, I2, I3, I4, I5, I6, I7, I8, I9,
];

pub type Unit = [CellId; 9];

pub const BOXES: [Unit; 9] = [
    [A1, A2, A3, B1, B2, B3, C1, C2, C3],
    [A4, A5, A6, B4, B5, B6, C4, C5, C6],
    [A7, A8, A9, B7, B8, B9, C7, C8, C9],
    [D1, D2, D3, E1, E2, E3, F1, F2, F3],
    [D4, D5, D6, E4, E5, E6, F4, F5, F6],
    [D7, D8, D9, E7, E8, E9, F7, F8, F9],
    [G1, G2, G3, H1, H2, H3, I1, I2, I3],
    [G4, G5, G6, H4, H5, H6, I4, I5, I6],
    [G7, G8, G9, H7, H8, H9, I7, I8, I9],
];
