use crate::{
    blockchain::{
        mempool::Mempool,
        processing::process_transaction,
    },
    wallet::wallet::Wallet,
};

const COIN: u64 = 100_000_000;

pub fn run_simulation() {
    println!("Initializing blockchain and mempool...");
    let mut blockchain = crate::blockchain::blockchain::Blockchain::new();
    let mut mempool = Mempool::new();

    println!("\nInitializing wallets...");
    let mut wallet_a = Wallet::new();
    let mut wallet_b = Wallet::new();
    println!("Wallet A address: {}", hex::encode(wallet_a.verifying_key.as_bytes()));
    println!("Wallet B address: {}", hex::encode(wallet_b.verifying_key.as_bytes()));
    println!("Wallet B public key: {:?}", wallet_b.verifying_key.as_bytes());

    println!("\nMinting initial funds for Wallet A...");
    let amount_to_mint = 100 * COIN;
    let utxo_id = format!("{}-{}", hex::encode(wallet_a.verifying_key.as_bytes()), wallet_a.nonce);
    wallet_a.utxos.insert(utxo_id.clone(), amount_to_mint);
    wallet_a.balance += amount_to_mint;
    blockchain.utxos.insert(utxo_id);
    println!("Wallet A balance: {} units", wallet_a.balance);
    
    println!("\nCreating transactions and adding to mempool...");
    let tx = wallet_a.create_transaction(wallet_b.verifying_key, 50);
    process_transaction(&mut mempool, &tx);
    println!("Transaction 1 added to mempool. Mempool size: {}", mempool.size());

    let tx2 = wallet_a.create_transaction(wallet_b.verifying_key, 30 * COIN);
    mempool.add_transaction(tx2.clone());
    println!("Transaction 2 added to mempool. Mempool size: {}", mempool.size());

    println!("\nMining a new block...");
    let transactions_for_block = mempool.get_transactions(10);
    let new_block = blockchain.create_block(transactions_for_block);
    if blockchain.add_block(new_block.clone()) {
        println!("New block added to the blockchain successfully.");
        for tx in &new_block.transactions {
            wallet_a.apply_transaction(tx);
            wallet_b.apply_transaction(tx);
        }
    } else {
        println!("Failed to add new block.");
    }

    println!("\nFinal state:");
    println!("  Number of blocks in blockchain: {}", blockchain.blocks.len());
    println!("  Mempool size: {}", mempool.size());
    println!("  Wallet A balance: {} units", wallet_a.balance);
    println!("  Wallet B balance: {} units", wallet_b.balance);
} 