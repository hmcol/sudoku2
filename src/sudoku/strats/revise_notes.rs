use crate::sudoku::{board::Board, pos::Candidate};

use super::{Strategy, StrategyResult};

#[derive(Clone, Copy, Debug)]
pub struct ReviseNotes;

impl Strategy for ReviseNotes {
    const NAME: &'static str = "Revise Notes";

    fn apply(board: &Board) -> StrategyResult {
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
    }
}
