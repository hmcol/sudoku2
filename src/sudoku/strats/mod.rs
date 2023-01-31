use std::fmt::Debug;

use crate::sudoku::board::Board;

use self::naked_subset::{NAKED_PAIR, NAKED_QUAD, NAKED_TRIPLE};
use self::revise_notes::REVISE_NOTES;
use self::singles::{FULL_HOUSE, HIDDEN_SINGLE, NAKED_SINGLE};

use super::pos::Candidate;

mod naked_subset;
mod revise_notes;
mod singles;

type StrategyFn = fn(&Board) -> StrategyResult;

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

#[derive(Clone, Copy)]
pub struct Strategy {
    pub name: &'static str,
    pub find: StrategyFn,
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

pub const STRATEGY_LIST: &[Strategy] = &[
    REVISE_NOTES,
    FULL_HOUSE,
    NAKED_SINGLE,
    HIDDEN_SINGLE,
    NAKED_PAIR,
    NAKED_TRIPLE,
    NAKED_QUAD,
];
