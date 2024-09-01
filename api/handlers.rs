// src/api/handlers.rs

use actix_web::{web, HttpResponse, Result};
use crate::models::{Block, ConsensusRecord};
use crate::services::{BlockService, ConsensusService};

/// Handles the creation of a new block.
/// Validates and processes the block data, returning the created block or an error response.
pub async fn handle_create_block(
    block: web::Json<Block>,
    block_service: web::Data<BlockService>,
) -> Result<HttpResponse> {
    let block_data = block.into_inner();

    // Validate the block
    if block_service.validate_block(&block_data) {
        // If valid, create the block
        let created_block = block_service.create_block(block_data);
        Ok(HttpResponse::Ok().json(created_block))
    } else {
        // If invalid, return an error response
        Ok(HttpResponse::BadRequest().body("Invalid block data"))
    }
}

/// Retrieves the current consensus status.
/// Fetches and returns the consensus status as a JSON response.
pub async fn handle_get_consensus_status(
    consensus_service: web::Data<ConsensusService>,
) -> Result<HttpResponse> {
    // Fetch the current consensus status
    let status = consensus_service.get_current_status();
    Ok(HttpResponse::Ok().json(status))
}
