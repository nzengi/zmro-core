// src/cli/commands.rs

use clap::{Parser, Subcommand};

/// ZMRO-Core: A command-line tool for managing and interacting with the blockchain.
#[derive(Parser)]
#[command(name = "zmro-core")]
#[command(about = "A command-line tool for the ZMRO-Core blockchain project", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initializes the blockchain or specific components
    Init {
        #[arg(short, long, default_value_t = String::from("default-config.toml"))]
        config: String,
    },
    /// Starts the blockchain node
    Start {
        #[arg(short, long, default_value_t = String::from("default-config.toml"))]
        config: String,
    },
    /// Validates the blockchain state
    Validate,
    /// Interacts with smart contracts
    Contract {
        #[arg(subcommand)]
        action: ContractCommands,
    },
}

#[derive(Subcommand)]
pub enum ContractCommands {
    /// Deploys a smart contract to the blockchain
    Deploy {
        #[arg(short, long)]
        contract: String,
    },
    /// Calls a function on a deployed smart contract
    Call {
        #[arg(short, long)]
        function: String,
        #[arg(short, long)]
        params: Option<String>,
    },
}
