use rand::rngs::OsRng;
use ed25519_dalek::{Keypair, Signature, PublicKey, Signer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub balance: f64,
    pub keyPair: Keypair,
    pub publicKey: PublicKey
}