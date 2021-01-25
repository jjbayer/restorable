use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    deleted: bool,
    last_modified: String,
    last_opened_page: Option<i32>,
    metadatamodified: bool,
    modified: bool,
    parent: String,
    pinned: bool,
    synced: bool,
    r#type: String,
    version: i32,
    visible_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Content {
    pages: Vec<String>,
}

#[derive(Debug)]
pub struct Notebook {
    metadata: Metadata,
    content: Content,
}

impl Notebook {
    pub fn load(path: &str) -> Result<Notebook, Box<dyn Error>> {
        let metadata: Metadata = parse(path, ".metadata")?;
        let content: Content = parse(path, ".content")?;

        Ok(Notebook { metadata, content })
    }

    pub fn name(&self) -> &str {
        &self.metadata.visible_name
    }
}

fn parse<T: DeserializeOwned>(path: &str, postfix: &str) -> Result<T, Box<dyn Error>> {
    let full_path = format!("{}{}", path, postfix);
    let json_str = std::fs::read_to_string(full_path)?;
    let content: T = serde_json::from_str(&json_str)?;

    Ok(content)
}
