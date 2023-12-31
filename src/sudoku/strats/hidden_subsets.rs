use itertools::Itertools;

use crate::{
    bitset::Set,
    sudoku::{pos::UnitClass, Board, Cell, Digit, Unit},
};

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
    for unit in Unit::list() {
        let unsolved_digits: Set<Digit> = unit
            .cells_iter()
            .filter_map(|cell| board.get_notes(&cell))
            .sum();

        for digit_vec in unsolved_digits.iter().combinations(N) {
            let cell_set: Set<Cell> = digit_vec
                .iter()
                .map(|&digit| unit.cells_set() & board.cells_with_note(digit))
                .sum();

            if cell_set.len() != N {
                continue;
            }

            let digit_set: Set<Digit> = digit_vec.into_iter().collect();

            let mut eliminations = Set::new();

            for cell in cell_set {
                let notes = board.get_notes(&cell).unwrap();

                eliminations |= (*notes - digit_set).map(|digit| (cell, digit).into());
            }

            if eliminations.is_empty() {
                continue;
            }

            let highlights = cell_set
                .iter()
                .cartesian_product(digit_set.iter())
                .filter(|(cell, digit)| board.has_note(cell, *digit))
                .map(|(cell, digit)| (cell, digit).into())
                .collect();

            return StrategyResult {
                eliminations,
                highlights,
                ..Default::default()
            };
        }
    }

    StrategyResult::default()
}
