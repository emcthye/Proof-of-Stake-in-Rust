use crate::util::Util;
use crate::wallet::Wallet;
use crate::{block, transaction::Transaction};
use chrono::prelude::*;
use ed25519_dalek::{PublicKey, Signature};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

pub const DIFFICULTY_PREFIX: &str = "00";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub txn: Vec<Transaction>,
    pub validator: String,
    pub signature: String,
    pub difficulty: u32,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.previous_hash == other.previous_hash
    }
}

impl Block {
    pub fn new(
        id: usize,
        previous_hash: String,
        timestamp: i64,
        txn: Vec<Transaction>,
        difficulty: u32,
        mut validator_wallet: Wallet,
    ) -> Self {
        let validator = validator_wallet.get_public_key();
        let hash = block::calculate_hash(
            &id,
            &timestamp,
            &previous_hash,
            &txn,
            &validator,
            &difficulty,
        );
        let signature = validator_wallet.sign(&hash);
        Self {
            id,
            hash,
            previous_hash,
            timestamp,
            txn,
            validator,
            signature,
            difficulty,
        }
    }

    pub fn genesis() -> Self {
        let id = 0;
        let timestamp = 1650205976;
        let previous_hash = String::from("genesis");
        let txn = vec![];
        let validator = String::from("genesis");
        let signature = String::from("genesis");
        let difficulty = 5;

        let hash = block::calculate_hash(
            &id,
            &timestamp,
            &previous_hash,
            &txn,
            &validator,
            &difficulty,
        );

        Self {
            id,
            hash,
            previous_hash,
            timestamp,
            txn,
            validator,
            signature,
            difficulty,
        }
    }

    pub fn verify_block_signature(block: &Block) -> bool {
        info!("verifying block...");
        let hash = block::calculate_hash(
            &block.id,
            &block.timestamp,
            &block.previous_hash,
            &block.txn,
            &block.validator,
            &block.difficulty,
        );

        Util::verifySignature(&block.validator, &hash, &block.signature)
    }
}

pub fn calculate_hash(
    id: &usize,
    timestamp: &i64,
    previous_hash: &str,
    txn: &Vec<Transaction>,
    validator: &String,
    difficulty: &u32,
) -> String {
    info!("calculating hash...");
    let hash = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "transactions": txn,
        "timestamp": timestamp,
        "validator": validator,
        "difficulty": difficulty,
    });

    Util::hash(&hash.to_string())
}
