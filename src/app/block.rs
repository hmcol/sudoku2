use yew::prelude::*;

use crate::sudoku::Cell;

use super::cell::CellComponent;

// =============================================================================

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    pub cells: [Cell; 9],
}

#[function_component]
pub fn BlockComponent(props: &BlockProps) -> Html {
    let cells = props.cells.map(|cell| {
        html! {
            <CellComponent {cell} />
        }
    });

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("block")}>
            { for cells }
        </div>
    }
}
