use crate::transaction::Transaction;

pub struct Validator {
    pub accounts: Vec<String>,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            accounts: vec![
                "230681c76f00b412ccf7757a8449c448a04acd735e497a7612b66d8bfcb8e576".to_string(),
                "5aede624154386ca358af195e13a46981b917ee8279f30a67d7a211a3d3e7243".to_string()
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
