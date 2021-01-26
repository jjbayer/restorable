mod notebook;
mod page;
use notebook::Notebook;

fn main() {
    let mut cli_args = std::env::args();
    cli_args.next();
    match cli_args.next() {
        None => {}
        Some(arg) => {
            println!("{}", arg);
            let notebook = Notebook::load(&arg).unwrap();

            println!("Notebook '{}' with pages:", notebook.name());
            for page in notebook.pages {
                println!("  {:?}", page);
            }
        }
    };
}
