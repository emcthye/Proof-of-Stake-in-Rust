use ed25519_dalek::{PublicKey, Signature, Verifier};
use sha256::digest_bytes;
use uuid::Uuid;

pub struct Util;

impl Util {
    pub fn id() -> Uuid {
        Uuid::new_v4()
    }

    pub fn verifySignature(
        fromPublicKey: &String,
        message: &String,
        fromSignature: &String,
    ) -> bool {
        let publicKey = PublicKey::from_bytes(
            &hex::decode(fromPublicKey).expect("PublicKey Hex to Byte conversion"),
        )
        .unwrap();
        let signature = &Signature::from_bytes(
            &hex::decode(fromSignature).expect("Signature Hex to Byte conversion"),
        )
        .unwrap();
        publicKey.verify(message.as_bytes(), signature).is_ok()
    }

    pub fn hash(data: &String) -> String {
        digest_bytes(data.as_bytes())
    }
}
