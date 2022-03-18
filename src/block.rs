use crate::util::Util;
use crate::wallet::Wallet;
use crate::{block, transaction::Transaction};
use chrono::prelude::*;
use ed25519_dalek::{PublicKey, Signature};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

pub const DIFFICULTY_PREFIX: &str = "00";

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: usize,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub txn: Vec<Transaction>,
    pub validator: String,
    pub signature: String,
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
        txn: Vec<Transaction>,
        validator_wallet: &mut Wallet,
    ) -> Self {
        info!("Creating block...");
        let timestamp = Utc::now().timestamp();
        let hash = block::calculate_hash(&id, &timestamp, &previous_hash, &txn);
        let validator = validator_wallet.getPublicKey();
        let signature = validator_wallet.sign(&hash);
        Self {
            id,
            hash,
            previous_hash,
            timestamp,
            txn,
            validator,
            signature,
        }
    }

    pub fn verify_block_signature(block: &Block) -> bool {
        info!("Verifying block...");
        let data = serde_json::json!({
            "id": block.id,
            "previous_hash": block.previous_hash,
            "transactions": block.txn,
            "timestamp": block.timestamp
        });

        Util::verifySignature(
            PublicKey::from_bytes(block.validator.as_bytes()).unwrap(),
            &data.to_string(),
            &Signature::from_bytes(block.signature.as_bytes()).unwrap(),
        )
    }
}

pub fn calculate_hash(
    id: &usize,
    timestamp: &i64,
    previous_hash: &str,
    txn: &Vec<Transaction>,
) -> String {
    let hash = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "transactions": txn,
        "timestamp": timestamp
    });

    Util::hash(&hash.to_string())
}
