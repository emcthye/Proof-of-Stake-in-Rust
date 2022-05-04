use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::blockchain::Blockchain;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub key_pair: String,
}

impl Wallet {
    pub fn new() -> Wallet {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let pub_key = hex::encode(keypair.public.to_bytes());
        println!("Your Public Key {}", pub_key);
        let keypair = hex::encode(keypair.to_bytes());
        println!("Your Key Pair {}", keypair);
        Self { key_pair: keypair }
    }

    pub fn generate_wallet() {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let pub_key = hex::encode(keypair.public.to_bytes());
        println!("Your Public Key {}", pub_key);
        println!("Your Key Pair {:?}", hex::encode(keypair.to_bytes()));
    }

    fn get_keypair(keypair_str: &String) -> Keypair {
        Keypair::from_bytes(&hex::decode(keypair_str).expect("Hex to Byte conversion"))
            .expect("Byte to Keypair conversion")
    }

    pub fn get_wallet(keypair: String) -> Wallet {
        Self { key_pair: keypair }
    }

    pub fn sign(&mut self, data_hash: &String) -> String {
        hex::encode(Wallet::get_keypair(&self.key_pair).sign(data_hash.as_bytes()))
    }

    pub fn get_public_key(&mut self) -> String {
        hex::encode(Wallet::get_keypair(&self.key_pair).public.as_bytes())
    }

    pub fn get_balance<'a>(&mut self, blockchain: &'a mut Blockchain) -> &'a f64 {
        blockchain.get_balance(&self.get_public_key())
    }
}
