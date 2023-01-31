use std::fmt::Debug;

use crate::sudoku::{Board, Candidate};

// import strategies -----------------------------------------------------------

mod naked_subset;
use naked_subset::{NAKED_PAIR, NAKED_QUAD, NAKED_TRIPLE};

mod revise_notes;
use self::revise_notes::REVISE_NOTES;

mod singles;
use self::singles::{FULL_HOUSE, HIDDEN_SINGLE, NAKED_SINGLE};

// -----------------------------------------------------------------------------

pub const STRATEGY_LIST: &[Strategy] = &[
    REVISE_NOTES,
    FULL_HOUSE,
    NAKED_SINGLE,
    HIDDEN_SINGLE,
    NAKED_PAIR,
    NAKED_TRIPLE,
    NAKED_QUAD,
];

// strategy --------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct Strategy {
    pub name: &'static str,
    pub find: fn(&Board) -> StrategyResult,
}

impl PartialEq for Strategy {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Strategy {}

impl Debug for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StrategyObject")
            .field("name", &self.name)
            .finish()
    }
}

// result ----------------------------------------------------------------------

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StrategyResult {
    pub solutions: Vec<Candidate>,
    pub eliminations: Vec<Candidate>,
    pub highlights: Vec<Candidate>,
    pub highlights2: Vec<Candidate>,
}

impl StrategyResult {
    pub fn is_nontrivial(&self) -> bool {
        !self.solutions.is_empty() || !self.eliminations.is_empty()
    }
}
