use yew::prelude::*;

use crate::sudoku::Block;

use super::cell::CellComponent;

// =============================================================================

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    pub block: Block,
}

#[function_component]
pub fn BlockComponent(props: &BlockProps) -> Html {
    let cells = props.block.cells_iter().map(|cell| {
        html! {
            <CellComponent {cell} />
        }
    });

    // render ------------------------------------------------------------------

    html! {
        <div class={classes!("grid", "grid-cols-3", "gap-px", "bg-border")}>
            { for cells }
        </div>
    }
}
