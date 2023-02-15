use itertools::Itertools;

use crate::{
    bitset::Set,
    sudoku::{pos::UnitClass, Board, Candidate, Cell, Unit},
};

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
    for unit in Unit::list() {
        let unsolved_cells: Set<Cell> = unit
            .array()
            .iter()
            .filter(|cell| board.is_notes(cell))
            .copied()
            .collect();

        for cell_vec in unsolved_cells.iter().combinations(N) {
            let cell_set: Set<Cell> = cell_vec.into_iter().collect();

            let digit_set = cell_set
                .iter()
                .map(|cell| board.get_notes(&cell).unwrap())
                .fold(Set::new(), |acc, &notes| acc | notes);

            if digit_set.len() != N {
                continue;
            }

            let mut eliminations = Vec::new();

            for cell in (unsolved_cells - cell_set).iter() {
                let notes = board.get_notes(&cell).unwrap();

                let elims = (*notes & digit_set)
                    .iter()
                    .map(|digit| Candidate::from_cell_and_digit(cell, digit));

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
