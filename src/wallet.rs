use ed25519_dalek::{Keypair, Signer, PublicKey};
use serde::{Deserialize, Serialize};
use rand::rngs::OsRng;

pub struct Wallet {
    pub balance: f64,
    pub keyPair: Keypair
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        Self {
            balance: 100.0,
            keyPair: Keypair::generate(&mut csprng)
        }
    }

    pub fn sign(&mut self, dataHash: &String) -> String {
        hex::encode(self.keyPair.sign(dataHash.as_bytes()))
    }

    pub fn getPublicKey(&mut self) -> String {
        hex::encode(self.keyPair.public.as_bytes())
    }
}
