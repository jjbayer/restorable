mod json;
mod linefile;
mod node;
mod notebook;
mod page;
mod render;

use crate::node::{parse_nodes, Node};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    SetDir { path: String },
    Ls,
    Cd { rel_path: PathBuf },
}

#[derive(Serialize, Deserialize, Default)]
struct Config {
    xochitl_dir: String,
    current_folder: PathBuf,
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
        Command::Ls => {
            check_configuration(&config)?;

            let root_node = parse_nodes(&config.xochitl_dir)?;
            let descendant = root_node.get_descendant_by_name(&config.current_folder);

            match descendant {
                Some(node) => print_dir(&node),
                None => print_dir(&root_node),
            };
        }
        Command::Cd { rel_path } => {
            check_configuration(&config)?;
            config.current_folder.push(rel_path);
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
