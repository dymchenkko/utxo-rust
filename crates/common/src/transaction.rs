use bincode;
use ed25519_dalek::{Signature, Signer, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: [u8; 32],
    pub sender: VerifyingKey,
    pub receiver: VerifyingKey,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Option<Signature>,
}

impl Transaction {
    pub fn new(sender: VerifyingKey, receiver: VerifyingKey, amount: u64, nonce: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(sender.as_bytes());
        hasher.update(receiver.as_bytes());
        hasher.update(&amount.to_le_bytes());
        hasher.update(&nonce.to_le_bytes());
        let id: [u8; 32] = hasher.finalize().into();

        Transaction {
            id,
            sender,
            receiver,
            amount,
            nonce,
            signature: None,
        }
    }

    pub fn sign(&mut self, signing_key: &ed25519_dalek::SigningKey) {
        let message = self.calculate_hash();
        self.signature = Some(signing_key.sign(&message));
    }

    pub fn verify(&self) -> bool {
        if let Some(signature) = self.signature {
            self.sender.verify(&self.id, &signature).is_ok()
        } else {
            false
        }
    }

    fn calculate_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        let mut transaction_data = self.clone();
        transaction_data.signature = None;
        let serialized_data = bincode::serialize(&transaction_data).unwrap();
        hasher.update(&serialized_data);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        // This test needs to be adapted as we can't use Wallet here directly
    }
}
