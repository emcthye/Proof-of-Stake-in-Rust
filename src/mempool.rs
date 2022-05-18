use log::warn;

use crate::transaction::Transaction;

pub struct Mempool {
    pub transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, txn: Transaction) {
        self.transactions.push(txn);
    }

    pub fn validate_transactions(&mut self) -> Vec<Transaction> {
        self.transactions
            .drain(..)
            .filter(|txn| Transaction::verify_txn(txn).is_ok())
            .collect()
    }

    pub fn transaction_exists(&mut self, txn: &Transaction) -> bool {
        self.transactions.contains(txn)
    }

    pub fn clear(&mut self) {
        self.transactions.clear()
    }
}
