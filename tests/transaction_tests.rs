
use ed25519_dalek::SigningKey;
use eco_weave::Transaction;


#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

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
        tx.timestamp = u64::MAX; // Invalid timestamp in the future.
        assert!(tx.validate().is_err());
    }

    #[test]
    fn test_sign_and_verify_transaction() {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let mut tx = Transaction::new("tx1", "Payload");
        tx.sign(&signing_key);

        assert!(tx.signature.is_some());
        assert!(tx.validate_signature(&verifying_key).is_ok());
    }

    #[test]
    fn test_invalid_signature() {
        let mut rng = OsRng;
        let signing_key1 = SigningKey::generate(&mut rng);
        let signing_key2 = SigningKey::generate(&mut rng);

        let verifying_key2 = signing_key2.verifying_key();

        let mut tx = Transaction::new("tx1", "Payload");
        tx.sign(&signing_key1);

        assert!(tx.validate_signature(&verifying_key2).is_err());
    }
}
