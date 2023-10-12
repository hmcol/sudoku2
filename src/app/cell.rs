use yew::prelude::*;

use crate::{
    bitset::Set,
    sudoku::{Candidate, Cell, CellData, Digit, SolverAction},
};

use super::SolverHandle;

// =============================================================================

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell: Cell,
}

#[function_component]
pub fn CellComponent(props: &CellProps) -> Html {
    // get contexts ------------------------------------------------------------

    let solver = use_context::<SolverHandle>().expect("Solver context not found");

    // read props --------------------------------------------------------------

    let cell = props.cell;

    // derive attributes -------------------------------------------------------

    let lowlight = if let Some(focus_digit) = solver.focus_digit {
        match solver.board.get_data(&cell) {
            CellData::Digit(digit) => *digit != focus_digit,
            CellData::Notes(notes) => !notes.contains(focus_digit),
        }
    } else {
        false
    }
    .then_some("bg-dark");

    let on_click = Callback::<MouseEvent>::from(|_| ());

    let content = match solver.board.get_data(&cell) {
        CellData::Digit(digit) => {
            let is_given = solver.given.contains(cell);

            html! {
                <CellDigit digit={*digit} {is_given} />
            }
        }
        CellData::Notes(notes) => {
            html! {
                <CellNotes notes={*notes} />
            }
        }
    };

    // render ------------------------------------------------------------------

    html! {
        <ContextProvider<Cell> context={cell}>
            <div class={classes!("w-8", "h-8", "overflow-hidden", "text-xl", "text-other", "bg-base", "select-none", lowlight)}
                onclick={on_click}
            >
                { content }
            </div>
        </ContextProvider<Cell>>
    }
}

#[derive(Properties, PartialEq)]
struct CellDigitProps {
    digit: Digit,
    is_given: bool,
}

#[function_component]
fn CellDigit(props: &CellDigitProps) -> Html {
    // get contexts ------------------------------------------------------------

    let solver = use_context::<SolverHandle>().expect("Solver context not found");

    // read props --------------------------------------------------------------

    let digit = props.digit;

    // derive attributes -------------------------------------------------------

    let given = props.is_given.then_some(classes!("font-bold", "text-light", "bg-dark"));
    let focus = (solver.focus_digit == Some(digit)).then_some("bg-focus-red");

    let on_click: Callback<MouseEvent> = {
        let solver = solver;
        Callback::from(move |_| solver.dispatch(SolverAction::SetFocus(Some(digit))))
    };

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("w-full", "h-full", "flex", "items-center", "justify-center", given, focus)}
            onclick={on_click}
        >
            { digit.to_string() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CellNotesProps {
    notes: Set<Digit>,
}

#[function_component]
fn CellNotes(props: &CellNotesProps) -> Html {
    let notes = Digit::list().map(|digit| {
        let is_shown = props.notes.contains(digit);

        html! {
            <Note {digit} {is_shown} />
        }
    });

    html! {
        <div class={classes!("w-full", "h-full", "grid", "grid-cols-3", "grid-rows-3")}>
            { for notes }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct NoteProps {
    digit: Digit,
    is_shown: bool,
}

#[function_component]
fn Note(props: &NoteProps) -> Html {
    // use contexts -----------------------------------------------------------

    let solver = use_context::<SolverHandle>().expect("Solver context not found");
    let cell = use_context::<Cell>().expect("Cell context not found");

    // read props --------------------------------------------------------------

    // derive attributes -------------------------------------------------------

    let content = if props.is_shown {
        props.digit.to_string()
    } else {
        " ".to_string()
    };

    let c = Candidate::from_cell_and_digit(cell, props.digit);

    let color = solver.result.as_ref().and_then(|result| {
        let solution = result.solutions.contains(c).then_some("bg-highlight-green");
        let elimination = result.eliminations.contains(c).then_some("bg-highlight-red");
        let highlight = result.highlights.contains(c).then_some("bg-highlight-blue");
        let highlight2 = result.highlights2.contains(c).then_some("bg-highlight-yellow");

        solution.or(elimination).or(highlight).or(highlight2)
    });

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("h-full", "w-full", "flex", "items-center", "justify-center", "overflow-hidden", "text-[40%]", "font-bold", color)}>
            { content }
        </div>
    }
}