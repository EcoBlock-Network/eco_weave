use rand::Rng;

use crate::{node::Node, Transaction};
use std::collections::HashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::sleep;
use std::time::Duration;
use std::collections::HashSet;


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


    pub async fn weighted_random_walk(&self, start_id: &str) -> Option<String> {
        let mut current_id = start_id.to_string();
        let mut rng = rand::thread_rng();
        let mut visited = HashSet::new(); 

        println!("Starting WRW from: {}", start_id);

        while let Some(_transaction) = self.transactions.get(&current_id) {
            visited.insert(current_id.clone()); 
            let mut neighbors = vec![];

            for neighbor_id in self.get_neighbors(&current_id) {
                if !visited.contains(&neighbor_id) {
                    if let Some(neighbor) = self.transactions.get(&neighbor_id) {
                        neighbors.push((neighbor_id.clone(), neighbor.weight));
                    }
                }
            }

            if neighbors.is_empty() {
                println!("No unvisited neighbors found for: {}", current_id);
                break;
            }

            let total_weight: u32 = neighbors.iter().map(|(_, weight)| *weight).sum();
            let choice = rng.gen_range(0..total_weight);
            let mut cumulative_weight = 0;

            for (neighbor_id, weight) in neighbors {
                cumulative_weight += weight;
                if cumulative_weight > choice {
                    println!("Selected neighbor: {}", neighbor_id);
                    current_id = neighbor_id;
                    break;
                }
            }
        }

        Some(current_id)
    }
        

    fn get_neighbors(&self, transaction_id: &str) -> Vec<String> {
        self.nodes
            .get(transaction_id)
            .map_or(vec![], |node| node.neighbors.clone())
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

    pub async fn propagate_transaction(
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

            // Add transation localy if not already present
            if !self.transactions.contains_key(&transaction.id) {
                self.add_transaction(transaction.clone());
            }
            propagated_count += 1;
            
            //Create futures for the neighbors
            let futures : FuturesUnordered<_> = self
                .get_neighbors(&current_node_id)
                .into_iter()
                .filter(|neighbor_id| !visited.contains(neighbor_id))
                .map(|neighbor_id| async move {
                    //Simulate propagation time
                    sleep(Duration::from_secs(1)).await;
                    neighbor_id
                })
                .collect();

            //Resolve futures
            let results: Vec<_> = futures.collect().await;
            queue.extend(results);
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

    #[tokio::test]
    async fn test_propagate_transaction() {
        let mut tangle = Tangle::new();

        tangle.add_node("node1");
        tangle.add_node("node2");
        tangle.add_node("node3");
        tangle.connect_nodes("node1", "node2");
        tangle.connect_nodes("node2", "node3");

        let tx = Transaction::new("tx1", "Hello, Tangle!");

        let propagated_count = tangle.propagate_transaction(tx.clone(), "node1").await;
        assert_eq!(propagated_count, 3);

        assert!(tangle.transactions.contains_key("tx1"));
        assert_eq!(tangle.transactions.len(), 1);
    }


    #[tokio::test]
    async fn test_propagate_transaction_with_missing_node() {
        let mut tangle = Tangle::new();

        let tx = Transaction::new("tx1", "Hello, Tangle!");

        let propagated_count = tangle.propagate_transaction(tx.clone(), "missing_node").await;

        assert_eq!(propagated_count, 0);
        assert!(tangle.transactions.is_empty());
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

    #[tokio::test]
    async fn test_weighted_random_walk() {
        let mut tangle = Tangle::new();

        // Ajouter les transactions comme nœuds
        tangle.add_node("tx1");
        tangle.add_node("tx2");

        // Connecter les transactions
        tangle.connect_nodes("tx1", "tx2");

        // Ajouter les transactions au Tangle
        let mut tx1 = Transaction::new("tx1", "Payload");
        let mut tx2 = Transaction::new("tx2", "Payload");
        tx1.weight = 10;
        tx2.weight = 20;

        tangle.add_transaction(tx1);
        tangle.add_transaction(tx2);

        // Lancer le WRW
        let result = tangle.weighted_random_walk("tx1").await;

        // Vérifiez que le résultat est valide
        assert!(
            result == Some("tx1".to_string()) || result == Some("tx2".to_string()),
            "Unexpected WRW result: {:?}",
            result
        );
    }


}
