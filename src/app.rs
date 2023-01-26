use std::collections::HashSet;

use yew::prelude::*;

use crate::sudoku::{
    board::{Board, Cell, CellContent},
    id::{Digit, DIGITS},
};

// use crate::sudoku::{Board, Cell};

#[function_component(App)]
pub fn app() -> Html {
    let board = use_state(|| {
        Board::from_string(
            "607005010580007900000060000005000009000936000300000400000080000003600094050200806",
        )
    });

    html! {
        <div class={classes!("app")}>
            <Grid
                board={(*board).clone()}
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

#[function_component(Grid)]
fn grid(props: &GridProps) -> Html {
    let board = &props.board;

    let boxes = board.boxes().map(|id_box| {
        let cells = id_box.map(Clone::clone);

        html! {
            <BoxComponent cells={cells} />
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
    cells: [Cell; 9],
}

#[function_component(BoxComponent)]
fn box_component(props: &BoxProps) -> Html {
    let cells = props.cells.clone();

    let cells = cells.map(|cell| {
        html! {
            <CellComponent cell={cell} />
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
    cell: Cell,
}

#[function_component(CellComponent)]
fn cell_component(props: &CellProps) -> Html {
    let content = props.cell.content.clone();

    let content = match content {
        CellContent::Digit(digit, given) => {
            html! {
                <CellDigit digit={digit} given={given} />
            }
        }
        CellContent::Notes(notes) => {
            html! {
                <CellNotes notes={notes} />
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

#[function_component(CellDigit)]
fn cell_digit(props: &CellDigitProps) -> Html {
    let digit = props.digit.to_string();
    let given = props.given.then_some("given");

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

#[function_component(CellNotes)]
fn cell_notes(props: &CellNotesProps) -> Html {
    let notes = props.notes.clone();

    let notes = DIGITS.map(|digit| {
        html! {
            <Note digit={digit} is_shown={notes.contains(&digit)} />
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

#[function_component(Note)]
fn note(props: &NoteProps) -> Html {
    let digit = props.digit.to_string();
    let shown = props.is_shown.then_some("shown");

    html! {
        <div class={classes!("note", shown)}>
            { digit }
        </div>
    }
}
