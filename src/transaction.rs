use std::time::{SystemTime, UNIX_EPOCH};
const MAX_PAYLOAD_SIZE: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub id: String,
    pub payload: String,
    pub timestamp: u64,
}

fn is_valid_id(id: &str) -> bool {
    id.chars().all(|c| c.is_alphanumeric() || c == '-')
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

    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("Transaction ID cannot be empty".into());
        }
        if !is_valid_id(&self.id) {
            return Err("Transaction ID must be alphanumeric with optional dashes".into());
        }
        if self.payload.trim().is_empty() {
            return Err("Transaction payload cannot be empty".into());
        }
        if self.payload.len() > MAX_PAYLOAD_SIZE {
            return Err(format!(
                "Transaction payload exceeds maximum size of {} characters",
                MAX_PAYLOAD_SIZE
            ));
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        if self.timestamp > now {
            return Err("Transaction timestamp cannot be in the future".into());
        }
        Ok(())
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

    #[test]
    fn test_validate_valid_transaction() {
        let tx = Transaction::new("tx1", "Valid payload");
        assert!(tx.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_id() {
        let tx = Transaction::new("", "Payload");
        assert!(tx.validate().is_err());
    }

    #[test]
    fn test_validate_empty_payload() {
        let tx = Transaction::new("tx2", "");
        assert!(tx.validate().is_err());
    }

    #[test]
    fn test_validate_payload_size() {
        let valid_payload = "a".repeat(256);
        let tx_valid = Transaction::new("tx1", valid_payload.clone());
        assert!(tx_valid.validate().is_ok());

        let invalid_payload = "a".repeat(257);
        let tx_invalid = Transaction::new("tx2", invalid_payload);
        assert!(tx_invalid.validate().is_err());
    }

    #[test]
    fn test_validate_id_format() {
        let tx_valid = Transaction::new("valid-id-123", "Payload");
        assert!(tx_valid.validate().is_ok());

        let tx_invalid = Transaction::new("invalid id!", "Payload");
        assert!(tx_invalid.validate().is_err());
    }

    #[test]
    fn test_validate_timestamp() {
        let mut tx = Transaction::new("tx1", "Payload");
        tx.timestamp = u64::MAX; // Faux timestamp dans le futur.
        assert!(tx.validate().is_err());
    }
}
