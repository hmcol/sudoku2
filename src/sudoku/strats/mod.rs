use std::fmt::Debug;

use crate::sudoku::board::Board;

use self::naked_subset::{NakedPair, NakedQuad, NakedTriple};
use self::revise_notes::ReviseNotes;
use self::singles::{FullHouse, HiddenSingle, NakedSingle};

use super::pos::Candidate;

mod naked_subset;
mod revise_notes;
mod singles;

type StrategyFn = fn(&Board) -> StrategyResult;

trait Strategy: Copy + Debug {
    const NAME: &'static str;
    fn apply(board: &Board) -> StrategyResult;
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StrategyResult {
    solutions: Vec<Candidate>,
    eliminations: Vec<Candidate>,
    highlights: Vec<Candidate>,
    highlights2: Vec<Candidate>,
}

impl StrategyResult {
    pub fn is_nontrivial(&self) -> bool {
        !self.solutions.is_empty() || !self.eliminations.is_empty()
    }
}

macro_rules! strat_tuples {
    [$($name:ident),* $(,)?] => {
        [
            $( ($name::NAME, $name::apply), )*
        ]

    };
}

const STRATEGY_LIST: &[(&str, StrategyFn)] = &strat_tuples![
    ReviseNotes,
    FullHouse,
    NakedSingle,
    HiddenSingle,
    NakedPair,
    NakedTriple,
    NakedQuad,
];
