use crate::sudoku::Candidate;

use super::{Strategy, StrategyResult};

pub const REVISE_NOTES: Strategy = Strategy {
    name: "Revise Notes",
    find: |board| {
        let mut result = StrategyResult::default();

        for cell in board.iter_unsolved_cells() {
            let elims = board
                .get_notes_set(cell)
                .into_iter()
                .filter(|&digit| {
                    cell.iter_neighbors()
                        .any(|neighbor| board.get_digit(neighbor) == Some(digit))
                })
                .map(|digit| Candidate::from_cell_and_digit(cell, digit));

            result.eliminations.extend(elims);
        }

        result
    },
};
