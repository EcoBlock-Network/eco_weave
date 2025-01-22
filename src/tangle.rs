use crate::{node::Node, Transaction};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Tangle {
    pub nodes: HashMap<String, Node>,
    pub transactions: HashMap<String, Transaction>,
}

impl Default for Tangle {
    fn default() -> Self {
        Self::new()
    }
}

impl Tangle {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            transactions: HashMap::new(),
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
        if let (Some(mut node1), Some(mut node2)) = (self.nodes.remove(id1), self.nodes.remove(id2))
        {
            node1.add_neighbor(id2);
            node2.add_neighbor(id1);
            self.nodes.insert(id1.to_string(), node1);
            self.nodes.insert(id2.to_string(), node2);
            true
        } else {
            false
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        if self.transactions.contains_key(&transaction.id) {
            return false;
        }

        if let Err(error) = transaction.validate() {
            eprintln!("Transaction validation failed: {}", error);
            return false;
        }

        self.transactions
            .insert(transaction.id.clone(), transaction);
        true
    }

    pub fn propagate_transaction(
        &mut self,
        transaction: Transaction,
        start_node_id: &str,
    ) -> usize {
        if !self.nodes.contains_key(start_node_id) {
            return 0;
        }
        let mut visited = std::collections::HashSet::new();
        let mut queue = vec![start_node_id.to_string()];
        let mut propagated_count = 0;
        while let Some(current_node_id) = queue.pop() {
            if visited.contains(&current_node_id) {
                continue;
            }
            visited.insert(current_node_id.clone());
            if !self.transactions.contains_key(&transaction.id) {
                self.add_transaction(transaction.clone());
            }
            propagated_count += 1;
            if let Some(node) = self.nodes.get(&current_node_id) {
                for neighbor_id in &node.neighbors {
                    if !visited.contains(neighbor_id) {
                        queue.push(neighbor_id.clone());
                    }
                }
            }
        }
        propagated_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transaction;

    #[test]
    fn test_add_transaction() {
        let mut tangle = Tangle::new();
        let tx = Transaction::new("tx1", "Sample transaction");
        assert!(tangle.add_transaction(tx.clone()));
        assert_eq!(tangle.transactions.len(), 1);
        assert_eq!(tangle.transactions["tx1"], tx);
    }

    #[test]
    fn test_duplicate_transaction() {
        let mut tangle = Tangle::new();
        let tx = Transaction::new("tx1", "Duplicate test");
        tangle.add_transaction(tx.clone());
        assert!(!tangle.add_transaction(tx));
    }

    #[test]
    fn test_propagate_transaction() {
        let mut tangle = Tangle::new();

        tangle.add_node("node1");
        tangle.add_node("node2");
        tangle.add_node("node3");
        tangle.connect_nodes("node1", "node2");
        tangle.connect_nodes("node2", "node3");

        let tx = Transaction::new("tx1", "Hello, Tangle!");

        let propagated_count = tangle.propagate_transaction(tx.clone(), "node1");
        assert_eq!(propagated_count, 3);

        assert!(tangle.transactions.contains_key("tx1"));
        assert_eq!(tangle.transactions.len(), 1);
    }

    #[test]
    fn test_propagate_transaction_with_missing_node() {
        let mut tangle = Tangle::new();

        let tx = Transaction::new("tx1", "Hello, Tangle!");

        let propagated_count = tangle.propagate_transaction(tx, "missing_node");
        assert_eq!(propagated_count, 0);
    }

    #[test]
    fn test_add_valid_transaction() {
        let mut tangle = Tangle::default();
        let tx = Transaction::new("tx1", "Valid payload");
        assert!(tangle.add_transaction(tx));
    }

    #[test]
    fn test_add_duplicate_transaction() {
        let mut tangle = Tangle::default();
        let tx = Transaction::new("tx1", "Valid payload");
        assert!(tangle.add_transaction(tx.clone()));
        assert!(!tangle.add_transaction(tx)); // Duplication refusée.
    }

    #[test]
    fn test_add_invalid_transaction_empty_id() {
        let mut tangle = Tangle::default();
        let tx = Transaction::new("", "Payload");
        assert!(!tangle.add_transaction(tx)); // Validation échoue.
    }

    #[test]
    fn test_add_invalid_transaction_empty_payload() {
        let mut tangle = Tangle::default();
        let tx = Transaction::new("tx2", "");
        assert!(!tangle.add_transaction(tx)); // Validation échoue.
    }
}
