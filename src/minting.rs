use crate::wallet::Wallet;
use sha2::{Digest, Sha256};

pub fn mint(wallet: &mut Wallet, amount: u64) {
    let mut hasher = Sha256::new();
    hasher.update(wallet.verifying_key.as_bytes());
    hasher.update(&amount.to_le_bytes());
    hasher.update(&wallet.nonce.to_le_bytes());
    let utxo_id = hex::encode(hasher.finalize());

    wallet.utxos.insert(utxo_id, amount);
    wallet.nonce += 1;
} 