use crate::transaction::Transaction;

const TRANSACTION_THRESHOLD: usize = 3;

pub struct Mempool {
    pub transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }

    fn threshold_reached(&mut self) -> bool {
        self.transactions.len() > TRANSACTION_THRESHOLD
    }

    pub fn add_transaction(&mut self, txn: Transaction) -> bool {
        self.transactions.push(txn);
        self.threshold_reached()
    }

    pub fn validate_transactions(&mut self) -> Vec<Transaction> {
        self.transactions
            .drain(..)
            .filter(|txn| Transaction::verify_txn(txn))
            .collect()
    }

    pub fn transaction_exists(&mut self, txn: &Transaction) -> bool {
        self.transactions.contains(txn)
    }

    pub fn clear(&mut self) {
        self.transactions.clear()
    }
}
