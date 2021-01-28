mod json;
mod linefile;
mod node;
mod notebook;
mod page;
mod render;

use crate::node::{parse_nodes, Node};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Subcommand {
    Tree { path: String },
}

fn main() {
    let command = Subcommand::from_args();
    match command {
        Subcommand::Tree { path } => match parse_nodes(&path) {
            Ok(root_node) => root_node.walk(&|node, depth| {
                if node.id == Node::ROOT_ID {
                    println!("Documents:");
                } else {
                    for _ in 0..depth {
                        print!("  ");
                    }
                    println!("- {}", node.name());
                }
            }),
            Err(e) => {
                eprintln!("{}", e);
            }
        },
    }
}
