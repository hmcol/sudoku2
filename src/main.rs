mod app;
mod sudoku;
mod util;

// =============================================================================

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<app::App>::new().render();
}
