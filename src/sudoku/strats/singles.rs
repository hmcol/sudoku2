use itertools::Itertools;

use crate::sudoku::{
    digit::Digit,
    pos::{Candidate, CELLS_BY_UNIT},
};

use super::{Strategy, StrategyResult};

pub const FULL_HOUSE: Strategy = Strategy {
    name: "Full House",
    find: |board| {
        let mut result = StrategyResult::default();

        for cell in board.iter_unsolved_cells() {
            let notes: Vec<Digit> = board.get_notes_set(cell).into_iter().collect();

            if notes.len() != 1 {
                continue;
            }

            let digit = *notes.first().unwrap();

            let candidate = Candidate::from_cell_and_digit(cell, digit);

            result.solutions.push(candidate);
        }

        result
    },
};

pub const NAKED_SINGLE: Strategy = Strategy {
    name: "Naked Single",
    find: |board| {
        let mut result = StrategyResult::default();

        for cell in board.iter_unsolved_cells() {
            let notes: Vec<Digit> = board.get_notes_set(cell).into_iter().collect();

            if notes.len() != 1 {
                continue;
            }

            let digit = *notes.first().unwrap();

            let candidate = Candidate::from_cell_and_digit(cell, digit);

            result.solutions.push(candidate);
        }

        result
    },
};

pub const HIDDEN_SINGLE: Strategy = Strategy {
    name: "Hidden Single",
    find: |board| {
        let mut result = StrategyResult::default();

        for digit in Digit::list() {
            for unit in CELLS_BY_UNIT {
                let candidate_cells = unit
                    .iter()
                    .copied()
                    .filter(|&cell| board.get_content(cell).has_note(digit))
                    .collect_vec();

                if candidate_cells.len() != 1 {
                    continue;
                }

                let cell = *candidate_cells.first().unwrap();

                let candidate = Candidate::from_cell_and_digit(cell, digit);

                result.solutions.push(candidate);
            }
        }

        result
    },
};
