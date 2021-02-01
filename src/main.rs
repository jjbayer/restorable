mod json;
mod linefile;
mod node;
mod notebook;
mod page;
mod render;

use crate::node::{parse_nodes, Node};
use crate::notebook::Notebook;
use crate::render::render_notebook;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
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
                    for _ in ancestors {
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
                Some(node) => render(&config, &node, &output_path)?,
            }
        }
        Command::RenderAll { output_directory } => {
            check_configuration(&config)?;
            match output_directory.canonicalize() {
                Err(_) => eprintln!("Directory does not exist: {:#?}", output_directory),
                Ok(output_directory) => {
                    let root_node = parse_nodes(&config.xochitl_dir)?;
                    root_node.walk(&|node, ancestors| {
                        if node.is_notebook() {
                            let mut full_path = output_directory.clone();
                            for node in ancestors {
                                full_path.push(node.name());
                            }
                            match full_path.extension() {
                                Some(_) => {
                                    // Nothing to do, already a rendered file.
                                }
                                None => {
                                    full_path.set_extension("pdf");
                                    if let Some(parent) = full_path.parent() {
                                        match std::fs::create_dir_all(parent) {
                                            Err(_) => eprintln!(
                                                "WARNING: Failed to create directory {:#?}",
                                                parent
                                            ),
                                            Ok(_) => match render(&config, &node, &full_path) {
                                                Err(_) => eprintln!(
                                                    "WARNING: Failed to render notebook '{}'",
                                                    node.name()
                                                ),
                                                Ok(_) => {}
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            }
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

fn render(config: &Config, node: &Node, output_path: &Path) -> Result<(), Box<dyn Error>> {
    if node.is_notebook() {
        let filename = Path::join(&PathBuf::from(&config.xochitl_dir), &node.id);
        let filename = filename.to_str().unwrap();
        let notebook = Notebook::load(filename)?;

        let mut file = File::create(output_path)?;

        println!("Rendering notebook {}...", node.name());
        render_notebook(notebook, &mut file)?;
    } else {
        eprintln!("Not a notebook: {:#?}", node.name());
    }

    Ok(())
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
