use yew::prelude::*;

use crate::sudoku::Block;

use super::block::BlockComponent;

// =============================================================================

#[function_component]
pub fn Grid() -> Html {
    let blocks = Block::list().map(|block_id| {
        html! {
            <BlockComponent block={block_id} />
        }
    });

    html! {
        <div class={classes!("grid", "grid-cols-3", "gap-0.5", "p-px", "bg-border")}>
            { for blocks }
        </div>
    }
}
