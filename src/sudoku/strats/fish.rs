use itertools::Itertools;

use crate::{
    bitset::Set,
    sudoku::{pos::UnitClass, Board, Cell, Col, Digit, Row},
};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const X_WING: Strategy = Strategy {
    name: "X-Wing",
    find: find_basic_fish::<2>,
};

pub const SWORDFISH: Strategy = Strategy {
    name: "Swordfish",
    find: find_basic_fish::<3>,
};
pub const JELLYFISH: Strategy = Strategy {
    name: "Jellyfish",
    find: find_basic_fish::<4>,
};

// =============================================================================

fn find_basic_fish<const N: usize>(board: &Board) -> StrategyResult {
    let result = find_fish::<N, Row, Col>(board);

    if result.is_nontrivial() {
        return result;
    }

    find_fish::<N, Col, Row>(board)
}

fn find_fish<const N: usize, Base: UnitClass, Cover: UnitClass>(board: &Board) -> StrategyResult {
    for x in Digit::list() {
        for base_cells in Base::iter_all()
            .map(|unit| unit.cells_set() & board.cells_with_note(x))
            .filter(Set::is_nonempty)
            .combinations(N)
            .map(|units| units.into_iter().sum::<Set<Cell>>())
        {
            let cover_cells = Cover::iter_all()
                .map(|unit| unit.cells_set() & board.cells_with_note(x))
                .filter(Set::is_nonempty)
                .combinations(N)
                .map(|cover_units| cover_units.into_iter().sum())
                .find(|cover_cells| &base_cells <= cover_cells);

            let Some(cover_cells) = cover_cells else {
                continue;
            };

            let elim_set = cover_cells - base_cells;

            if elim_set.is_empty() {
                continue;
            }

            let eliminations = elim_set.iter().map(|cell| (cell, x).into()).collect();

            return StrategyResult {
                eliminations,
                ..Default::default()
            };
        }
    }

    StrategyResult::default()
}
