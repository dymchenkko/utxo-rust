use crate::{blockchain::mempool::Mempool, wallet::wallet::Wallet};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const COIN: u64 = 100_000_000;

pub fn run_simulation() {
    println!("Initializing blockchain and mempool...");
    let mut blockchain = crate::blockchain::blockchain::Blockchain::new();
    let mut mempool = Mempool::new();

    println!("\nInitializing wallets...");
    let mut wallet_a = Wallet::new();
    let wallet_b = Wallet::new();
    println!(
        "Wallet A address: {}",
        hex::encode(wallet_a.verifying_key.as_bytes())
    );
    println!(
        "Wallet B address: {}",
        hex::encode(wallet_b.verifying_key.as_bytes())
    );
    println!(
        "Wallet B public key: {:?}",
        wallet_b.verifying_key.as_bytes()
    );

    println!("\nMinting initial funds for Wallet A...");
    let amount_to_mint = 100 * COIN;
    let utxo_id = format!(
        "{}-{}",
        hex::encode(wallet_a.verifying_key.as_bytes()),
        wallet_a.nonce
    );
    wallet_a.balance += amount_to_mint;
    blockchain.utxos.insert(utxo_id);
    println!("Wallet A balance: {} units", wallet_a.balance);

    println!("\nCreating transactions and adding to mempool...");
    let tx = wallet_a.create_transaction(wallet_b.verifying_key, 5 * COIN);
    mempool.add_transaction(tx.clone());
    println!(
        "Transaction 1 added to mempool. Mempool size: {}",
        mempool.size()
    );

    let tx2 = wallet_a.create_transaction(wallet_b.verifying_key, 3 * COIN);
    mempool.add_transaction(tx2.clone());
    println!(
        "Transaction 2 added to mempool. Mempool size: {}",
        mempool.size()
    );

    println!("\nMining a new block...");
    let transactions_for_block = mempool.get_transactions(10);
    println!("Transactions for block: {:?}", transactions_for_block.len());
    let new_block = blockchain.create_block(transactions_for_block);
    println!("Block size: {:?}", new_block.transactions.len());
    if blockchain.add_block(new_block.clone()) {
        println!("New block added to the blockchain successfully.");
    } else {
        println!("Failed to add new block.");
    }

    println!("\nFinal state:");
    println!(
        "  Number of blocks in blockchain: {}",
        blockchain.blocks.len()
    );
    println!("  Mempool size: {}", mempool.size());
    println!("  Wallet A balance: {} units", wallet_a.balance);
    println!("  Wallet B balance: {} units", wallet_b.balance);

    // Create shared state for the thread
    let blockchain_arc = Arc::new(Mutex::new(blockchain));
    let wallet_b_arc = Arc::new(Mutex::new(wallet_b));

    // Clone the Arc references for the thread
    let blockchain_clone = blockchain_arc.clone();
    let wallet_b_clone = wallet_b_arc.clone();

    println!("\nStarting database monitoring thread...");
    let _ = thread::spawn(move || {
        let mut last_processed_block_count = 0;

        loop {
            // Query the blockchain database (in this case, our in-memory blockchain)
            let blockchain_guard = blockchain_clone.lock().unwrap();
            let current_block_count = blockchain_guard.blocks.len();

            // Check if new blocks were added
            if current_block_count > last_processed_block_count {
                println!("New blocks detected! Processing transactions...");

                // Process new blocks and their transactions
                for block_idx in last_processed_block_count..current_block_count {
                    let block = &blockchain_guard.blocks[block_idx];

                    for transaction in &block.transactions {
                        // Update receiver wallet balance
                        let mut wallet_guard = wallet_b_clone.lock().unwrap();
                        wallet_guard.apply_transaction(transaction);

                        println!(
                            "Applied transaction: {} -> {} (amount: {})",
                            hex::encode(transaction.sender.as_bytes())[..8].to_string(),
                            hex::encode(transaction.receiver.as_bytes())[..8].to_string(),
                            transaction.amount
                        );
                    }
                }

                // Update the last processed block count
                last_processed_block_count = current_block_count;

                // Print updated wallet balance
                let wallet_guard = wallet_b_clone.lock().unwrap();
                println!("Updated Wallet B balance: {} units", wallet_guard.balance);
            }

            // Drop the guards before sleeping
            drop(blockchain_guard);

            // Sleep for 1 second before next query
            thread::sleep(Duration::from_secs(1));
        }
    })
    .join();
}
