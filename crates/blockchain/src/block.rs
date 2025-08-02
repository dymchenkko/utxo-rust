use common::transaction::Transaction;
use hex;
use sha2::{Digest, Sha256};
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct BlockHeader {
    pub previous_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u32,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(previous_block_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let merkle_root = Self::calculate_merkle_root(&transactions);
        let header = BlockHeader {
            previous_block_hash,
            merkle_root,
            timestamp,
            nonce: 0,
        };
        Self {
            header,
            transactions,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let header_string = format!(
            "{}{}{}{}",
            self.header.previous_block_hash,
            self.header.merkle_root,
            self.header.timestamp,
            self.header.nonce
        );
        hasher.update(header_string.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return String::from("0").repeat(64);
        }

        let mut hashes: Vec<String> = transactions.iter().map(|tx| hex::encode(tx.id)).collect();

        while hashes.len() > 1 {
            hashes = hashes
                .chunks(2)
                .map(|chunk| {
                    let mut hasher = Sha256::new();
                    hasher.update(chunk[0].as_bytes());

                    // If a chunk has only one hash (at the end of an odd-length list),
                    // it is hashed with a duplicate of itself.
                    let second_hash = if chunk.len() > 1 {
                        &chunk[1]
                    } else {
                        &chunk[0]
                    };
                    hasher.update(second_hash.as_bytes());

                    format!("{:x}", hasher.finalize())
                })
                .collect();
        }

        // The loop continues until only one hash remains, which is the Merkle Root.
        hashes.pop().unwrap()
    }
}
