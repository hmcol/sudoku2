use std::collections::HashSet;

use itertools::Itertools;

use crate::sudoku::{Board, Candidate, Digit};

use super::{Strategy, StrategyResult};

// =============================================================================

pub const INTERSECTION_POINTING: Strategy = Strategy {
    name: "Intersection Pointing",
    find: find_intersection::<2>,
};

pub const INTERSECTION_CLAIMING: Strategy = Strategy {
    name: "Intersection Claiming",
    find: find_intersection::<3>,
};

// =============================================================================

fn find_intersection<const N: usize>(board: &Board) -> StrategyResult {
    
    StrategyResult::default()
}
