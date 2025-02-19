use std::time::{SystemTime, UNIX_EPOCH};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::Rng;

const MAX_PAYLOAD_SIZE: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub id: String,
    pub payload: String,
    pub timestamp: u64,
    nonce: u64,
    pub signature: Option<Signature>,
    pub weight: u32,
    pub confirmed: bool,
}

fn is_valid_id(id: &str) -> bool {
    id.chars().all(|c| c.is_alphanumeric() || c == '-')
}


impl Transaction {
    pub fn new(id: impl Into<String>, payload: impl Into<String>) -> Result<Self, String> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err("transactionInvalidId".to_string());
        }

        if !is_valid_id(&id) {
            return Err("transactionInvalidIdFormat".to_string());
        }

        let payload = payload.into();
        if payload.trim().is_empty() {
            return Err("transactionInvalidPayload".to_string());
        }

        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err("transactionPayloadTooLarge".to_string());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        let nonce = rand::thread_rng().gen::<u64>();

        Ok(Self {
            id,
            payload,
            timestamp,
            nonce,
            signature: None,
            weight: 0,
            confirmed: false,
        })
    }



    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("transactionInvalidId: ID is empty".into());
        }

        if !is_valid_id(&self.id) {
            return Err(format!("transactionInvalidIdFormat: {}", self.id));
        }

        if self.payload.trim().is_empty() {
            return Err("transactionInvalidPayload: Payload is empty".into());
        }

        if self.payload.len() > MAX_PAYLOAD_SIZE {
            return Err(format!(
                "transactionPayloadTooLarge: {} bytes (max: {} bytes)",
                self.payload.len(),
                MAX_PAYLOAD_SIZE
            ));
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;

        if self.timestamp > now {
            return Err(format!(
                "transactionTimestampInvalid: {} (now: {})",
                self.timestamp, now
            ));
        }

        Ok(())
    }


    pub fn calculate_weight(&self, approvals: usize) -> u32 {
        (approvals as u32).max(1)
    }

    fn serialize(&self) -> String {
        format!("{}:{}:{}:{}", self.id, self.payload, self.timestamp, self.nonce)
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

    pub fn confirm(&mut self) {
        self.confirmed = true;
    }
}

