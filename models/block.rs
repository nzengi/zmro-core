// src/models/block.rs

use serde::{Serialize, Deserialize};

/// Represents a block in the blockchain.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub id: String,
    pub timestamp: u64,
    pub previous_hash: String,
    pub transactions_root: String,
    pub state_root: String,
    pub difficulty: u32,
    pub nonce: u64,
    pub validator: String,
    pub signature: String,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub transaction_count: u32,
}

impl Block {
    /// Creates a new Block instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the block.
    /// * `timestamp` - The timestamp when the block was created.
    /// * `previous_hash` - The hash of the previous block in the chain.
    /// * `transactions_root` - The root hash of the transactions in the block.
    /// * `state_root` - The root hash of the state tree after applying the block.
    /// * `difficulty` - The difficulty target of the block.
    /// * `nonce` - The nonce used in the mining process.
    /// * `validator` - The identifier of the block validator.
    /// * `signature` - The digital signature of the block.
    /// * `gas_limit` - The maximum amount of gas that can be used by transactions in the block.
    /// * `gas_used` - The actual amount of gas used by transactions in the block.
    /// * `transaction_count` - The number of transactions included in the block.
    ///
    /// # Returns
    ///
    /// * `Block` - A new Block instance.
    pub fn new(
        id: String,
        timestamp: u64,
        previous_hash: String,
        transactions_root: String,
        state_root: String,
        difficulty: u32,
        nonce: u64,
        validator: String,
        signature: String,
        gas_limit: u64,
        gas_used: u64,
        transaction_count: u32,
    ) -> Self {
        Block {
            id,
            timestamp,
            previous_hash,
            transactions_root,
            state_root,
            difficulty,
            nonce,
            validator,
            signature,
            gas_limit,
            gas_used,
            transaction_count,
        }
    }

    /// Validates the block structure.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns true if the block is valid, otherwise false.
    pub fn validate(&self) -> bool {
        // Basic validation logic
        !self.id.is_empty()
            && !self.previous_hash.is_empty()
            && !self.transactions_root.is_empty()
            && !self.state_root.is_empty()
            && self.difficulty > 0
            && self.nonce > 0
            && !self.validator.is_empty()
            && !self.signature.is_empty()
            && self.gas_used <= self.gas_limit
            && self.transaction_count > 0
    }

    /// Generates a hash for the block.
    ///
    /// # Returns
    ///
    /// * `String` - The hash of the block.
    pub fn hash(&self) -> String {
        // This is a placeholder for a real hash computation.
        // In a real implementation, you would use a cryptographic hash function like SHA-256.
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.id,
            self.timestamp,
            self.previous_hash,
            self.transactions_root,
            self.state_root,
            self.difficulty,
            self.nonce,
            self.validator,
            self.signature,
            self.gas_limit,
            self.gas_used
        )
    }
}
