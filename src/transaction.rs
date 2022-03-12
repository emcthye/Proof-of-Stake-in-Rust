use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TRANSACTION_FEE: f64 = 1.0;

pub enum TransactionType {
    TRANSACTION,
    STAKE,
    VALIDATOR
}

pub struct TransactionInput {
    pub timestamp: i64,
    pub from: String,
    pub signature: String
}

pub struct TransactionOutput {
    pub to: String,
    pub amount: f64,
    pub fee: f64
}

pub struct Transaction {
    pub id: Uuid,
    pub index: u32,
    pub txn_type: TransactionType,
    pub txn_input: TransactionInput,
    pub txn_output: TransactionOutput,

}