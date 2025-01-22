use crate::{node::Node, Transaction};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Tangle {
    pub nodes: HashMap<String, Node>,
    pub transactions: HashMap<String, Transaction>,
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
        self.transactions
            .insert(transaction.id.clone(), transaction);
        true
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
}
