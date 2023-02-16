use itertools::Itertools;

use crate::{
    sudoku::{Digit, Unit, Cell},
    util::TryIntoArray, bitset::Set,
};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const FULL_HOUSE: Strategy = Strategy {
    name: "Full House",
    find: |board| {
        let mut solutions = Vec::new();

        for unit in Unit::list() {
            let unsolved_cells: Set<Cell> = unit
                .cells_iter()
                .filter(|&cell| board.get_digit(&cell).is_none())
                .collect();

            let Ok(cell) = unsolved_cells.try_singleton() else {
                continue;
            };

            let notes = board.get_notes(&cell).unwrap();

            let Ok(digit) = notes.try_singleton() else {
                // if all steps were valid, this should never happen.
                // possible if user manually takes invalid step tho
                // should raise some kind of error, but maybe not here
                continue;
            };

            solutions.push((cell, digit).into());
        }

        StrategyResult {
            solutions,
            ..Default::default()
        }
    },
};

pub const NAKED_SINGLE: Strategy = Strategy {
    name: "Naked Single",
    find: |board| {
        let mut result = StrategyResult::default();

        for cell in board.iter_unsolved_cells() {
            let notes = board.get_notes(&cell).unwrap();

            let Ok(digit) = notes.try_singleton() else {
                continue;
            };

            result.solutions.push((cell, digit).into());
        }

        result
    },
};

pub const HIDDEN_SINGLE: Strategy = Strategy {
    name: "Hidden Single",
    find: |board| {
        let mut result = StrategyResult::default();

        for digit in Digit::list() {
            for unit in Unit::list() {
                let candidate_cells = unit
                    .cells_iter()
                    .filter(|cell| board.has_note(cell, digit))
                    .collect_vec();

                let Ok(cell) = candidate_cells.try_singleton() else {
                    continue;
                };

                result.solutions.push((cell, digit).into());
            }
        }

        result
    },
};
