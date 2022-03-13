use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use log::{error, info, warn};
use ed25519_dalek::{Signature, PublicKey};
use crate::util::Util;
use crate::wallet::Wallet;
use crate::block;

pub const DIFFICULTY_PREFIX: &str = "00";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub validator: String,
    pub signature: String
}

impl Block {

    pub fn new(id: u64, previous_hash: String, data: String, validator_wallet: &mut Wallet ) -> Self {
        info!("Creating block...");
        let timestamp = Utc::now().timestamp();
        let hash = hex::encode(block::calculate_hash(id, timestamp, &previous_hash, &data));
        let validator = validator_wallet.getPublicKey();
        let signature = validator_wallet.sign(&hash);
        Self {
            id,
            hash,
            previous_hash,
            timestamp,
            data,
            validator,
            signature
        }
    }

    pub fn verify_block(block: &Block) -> bool {
        info!("Verifying block...");
        let data = serde_json::json!({
            "id": block.id,
            "previous_hash": block.previous_hash,
            "data": block.data,
            "timestamp": block.timestamp
        });

        Util::verifySignature(
            PublicKey::from_bytes(block.validator.as_bytes()).unwrap(), 
            &data.to_string(), 
            &Signature::from_bytes(block.signature.as_bytes()).unwrap())
    }

}

pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp
    });

    Util::hash(&data.to_string()).as_bytes().to_owned()
}