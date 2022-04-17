use chrono::prelude::*;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

use crate::account::Account;
use crate::block;
use crate::block::Block;
use crate::mempool::Mempool;
use crate::stake::Stake;
use crate::transaction::*;
use crate::util::Util;
use crate::validator::Validator;
use crate::wallet::Wallet;
use num_bigint::BigUint;
use sha256::digest;

const BLOCK_GENERATION_INTERVAL_SECONDS: usize = 30;
const DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS: usize = 1;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Mempool,
    pub wallet: Wallet,
    pub accounts: Account,
    pub stakes: Stake,
    pub validators: Validator,
}

impl Blockchain {
    pub fn new(wallet: Wallet) -> Self {
        let genesis = Blockchain::genesis(wallet.clone());
        Self {
            chain: vec![genesis],
            mempool: Mempool::new(),
            wallet: wallet,
            accounts: Account::new(),
            stakes: Stake::new(),
            validators: Validator::new(),
        }
    }

    pub fn create_txn(
        sender_wallet: &mut Wallet,
        to: String,
        amount: f64,
        txn_type: TransactionType,
    ) -> Transaction {
        Transaction::new(sender_wallet, to, amount, txn_type)
    }

    pub fn txn_exist(&mut self, txn: &Transaction) -> bool {
        self.mempool.transaction_exists(txn)
    }

    pub fn add_txn(&mut self, txn: Transaction) -> bool {
        self.mempool.add_transaction(txn)
    }

    pub fn get_difficulty(&mut self) -> u32 {
        let last_block = self.chain.last().unwrap();
        if last_block.id % DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS == 0 && last_block.id != 0 {
            let prev_difficulty_block =
                &self.chain[self.chain.len() - 1 - DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS];
            info!("last block time {}", last_block.timestamp);
            info!("prev block time {}", prev_difficulty_block.timestamp);
            let time_taken = last_block.timestamp - prev_difficulty_block.timestamp;
            info!("time_taken {}", time_taken);
            let time_expected =
                DIFFICULTY_ADJUSTMENT_INTERVAL_BLOCKS * BLOCK_GENERATION_INTERVAL_SECONDS;
            if time_taken < (time_expected / 2) as i64 {
                info!("Increase Difficulty");
                prev_difficulty_block.difficulty + 1
            } else if time_taken > (time_expected * 2) as i64 {
                info!("Decrease Difficulty");
                if prev_difficulty_block.difficulty <= 1 {
                    1
                } else {
                    prev_difficulty_block.difficulty - 1
                }
            } else {
                info!("Maintain Difficulty");
                prev_difficulty_block.difficulty
            }
        } else {
            info!("Reuse Difficulty");
            last_block.difficulty
        }
    }

    pub fn genesis(wallet: Wallet) -> Block {
        info!("Creating genesis block...");
        Block::new(0, String::from("genesis"), 1650205976, vec![], 5, wallet)
    }

    pub fn mine_block_by_stake(&mut self) -> Option<Block> {
        if self.mempool.transactions.is_empty() {
            // info!("Skip mining because no transaction in mempool");
            return None;
        }

        let balance = self
            .stakes
            .get_balance(&self.wallet.get_public_key())
            .clone();

        let difficulty = self.get_difficulty();
        info!("Mining new block with difficulty {}", difficulty);

        let timestamp = Utc::now().timestamp();
        let previous_hash = self.chain.last().unwrap().previous_hash.clone();
        let address = self.wallet.get_public_key();

        if Blockchain::is_staking_valid(balance, difficulty, timestamp, &previous_hash, &address) {
            Some(self.create_block(timestamp))
        } else {
            None
        }
    }

    pub fn is_staking_valid(
        balance: u64,
        difficulty: u32,
        timestamp: i64,
        previous_hash: &String,
        address: &String,
    ) -> bool {
        let base = BigUint::new(vec![2]);
        let big_balance_diff_mul = base.pow(256) * balance as u32;
        let big_balance_diff = big_balance_diff_mul / difficulty as u64;
        // println!("big_balance_diff {:?}", big_balance_diff);

        let data_str = format!("{}{}{}", previous_hash, address, timestamp.to_string());
        println!(
            "previous_hash: {} address: {} timestamp: {}",
            previous_hash,
            address,
            timestamp.to_string()
        );

        let sha256_hash = digest(data_str);

        println!("sha256_hash {}", sha256_hash);

        let decimal_staking_hash = BigUint::parse_bytes(&sha256_hash.as_bytes(), 16).expect("msg");
        println!("decimalStakingHash {:?}", decimal_staking_hash);

        decimal_staking_hash <= big_balance_diff
    }

    pub fn create_block(&mut self, timestamp: i64) -> Block {
        info!("Creating new block...");

        Block::new(
            self.chain.len(),
            self.chain.last().unwrap().hash.clone(),
            timestamp,
            self.mempool.validate_transactions(),
            self.get_difficulty(),
            self.wallet.clone(),
        )
    }

    pub fn is_valid_block(&mut self, block: Block) -> bool {
        let prev_block = self.chain.last().unwrap();

        if block.previous_hash != prev_block.hash {
            warn!(
                "block with id: {} has wrong previous hash {} vs {} ",
                block.id, block.previous_hash, prev_block.hash
            );

            return false;
        } else if block.hash
            != block::calculate_hash(
                &block.id,
                &block.timestamp,
                &block.previous_hash,
                &block.txn,
            )
        {
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        } else if prev_block.id + 1 != block.id {
            warn!(
                "block with id: {} is not the next block after the latest: {}",
                block.id, prev_block.id
            );
            return false;
        } else if !Block::verify_block_signature(&block) {
            warn!(
                "block with id: {} has invalid validator signature",
                block.id
            );
            return false;
        } else if !Blockchain::is_staking_valid(
            self.stakes.get_balance(&block.validator).clone(),
            block.difficulty,
            block.timestamp,
            &block.previous_hash,
            &block.validator,
        ) {
            warn!("block with id: {} has invalid validator", block.id);
            return false;
        }

        self.add_new_block(block);
        true
    }

    pub fn add_new_block(&mut self, block: Block) {
        self.execute_txn(&block);
        info!("Add new block to current chain");
        self.chain.push(block);
        self.mempool.clear();
    }

    pub fn verify_leader(&mut self, block: &Block) -> bool {
        self.stakes.get_max(&self.validators.accounts) == block.validator
    }

    pub fn get_leader(&mut self) -> String {
        self.stakes.get_max(&self.validators.accounts)
    }

    pub fn replace_chain(&mut self, chain: &Vec<Block>) {
        if chain.len() <= self.chain.len() {
            warn!("Received chain is not longer than the current chain");
            return;
        } else if !self.is_valid_chain(chain) {
            warn!("Received chain is invalid");
            return;
        }

        info!("Replacing current chain with new chain");

        self.reset_state();
        self.execute_chain(chain);
        self.chain = chain.clone();
    }

    pub fn is_valid_chain(&mut self, chain: &Vec<Block>) -> bool {
        if *chain.first().unwrap() != Blockchain::genesis(self.wallet.clone()) {
            return false;
        }

        for i in 0..chain.len() {
            if i == 0 {
                continue;
            };

            let block = &chain[i];
            let prev_block = &chain[i - 1];

            if prev_block.hash != block.previous_hash {
                warn!("block with id: {} has wrong previous hash", block.id);
                return false;
            } else if prev_block.id + 1 != block.id {
                warn!(
                    "block with id: {} is not the next block after the latest: {}",
                    block.id, prev_block.id
                );
                return false;
            }
        }
        true
    }

    pub fn reset_state(&mut self) {
        let genesis = Blockchain::genesis(self.wallet.clone());
        self.chain = vec![genesis];
        self.accounts = Account::new();
        self.stakes = Stake::new();
        self.validators = Validator::new();
    }

    pub fn execute_chain(&mut self, chain: &Vec<Block>) {
        chain.iter().for_each(|block| self.execute_txn(block));
    }

    pub fn execute_txn(&mut self, block: &Block) {
        block.txn.iter().for_each(|txn| match txn.txn_type {
            TransactionType::TRANSACTION => {
                // Transfer amount
                self.accounts.transfer(
                    &txn.txn_input.from,
                    &txn.txn_output.to,
                    &txn.txn_output.amount,
                );
                // Transfer fee
                self.accounts
                    .transfer(&txn.txn_input.from, &block.validator, &txn.txn_output.fee);
            }
            TransactionType::STAKE => {
                self.stakes.update(&txn);
                self.accounts
                    .decrement(&txn.txn_input.from, &txn.txn_output.amount);
                // Transfer fee
                self.accounts
                    .transfer(&txn.txn_input.from, &block.validator, &txn.txn_output.fee);
            }
            TransactionType::VALIDATOR => {
                if self.validators.update(&txn) {
                    self.accounts
                        .decrement(&txn.txn_input.from, &txn.txn_output.amount);
                    // Transfer fee
                    self.accounts.transfer(
                        &txn.txn_input.from,
                        &block.validator,
                        &txn.txn_output.fee,
                    );
                }
            }
        });
    }

    pub fn get_balance(&mut self, public_key: &String) -> &f64 {
        self.accounts.get_balance(public_key)
    }

    // -------- Old

    // pub fn try_add_block(&mut self, block: Block) {
    //     let latest_block = self.chain.last().expect("there is at least one block");
    //     if self.is_block_valid(&block, latest_block) {
    //         self.chain.push(block);
    //     } else {
    //         error!("could not add block - invalid");
    //     }
    // }

    // pub fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
    //     if block.previous_hash != previous_block.hash {
    //         warn!("block with id: {} has wrong previous hash", block.id);
    //         return false;
    //     } else if !Util::hash_to_binary_representation(
    //         &hex::decode(&block.hash).expect("can decode from hex"),
    //     )
    //     .starts_with(block::DIFFICULTY_PREFIX)
    //     {
    //         warn!("block with id: {} has invalid difficulty", block.id);
    //         return false;
    //     } else if block.id != previous_block.id + 1 {
    //         warn!(
    //             "block with id: {} is not the next block after the latest: {}",
    //             block.id, previous_block.id
    //         );
    //         return false;
    //     } else if hex::encode(block::calculate_hash(
    //         &block.id,
    //         &block.timestamp,
    //         &block.previous_hash,
    //         &block.txn,
    //     )) != block.hash
    //     {
    //         warn!("block with id: {} has invalid hash", block.id);
    //         return false;
    //     }
    //     true
    // }

    // pub fn is_chain_valid(&self, chain: &[Block]) -> bool {
    //     for i in 0..chain.len() {
    //         if i == 0 {
    //             continue;
    //         }
    //         let first = chain.get(i - 1).expect("has to exist");
    //         let second = chain.get(i).expect("has to exist");
    //         if !self.is_block_valid(second, first) {
    //             return false;
    //         }
    //     }
    //     true
    // }

    // We always choose the longest valid chain
    // pub fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
    //     let is_local_valid = self.is_chain_valid(&local);
    //     let is_remote_valid = self.is_chain_valid(&remote);

    //     if is_local_valid && is_remote_valid {
    //         if local.len() >= remote.len() {
    //             local
    //         } else {
    //             remote
    //         }
    //     } else if is_remote_valid && !is_local_valid {
    //         remote
    //     } else if !is_remote_valid && is_local_valid {
    //         local
    //     } else {
    //         panic!("local and remote chains are both invalid");
    //     }
    // }
}
