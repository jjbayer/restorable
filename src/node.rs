use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_dir;
use std::rc::Rc;

use crate::json;

#[derive(Clone, Debug, Deserialize)]
pub enum NodeType {
    CollectionType,
    DocumentType,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    deleted: bool,
    last_modified: String,
    last_opened_page: Option<i32>,
    metadatamodified: bool,
    modified: bool,
    parent: String,
    pinned: bool,
    r#type: NodeType,
    synced: bool,
    version: i32,
    pub visible_name: String,
}

#[derive(Default)]
pub struct Node {
    id: String,
    pub metadata: Option<Metadata>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn parent_id(&self) -> Option<&str> {
        if let Some(metadata) = &self.metadata {
            if !metadata.parent.is_empty() {
                return Some(&metadata.parent);
            }
        }

        None
    }

    pub fn walk<F: Fn(&Self, i32)>(&self, f: &F) {
        self.walk_with_depth(0, f);
    }

    fn walk_with_depth<F: Fn(&Self, i32)>(&self, depth: i32, f: &F) {
        f(self, depth);
        for child in self.children.borrow().iter() {
            child.walk_with_depth(depth + 1, f);
        }
    }
}

pub fn parse_nodes(path: &str) -> Result<Vec<Rc<Node>>, Box<dyn Error>> {
    let directory = read_dir(path)?;

    let mut graph = GraphBuilder::new();

    for entry in directory {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "metadata" {
                if let Some(node_id) = path.file_stem() {
                    let metadata: Metadata = json::parse(&path)?;
                    if let Some(id) = node_id.to_str() {
                        graph.add(Rc::new(Node {
                            id: id.to_owned(),
                            metadata: Some(metadata),
                            children: RefCell::new(vec![]),
                        }));
                    }
                }
            }
        }
    }

    Ok(graph.get_root_nodes())
}

struct GraphBuilder {
    map: HashMap<String, Rc<Node>>,
}

impl GraphBuilder {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, node: Rc<Node>) {
        if let Some(placeholder) = self.map.remove(&node.id) {
            // There was a node holding children. Transfer the children.
            let mut target = node.children.borrow_mut();
            target.clear();
            for node in placeholder.children.borrow().iter() {
                target.push(node.clone());
            }
        }

        self.map.insert(node.id.clone(), node.clone());

        // Insert as child of parent
        if let Some(parent_id) = node.parent_id() {
            let parent = self
                .map
                .entry(parent_id.to_owned())
                .or_insert(Rc::new(Node {
                    id: parent_id.to_owned(),
                    metadata: None,
                    children: RefCell::new(vec![]),
                }));
            parent.children.borrow_mut().push(node.clone());
        }
    }

    fn get_root_nodes(&self) -> Vec<Rc<Node>> {
        self.map
            .values()
            .filter(|node| node.parent_id().is_none())
            .map(|node| node.clone())
            .collect()
    }
}
