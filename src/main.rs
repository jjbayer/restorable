mod json;
mod linefile;
mod node;
mod notebook;
mod page;
mod render;

use crate::node::{parse_nodes, Node};
use serde::{Deserialize, Serialize};
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    SetDir { path: String },
    Tree,
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

fn run() -> Result<(), Box<dyn Error>> {
    let mut config: Config = confy::load(APP_NAME)?;

    let command = Command::from_args();
    match command {
        Command::SetDir { path } => {
            config.xochitl_dir = path;
        }
        Command::Tree => {
            check_configuration(&config)?;

            let root_node = parse_nodes(&config.xochitl_dir)?;
            root_node.walk(&|node, depth| {
                for _ in 0..depth {
                    print!("  ");
                }
                println!("- {}", node.name());
            });
        }
    }

    confy::store(APP_NAME, config)?;

    Ok(())
}

fn check_configuration(config: &Config) -> Result<(), ConfigMissing> {
    if config.xochitl_dir.is_empty() {
        Err(ConfigMissing {})
    } else {
        Ok(())
    }
}

fn print_dir(parent: &Node) {
    for child in parent.children.borrow().iter() {
        println!("{}", child.name());
    }
}

#[derive(Debug)]
struct ConfigMissing;

impl ConfigMissing {
    const MESSAGE: &'static str = "Please run `restorable set-dir /path/to/xochitl`";
}

impl Error for ConfigMissing {
    fn description(&self) -> &str {
        Self::MESSAGE
    }
}

impl std::fmt::Display for ConfigMissing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::MESSAGE)
    }
}
