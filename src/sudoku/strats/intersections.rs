use crate::sudoku::{pos::UnitClass, Block, Board, Digit, Line};

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
        let x_cells = board.cells_with_note(x);

        for base in Base::iter_all() {
            let base_cells = base.cells_set() & x_cells;

            if base_cells.len() < 2 {
                continue;
            }

            let cover_cells = Cover::iter_all()
                .map(UnitClass::cells_set)
                .find(|cover_cells| &base_cells <= cover_cells);

            let Some(cover_cells) = cover_cells else { continue };

            let eliminations = ((cover_cells & x_cells) - base_cells).map(|cell| (cell, x).into());

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
