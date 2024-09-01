// src/contracts/mro_contract.rs

use web3::contract::{Contract, Options};
use web3::types::{Address, U256};
use crate::contracts::abi::ABI;
use web3::transports::Http;
use web3::Web3;

/// MROContract struct provides functionality for interacting with the MRO smart contract.
pub struct MROContract {
    contract: Contract<Http>,
}

impl MROContract {
    /// Creates a new MROContract instance.
    ///
    /// # Arguments
    ///
    /// * `web3_url` - The URL of the Web3 provider (e.g., Infura or local node).
    /// * `abi` - The ABI of the MRO smart contract.
    /// * `contract_address` - The address of the deployed smart contract.
    ///
    /// # Returns
    ///
    /// * `MROContract` - A new instance of the MROContract struct.
    pub fn new(web3_url: &str, abi: ABI, contract_address: &str) -> Self {
        let transport = Http::new(web3_url).expect("Failed to connect to Web3 provider");
        let web3 = Web3::new(transport);
        let contract = Contract::from_json(
            web3.eth(),
            Address::from_slice(&hex::decode(contract_address).expect("Invalid contract address")),
            abi.get_abi_json().to_string().as_bytes(),
        ).expect("Failed to create contract instance");

        MROContract { contract }
    }

    /// Calls a function on the MRO smart contract.
    ///
    /// # Arguments
    ///
    /// * `function_name` - The name of the function to call.
    /// * `params` - The parameters to pass to the function.
    ///
    /// # Returns
    ///
    /// * `Result<U256, web3::Error>` - The result of the function call, wrapped in a Result.
    pub fn call_function(&self, function_name: &str, params: Vec<U256>) -> Result<U256, web3::Error> {
        let function_signature = self.contract.abi().function(function_name)?;
        let result: U256 = self.contract.query(
            function_signature.name.clone(),
            params,
            None,
            Options::default(),
            None,
        ).await?;
        Ok(result)
    }
}
