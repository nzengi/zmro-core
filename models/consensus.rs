// src/models/consensus.rs

use serde::{Serialize, Deserialize};

/// Represents the result of a consensus process for a block.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConsensusRecord {
    pub block_id: String,
    pub resonance_score: f64,
    pub stability_score: f64,
    pub validator: String,
    pub timestamp: u64,
}

impl ConsensusRecord {
    /// Creates a new ConsensusRecord instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The ID of the block that achieved consensus.
    /// * `resonance_score` - The resonance score of the block.
    /// * `stability_score` - The stability score of the block.
    /// * `validator` - The identifier of the validator.
    /// * `timestamp` - The timestamp when the consensus was reached.
    ///
    /// # Returns
    ///
    /// * `ConsensusRecord` - A new ConsensusRecord instance.
    pub fn new(
        block_id: String,
        resonance_score: f64,
        stability_score: f64,
        validator: String,
        timestamp: u64,
    ) -> Self {
        ConsensusRecord {
            block_id,
            resonance_score,
            stability_score,
            validator,
            timestamp,
        }
    }

    /// Validates the consensus record.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns true if the consensus record is valid, otherwise false.
    pub fn validate(&self) -> bool {
        // Basic validation logic
        !self.block_id.is_empty()
            && self.resonance_score >= 0.0
            && self.stability_score >= 0.0
            && !self.validator.is_empty()
            && self.timestamp > 0
    }
}
