use crate::linefile::LineFile;
use crate::notebook::parse;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug)]
pub struct Page {
    metadata: Metadata,
    linefile: LineFile,
}

impl Page {
    pub fn load(path: &str, id: &str) -> Result<Page, Box<dyn Error>> {
        let page_path = format!("{}/{}", path, id);

        let linefile = LineFile::parse(&format!("{}.rm", page_path))?;

        match parse::<Metadata>(&page_path, "-metadata.json") {
            Ok(metadata) => Ok(Page { metadata, linefile }),
            Err(_) => {
                eprintln!("WARNING: Failed to load metadata for page {}", id);
                Ok(Page {
                    metadata: Metadata { layers: vec![] },
                    linefile,
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
