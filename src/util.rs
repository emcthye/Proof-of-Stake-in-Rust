use uuid::Uuid;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use sha256::digest_bytes;

pub struct Util;

impl Util {
    pub fn id() -> Uuid {
        Uuid::new_v4()
    }
    
    pub fn verifySignature(publicKey: PublicKey, message: &String, signature: &Signature) -> bool {
        publicKey.verify(message.as_bytes(), signature).is_ok()
    }
    
    pub fn hash(data: &String) -> String {
        digest_bytes(data.as_bytes())
    }

    pub fn hash_to_binary_representation(hash: &[u8]) -> String {
        let mut res: String = String::default();
        for c in hash {
            res.push_str(&format!("{:b}", c));
        }
        res
    }
}
