use std::collections::HashSet;

use itertools::Itertools;

use crate::sudoku::{pos::CELLS_BY_UNIT, Board, Candidate, Cell, Digit};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const HIDDEN_PAIR: Strategy = Strategy {
    name: "Hidden Pair",
    find: find_hidden_subset::<2>,
};

pub const HIDDEN_TRIPLE: Strategy = Strategy {
    name: "Hidden Triple",
    find: find_hidden_subset::<3>,
};

pub const HIDDEN_QUAD: Strategy = Strategy {
    name: "Hidden Quad",
    find: find_hidden_subset::<4>,
};

// =============================================================================

fn find_hidden_subset<const N: usize>(board: &Board) -> StrategyResult {
    for unit in CELLS_BY_UNIT {
        let unsolved_cells: HashSet<Cell> = unit
            .iter()
            .copied()
            .filter(|cell| board.is_notes(cell))
            .collect();

        for digit_vec in Digit::list().combinations(N) {
            let digit_set: HashSet<Digit> = digit_vec.into_iter().collect();

            let cell_set: HashSet<Cell> = unsolved_cells
                .iter()
                .filter(|cell| board.get_notes(cell).unwrap().is_subset(&digit_set))
                .copied()
                .collect();

            if cell_set.len() != N {
                continue;
            }

            let mut eliminations = Vec::new();

            for cell in cell_set {
                let notes = board.get_notes(&cell).unwrap();

                let elims = notes
                    .difference(&digit_set)
                    .map(|&digit| Candidate::from_cell_and_digit(cell, digit));

                eliminations.extend(elims);
            }

            if eliminations.is_empty() {
                continue;
            }

            return StrategyResult {
                eliminations,
                ..StrategyResult::default()
            };
        }
    }

    StrategyResult::default()
}
