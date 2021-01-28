mod folder;
mod json;
mod linefile;
mod notebook;
mod page;
mod render;

use crate::folder::{parse_folders, Node};
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
                    folder.walk(&|node, depth| {
                        for _ in 0..depth {
                            print!("  ");
                        }
                        println!(
                            "- {}",
                            match &node.metadata {
                                Some(m) => &m.visible_name,
                                None => "<unnamed>",
                            }
                        );
                    })
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
    }
}
