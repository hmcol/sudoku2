use crate::{bitset::Set, sudoku::Candidate};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const REVISE_NOTES: Strategy = Strategy {
    name: "Revise Notes",
    find: |board| {
        let eliminations: Set<Candidate> = board
            .iter_solved()
            .map(|solved_cell| {
                let digit = board.get_digit(&solved_cell).unwrap();

                (solved_cell.neighbors() & board.cells_with_note(digit))
                    .map(|cell| (cell, digit).into())
            })
            .sum();

        StrategyResult {
            eliminations,
            ..Default::default()
        }
    },
};
