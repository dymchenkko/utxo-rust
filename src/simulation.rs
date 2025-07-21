use crate::{
    minting::mint,
    processing::process_transaction,
    transaction::{Transaction, validate_transaction},
    wallet::Wallet,
};

pub fn run() {
    println!("Initializing wallets...");

    let mut wallet_a = Wallet::new();
    println!("\nWallet A created:");
    println!("  Private Key: {}", hex::encode(wallet_a.signing_key.to_bytes()));
    println!("  Public Key (Address): {}", hex::encode(wallet_a.verifying_key.as_bytes()));

    let mut wallet_b = Wallet::new();
    println!("\nWallet B created:");
    println!("  Private Key: {}", hex::encode(wallet_b.signing_key.to_bytes()));
    println!("  Public Key (Address): {}", hex::encode(wallet_b.verifying_key.as_bytes()));

    println!("\nFunding Wallet A with minted UTXOs...");
    mint(&mut wallet_a, 100);
    mint(&mut wallet_a, 50);
    println!("Wallet A balance: {}", wallet_a.balance());
    println!("Wallet A nonce: {}", wallet_a.nonce);

    println!("\nCreating and signing a transaction from Wallet A to Wallet B for 120...");
    let mut transaction = Transaction::new(
        wallet_a.verifying_key,
        wallet_b.verifying_key,
        120,
        wallet_a.nonce,
    );
    transaction.sign(&wallet_a);
    println!("Transaction ID: {}", hex::encode(transaction.id));

    println!("\nValidating transaction...");
    if validate_transaction(&transaction, &wallet_a) {
        println!("Transaction is valid. Processing...");
        process_transaction(&mut transaction, &mut wallet_a, &mut wallet_b);
        println!("Transaction processed.");
        println!("\nFinal balances:");
        println!("  Wallet A balance: {}", wallet_a.balance());
        println!("  Wallet A nonce: {}", wallet_a.nonce);
        println!("  Wallet B balance: {}", wallet_b.balance());
    } else {
        println!("Transaction is invalid");
    }
} 