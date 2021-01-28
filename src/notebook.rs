use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

use crate::json;
use crate::page::Page;
#[derive(Debug, Deserialize)]
struct Content {
    pages: Vec<String>,
}

#[derive(Debug)]
pub struct Notebook {
    content: Content,
    pub pages: Vec<Page>,
}

impl Notebook {
    pub fn load(path: &str) -> Result<Notebook, Box<dyn Error>> {
        let content: Content = parse(path, ".content")?;

        let mut pages: Vec<Page> = vec![];
        for page_id in &content.pages {
            match Page::load(path, page_id) {
                Err(e) => {
                    return Err(e);
                }
                Ok(page) => {
                    pages.push(page);
                }
            }
        }

        Ok(Notebook { content, pages })
    }
}

pub fn parse<T: DeserializeOwned>(path: &str, postfix: &str) -> Result<T, Box<dyn Error>> {
    let full_path = format!("{}{}", path, postfix);
    json::parse(Path::new(&full_path))
}
