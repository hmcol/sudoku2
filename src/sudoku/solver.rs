use log::{error, info};
use stdweb::web::Date;

use super::{Board, Digit, Strategy, StrategyResult, STRATEGY_LIST};

// =============================================================================

pub enum Action {
    Reset,
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
    pub board: Board,
    pub result: Option<StrategyResult>,
    pub focus_digit: Option<Digit>,
}

impl Solver {
    // constructors ------------------------------------------------------------

    pub fn new() -> Self {
        Solver {
            history: Vec::new(),
            board: Board::from_string(
                "607005010580007900000060000005000009000936000300000400000080000003600094050200806",
            ),
            strategies: STRATEGY_LIST.to_vec(),
            result: None,
            focus_digit: None,
        }
    }

    // actions -----------------------------------------------------------------

    pub fn take_action(mut self, action: Action) -> Self {
        match action {
            Action::Reset => self.reset(),
            Action::Undo => self.undo(),
            Action::Step => self.step(),
            Action::SetFocus(digit) => self.set_focus(digit),
        }

        self
    }

    fn reset(&mut self) {
        self.history.clear();
        self.board.reset();
        self.result = None;
        self.focus_digit = None;
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
        self.focus_digit = digit;
    }

    // mutating action helpers -------------------------------------------------

    fn remember_board(&mut self) {
        self.history.push(self.board.clone());
    }

    fn find_next_strategy(&mut self) {
        for strategy in &self.strategies {
            let result = (strategy.find)(&self.board);

            if result.is_nontrivial() {
                info!("Found strategy: {}", strategy.name);
                // debug!("Result: {:#?}", result);

                self.result = Some(result);
                return;
            }
        }

        info!("no strategy found");
    }

    fn apply_current_result(&mut self) {
        // .take() takes ownership of the result, leaving self.result as None.
        // This is necessary because we need to borrow self.result mutably in
        // order to call self.remember_board().
        let Some(result) = self.result.take() else {
            error!("Solver::apply_current_result() called with no result");
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
