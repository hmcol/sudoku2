use crate::{bitset::Set, sudoku::Candidate};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const REVISE_NOTES: Strategy = Strategy {
    name: "Revise Notes",
    find: |board| {
        let mut elims = Set::<Candidate>::new();

        for cell in board.iter_solved() {
            let x = board.get_digit(&cell).unwrap();

            elims |= (cell.neighbors() & board.cells_with_note(x)).map(|cell| (cell, x).into());
        }

        StrategyResult {
            eliminations: elims.into_iter().collect(),
            ..Default::default()
        }
    },
};
