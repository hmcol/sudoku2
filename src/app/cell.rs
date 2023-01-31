use std::collections::HashSet;

use log::info;
use yew::prelude::*;

use crate::sudoku::{pos::Cell, digit::Digit, board::Board, cell::CellContent};

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell: Cell,
}

#[function_component]
pub fn CellComponent(props: &CellProps) -> Html {
    let board = use_context::<Board>().unwrap();

    let cell = props.cell;

    let content = match board.get_content(cell).clone() {
        CellContent::Digit(digit, given) => {
            html! {
                <CellDigit {digit} {given} />
            }
        }
        CellContent::Notes(notes) => {
            html! {
                <CellNotes {notes} />
            }
        }
    };


    let on_click: Callback<MouseEvent> = Callback::from(move |_| {
        info!("click on cell {cell}")
    });

    html! {
        <div class={classes!("cell")}
            onclick={on_click}
        >
            { content }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CellDigitProps {
    digit: Digit,
    given: bool,
}

#[function_component]
fn CellDigit(props: &CellDigitProps) -> Html {
    let given = props.given.then_some("given");
    let digit = props.digit.to_string();

    html! {
        <div class={classes!("cell-digit", given)}>
            { digit }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CellNotesProps {
    notes: HashSet<Digit>,
}

#[function_component]
fn CellNotes(props: &CellNotesProps) -> Html {
    let notes = Digit::list().map(|digit| {
        let is_shown = props.notes.contains(&digit);

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
    let digit = props.digit.to_string();
    let shown = props.is_shown.then_some("shown");

    html! {
        <div class={classes!("note", shown)}>
            { digit }
        </div>
    }
}
