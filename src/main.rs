mod linefile;
mod notebook;
mod page;
mod render;

use notebook::Notebook;
use render::render_notebook;

fn main() {
    let mut cli_args = std::env::args();
    cli_args.next();
    match cli_args.next() {
        None => {}
        Some(arg) => {
            let notebook = Notebook::load(&arg).unwrap();

            let document = render_notebook(notebook);

            svg::write(std::io::stdout(), &document).unwrap();
        }
    };
}
