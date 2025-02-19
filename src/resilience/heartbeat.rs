use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::time::sleep;

pub struct HeartbeatMonitor {
    last_seen: HashMap<String, SystemTime>,
    timeout: Duration,
}

impl HeartbeatMonitor {
    pub fn new(timeout: Duration) -> Self {
        Self {
            last_seen: HashMap::new(),
            timeout,
        }
    }

    pub fn update_heartbeat(&mut self, node_id: String) {
        self.last_seen.insert(node_id, SystemTime::now());
    }

    pub async fn check_inactive_nodes(&self) -> Vec<String> {
        let mut inactive_nodes = Vec::new();
        let now = SystemTime::now();

        for (node_id, last_time) in &self.last_seen {
            if now.duration_since(*last_time).unwrap_or(Duration::ZERO) > self.timeout {
                inactive_nodes.push(node_id.clone());
            }
        }

        sleep(self.timeout).await;
        inactive_nodes
    }
}
