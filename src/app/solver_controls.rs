use std::rc::Rc;

use yew::prelude::*;

pub use crate::sudoku::solver::{Action as SolverAction, Solver};

#[derive(PartialEq, Properties)]
pub struct SolverControlsProps {
    pub on_reset: Callback<()>,
    pub on_step: Callback<()>,
    pub on_undo: Callback<()>,
}

#[function_component]
pub fn SolverControls(props: &SolverControlsProps) -> Html {
    let SolverControlsProps {
        on_reset,
        on_step,
        on_undo,
    } = props;

    html! {
        <div class={classes!("solver-controls")}>
            { button("reset", on_reset) }
            { button("undo", on_undo) }
            { button("step", on_step) }
        </div>
    }
}

fn button(text: &str, on_click: &Callback<()>) -> Html {
    let on_click = on_click.clone();

    let on_click = Callback::<MouseEvent>::from(move |_| on_click.emit(()));

    html! {
        <button onclick={on_click}>{text}</button>
    }
}

impl Reducible for Solver {
    type Action = SolverAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let solver = (*self).clone();

        solver.take_action(action).into()
    }
}
