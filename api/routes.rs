// src/api/routes.rs

use actix_web::web;
use crate::api::handlers::{handle_create_block, handle_get_consensus_status};

/// Configures the API routes.
/// This function sets up the HTTP routes and associates them with their corresponding handlers.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Route for creating a new block
    cfg.service(
        web::resource("/block")
            .route(web::post().to(handle_create_block))
    );

    // Route for retrieving the consensus status
    cfg.service(
        web::resource("/consensus")
            .route(web::get().to(handle_get_consensus_status))
    );
}
