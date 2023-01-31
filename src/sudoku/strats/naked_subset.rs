use std::collections::HashSet;

use itertools::Itertools;

use crate::sudoku::{
    board::Board,
    digit::Digit,
    pos::{Candidate, Cell, CELLS_BY_UNIT},
};

use super::{Strategy, StrategyResult};

#[derive(Clone, Copy, Debug)]
pub struct NakedPair;

impl Strategy for NakedPair {
    const NAME: &'static str = "Naked Pair";

    fn apply(board: &Board) -> StrategyResult {
        naked_subset::<2>(board)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NakedTriple;

impl Strategy for NakedTriple {
    const NAME: &'static str = "Naked triple";

    fn apply(board: &Board) -> StrategyResult {
        naked_subset::<3>(board)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NakedQuad;

impl Strategy for NakedQuad {
    const NAME: &'static str = "Naked quad";

    fn apply(board: &Board) -> StrategyResult {
        naked_subset::<4>(board)
    }
}

fn naked_subset<const N: usize>(board: &Board) -> StrategyResult {
    let mut result = StrategyResult::default();

    for unit in CELLS_BY_UNIT {
        let unsolved_cells: HashSet<Cell> = unit
            .iter()
            .copied()
            .filter(|&cell| board.get_content(cell).is_notes())
            .collect();

        for cell_vec in unsolved_cells.iter().combinations(N) {
            let cell_set: HashSet<Cell> = cell_vec.into_iter().copied().collect();

            let digit_set: HashSet<Digit> = cell_set
                .iter()
                .map(|&cell| board.get_notes_set(cell))
                .reduce(|acc, notes| &acc | &notes)
                .unwrap();

            if digit_set.len() != N {
                continue;
            }

            for &cell in unsolved_cells.difference(&cell_set) {
                let notes = board.get_notes_set(cell);

                let elims = notes
                    .intersection(&digit_set)
                    .map(|&digit| Candidate::from_cell_and_digit(cell, digit));

                result.eliminations.extend(elims);
            }

            if result.is_nontrivial() {
                return result;
            }
        }
    }

    result
}
