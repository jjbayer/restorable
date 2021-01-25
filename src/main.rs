mod notebook;
use notebook::Notebook;

fn main() {
    let mut cli_args = std::env::args();
    cli_args.next();
    match cli_args.next() {
        None => {}
        Some(arg) => {
            println!("{}", arg);
            let notebook = Notebook::load(&arg).unwrap();

            println!("Notebook {}", notebook.name());
        }
    };
}
