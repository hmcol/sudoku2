use log::{debug, info};

use super::{board::Board, pos::Candidate, strats::{Strategy, STRATEGY_LIST, StrategyResult}};

pub enum Action {
    Reset,
    Undo,
    Step,
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Solver {
    history: Vec<Board>,
    pub board: Board,
    strategies: Vec<Strategy>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            history: Vec::new(),
            board: Board::from_string(
                "607005010580007900000060000005000009000936000300000400000080000003600094050200806",
            ),
            strategies: STRATEGY_LIST.to_vec(),
        }
    }

    // -------------------------------------------------------------------------

    pub fn take_action(mut self, action: Action) -> Self {
        match action {
            Action::Reset => self.reset(),
            Action::Undo => self.undo(),
            Action::Step => self.step(),
        }
    }

    pub fn reset(mut self) -> Self {
        self.history.clear();

        self.board.reset();

        self
    }

    pub fn undo(mut self) -> Self {
        if let Some(board) = self.history.pop() {
            self.board = board;
        }

        self
    }

    pub fn step(mut self) -> Self {
        self.find_next_strategy();

        info!("Solver took a step.");

        self
    }

    // -------------------------------------------------------------------------

    fn remember_board(&mut self) {
        self.history.push(self.board.clone());
    }

    pub fn input_solutions(&mut self, solutions: Vec<Candidate>) {
        self.remember_board();

        for solution in solutions {
            self.board.input_solution(solution);
        }
    }

    // -------------------------------------------------------------------------

    pub fn find_next_strategy(&mut self) -> Option<StrategyResult> {
        for strategy in &self.strategies {
            let result = (strategy.find)(&self.board);

            if result.is_nontrivial() {
                debug!("Found strategy: {}", strategy.name);
                debug!("Result: {:#?}", result);

                return Some(result)
            }
        }

        

        None
    }
}
