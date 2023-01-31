use itertools::Itertools;

use crate::sudoku::{
    board::Board,
    digit::Digit,
    pos::{Candidate, Cell, CELLS_BY_UNIT},
};

use super::{Strategy, StrategyResult};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FullHouse;

impl Strategy for FullHouse {
    const NAME: &'static str = "Full House";

    fn apply(board: &Board) -> StrategyResult {
        let mut result = StrategyResult::default();

        for unit in CELLS_BY_UNIT {
            let unsolved_cells: Vec<Cell> = unit
                .iter()
                .filter(|&cell| board.get_content(*cell).is_notes())
                .copied()
                .collect();

            if unsolved_cells.len() == 1 {
                let cell = *unsolved_cells.first().unwrap();

                let notes = board.get_content(cell).get_notes().unwrap_or_else(|| {
                    panic!("Full House strategy failed to get notes for {cell}",)
                });

                let digit = *notes.iter().next().unwrap_or_else(|| {
                    panic!("Full House strategy failed to find a candidate for {cell}",)
                });

                result
                    .solutions
                    .push(Candidate::from_cell_and_digit(cell, digit));
            }
        }

        result
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NakedSingle;

impl Strategy for NakedSingle {
    const NAME: &'static str = "Naked Single";

    fn apply(board: &Board) -> StrategyResult {
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
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HiddenSingle;

impl Strategy for HiddenSingle {
    const NAME: &'static str = "Hidden Single";

    fn apply(board: &Board) -> StrategyResult {
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
    }
}
