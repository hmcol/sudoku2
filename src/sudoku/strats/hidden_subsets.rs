use std::collections::HashSet;

use itertools::Itertools;

use crate::sudoku::{Board, Candidate, Digit, Unit};

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
        let unsolved_digits_in_unit = unit
            .cells_iter()
            .filter_map(|cell| board.get_notes(&cell))
            .fold(HashSet::new(), |acc, notes| &acc | notes);

        for digit_vec in unsolved_digits_in_unit.into_iter().combinations(N) {
            let digit_set: HashSet<Digit> = digit_vec.into_iter().collect();

            let cell_set = unit
                .cells_iter()
                .filter_map(|cell| board.get_notes(&cell).map(|notes| (cell, notes)))
                .filter(|(_, notes)| !(*notes & &digit_set).is_empty())
                .collect_vec();

            if cell_set.len() != N {
                continue;
            }

            let mut eliminations = Vec::new();

            for (cell, notes) in cell_set {
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
