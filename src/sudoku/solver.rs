use super::{board::Board, pos::Candidate, view::View};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Solver {
    history: Vec<Board>,
    pub board: Board,
    pub view: View,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            history: Vec::new(),
            board: Board::from_string(
                "607005010580007900000060000005000009000936000300000400000080000003600094050200806",
            ),
            view: View::default(),
        }
    }

    pub fn reset(mut self) -> Self {
        self.history.clear();

        self.board.reset();

        self
    }

    //

    fn remember_board(&mut self) {
        self.history.push(self.board.clone());
    }

    pub fn input_solutions(&mut self, solutions: Vec<Candidate>) {
        self.remember_board();

        for solution in solutions {
            self.board.input_solution(solution);
        }
    }
}
