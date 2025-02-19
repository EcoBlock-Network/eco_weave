use std::collections::HashMap;
use std::time::{Duration, SystemTime};
pub struct TransactionCache {
    pub cache: HashMap<String, (String, SystemTime)>,
    pub cache_lifetime: Duration,
}

impl TransactionCache {
    pub fn new(cache_lifetime: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            cache_lifetime,
        }
    }

    pub fn add_transaction(&mut self, tx_id: String, payload: String) {
        self.cache.insert(tx_id, (payload, SystemTime::now()));
    }

    pub fn cleanup_expired(&mut self) {
        let now = SystemTime::now();
        self.cache.retain(|_, (_, timestamp)| {
            now.duration_since(*timestamp).unwrap_or(Duration::ZERO) < self.cache_lifetime
        });
    }

}