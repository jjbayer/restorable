use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_dir;
use std::path::Path;
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
    pub id: String,
    pub metadata: Option<Metadata>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub const ROOT_ID: &'static str = "root";

    pub fn name(&self) -> &str {
        match &self.metadata {
            Some(metadata) => &metadata.visible_name,
            None => "",
        }
    }

    pub fn node_type(&self) -> NodeType {
        match &self.metadata {
            None => NodeType::CollectionType,
            Some(m) => m.r#type.clone(),
        }
    }

    pub fn is_notebook(&self) -> bool {
        match &self.metadata {
            Some(metadata) => match &metadata.r#type {
                NodeType::DocumentType => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn get_descendant_by_name(&self, path: &Path) -> Option<Rc<Node>> {
        let mut parts = path.components();
        let name = parts.next()?;
        let child = self.get_child_by_name(name.as_os_str().to_str()?)?;
        let rest_of_path = parts.as_path();
        if rest_of_path.eq(Path::new("")) {
            Some(child)
        } else {
            child.get_descendant_by_name(rest_of_path)
        }
    }

    pub fn walk<F: Fn(&Self, &Vec<Rc<Node>>)>(&self, f: &F) {
        let mut ancestors = vec![];
        self.walk_with_ancestors(&mut ancestors, f);
    }

    fn walk_with_ancestors<F: Fn(&Self, &Vec<Rc<Node>>)>(
        &self,
        ancestors: &mut Vec<Rc<Node>>,
        f: &F,
    ) {
        f(self, ancestors);
        let children = self.children.borrow();
        for child in children.iter() {
            ancestors.push(child.clone());
            child.walk_with_ancestors(ancestors, f);
            ancestors.pop();
        }
    }

    fn parent_id(&self) -> Option<&str> {
        if let Some(metadata) = &self.metadata {
            if !metadata.parent.is_empty() {
                return Some(&metadata.parent);
            }
        }

        None
    }

    fn get_child_by_name(&self, name: &str) -> Option<Rc<Node>> {
        for child in self.children.borrow().iter() {
            if child.name() == name {
                return Some(child.clone());
            }
        }

        None
    }
}

pub fn parse_nodes(path: &str) -> Result<Node, Box<dyn Error>> {
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

    let root_nodes = graph.get_root_nodes();

    Ok(Node {
        id: Node::ROOT_ID.to_owned(),
        metadata: None,
        children: RefCell::new(root_nodes),
    })
}

pub struct GraphBuilder {
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
