use serde::de::DeserializeOwned;
use std::error::Error;
use std::path::Path;

pub fn parse<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn Error>> {
    let json_str = std::fs::read_to_string(path)?;
    let content: T = serde_json::from_str(&json_str)?;

    Ok(content)
}
