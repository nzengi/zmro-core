// src/services/consensus_service.rs

use crate::consensus::resonance::find_best_resonance;
use crate::models::{Block, ConsensusRecord};

/// ConsensusService provides functionality to manage the consensus process.
pub struct ConsensusService;

impl ConsensusService {
    /// Executes the consensus process to find the best block based on resonance score.
    ///
    /// # Arguments
    ///
    /// * `current_block` - The current block for which consensus is being determined.
    /// * `past_blocks` - A vector of past blocks to compare against.
    ///
    /// # Returns
    ///
    /// * `ConsensusRecord` - The record of the best block determined by the consensus process.
    pub fn execute_consensus(&self, current_block: Block, past_blocks: Vec<Block>) -> ConsensusRecord {
        find_best_resonance(past_blocks, current_block)
    }

    /// Retrieves the current consensus status.
    ///
    /// # Returns
    ///
    /// * `String` - A placeholder response indicating the consensus status.
    pub fn get_current_status(&self) -> String {
        // This would typically involve querying the state of the consensus mechanism.
        // Here, we return a placeholder response.
        "Consensus is stable".to_string()
    }
}
