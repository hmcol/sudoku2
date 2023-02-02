use std::collections::HashSet;

use itertools::Itertools;

use crate::sudoku::{pos::CELLS_BY_UNIT, Board, Candidate, Cell, Digit};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const NAKED_PAIR: Strategy = Strategy {
    name: "Naked Pair",
    find: find_naked_subset::<2>,
};

pub const NAKED_TRIPLE: Strategy = Strategy {
    name: "Naked Triple",
    find: find_naked_subset::<3>,
};

pub const NAKED_QUAD: Strategy = Strategy {
    name: "Naked Quad",
    find: find_naked_subset::<4>,
};

// =============================================================================

fn find_naked_subset<const N: usize>(board: &Board) -> StrategyResult {
    for unit in CELLS_BY_UNIT {
        let unsolved_cells: HashSet<Cell> = unit
            .iter()
            .copied()
            .filter(|cell| board.is_notes(cell))
            .collect();

        for cell_vec in unsolved_cells.iter().combinations(N) {
            let cell_set: HashSet<Cell> = cell_vec.into_iter().copied().collect();

            let digit_set: HashSet<Digit> = cell_set
                .iter()
                .map(|cell| board.get_notes(cell).unwrap())
                .fold(HashSet::new(), |acc, notes| &acc | notes);

            if digit_set.len() != N {
                continue;
            }

            let mut eliminations = Vec::new();

            for &cell in unsolved_cells.difference(&cell_set) {
                let notes = board.get_notes(&cell).unwrap();

                let elims = notes
                    .intersection(&digit_set)
                    .map(|&digit| Candidate::from_cell_and_digit(cell, digit));

                eliminations.extend(elims);
            }

            if eliminations.is_empty() {
                continue;
            }

            return StrategyResult {
                eliminations,
                ..Default::default()
            };
        }
    }

    StrategyResult::default()
}
