use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use log::{error, info, warn};
use crate::block;
use crate::block::Block;
use crate::util::Util;
use crate::wallet::Wallet;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub wallet: Wallet
}

impl Blockchain {
    pub fn new() -> Self {
        Self { 
            blocks: vec![],
            wallet: Wallet::new()
        }
    }

    pub fn genesis(&mut self) {
        info!("Creating genesis block...");
        let genesis_block = Block::new(
            0,
            String::from("genesis"),
            String::from("Genesis Block"),
            &mut self.wallet
            );
        self.blocks.push(genesis_block);
    }

    pub fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is at least one block");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
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
        } else if hex::encode(block::Block::calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data
        )) != block.hash
        {
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }
        true
    }

    pub fn verify_leader(block: &Block) -> bool {
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