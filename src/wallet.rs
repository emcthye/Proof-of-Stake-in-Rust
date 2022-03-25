use ed25519_dalek::{Keypair, PublicKey, Signer};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::blockchain::Blockchain;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub keyPair: String,
}

impl Wallet {
    pub fn new() -> Wallet {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let keypair = hex::encode(keypair.to_bytes());
        println!("Key Pair {}", keypair);
        Self { keyPair: keypair }
    }

    pub fn generate_wallet() {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        println!("Key Pair {:?}", hex::encode(keypair.to_bytes()));
    }

    fn get_keypair(keypair_str: &String) -> Keypair {
        Keypair::from_bytes(&hex::decode(keypair_str).expect("Hex to Byte conversion"))
            .expect("Byte to Keypair conversion")
    }

    pub fn get_wallet(keypair: String) -> Wallet {
        Self { keyPair: keypair }
    }

    pub fn sign(&mut self, dataHash: &String) -> String {
        hex::encode(Wallet::get_keypair(&self.keyPair).sign(dataHash.as_bytes()))
    }

    pub fn get_public_key(&mut self) -> String {
        hex::encode(Wallet::get_keypair(&self.keyPair).public.as_bytes())
    }

    pub fn get_balance<'a>(&mut self, blockchain: &'a mut Blockchain) -> &'a f64 {
        blockchain.get_balance(&self.get_public_key())
    }
}
