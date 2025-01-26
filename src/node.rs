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
