use actix_web::{App, HttpServer, middleware::Logger};
use clap::Parser;
use cli::commands::{Cli, Commands};

mod api;
mod config;
mod models;
mod services;
mod consensus;
mod storage;
mod contracts;
mod utils;
mod cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    // Parse CLI arguments
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { config } => {
            println!("Initializing with config: {}", config);
            // Initialize the blockchain with the provided configuration
        }
        Commands::Start { config } => {
            println!("Starting the node with config: {}", config);
            // Start the blockchain node
        }
        Commands::Validate => {
            println!("Validating blockchain state...");
            // Validate the blockchain
        }
        Commands::Contract { action } => match action {
            ContractCommands::Deploy { contract } => {
                println!("Deploying contract: {}", contract);
                // Deploy the specified contract
            }
            ContractCommands::Call { function, params } => {
                println!("Calling function: {} with params: {:?}", function, params);
                // Call the specified function on a smart contract
            }
        },
    }

    Ok(())
}
