use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::{cell::CellContent, Candidate, Cell, Digit};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Board {
    cells: HashMap<Cell, CellContent>,
    notes: HashMap<Cell, HashSet<Digit>>,
    digits: HashMap<Cell, Digit>,
    givens: HashSet<Cell>,
}

impl Board {
    // constructors ------------------------------------------------------------

    pub fn new() -> Board {
        let cells = Cell::list()
            .map(|cell| (cell, CellContent::default()))
            .collect();

        let notes = Cell::list().map(|cell| (cell, Digit::full_set())).collect();

        Board {
            cells,
            notes,
            digits: HashMap::new(),
            givens: HashSet::new(),
        }
    }

    pub fn from_string(string: &str) -> Board {
        let mut cells = HashMap::new();
        let mut notes = HashMap::new();
        let mut digits = HashMap::new();
        let mut givens = HashSet::new();

        for cell in Cell::list() {
            let index = cell.as_index();

            let digit = string.get(index..(index + 1)).and_then(|s| s.parse().ok());

            let data = match digit {
                Some(digit) => CellContent::new_given(digit),
                None => CellContent::default(),
            };

            cells.insert(cell, data);

            match digit {
                Some(digit) => {
                    digits.insert(cell, digit);
                    givens.insert(cell);
                }
                None => {
                    notes.insert(cell, HashSet::from_iter(Digit::list()));
                }
            }
        }

        Board {
            cells,
            notes,
            digits,
            givens,
        }
    }

    // cell getters ------------------------------------------------------------

    pub fn get_content(&self, cell: Cell) -> &CellContent {
        self.cells
            .get(&cell)
            .unwrap_or_else(|| panic!("Cell {cell} not found in board"))
    }

    pub fn get_digit(&self, cell: Cell) -> Option<Digit> {
        self.get_content(cell).get_digit()
    }

    pub fn get_notes(&self, cell: Cell) -> Option<&HashSet<Digit>> {
        self.get_content(cell).get_notes()
    }

    pub fn get_notes_set(&self, cell: Cell) -> HashSet<Digit> {
        self.get_notes(cell)
            .cloned()
            .unwrap_or(HashSet::from_iter(Digit::list()))
    }

    pub fn get_notes_vec(&self, cell: Cell) -> Vec<Digit> {
        self.get_notes(cell)
            .cloned()
            .map(|notes| notes.into_iter().collect_vec())
            .unwrap_or(Digit::list().collect_vec())
    }

    // mutators ----------------------------------------------------------------

    pub fn reset(&mut self) {
        for (_, content) in self.cells.iter_mut() {
            if content.is_given() {
                continue;
            }

            *content = CellContent::default();
        }
    }

    pub fn input_solution(&mut self, candidate: Candidate) {
        self.cells
            .insert(candidate.cell(), CellContent::new_digit(candidate.digit()));
    }

    pub fn input_elimination(&mut self, candidate: Candidate) {
        unimplemented!()
    }

    // iterators ---------------------------------------------------------------

    pub fn iter_unsolved_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        self.cells
            .iter()
            .filter(|(_, content)| content.is_notes())
            .map(|(cell, _)| *cell)
    }
}
