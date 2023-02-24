use itertools::Itertools;

use crate::{
    bitset::Set,
    sudoku::{pos::UnitClass, Board, Candidate, Cell, Digit, Unit},
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
        let unsolved_cells = unit.cells_set() & board.cells_unsolved();

        for cell_vec in unsolved_cells.iter().combinations(N) {
            let digit_set: Set<Digit> = cell_vec
                .iter()
                .map(|cell| board.get_notes(cell).unwrap())
                .sum();

            if digit_set.len() != N {
                continue;
            }

            let cell_set: Set<Cell> = cell_vec.into_iter().collect();

            let eliminations: Set<Candidate> = (unsolved_cells - cell_set)
                .iter()
                .map(|cell| {
                    let notes = board.get_notes(&cell).unwrap();

                    (*notes & digit_set).map(|digit| (cell, digit).into())
                })
                .sum();

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
