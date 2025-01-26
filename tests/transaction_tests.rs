#[cfg(test)]
mod tests {
    use eco_weave::Transaction;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    const MAX_PAYLOAD_SIZE: usize = 256;


    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new("tx1", r#"{"temperature": 25}"#).unwrap();
        assert_eq!(tx.id, "tx1");
        assert_eq!(tx.payload, r#"{"temperature": 25}"#);
        assert!(tx.timestamp > 0);
    }

    #[test]
    fn test_validate_valid_transaction() {
        let tx = Transaction::new("tx1", r#"{"temperature": 25}"#).unwrap();
        assert!(tx.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_id() {
        let tx = Transaction::new("", r#"{"temperature": 25}"#);
        assert!(tx.is_err()); // Vérifie que la création échoue.
    }

    #[test]
    fn test_validate_empty_payload() {
        let tx = Transaction::new("tx2", "");
        assert!(tx.is_err()); // Vérifie que la création échoue.
    }


    #[test]
    fn test_validate_payload_size() {
        let valid_payload = "a".repeat(MAX_PAYLOAD_SIZE);
        let tx_valid = Transaction::new("tx1", valid_payload.clone()).unwrap();
        assert!(tx_valid.validate().is_ok());

        let invalid_payload = "a".repeat(MAX_PAYLOAD_SIZE + 1);
        let tx_invalid = Transaction::new("tx2", invalid_payload);
        assert!(tx_invalid.is_err());
    }


    #[test]
    fn test_validate_id_format() {
        // ID valide
        let tx_valid = Transaction::new("valid-id-123", "Payload").unwrap();
        assert!(
            tx_valid.validate().is_ok(),
            "Transaction with valid ID format should pass"
        );

        // ID invalide
        let tx_invalid = Transaction::new("invalid id!", "Payload");
        assert!(
            tx_invalid.is_err(),
            "Transaction with invalid ID format should fail"
        );
    }



    #[test]
    fn test_validate_timestamp() {
        let mut tx = Transaction::new("tx1", r#"{"temperature": 25}"#).unwrap();
        tx.timestamp = u64::MAX; // Timestamp invalide (dans le futur).
        assert!(tx.validate().is_err());
    }

    #[test]
    fn test_sign_and_verify_transaction() {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let mut tx = Transaction::new("tx1", r#"{"temperature": 25}"#).unwrap();
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

        let mut tx = Transaction::new("tx1", r#"{"temperature": 25}"#).unwrap();
        tx.sign(&signing_key1);

        assert!(tx.validate_signature(&verifying_key2).is_err());
    }
}
