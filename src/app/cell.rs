use yew::prelude::*;

use crate::{
    bitset::Set,
    sudoku::{Candidate, Cell, CellData, Digit},
};

use super::{ClickCallbacks, SolverHandle};

// =============================================================================

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell: Cell,
}

#[function_component]
pub fn CellComponent(props: &CellProps) -> Html {
    // get contexts ------------------------------------------------------------

    let solver = use_context::<SolverHandle>().expect("Solver context not found");
    let callbacks = use_context::<ClickCallbacks>().expect("ClickCallbacks context not found");

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
    }.then_some("lowlight");

    let on_click = Callback::<MouseEvent>::from(move |_| callbacks.on_click_cell.emit(cell));

    let content = match solver.board.get_data(&cell).clone() {
        CellData::Digit(digit) => {
            let is_given = solver.board.is_given(&cell);

            html! {
                <CellDigit {digit} {is_given} />
            }
        }
        CellData::Notes(notes) => {
            html! {
                <CellNotes {notes} />
            }
        }
    };

    // render ------------------------------------------------------------------

    html! {
        <ContextProvider<Cell> context={cell}>
            <div class={classes!("cell", lowlight)}
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
    let callbacks = use_context::<ClickCallbacks>().expect("ClickCallbacks context not found");

    // read props --------------------------------------------------------------

    let digit = props.digit;

    // derive attributes -------------------------------------------------------

    let given = props.is_given.then_some("given");
    let focus = (solver.focus_digit == Some(digit)).then_some("focus");

    let on_click = Callback::<MouseEvent>::from(move |_| callbacks.on_click_digit.emit(digit));

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("cell-digit", given, focus)}
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
        <div class={classes!("cell-notes")}>
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
        let solution = result.solutions.contains(&c).then_some("green");
        let elimination = result.eliminations.contains(&c).then_some("red");
        let highlight = result.highlights.contains(&c).then_some("blue");
        let highlight2 = result.highlights2.contains(&c).then_some("yellow");

        solution.or(elimination).or(highlight).or(highlight2)
    });

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("note", color)}>
            { content }
        </div>
    }
}
