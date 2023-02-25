use multimap::MultiMap;

use crate::{
    sudoku::{pos::UnitClass, Block, Board, Candidate, Col, Digit, Row},
    util::{IterArrayCombinations, TryIntoArray},
};

// =============================================================================

pub struct LinkGraph {
    neighbors: MultiMap<Candidate, Candidate>,
}

impl LinkGraph {
    pub fn new<L: LinkClass>(board: &Board) -> Self {
        let mut neighbors = MultiMap::new();

        L::add_links(board, &mut neighbors);

        Self { neighbors }
    }

    pub fn neighbors(&self, candidate: &Candidate) -> Vec<Candidate> {
        let vec = self.neighbors.get_vec(candidate);

        match vec {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        }
    }
}

// =============================================================================

pub trait LinkClass {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>);
}

pub struct Bivalue;

impl LinkClass for Bivalue {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        for cell in board.iter_unsolved() {
            let notes = board.get_notes(&cell).unwrap();

            let Ok([digit_a, digit_b]) = notes.try_into_array() else { continue };

            let a = (cell, digit_a).into();
            let b = (cell, digit_b).into();

            neighbors.insert(a, b);
            neighbors.insert(b, a);
        }
    }
}

fn add_links_bilocal<U: UnitClass>(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
    for x in Digit::list() {
        let x_cells = board.cells_with_note(x);

        for unit in U::iter_all() {
            let x_unit_cells = unit.cells_set() & x_cells;

            let Ok([cell_a, cell_b]) = x_unit_cells.try_into_array() else { continue };

            let a = (cell_a, x).into();
            let b = (cell_b, x).into();

            neighbors.insert(a, b);
            neighbors.insert(b, a);
        }
    }
}

pub struct BilocalRow;

impl LinkClass for BilocalRow {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        add_links_bilocal::<Row>(board, neighbors);
    }
}

pub struct BilocalCol;

impl LinkClass for BilocalCol {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        add_links_bilocal::<Col>(board, neighbors);
    }
}

pub struct BilocalBlock;

impl LinkClass for BilocalBlock {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        add_links_bilocal::<Block>(board, neighbors);
    }
}

pub struct Bilocal;

impl LinkClass for Bilocal {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        BilocalRow::add_links(board, neighbors);
        BilocalCol::add_links(board, neighbors);
        BilocalBlock::add_links(board, neighbors);
    }
}

pub struct Strong;

impl LinkClass for Strong {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        Bivalue::add_links(board, neighbors);
        Bilocal::add_links(board, neighbors);
    }
}

pub struct WeakCell;

impl LinkClass for WeakCell {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        for cell in board.iter_unsolved() {
            let notes = board.get_notes(&cell).unwrap();

            for digits in notes.iter().array_combinations::<2>() {
                let a = (cell, digits[0]).into();
                let b = (cell, digits[1]).into();

                neighbors.insert(a, b);
                neighbors.insert(b, a);
            }
        }
    }
}

fn add_links_weak_local<U: UnitClass>(
    board: &Board,
    neighbors: &mut MultiMap<Candidate, Candidate>,
) {
    for x in Digit::list() {
        let x_cells = board.cells_with_note(x);

        for unit in U::iter_all() {
            let x_unit_cells = unit.cells_set() & x_cells;

            for cells in x_unit_cells.iter().array_combinations::<2>() {
                let a = (cells[0], x).into();
                let b = (cells[1], x).into();

                neighbors.insert(a, b);
                neighbors.insert(b, a);
            }
        }
    }
}

pub struct WeakRow;

impl LinkClass for WeakRow {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        add_links_weak_local::<Row>(board, neighbors);
    }
}

pub struct WeakCol;

impl LinkClass for WeakCol {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        add_links_weak_local::<Col>(board, neighbors);
    }
}

pub struct WeakBlock;

impl LinkClass for WeakBlock {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        add_links_weak_local::<Block>(board, neighbors);
    }
}

pub struct WeakUnit;

impl LinkClass for WeakUnit {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        WeakRow::add_links(board, neighbors);
        WeakCol::add_links(board, neighbors);
        WeakBlock::add_links(board, neighbors);
    }
}

pub struct Weak;

impl LinkClass for Weak {
    fn add_links(board: &Board, neighbors: &mut MultiMap<Candidate, Candidate>) {
        WeakCell::add_links(board, neighbors);
        WeakUnit::add_links(board, neighbors);
    }
}
