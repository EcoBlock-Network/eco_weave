use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub id: String,
    pub payload: String,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(id: impl Into<String>, payload: impl Into<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        Self {
            id: id.into(),
            payload: payload.into(),
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new("tx1", "Hello, Tangle!");
        assert_eq!(tx.id, "tx1");
        assert_eq!(tx.payload, "Hello, Tangle!");
        assert!(tx.timestamp > 0);
    }
}
