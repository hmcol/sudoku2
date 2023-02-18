use yew::prelude::*;

mod block;

mod cell;

mod grid;
use grid::Grid;

mod solver_controls;
use solver_controls::{Solver, SolverControls};

// =============================================================================

pub type SolverHandle = UseReducerHandle<Solver>;

#[function_component]
pub fn App() -> Html {
    // state -------------------------------------------------------------------

    let solver_handle = use_reducer(Solver::new);

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("app")}>
            <ContextProvider<SolverHandle> context={solver_handle.clone()}>
                <Grid />
                <div class={classes!("game-info")}>
                    <SolverControls />
                </div>
            </ContextProvider<SolverHandle>>
        </div>

    }
}
