use common::transaction::Transaction;
use std::collections::HashMap;

pub struct Mempool {
    transactions: HashMap<String, Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        if self.transactions.contains_key(&hex::encode(transaction.id)) {
            return false; 
        }
        self.transactions.insert(hex::encode(transaction.id), transaction);
        true
    }

    pub fn get_transactions(&mut self, count: usize) -> Vec<Transaction> {
        let mut transactions_to_return = Vec::new();
        let keys: Vec<String> = self.transactions.keys().cloned().take(count).collect();

        for key in keys {
            if let Some(transaction) = self.transactions.remove(&key) {
                transactions_to_return.push(transaction);
            }
        }
        transactions_to_return
    }

    pub fn size(&self) -> usize {
        self.transactions.len()
    }
} 