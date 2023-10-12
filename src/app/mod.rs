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

    let solver = use_reducer(Solver::new);

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("flex", "bg-orange-200")}>
            <ContextProvider<SolverHandle> context={solver.clone()}>
                <Grid />
                <div class={classes!("w-1/3", "flex", "flex-col", "items-start", "ml-10")}>
                    <SolverControls />
                </div>
            </ContextProvider<SolverHandle>>
        </div>
    }
}
