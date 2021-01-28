mod folder;
mod json;
mod linefile;
mod notebook;
mod page;
mod render;

use crate::folder::parse_folders;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Subcommand {
    Tree { path: String },
}

fn main() {
    let command = Subcommand::from_args();
    match command {
        Subcommand::Tree { path } => match parse_folders(&path) {
            Ok(folders) => {
                for folder in folders {
                    if let Some(metadata) = &folder.metadata {
                        println!("{:?}", metadata.visible_name);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
    }
}
