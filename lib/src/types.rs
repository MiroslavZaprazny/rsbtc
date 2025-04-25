use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::U256;

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

    pub fn hash() -> ! {
        todo!()
    }
}

pub struct BlockHeader {
    timestamp: DateTime<Utc>,
    nonce: u64,
    prev_block_hash: [u8; 32],
    merkle_root: [u8; 32],
    target: U256,
}

impl BlockHeader {
    pub fn new( 
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
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

    pub fn hash() -> ! {
        todo!()
    }
}

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

    pub fn hash() -> ! {
        todo!()
    }
}

pub struct TransactionInput {
    prev_transaction_output_hash: [u8; 32],
    signature: [u8; 64],
}
pub struct TransactionOutput {
    value: u64,
    unique_id: Uuid,
    pubkey: [u8; 33],
}
