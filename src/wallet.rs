use std::collections::HashMap;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    pub utxos: HashMap<String, u64>,
    pub nonce: u64,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        // 1. A new private key (SigningKey) is generated here.
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        // 2. The public key (VerifyingKey) is derived from the private key.
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
            utxos: HashMap::new(),
            nonce: 0,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn balance(&self) -> u64 {
        self.utxos.values().sum()
    }
} 