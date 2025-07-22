use crate::{
    mempool::Mempool,
};
use common::transaction::Transaction;

pub fn process_transaction(
    mempool: &mut Mempool,
    transaction: &Transaction,
) {
    if transaction.verify() {
        mempool.add_transaction(transaction.clone());
    }
} 