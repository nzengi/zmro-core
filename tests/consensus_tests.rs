// src/tests/consensus_tests.rs

use crate::models::Block;
use crate::consensus::validator::validate_block;

#[test]
fn test_validate_block() {
    let block = Block {
        id: "test_block".to_string(),
        timestamp: 1625232000,
        previous_hash: "previous_hash".to_string(),
        transactions_root: "tx_root".to_string(),
        state_root: "state_root".to_string(),
        difficulty: 1,
        nonce: 100,
        validator: "validator".to_string(),
        signature: "signature".to_string(),
        gas_limit: 1000000,
        gas_used: 500000,
        transaction_count: 10,
    };

    assert!(validate_block(&block));
}
