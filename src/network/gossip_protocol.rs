use std::collections::HashSet;
use tokio::time::Duration;
use rand::seq::SliceRandom;
use crate::{Tangle, Transaction};

const GOSSIP_FANOUT: usize = 2;
const GOSSIP_DELAY: Duration = Duration::from_millis(50);

pub struct GossipProtocol {
    pub tangle: Tangle,
}

impl GossipProtocol {
    pub fn new(tangle: Tangle) -> Self {
        Self { tangle }
    }

    pub async fn propagate_transaction(&mut self, transaction: Transaction, origin_node: &str) {
        let mut visited_nodes: HashSet<String> = HashSet::new();
        let mut queue = vec![origin_node.to_string()];

        while let Some(node_id) = queue.pop() {
            if visited_nodes.contains(&node_id) {
                continue;
            }
            visited_nodes.insert(node_id.clone());

            let neighbors = self.tangle.get_neighbors(&node_id);
            let selected_neighbors: Vec<String> = neighbors
                .choose_multiple(&mut rand::thread_rng(), GOSSIP_FANOUT)
                .cloned()
                .collect();

            for neighbor in selected_neighbors {
                if !visited_nodes.contains(&neighbor) {
                    queue.push(neighbor.clone());
                }

                if !self.tangle.transactions.contains_key(&transaction.id) {
                    self.tangle.add_transaction(transaction.clone());
                }
            }

            tokio::time::sleep(GOSSIP_DELAY).await;
        }
    }
}