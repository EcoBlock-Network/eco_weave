use crate::node::Node;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Tangle {
    pub nodes: HashMap<String, Node>,
}

impl Tangle {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: impl Into<String>) -> bool {
        let id = id.into();
        if self.nodes.contains_key(&id) {
            return false;
        }
        self.nodes.insert(id.clone(), Node::new(id));
        true
    }

    pub fn connect_nodes(&mut self, id1: &str, id2: &str) -> bool {
        if let (Some(mut node1), Some(mut node2)) = (
            self.nodes.remove(id1),
            self.nodes.remove(id2),
        ) {
            node1.add_neighbor(id2);
            node2.add_neighbor(id1);
            self.nodes.insert(id1.to_string(), node1);
            self.nodes.insert(id2.to_string(), node2);
            true
        } else {
            false
        }
    }
}