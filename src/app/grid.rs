use yew::prelude::*;

use crate::sudoku::{Block, CELLS_BY_BLOCK};

use super::block::BlockComponent;

// =============================================================================

#[function_component]
pub fn Grid() -> Html {
    let blocks = Block::list().map(|block_id| {
        let cells = CELLS_BY_BLOCK[block_id.as_index()];

        html! {
            <BlockComponent {cells} />
        }
    });

    html! {
        <div class={classes!("grid")}>
            { for blocks }
        </div>
    }
}
