use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stake {
    pub accounts: Vec<String>,
    pub balances: HashMap<String, f64>,
}

impl Stake {
    pub fn new() -> Self {
        Self {
            accounts: vec![String::from(
                "5aad9b5e21f63955e8840e8b954926c60e0e2d906fdbc0ce1e3afe249a67f614",
            )],
            balances: HashMap::from([(
                String::from("5aad9b5e21f63955e8840e8b954926c60e0e2d906fdbc0ce1e3afe249a67f614"),
                0.00,
            )]),
        }
    }

    pub fn initialize(&mut self, address: &String) {
        if !self.balances.contains_key(address) {
            self.balances.insert(address.to_string(), 0.00);
            self.accounts.push(address.to_string());
        }
    }

    pub fn add_stake(&mut self, from: &String, amount: f64) {
        self.initialize(from);
        self.balances.get_mut(from).unwrap() += amount;
    }

    pub fn get_max(addresses: Vec<String>) -> String {
        let mut balance = -1;
        let mut leader = null;
        addresses.iter().for_each(|addr|
            if self.balances.get(addr) > balance {
                balance = self.balances.get(addr);
                leader = addr;
            }
        );
        leader
    }

    pub fn update(&mut self, txn: &Transaction) {
        self.add_stake(&txn.input.from, txn.amount)
    }

//     pub fn transfer(&mut self, from: &String, to: &String, amount: f64) {
//         self.initialize(from);
//         self.initialize(to);
//         self.increment(to, amount);
//         self.decrement(from, amount);
//     }

//     fn increment(&mut self, to: &String, amount: f64) {
//         self.balances.get_mut(to).unwrap() += amount;
//     }

//     fn decrement(&mut self, from: &String, amount: f64) {
//         (*self.balances.get_mut(from).unwrap()) -= amount;
//     }

//     fn getBalance(&mut self, address: &String) -> &f64 {
//         self.initialize(address);
//         self.balances.get(address).unwrap()
//     }
// }
