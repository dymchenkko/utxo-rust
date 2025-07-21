use crate::{
    transaction::Transaction,
    wallet::Wallet,
};

pub fn process_transaction(
    transaction: &mut Transaction,
    sender_wallet: &mut Wallet,
    receiver_wallet: &mut Wallet,
) {
    let mut spent_utxos = Vec::new();
    let mut collected_amount = 0;

    for (utxo_id, value) in sender_wallet.utxos.iter() {
        if collected_amount < transaction.amount {
            collected_amount += value;
            spent_utxos.push(utxo_id.clone());
        } else {
            break;
        }
    }

    for utxo_id in spent_utxos {
        sender_wallet.utxos.remove(&utxo_id);
    }

    let change = collected_amount - transaction.amount;
    if change > 0 {
        let change_utxo_id = format!("{}:1", hex::encode(transaction.id));
        sender_wallet
            .utxos
            .insert(change_utxo_id, change);
    }

    let receiver_utxo_id = format!("{}:0", hex::encode(transaction.id));
    receiver_wallet
        .utxos
        .insert(receiver_utxo_id, transaction.amount);

    sender_wallet.nonce += 1;
} 