use std::time::{SystemTime, UNIX_EPOCH};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

const MAX_PAYLOAD_SIZE: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub id: String,
    pub payload: String,
    pub timestamp: u64,
    pub signature: Option<Signature>,
    pub weight: u32,
    pub confirmed: bool,
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
            signature: None,
            weight: 0,
            confirmed: false,
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

    pub fn calculate_weight(&self, approvals: usize) -> u32 {
        // Exemple simple : poids basé sur le nombre d'approbations.
        (approvals as u32).max(1)
    }

    pub fn sign(&mut self, signing_key: &SigningKey) {
        let data = self.serialize();
        self.signature = Some(signing_key.sign(data.as_bytes()));
    }

    pub fn validate_signature(&self, verifying_key: &VerifyingKey) -> Result<(), String> {
        if let Some(signature) = &self.signature {
            let data = self.serialize();
            verifying_key
                .verify(data.as_bytes(), signature)
                .map_err(|_| "Invalid signature".to_string())
        } else {
            Err("Transaction is not signed".to_string())
        }
    }

    fn serialize(&self) -> String {
        format!("{}:{}:{}", self.id, self.payload, self.timestamp)
    }

    pub fn confirm(&mut self) {
        self.confirmed = true;
    }
}

