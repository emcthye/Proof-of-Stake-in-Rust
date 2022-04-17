use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stake {
    pub accounts: Vec<String>,
    pub balances: HashMap<String, u64>,
}

impl Stake {
    pub fn new() -> Self {
        Self {
            accounts: vec![
                "230681c76f00b412ccf7757a8449c448a04acd735e497a7612b66d8bfcb8e576".to_string(),
                "5aede624154386ca358af195e13a46981b917ee8279f30a67d7a211a3d3e7243".to_string(),
            ],
            balances: HashMap::from([
                (
                    "230681c76f00b412ccf7757a8449c448a04acd735e497a7612b66d8bfcb8e576".to_string(),
                    1,
                ),
                (
                    "5aede624154386ca358af195e13a46981b917ee8279f30a67d7a211a3d3e7243".to_string(),
                    100,
                ),
            ]),
        }
    }

    pub fn initialize(&mut self, address: &String) {
        if !self.balances.contains_key(address) {
            self.balances.insert(address.to_string(), 0);
            self.accounts.push(address.to_string());
        }
    }

    pub fn add_stake(&mut self, from: &String, amount: &u64) {
        self.initialize(from);
        *self.balances.get_mut(from).unwrap() += amount;
    }

    pub fn get_max(&mut self, addresses: &Vec<String>) -> String {
        let key = self
            .balances
            .iter()
            .filter(|addr| addresses.contains(&addr.0))
            .collect::<HashMap<_, _>>();
        key.iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .unwrap()
            .to_string()
    }

    pub fn update(&mut self, txn: &Transaction) {
        self.add_stake(&txn.txn_input.from, &(*&txn.txn_output.amount as u64))
    }

    pub fn get_balance(&mut self, address: &String) -> &u64 {
        self.initialize(address);
        self.balances.get(address).unwrap()
    }
}
