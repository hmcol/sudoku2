use std::collections::HashSet;

use log::info;
use yew::prelude::*;

use crate::sudoku::{
    board::Board,
    cell::CellContent,
    pos::{Cell, Digit},
};

use super::{view::View, ClickCallbacks};

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell: Cell,
}

#[function_component]
pub fn CellComponent(props: &CellProps) -> Html {
    let board = use_context::<Board>().unwrap();
    let callbacks = use_context::<ClickCallbacks>().unwrap();

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

    let on_click = Callback::<MouseEvent>::from(move |_| callbacks.on_click_cell.emit(cell));

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
    let view = use_context::<View>().unwrap();
    let callbacks = use_context::<ClickCallbacks>().unwrap();

    let digit = props.digit;

    let given = props.given.then_some("given");
    let focus = (view.focus_digit == Some(digit)).then_some("focus");

    let on_click = Callback::<MouseEvent>::from(move |_| callbacks.on_click_digit.emit(digit));

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
