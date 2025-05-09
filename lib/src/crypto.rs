use ecdsa::signature::rand_core::OsRng;
use k256::ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signature(ECDSASignature);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PublicKey(VerifyingKey);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrivateKey(
    #[serde(with = "signkey_serde")]
    pub SigningKey
);

impl PrivateKey {
    pub fn new() -> Self {
        PrivateKey(SigningKey::random(&mut OsRng))
    }
}

mod signkey_serde {
    use serde::Deserialize;
    pub fn serialize<S>(
        key: &super::SigningKey,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<super::SigningKey, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::<u8>::deserialize(deserializer)?;

        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}
