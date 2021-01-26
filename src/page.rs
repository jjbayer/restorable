use crate::notebook::parse;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Page {
    metadata: Metadata,
}

impl Page {
    pub fn load(path: &str, id: &str) -> Result<Page, Box<dyn Error>> {
        let filename = format!("{}/{}-metadata.json", path, id);

        match parse::<Metadata>(&filename, "") {
            Ok(metadata) => Ok(Page { metadata }),
            Err(_) => {
                eprintln!("WARNING: Failed to load metadata for page {}", id);
                Ok(Page {
                    metadata: Metadata { layers: vec![] },
                })
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Metadata {
    layers: Vec<Layer>,
}

#[derive(Debug, Deserialize)]
struct Layer {
    name: String,
}
