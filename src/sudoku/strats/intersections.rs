use std::collections::HashSet;

use itertools::Itertools;

use crate::sudoku::{pos::UnitClass, Block, Board, Candidate, Cell, Digit, Line};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const INTERSECTION_POINTING: Strategy = Strategy {
    name: "Intersection Pointing",
    find: find_intersection::<Block, Line>,
};

pub const INTERSECTION_CLAIMING: Strategy = Strategy {
    name: "Intersection Claiming",
    find: find_intersection::<Line, Block>,
};

// =============================================================================

fn find_intersection<Base: UnitClass, Cover: UnitClass>(board: &Board) -> StrategyResult {
    for x in Digit::list() {
        for base_unit in Base::all_vec() {
            let x_base_cells: HashSet<Cell> = base_unit
                .array()
                .iter()
                .filter(|cell| board.has_note(cell, x))
                .copied()
                .collect();

            if x_base_cells.len() < 2 {
                continue;
            }

            let Some(cover_unit) = Cover::all_vec().into_iter()
                .find(|cover| cover.cells_set().is_superset(&x_base_cells))
            else {
                continue;
            };

            let eliminations = cover_unit
                .array()
                .iter()
                .into_iter()
                .filter(|cell| board.has_note(cell, x))
                .filter(|cell| !x_base_cells.contains(cell))
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
