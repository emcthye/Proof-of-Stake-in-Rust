use chrono::prelude::*;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

use crate::account::Account;
use crate::block;
use crate::block::Block;
use crate::stake::Stake;
use crate::transaction::Transaction;
use crate::util::Util;
use crate::validator::Validator;
use crate::wallet::Wallet;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub wallet: Wallet,
    pub accounts: Account,
    pub stakes: Stake,
    pub validators: Validator,
}

impl Blockchain {
    pub fn new(wallet: Wallet) -> Self {
        Self {
            chain: vec![],
            wallet: wallet,
            accounts: Account::new(),
            stakes: Stake::new(),
            validators: Validator::new(),
        }
    }

    pub fn genesis(&mut self) {
        info!("Creating genesis block...");
        let genesis_block = Block::new(
            0,
            String::from("genesis"),
            String::from("Genesis Block"),
            &mut self.wallet,
        );
        self.chain.push(genesis_block);
    }

    pub fn create_block(&mut self, txns: &Vec<Transaction>) -> Block {
        let block = Block::new(
            self.chain.len(),
            self.chain.last().unwrap().hash,
            serde_json::to_string(txns).unwrap(),
            &mut self.wallet,
        );

        self.chain.push(block);
        block
    }

    pub fn is_valid_block(&mut self, block: &Block) {
        if block.previous_hash == self.chain.last().unwrap().hash
            && block.hash
                == block::calculate_hash(
                    &block.id,
                    &block.timestamp,
                    &block.previous_hash,
                    &block.data,
                )
            && Block::verify_block_signature(block)
            && self.verify_leader(block)
        {
            self.chain.push(block);
        }
    }

    pub fn execute_txn(block: &Block) {
        let txns = serde_json::from_str(&block.data).unwrap();
    }

    pub fn try_add_block(&mut self, block: Block) {
        let latest_block = self.chain.last().expect("there is at least one block");
        if self.is_block_valid(&block, latest_block) {
            self.chain.push(block);
        } else {
            error!("could not add block - invalid");
        }
    }

    pub fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !Util::hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"),
        )
        .starts_with(block::DIFFICULTY_PREFIX)
        {
            warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, previous_block.id
            );
            return false;
        } else if hex::encode(block::calculate_hash(
            &block.id,
            &block.timestamp,
            &block.previous_hash,
            &block.data,
        )) != block.hash
        {
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }

    pub fn verify_leader(&mut self, block: &Block) -> bool {
        self.stakes.get_max(&self.validators.accounts) == block.validator
    }

    pub fn is_chain_valid(&self, chain: &[Block]) -> bool {
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !self.is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    // We always choose the longest valid chain
    pub fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("local and remote chains are both invalid");
        }
    }
}
