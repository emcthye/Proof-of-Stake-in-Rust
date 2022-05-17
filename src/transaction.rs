use crate::util::Util;
use crate::wallet::Wallet;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const TRANSACTION_FEE: f64 = 1.0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransactionType {
    TRANSACTION,
    STAKE,
    VALIDATOR,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionInput {
    pub timestamp: i64,
    pub from: String,
    pub signature: String,
}

impl TransactionInput {
    pub fn new(sender_wallet: &mut Wallet, txn_output: &String) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            from: sender_wallet.get_public_key(),
            signature: sender_wallet.sign(txn_output),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionOutput {
    pub to: String,
    pub amount: f64,
    pub fee: f64,
}

impl TransactionOutput {
    pub fn new(to: String, amount: f64, fee: f64) -> Self {
        Self {
            to: to,
            amount: amount,
            fee: fee,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub txn_type: TransactionType,
    pub txn_input: TransactionInput,
    pub txn_output: TransactionOutput,
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Transaction {
    pub fn new(
        sender_wallet: &mut Wallet,
        to: String,
        amount: f64,
        txn_type: TransactionType,
    ) -> Result<Self, serde_json::Error> {
        let txn_output = TransactionOutput::new(to, amount, TRANSACTION_FEE);
        let serialized = match serde_json::to_string(&txn_output) {
            Ok(serialized) => serialized,
            Err(e) => return Err(e),
        };
        let txn_input = TransactionInput::new(sender_wallet, &serialized);

        Ok(Self {
            id: Util::id(),
            txn_type: txn_type,
            txn_output: txn_output,
            txn_input: txn_input,
        })
    }

    pub fn verify_txn(txn: &Transaction) -> Result<bool, serde_json::Error> {
        let txn_message = match serde_json::to_string(&txn.txn_output) {
            Ok(txn_message) => txn_message,
            Err(e) => return Err(e),
        };

        let result = match Util::verify_signature(
            &txn.txn_input.from,
            &txn_message,
            &txn.txn_input.signature,
        ) {
            Ok(result) => result,
            Err(e) => return Err(e),
        };

        Ok(result)
    }
}
