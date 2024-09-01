// src/utils/logging.rs

use log::{info, warn, error};

/// Logs an informational message.
///
/// # Arguments
///
/// * `message` - The message to log.
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// Logs a warning message.
///
/// # Arguments
///
/// * `message` - The message to log.
pub fn log_warn(message: &str) {
    warn!("{}", message);
}

/// Logs an error message.
///
/// # Arguments
///
/// * `message` - The message to log.
pub fn log_error(message: &str) {
    error!("{}", message);
}
