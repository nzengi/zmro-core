// src/consensus/validator.rs

use crate::models::Block;

/// Validates a block according to the consensus rules.
/// Checks if the block meets the required criteria, such as having a valid transaction count and gas usage.
pub fn validate_block(block: &Block) -> bool {
    // Validation logic: check if the transaction count is positive and gas usage is within limits
    block.transaction_count > 0 && block.gas_used <= block.gas_limit && !block.signature.is_empty()
}

/// Validates a series of blocks to ensure they all conform to consensus rules.
/// This is useful for validating a chain of blocks during sync or import operations.
pub fn validate_blocks(blocks: &[Block]) -> bool {
    blocks.iter().all(|block| validate_block(block))
}
