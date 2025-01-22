/// Représente un nœud dans le Tangle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub neighbors: Vec<String>,
}

impl Node {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: impl Into<String>) {
        let neighbor_id = neighbor_id.into();
        if !self.neighbors.contains(&neighbor_id) {
            self.neighbors.push(neighbor_id);
        }
    }

    pub fn is_neighbor(&self, neighbor_id: &str) -> bool {
        self.neighbors.contains(&neighbor_id.to_string())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new("node1");
        assert_eq!(node.id, "node1");
        assert!(node.neighbors.is_empty());
    }

    #[test]
    fn test_add_neighbor() {
        let mut node = Node::new("node1");
        node.add_neighbor("node2");
        assert_eq!(node.neighbors.len(), 1);
        assert!(node.is_neighbor("node2"));
    }

    #[test]
    fn test_no_duplicate_neighbors() {
        let mut node = Node::new("node1");
        node.add_neighbor("node2");
        node.add_neighbor("node2");
        assert_eq!(node.neighbors.len(), 1);
    }
}