use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub accounts: Vec<String>,
    pub balances: HashMap<String, f64>,
}

impl Account {
    pub fn new() -> Self {
        Self {
            accounts: vec![
                String::from("230681c76f00b412ccf7757a8449c448a04acd735e497a7612b66d8bfcb8e576"),
                String::from("5aede624154386ca358af195e13a46981b917ee8279f30a67d7a211a3d3e7243"),
            ],
            balances: HashMap::from([
                (
                    String::from(
                        "230681c76f00b412ccf7757a8449c448a04acd735e497a7612b66d8bfcb8e576",
                    ),
                    500.00,
                ),
                (
                    String::from(
                        "5aede624154386ca358af195e13a46981b917ee8279f30a67d7a211a3d3e7243",
                    ),
                    500.00,
                ),
            ]),
        }
    }

    pub fn initialize(&mut self, address: &String) {
        if !self.balances.contains_key(address) {
            self.balances.insert(address.to_string(), 0.00);
            self.accounts.push(address.to_string());
        }
    }

    pub fn transfer(&mut self, from: &String, to: &String, amount: &f64) {
        self.initialize(from);
        self.initialize(to);
        self.increment(to, amount);
        self.decrement(from, amount);
    }

    pub fn increment(&mut self, to: &String, amount: &f64) {
        (*self.balances.get_mut(to).unwrap()) += amount;
    }

    pub fn decrement(&mut self, from: &String, amount: &f64) {
        (*self.balances.get_mut(from).unwrap()) -= amount;
    }

    pub fn get_balance(&mut self, address: &String) -> &f64 {
        self.initialize(address);
        self.balances.get(address).unwrap()
    }
}
