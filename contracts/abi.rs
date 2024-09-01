// src/contracts/abi.rs

use serde_json::Value;
use std::collections::HashMap;

/// ABI struct provides functionality for interacting with smart contract ABIs.
pub struct ABI {
    abi_json: Value,
    function_signatures: HashMap<String, String>,
}

impl ABI {
    /// Creates a new ABI instance from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `abi_json_str` - A string slice containing the ABI JSON.
    ///
    /// # Returns
    ///
    /// * `ABI` - A new instance of the ABI struct.
    pub fn new(abi_json_str: &str) -> Self {
        let abi_json: Value = serde_json::from_str(abi_json_str).expect("Invalid ABI JSON format");
        let function_signatures = Self::extract_function_signatures(&abi_json);

        ABI { abi_json, function_signatures }
    }

    /// Extracts function signatures from the ABI JSON.
    ///
    /// # Arguments
    ///
    /// * `abi_json` - A reference to the ABI JSON value.
    ///
    /// # Returns
    ///
    /// * `HashMap<String, String>` - A map of function names to their signatures.
    fn extract_function_signatures(abi_json: &Value) -> HashMap<String, String> {
        let mut signatures = HashMap::new();
        if let Some(abi_array) = abi_json.as_array() {
            for item in abi_array {
                if let Some(function_name) = item.get("name") {
                    if let Some(signature) = item.get("signature") {
                        signatures.insert(
                            function_name.as_str().unwrap().to_string(),
                            signature.as_str().unwrap().to_string(),
                        );
                    }
                }
            }
        }
        signatures
    }

    /// Retrieves the function signature for a given function name.
    ///
    /// # Arguments
    ///
    /// * `function_name` - The name of the function whose signature is required.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The function signature if found, otherwise `None`.
    pub fn get_function_signature(&self, function_name: &str) -> Option<String> {
        self.function_signatures.get(function_name).cloned()
    }

    /// Returns the full ABI JSON.
    ///
    /// # Returns
    ///
    /// * `Value` - The JSON representation of the ABI.
    pub fn get_abi_json(&self) -> &Value {
        &self.abi_json
    }
}
