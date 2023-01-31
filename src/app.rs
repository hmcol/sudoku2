use log::info;
use yew::prelude::*;

use crate::sudoku::{board::Board, digit::Digit, pos::Cell, solver::Solver};

mod block;
mod cell;
mod grid;

use grid::Grid;

mod view;

use view::*;

#[function_component]
pub fn App() -> Html {
    let solver_handle = use_state(Solver::new);
    let solver = (*solver_handle).clone();

    let view_handle = use_reducer(View::default);
    let view = (*view_handle).clone();

    let on_click_cell = Callback::<Cell>::from(move |cell| info!("click on cell {cell}"));

    let on_click_digit = Callback::<Digit>::from(move |digit| {
        view_handle.dispatch(ViewAction::SetFocus(Some(digit)))
    });

    let callbacks = ClickCallbacks {
        on_click_cell,
        on_click_digit,
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
        </div>

    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ClickCallbacks {
    on_click_cell: Callback<Cell>,
    on_click_digit: Callback<Digit>,
}
