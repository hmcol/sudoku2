use log::info;
use yew::prelude::*;

use crate::sudoku::{
    board::Board,
    pos::{Cell, Digit},
};

mod block;

mod cell;

mod grid;
use grid::Grid;

mod view;
use view::*;

mod solver_controls;
use solver_controls::{Solver, SolverAction, SolverControls};

#[function_component]
pub fn App() -> Html {
    let solver_handle = use_reducer(Solver::new);
    let solver = (*solver_handle).clone();

    let view_handle = use_reducer(View::default);
    let view = (*view_handle).clone();

    let on_click_cell = Callback::<Cell>::from(move |cell| info!("click on cell {cell}"));

    let on_click_digit: Callback<Digit> = {
        let view_handle = view_handle.clone();
        Callback::from(move |digit| view_handle.dispatch(ViewAction::SetFocus(Some(digit))))
    };

    let callbacks = ClickCallbacks {
        on_click_cell,
        on_click_digit,
    };

    let on_reset: Callback<()> = {
        let solver_handle = solver_handle.clone();
        let view_handle = view_handle.clone();
        Callback::from(move |_| {
            solver_handle.dispatch(SolverAction::Reset);
            view_handle.dispatch(ViewAction::Reset);
        })
    };

    let on_undo: Callback<()> = {
        let solver_handle = solver_handle.clone();
        Callback::from(move |_| solver_handle.dispatch(SolverAction::Undo))
    };

    let on_step: Callback<()> = {
        let solver_handle = solver_handle.clone();
        Callback::from(move |_| solver_handle.dispatch(SolverAction::Step))
    };

    html! {
        <div class={classes!("app")}>
            <ContextProvider<ClickCallbacks> context={callbacks}>
            <ContextProvider<View> context={view}>
                <ContextProvider<Board> context={solver.board}>
                    <Grid />
                </ContextProvider<Board>>
            </ContextProvider<View>>
            </ContextProvider<ClickCallbacks>>
            <div class={classes!("game-info")}>
                <SolverControls
                    {on_reset}
                    {on_step}
                    {on_undo}
                />
            </div>
        </div>

    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ClickCallbacks {
    on_click_cell: Callback<Cell>,
    on_click_digit: Callback<Digit>,
}
