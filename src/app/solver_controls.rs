use std::rc::Rc;

use yew::prelude::*;

pub use crate::sudoku::{Solver, SolverAction};

use super::SolverHandle;

// =============================================================================

#[function_component]
pub fn SolverControls() -> Html {
    // get contexts ------------------------------------------------------------

    let solver = use_context::<SolverHandle>().expect("Solver context not found");

    // build callbacks ---------------------------------------------------------

    let on_reset: Callback<MouseEvent> = {
        let solver = solver.clone();
        Callback::from(move |_| solver.dispatch(SolverAction::Reset))
    };

    let on_undo: Callback<MouseEvent> = {
        let solver = solver.clone();
        Callback::from(move |_| solver.dispatch(SolverAction::Undo))
    };

    let on_step: Callback<MouseEvent> = {
        // let solver = solver.clone();
        Callback::from(move |_| solver.dispatch(SolverAction::Step))
    };

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("solver-controls")}>
            <button onclick={on_reset}>{"reset"}</button>
            <button onclick={on_undo}>{"undo"}</button>
            <button onclick={on_step}>{"step"}</button>
        </div>
    }
}

impl Reducible for Solver {
    type Action = SolverAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let solver = (*self).clone();

        solver.take_action(action).into()
    }
}
