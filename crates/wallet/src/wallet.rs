use common::transaction::Transaction;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub balance: u64,
    pub utxos: std::collections::HashMap<String, u64>,
    pub nonce: u64,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = (&signing_key).into();
        Self {
            signing_key,
            verifying_key,
            balance: 0,
            utxos: HashMap::new(),
            nonce: 0,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn create_transaction(&mut self, receiver_address: VerifyingKey, amount: u64) -> Transaction {
        self.balance -= amount;
        let mut tx = Transaction::new(self.verifying_key, receiver_address, amount, self.nonce);
        tx.sign(&self.signing_key);
        self.nonce += 1;
        tx
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) {
        if tx.receiver == self.verifying_key {
            self.balance += tx.amount;
        }
    }
} 