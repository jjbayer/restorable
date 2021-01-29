mod json;
mod linefile;
mod node;
mod notebook;
mod page;
mod render;

use crate::node::{parse_nodes, Node};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Tree,
    SetDir { path: String },
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    xochitl_dir: String,
}

const APP_NAME: &str = "restorable";

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut config: Config = confy::load(APP_NAME)?;

    let command = Command::from_args();
    match command {
        Command::SetDir { path } => {
            config.xochitl_dir = path;
            confy::store(APP_NAME, config)?;
        }
        Command::Tree => {
            if config.xochitl_dir.is_empty() {
                let cli_command = std::env::args().nth(0).unwrap();
                eprintln!("Please run `{} set-dir /path/to/your/xochitl`", cli_command);
                return Ok(());
            }

            let root_node = parse_nodes(&config.xochitl_dir)?;
            root_node.walk(&|node, depth| {
                if node.id == Node::ROOT_ID {
                    println!("Documents:");
                } else {
                    for _ in 0..depth {
                        print!("  ");
                    }
                    println!("- {}", node.name());
                }
            });
        }
    }

    Ok(())
}
