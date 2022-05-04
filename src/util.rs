use ed25519_dalek::{PublicKey, Signature, Verifier};
use sha256::digest_bytes;
use uuid::Uuid;

pub struct Util;

impl Util {
    pub fn id() -> Uuid {
        Uuid::new_v4()
    }

    pub fn verify_signature(
        from_public_key: &String,
        message: &String,
        from_signature: &String,
    ) -> bool {
        let public_key = PublicKey::from_bytes(
            &hex::decode(from_public_key).expect("PublicKey Hex to Byte conversion"),
        )
        .unwrap();
        let signature = &Signature::from_bytes(
            &hex::decode(from_signature).expect("Signature Hex to Byte conversion"),
        )
        .unwrap();
        public_key.verify(message.as_bytes(), signature).is_ok()
    }

    pub fn hash(data: &String) -> String {
        digest_bytes(data.as_bytes())
    }
}
