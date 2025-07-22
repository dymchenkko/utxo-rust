use crate::block::Block;
use common::transaction::Transaction;
use std::collections::HashSet;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub utxos: HashSet<String>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(String::from("0").repeat(64), vec![]);
        let utxos = HashSet::new();
        // In a real implementation, the genesis block might contain initial transactions
        // that create the first UTXOs. For simplicity, we start with an empty set.
        Self {
            blocks: vec![genesis_block],
            utxos,
        }
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        // Basic validation: the previous block hash must match.
        let previous_hash = self.blocks.last().unwrap().calculate_hash();
        if block.header.previous_block_hash != previous_hash {
            println!("Invalid previous block hash");
            return false;
        }

        // Update UTXO set based on the transactions in the new block.
        for tx in &block.transactions {
            // For each transaction, find the UTXOs it spends
            // This is a simplified model. A real implementation would need to
            // resolve transaction inputs to specific UTXOs.
            let input_utxo = format!("{}-{}", hex::encode(tx.sender.as_bytes()), tx.nonce - 1);
            if !self.utxos.contains(&input_utxo) {
                println!("Input UTXO not found or already spent: {}", input_utxo);
                return false;
            }
            self.utxos.remove(&input_utxo);

            // Add the new UTXOs created by this transaction
            let output_utxo = format!("{}-{}", hex::encode(tx.receiver.as_bytes()), tx.nonce);
            self.utxos.insert(output_utxo);
        }

        self.blocks.push(block);
        true
    }

    pub fn create_block(&self, transactions: Vec<Transaction>) -> Block {
        let previous_block_hash = self.blocks.last().unwrap().calculate_hash();
        Block::new(previous_block_hash, transactions)
    }

    pub fn get_last_block_hash(&self) -> String {
        self.blocks.last().unwrap().calculate_hash()
    }
} 