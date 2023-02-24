use crate::{
    bitset::Set,
    sudoku::{pos::UnitClass, Digit, Unit},
    util::TryIntoArray,
};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const FULL_HOUSE: Strategy = Strategy {
    name: "Full House",
    find: |board| {
        let mut solutions = Set::new();

        for unit in Unit::list() {
            let unsolved_cells = unit.cells_set() & board.cells_unsolved();

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

            solutions.insert((cell, digit).into());
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
        let mut solutions = Set::new();

        for cell in board.iter_unsolved() {
            let notes = board.get_notes(&cell).unwrap();

            let Ok(digit) = notes.try_singleton() else { continue };

            solutions.insert((cell, digit).into());
        }

        StrategyResult {
            solutions,
            ..Default::default()
        }
    },
};

pub const HIDDEN_SINGLE: Strategy = Strategy {
    name: "Hidden Single",
    find: |board| {
        let mut solutions = Set::new();

        for x in Digit::list() {
            for unit in Unit::list() {
                let x_cells = unit.cells_set() & board.cells_with_note(x);

                let Ok(cell) = x_cells.try_singleton() else { continue };

                solutions.insert((cell, x).into());
            }
        }

        StrategyResult {
            solutions,
            ..Default::default()
        }
    },
};
