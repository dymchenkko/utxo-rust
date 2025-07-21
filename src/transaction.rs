use serde::{Deserialize, Serialize};
use crate::wallet::Wallet;
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use sha2::{Sha256, Digest};

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

    pub fn sign(&mut self, wallet: &Wallet) {
        self.signature = Some(wallet.sign(&self.id));
    }

    pub fn verify(&self) -> bool {
        if let Some(signature) = self.signature {
            self.sender.verify(&self.id, &signature).is_ok()
        } else {
            false
        }
    }
}

pub fn validate_transaction(
    transaction: &Transaction,
    sender_wallet: &Wallet,
) -> bool {
    if sender_wallet.balance() < transaction.amount {
        return false;
    }

    transaction.verify()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::Wallet;

    #[test]
    fn test_sign_and_verify_transaction() {
        let mut wallet_a = Wallet::new();
        let wallet_b = Wallet::new();

        let mut transaction = Transaction::new(
            wallet_a.verifying_key,
            wallet_b.verifying_key,
            50,
            wallet_a.nonce,
        );
        transaction.sign(&mut wallet_a);

        assert!(transaction.verify());
    }

    #[test]
    fn test_validate_transaction() {
        let mut wallet_a = Wallet::new();
        let wallet_b = Wallet::new();
        wallet_a.utxos.insert("utxo1".to_string(), 100);

        let mut transaction = Transaction::new(
            wallet_a.verifying_key,
            wallet_b.verifying_key,
            50,
            wallet_a.nonce,
        );
        transaction.sign(&mut wallet_a);

        assert!(validate_transaction(&transaction, &wallet_a));
    }

    #[test]
    fn test_invalidate_transaction_insufficient_funds() {
        let mut wallet_a = Wallet::new();
        let wallet_b = Wallet::new();
        wallet_a.utxos.insert("utxo1".to_string(), 30);

        let mut transaction = Transaction::new(
            wallet_a.verifying_key,
            wallet_b.verifying_key,
            50,
            wallet_a.nonce,
        );
        transaction.sign(&mut wallet_a);

        assert!(!validate_transaction(&transaction, &wallet_a));
    }

    #[test]
    fn test_invalidate_tampered_transaction() {
        let mut wallet_a = Wallet::new();
        let wallet_b = Wallet::new();
        wallet_a.utxos.insert("utxo1".to_string(), 100);

        let mut transaction = Transaction::new(
            wallet_a.verifying_key,
            wallet_b.verifying_key,
            50,
            wallet_a.nonce,
        );
        transaction.sign(&mut wallet_a);

        // Tamper with the transaction
        transaction.amount = 1000;

        assert!(!validate_transaction(&transaction, &wallet_a));
    }
} 