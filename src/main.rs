mod app;
mod sudoku;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
