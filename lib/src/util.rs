use serde::{Deserialize, Serialize};
use crate::{sha256::Hash, types::Transaction};

#[derive(Serialize,Deserialize,Clone,Copy,Debug,PartialEq,Eq)]
pub struct MerkleRoot(Hash);

impl MerkleRoot {
    pub fn calculate(transactions: &[Transaction]) -> MerkleRoot {
        let mut layer: Vec<Hash> = vec![];

        for transaction in transactions {
            layer.push(Hash::hash(transaction));
        }

        while layer.len() > 1 {
            let mut tmp: Vec<Hash>= vec![];

            for pair in layer.chunks(2) {
                let left = pair[0];
                let right = pair.get(1).unwrap_or(&left);
                tmp.push(Hash::hash(&[left, *right]));
            }

            layer = tmp;
        }

        MerkleRoot(layer[0])
    }
}
