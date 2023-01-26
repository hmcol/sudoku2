use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!("app")}>
            <Grid
                board={"test board".to_string()}
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
    board: String,
}

#[function_component(Grid)]
fn grid(props: &GridProps) -> Html {
    let board = &props.board;

    let boxes = (0..9).map(|i| {
        html! {
            <BoxComponent key={i}/>
        }
    });

    html! {
        <div class={classes!("grid")}>
            { for boxes }
        </div>
    }
}

#[function_component(BoxComponent)]
fn box_component() -> Html {
    let cells = (0..9).map(|i| {
        html! {
            <CellComponent key={i}/>
        }
    });

    html! {
        <div class={classes!("box")}>
            { for cells }
        </div>
    }
}

#[function_component(CellComponent)]
fn cell_component() -> Html {
    html! {
        <div class={classes!("cell")}>
            <div class={classes!("cell-digit")}>
                { "X" }
            </div>
        </div>
    }
}
