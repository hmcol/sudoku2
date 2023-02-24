use crate::{
    bitset::Set,
    sudoku::{Board, Candidate},
};

// import strategies ===========================================================

mod revise_notes;
use revise_notes::REVISE_NOTES;

mod singles;
use singles::{FULL_HOUSE, HIDDEN_SINGLE, NAKED_SINGLE};

mod naked_subsets;
use naked_subsets::{NAKED_PAIR, NAKED_QUAD, NAKED_TRIPLE};

mod hidden_subsets;
use hidden_subsets::{HIDDEN_PAIR, HIDDEN_QUAD, HIDDEN_TRIPLE};

mod intersections;
use intersections::{INTERSECTION_CLAIMING, INTERSECTION_POINTING};

mod fish;
use fish::{JELLYFISH, SWORDFISH, X_WING};

mod bug;
use bug::BUG_PLUS_1;

// -----------------------------------------------------------------------------

/// list of all strategies in default order
pub const STRATEGY_LIST: &[Strategy] = &[
    REVISE_NOTES,
    FULL_HOUSE,
    NAKED_SINGLE,
    HIDDEN_SINGLE,
    NAKED_PAIR,
    HIDDEN_PAIR,
    INTERSECTION_POINTING,
    INTERSECTION_CLAIMING,
    NAKED_TRIPLE,
    HIDDEN_TRIPLE,
    NAKED_QUAD,
    HIDDEN_QUAD,
    X_WING,
    SWORDFISH,
    JELLYFISH,
    BUG_PLUS_1,
];

/// checklist:
/// - skyscraper,
/// - kite,
/// - turbotFish,
/// - xChainSimple,
/// - xChain,
/// - xyWing,
/// - xyzWing,
/// - wWing,
/// - ur1,
/// - ur2,
/// - ur4,
/// - ur5,
/// - hiddenRectangle,
/// - bugPlusOne,
/// - xyChain,
/// - aic,

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

impl std::fmt::Debug for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StrategyObject")
            .field("name", &self.name)
            .finish()
    }
}

// result ----------------------------------------------------------------------

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StrategyResult {
    pub solutions: Set<Candidate>,
    pub eliminations: Set<Candidate>,
    pub highlights: Set<Candidate>,
    pub highlights2: Set<Candidate>,
}

impl Default for StrategyResult {
    fn default() -> Self {
        Self {
            solutions: Set::new(),
            eliminations: Set::new(),
            highlights: Set::new(),
            highlights2: Set::new(),
        }
    }
}

impl StrategyResult {
    pub fn is_nontrivial(&self) -> bool {
        !self.solutions.is_empty() || !self.eliminations.is_empty()
    }
}
