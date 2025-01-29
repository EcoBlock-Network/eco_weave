use ed25519_dalek::VerifyingKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub neighbors: Vec<String>,
    pub verifying_key: VerifyingKey,
}

impl Node {
    pub fn new(id: impl Into<String>, verifying_key: VerifyingKey) -> Self {
        Self {
            id: id.into(),
            neighbors: Vec::new(),
            verifying_key,
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: impl Into<String>) {
        let neighbor_id = neighbor_id.into();
        if neighbor_id == self.id {
            return;
        }
        if !self.neighbors.contains(&neighbor_id) {
            self.neighbors.push(neighbor_id);
        }
    }

    pub fn is_neighbor(&self, neighbor_id: &str) -> bool {
        self.neighbors.contains(&neighbor_id.to_string())
    }
}
