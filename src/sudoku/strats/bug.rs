use crate::{
    bitset::Set,
    sudoku::{pos::UnitClass, Cell},
    util::TryIntoArray,
};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const BUG_PLUS_1: Strategy = Strategy {
    name: "Bug+1",
    find: |board| {
        let non_bivalue_cells: Set<Cell> = board
            .iter_unsolved()
            .filter(|cell| board.count_notes(cell) != 2)
            .collect();

        let Ok(bug_cell) = non_bivalue_cells.try_singleton() else {
            return StrategyResult::default();
        };

        let bug_notes = board.get_notes(&bug_cell).unwrap();

        if bug_notes.len() != 3 {
            return StrategyResult::default();
        }

        let bug_digit = bug_notes.iter().find(|&digit| {
            bug_cell
                .units()
                .iter()
                .all(|unit| (unit.cells_set() & board.cells_with_note(digit)).len() == 3)
        });

        let Some(bug_digit) = bug_digit else {
            return StrategyResult::default();
        };

        StrategyResult {
            solutions: vec![(bug_cell, bug_digit).into()],
            ..Default::default()
        }
    },
};
