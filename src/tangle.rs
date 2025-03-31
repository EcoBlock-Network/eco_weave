use ed25519_dalek::VerifyingKey;
use rand::Rng;

use crate::{node::Node, Transaction};
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::sleep;

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
                break;
            }

            let total_weight: u32 = neighbors.iter().map(|(_, weight)| *weight).sum();
            let choice = rng.gen_range(0..total_weight);
            let mut cumulative_weight = 0;

            for (neighbor_id, weight) in neighbors {
                cumulative_weight += weight;
                if cumulative_weight > choice {
                    current_id = neighbor_id;
                    break;
                }
            }
        }

        Some(current_id)
    }

    pub fn get_neighbors(&self, transaction_id: &str) -> Vec<String> {
        self.nodes
            .get(transaction_id)
            .map_or(vec![], |node| node.neighbors.clone())
    }

    pub fn add_node(&mut self, id: impl Into<String>, verifying_key: VerifyingKey) -> bool {
        let id = id.into();
        if self.nodes.contains_key(&id) {
            return false;
        }
        self.nodes.insert(id.clone(), Node::new(id, verifying_key));
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

    pub fn get_verifying_key(&self, node_id: &str) -> Option<&VerifyingKey> {
        self.nodes.get(node_id).map(|node| &node.verifying_key)
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        if self.transactions.contains_key(&transaction.id) {
            return false;
        }

        if let Err(_error) = transaction.validate() {
            return false;
        }

        if let Some(verifying_key) = self.get_verifying_key(&transaction.id) {
            if let Err(_error) = transaction.validate_signature(verifying_key) {
                return false;
            }
        } else {
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

            if !self.transactions.contains_key(&transaction.id) {
                self.add_transaction(transaction.clone());
            }
            propagated_count += 1;

            let futures: FuturesUnordered<_> = self
                .get_neighbors(&current_node_id)
                .into_iter()
                .filter(|neighbor_id| !visited.contains(neighbor_id))
                .map(|neighbor_id| async move {
                    sleep(Duration::from_secs(1)).await;
                    neighbor_id
                })
                .collect();

            let results: Vec<_> = futures.collect().await;
            queue.extend(results);
        }
        propagated_count
    }

    pub fn get_snapshot(&self) -> Vec<(String, Vec<String>)> {
        let mut snapshot = Vec::new();

        for (tx_id, _transaction) in &self.transactions {
            let neighbors = self.get_neighbors(tx_id);
            snapshot.push((tx_id.clone(), neighbors));
        }

        snapshot
    }
}
