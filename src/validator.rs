use crate::transaction::Transaction;

pub struct Validator {
    pub accounts: Vec<String>,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            accounts: vec![
                "5aad9b5e21f63955e8840e8b954926c60e0e2d906fdbc0ce1e3afe249a67f614".to_string(),
            ],
        }
    }

    pub fn update(&mut self, txn: &Transaction) -> bool {
        if txn.txn_output.amount >= 25.0 && txn.txn_output.to == "0".to_string() {
            self.accounts.push(txn.txn_input.from.to_string());
            return true;
        }
        false
    }
}
