// src/consensus/resonance.rs

use crate::models::{Block, ConsensusRecord};

/// Calculates the resonance score between the current block and a past block.
/// The resonance score is a measure of similarity or compatibility between blocks.
pub fn calculate_resonance_score(current_block: &Block, past_block: &Block) -> f64 {
    // Basic resonance score calculation based on block difficulty and timestamp difference
    let difficulty_factor = current_block.difficulty as f64 * past_block.difficulty as f64;
    let timestamp_difference = (current_block.timestamp as i64 - past_block.timestamp as i64).abs() as f64;
    
    // A simple formula that combines difficulty and timestamp difference
    // Adjust or expand this logic to include more factors as needed
    difficulty_factor / (timestamp_difference + 1.0)
}

/// Finds the past block with the highest resonance score relative to the current block.
/// This is used to identify the most compatible block from the past.
pub fn find_best_resonance(past_blocks: Vec<Block>, current_block: Block) -> ConsensusRecord {
    let mut best_score = f64::MIN;
    let mut best_block_id = String::new();

    for block in past_blocks {
        let score = calculate_resonance_score(&current_block, &block);
        if score > best_score {
            best_score = score;
            best_block_id = block.id.clone();
        }
    }

    // Return a consensus record with the best-matching block
    ConsensusRecord {
        block_id: best_block_id,
        resonance_score: best_score,
        stability_score: 0.0,  // Placeholder value for stability, to be calculated elsewhere
        validator: current_block.validator.clone(),
        timestamp: current_block.timestamp,
    }
}
