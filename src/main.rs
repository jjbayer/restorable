mod linefile;
mod notebook;
mod page;
mod render;

use notebook::Notebook;
use render::render_notebook;

fn main() {
    let input_path = std::env::args().nth(1).expect("no input path given");
    let output_path = std::env::args().nth(2).expect("no output path given");
    let notebook = Notebook::load(&input_path).expect("Failed to parse notebook");

    println!(
        "Exporting notebook '{}' to {}",
        notebook.name(),
        output_path
    );

    render_notebook(notebook, &output_path).expect("Failed to render notebook");
}
