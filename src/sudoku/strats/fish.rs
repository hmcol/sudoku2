use std::collections::HashSet;

use itertools::Itertools;

use crate::sudoku::{pos::UnitClass, Board, Cell, Col, Digit, Row, Candidate};

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
        let base_units_with_x = Base::all_vec()
            .into_iter()
            .filter(|base_unit| {
                base_unit
                    .cells_vec()
                    .into_iter()
                    .any(|cell| board.has_note(&cell, x))
            })
            .collect_vec();

        for base_units in base_units_with_x.into_iter().combinations(N) {
            let base_cells: HashSet<Cell> = base_units
                .into_iter()
                .flat_map(|base_unit| base_unit.cells_vec())
                .collect();

            let cover_units = Cover::all_vec()
                .into_iter()
                .combinations(N)
                .find(|cover_units| {
                    let cover_cells_set: HashSet<Cell> = cover_units
                        .iter()
                        .flat_map(|cover_unit| cover_unit.cells_vec())
                        .collect();

                    cover_cells_set.is_superset(&base_cells)
                });

            let Some(cover_units) = cover_units else {
                continue;
            };

            let cover_cells: HashSet<Cell> = cover_units
                .into_iter()
                .flat_map(|cover_unit| cover_unit.cells_vec())
                .collect();

            let eliminations = cover_cells
                .difference(&base_cells)
                .filter(|cell| board.has_note(cell, x))
                .copied()
                .map(|cell| Candidate::from_cell_and_digit(cell, x))
                .collect_vec();
            
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
