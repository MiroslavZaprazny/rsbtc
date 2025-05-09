use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{crypto::{PublicKey, Signature}, sha256::Hash, util::MerkleRoot, U256};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new () -> Self {
        Self {blocks: vec![]}
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new (header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            header,
            transactions
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    timestamp: DateTime<Utc>,
    nonce: u64,
    prev_block_hash: Hash,
    merkle_root: MerkleRoot,
    target: U256,
}

impl BlockHeader {
    pub fn new( 
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        Self {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new( 
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    ) -> Self {
        Self {
            inputs,
            outputs,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    prev_transaction_output_hash: Hash,
    signature: Signature
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    value: u64,
    unique_id: Uuid,
    pubkey: PublicKey
}

impl TransactionOutput {
    pub fn new (&self) -> Hash {
        Hash::hash(self)
    }
}
