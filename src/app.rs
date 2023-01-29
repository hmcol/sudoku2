use std::collections::HashSet;

use crate::sudoku::{
    board::Board,
    cell::{CellContent, CellData},
    digit::Digit,
    pos::{Box, Cell, CELLS_BY_BOX},
};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let board_handler = use_state(|| {
        Board::from_string(
            "607005010580007900000060000005000009000936000300000400000080000003600094050200806",
        )
    });

    let board = (*board_handler).clone();

    html! {
        <div class={classes!("app")}>
            <Grid
                {board}
                // selectedCells={this.state.selectedCells}
                // result={this.state.result}
                // focus={this.state.focus}
                // onClickCell={(id) => this.handleClickCell(id)}
                // onMouseMove={(id) => this.handleMouseMove(id)}
            />
        // <div className="game-info">
        //     <div>Sudoku</div>
        //     <NoteSelector
        //         inputMode={this.state.inputMode}
        //         onClick={(inputMode) => this.updateInputMode(inputMode)}
        //     />
        //     <SolverControls
        //         onReset={() => this.resetBoard()}
        //         onLoadString={() => this.loadBoardString()}
        //         onStep={() => void this.takeStep()}
        //         onUndo={() => this.undoStep()}
        //         onComplete={() => void this.tryComplete()}
        //     />
        //     <StrategyList
        //         onClick={(strat) => void this.checkStrategy(strat)}
        //         strategyStatus={this.state.strategyStatus}
        //     />
        // </div>
    </div>
    }
}

#[derive(Properties, PartialEq)]
struct GridProps {
    board: Board,
}

#[function_component]
fn Grid(props: &GridProps) -> Html {
    let boxes = Box::list().map(|box_id| {
        let board = props.board.clone();
        let cells = CELLS_BY_BOX[box_id.as_index()];

        html! {
            <BoxComponent {board} {cells} />
        }
    });

    html! {
        <div class={classes!("grid")}>
            { for boxes }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct BoxProps {
    board: Board,
    cells: [Cell; 9],
    // children: Children,
}

#[function_component]
fn BoxComponent(props: &BoxProps) -> Html {
    let cells = props.cells.map(|cell| {
        let data = props.board.get_data(cell).clone();

        html! {
            <CellComponent {data} />
        }
    });

    html! {
        <div class={classes!("box")}>
            { for cells }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CellProps {
    data: CellData,
}

#[function_component]
fn CellComponent(props: &CellProps) -> Html {
    let content = match props.data.content.clone() {
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

    html! {
        <div class={classes!("cell")}>
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
