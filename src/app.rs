use yew::prelude::*;

use crate::sudoku::{board::Board, solver::Solver, view::View};

mod block;
mod cell;
mod grid;

use grid::Grid;

#[function_component]
pub fn App() -> Html {
    let solver_handle = use_state(Solver::new);
    let solver = (*solver_handle).clone();

    html! {
        <div class={classes!("app")}>
            <ContextProvider<View> context={solver.view}>
                <ContextProvider<Board> context={solver.board}>
                    <Grid />
                </ContextProvider<Board>>
            </ContextProvider<View>>
        </div>

    }
}
