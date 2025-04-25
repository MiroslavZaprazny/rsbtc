use k256::ecdsa::{Signature as ECDSASignature, VerifyingKey, SigningKey};

pub struct Signature(ECDSASignature);
pub struct PublicKey(VerifyingKey);
pub struct PrivateKey(SigningKey);
