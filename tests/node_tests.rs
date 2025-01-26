#[cfg(test)]
mod tests {
    use eco_weave::Node;

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
