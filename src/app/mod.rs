use std::ops::Deref;

use log::info;
use yew::prelude::*;

use crate::sudoku::{Cell, Digit};

mod block;

mod cell;

mod grid;
use grid::Grid;

mod solver_controls;
use solver_controls::{Solver, SolverAction, SolverControls};

pub type SolverHandle = UseReducerHandle<Solver>;

#[function_component]
pub fn App() -> Html {
    // state -------------------------------------------------------------------

    let solver_handle = use_reducer(Solver::new);
    // let solver = solver_handle.deref();

    // callbacks ---------------------------------------------------------------

    let on_click_cell = Callback::<Cell>::from(move |cell| info!("click on cell {cell}"));

    let on_click_digit: Callback<Digit> = {
        let solver_handle = solver_handle.clone();
        Callback::from(move |digit| solver_handle.dispatch(SolverAction::SetFocus(Some(digit))))
    };

    let click_callbacks = ClickCallbacks {
        on_click_cell,
        on_click_digit,
    };

    // solver controls callbacks -----------------------------------------------

    let on_reset: Callback<()> = {
        let solver_handle = solver_handle.clone();
        Callback::from(move |_| solver_handle.dispatch(SolverAction::Reset))
    };

    let on_undo: Callback<()> = {
        let solver_handle = solver_handle.clone();
        Callback::from(move |_| solver_handle.dispatch(SolverAction::Undo))
    };

    let on_step: Callback<()> = {
        let solver_handle = solver_handle.clone();
        Callback::from(move |_| solver_handle.dispatch(SolverAction::Step))
    };

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("app")}>
            <ContextProvider<SolverHandle> context={solver_handle.clone()}>
                <ContextProvider<ClickCallbacks> context={click_callbacks}>
                    <Grid />
                </ContextProvider<ClickCallbacks>>
                <div class={classes!("game-info")}>
                    <SolverControls
                        {on_reset}
                        {on_step}
                        {on_undo}
                    />
                </div>
            </ContextProvider<SolverHandle>>
        </div>

    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ClickCallbacks {
    on_click_cell: Callback<Cell>,
    on_click_digit: Callback<Digit>,
}
