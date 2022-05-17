use ed25519_dalek::{PublicKey, Signature, Verifier};
use hex::FromHexError;
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
    ) -> Result<bool, FromHexError> {
        let public_key = match hex::decode(from_public_key) {
            Ok(public_key) => public_key,
            Err(e) => return Err(e),
        };
        let dalek_public_key = PublicKey::from_bytes(&public_key);

        let signature = match hex::decode(from_signature) {
            Ok(signature) => signature,
            Err(e) => return Err(e),
        };

        let dalek_signature = &Signature::from_bytes(&signature);

        Ok(dalek_public_key
            .verify(message.as_bytes(), dalek_signature)
            .is_ok())
    }

    pub fn hash(data: &String) -> String {
        digest_bytes(data.as_bytes())
    }
}
