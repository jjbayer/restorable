mod json;
mod linefile;
mod node;
mod notebook;
mod page;
mod render;

use crate::node::parse_nodes;
use crate::notebook::Notebook;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    SetDir {
        path: String,
    },
    Tree,
    RenderNotebook {
        notebook: PathBuf,
        output_path: PathBuf,
    },
    RenderAll {
        output_directory: PathBuf,
    },
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
            for child in root_node.children.borrow().iter() {
                child.walk(&|node, ancestors| {
                    for _ in 0..ancestors.len() {
                        print!("  ");
                    }
                    println!("- {}", node.name());
                });
            }
        }
        Command::RenderNotebook {
            notebook,
            output_path,
        } => {
            check_configuration(&config)?;

            let root_node = parse_nodes(&config.xochitl_dir)?;
            match root_node.get_descendant_by_name(&notebook) {
                None => {
                    eprintln!("Cannot find document {:#?}", notebook)
                }
                Some(node) => {
                    if node.is_notebook() {
                        let filename = Path::join(&PathBuf::from(&config.xochitl_dir), &node.id);
                        let filename = filename.to_str().unwrap();
                        let notebook = Notebook::load(filename)?;
                        render::render_notebook(notebook, output_path.to_str().unwrap())?;
                    } else {
                        eprintln!("Not a notebook: {:#?}", notebook);
                    }
                }
            }
        }
        Command::RenderAll { output_directory } => {
            check_configuration(&config)?;
            let root_node = parse_nodes(&config.xochitl_dir)?;
            panic!("Not implemented");
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
