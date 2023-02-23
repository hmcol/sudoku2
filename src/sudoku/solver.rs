use crate::bitset::{Set, Element};

use super::{Board, Digit, Strategy, StrategyResult, STRATEGY_LIST, Cell};

// =============================================================================

pub enum Action {
    Reset,
    LoadBoardString(String),
    Undo,
    Step,
    SetFocus(Option<Digit>),
}

// =============================================================================

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Solver {
    // private
    strategies: Vec<Strategy>,
    history: Vec<Board>,
    // public
    pub given: Set<Cell>,
    pub board: Board,
    pub result: Option<StrategyResult>,
    pub focus_digit: Option<Digit>,
}

impl Solver {
    // constructors ------------------------------------------------------------

    pub fn new() -> Self {
        Solver {
            strategies: STRATEGY_LIST.to_vec(),
            history: Vec::new(),
            given: Set::new(),
            board: Board::new(),
            result: None,
            focus_digit: None,
        }
    }

    // actions -----------------------------------------------------------------

    pub fn take_action(mut self, action: Action) -> Self {
        match action {
            Action::Reset => self.reset(),
            Action::LoadBoardString(string) => self.load_board_string(&string),
            Action::Undo => self.undo(),
            Action::Step => self.step(),
            Action::SetFocus(digit) => self.set_focus(digit),
        }

        self
    }

    fn reset(&mut self) {
        self.history.clear();
        self.board.clear();
        self.result = None;
        self.focus_digit = None;
    }

    fn load_board_string(&mut self, string: &str) {
        self.reset();

        for cell in Cell::list() {
            let i = cell.index();
            let digit = string.get(i..(i + 1)).and_then(|s| s.parse().ok());

            if let Some(digit) = digit {
                self.given.insert(cell);
                self.board.set_digit(cell, digit);
            }
        }        
    }

    fn undo(&mut self) {
        if let Some(board) = self.history.pop() {
            self.board = board;
        }
    }

    fn step(&mut self) {
        // let start = Date::now();

        match self.result {
            Some(_) => self.apply_current_result(),
            None => self.find_next_strategy(),
        }

        // let end = Date::now();
        // let elapsed_time = end - start;
        // info!("step took: {:?}", elapsed_time);
    }

    fn set_focus(&mut self, digit: Option<Digit>) {
        self.focus_digit = (digit != self.focus_digit).then_some(digit).flatten();
    }

    // mutating action helpers -------------------------------------------------

    fn remember_board(&mut self) {
        self.history.push(self.board.clone());
    }

    fn find_next_strategy(&mut self) {
        for strategy in &self.strategies {
            // debug!("Trying strategy: {}", strategy.name);

            let result = (strategy.find)(&self.board);

            if result.is_nontrivial() {
                log::info!("Found strategy: {}", strategy.name);
                // debug!("Result: {:#?}", result);

                self.result = Some(result);
                return;
            }
        }

        log::info!("no strategy found");
    }

    fn apply_current_result(&mut self) {
        // .take() takes ownership of the result, leaving self.result as None.
        // This is necessary because we need to borrow self.result mutably in
        // order to call self.remember_board().
        let Some(result) = self.result.take() else {
            log::error!("Solver::apply_current_result() called with no result");
            return;
        };

        self.remember_board();

        for solution in result.solutions {
            self.board.input_solution(solution);
        }

        for elimination in result.eliminations {
            self.board.input_elimination(elimination);
        }
    }
}
