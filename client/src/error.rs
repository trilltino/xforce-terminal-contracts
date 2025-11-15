//! # Error Definitions
//!
//! This module contains error types used throughout the client library.
//! All errors are defined using the `thiserror` crate for easy error handling
//! and conversion.

use thiserror::Error;

/// Error types for the XForce Terminal Contracts Client
///
/// This enum represents all possible errors that can occur when using the
/// client library. Each variant provides detailed error information for
/// debugging and user feedback.
///
/// # Error Variants
///
/// - `ClientError` - Errors related to client creation or configuration
/// - `ProgramError` - Errors from the Solana program execution
/// - `InvalidAccount` - Invalid account provided
/// - `TransactionFailed` - Transaction execution failed
/// - `NetworkError` - Network-related errors
/// - `SerializationError` - Serialization/deserialization errors
///
/// # Example
///
/// ```rust,no_run
/// use xforce_terminal_contracts_client::ContractError;
///
/// match result {
///     Ok(value) => println!("Success: {:?}", value),
///     Err(ContractError::TransactionFailed(msg)) => {
///         eprintln!("Transaction failed: {}", msg);
///     }
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
#[derive(Error, Debug)]
pub enum ContractError {
    /// Client error
    ///
    /// This error occurs when there is a problem creating or configuring
    /// the Anchor client.
    ///
    /// # Examples
    ///
    /// - Invalid cluster URL
    /// - Invalid keypair
    /// - Client creation failure
    #[error("Client error: {0}")]
    ClientError(String),

    /// Program error
    ///
    /// This error occurs when the Solana program returns an error.
    ///
    /// # Examples
    ///
    /// - Invalid instruction data
    /// - Account validation failure
    /// - Program execution error
    #[error("Program error: {0}")]
    ProgramError(String),

    /// Invalid account error
    ///
    /// This error occurs when an invalid account is provided.
    ///
    /// # Examples
    ///
    /// - Account not found
    /// - Invalid account type
    /// - Account ownership mismatch
    #[error("Invalid account: {0}")]
    InvalidAccount(String),

    /// Transaction failed error
    ///
    /// This error occurs when a transaction fails to execute.
    ///
    /// # Examples
    ///
    /// - Transaction simulation failure
    /// - Transaction confirmation timeout
    /// - Insufficient funds
    /// - Transaction rejection
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    /// Network error
    ///
    /// This error occurs when there is a network-related problem.
    ///
    /// # Examples
    ///
    /// - Connection timeout
    /// - Network unreachable
    /// - RPC server error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Serialization error
    ///
    /// This error occurs when there is a problem serializing or
    /// deserializing data.
    ///
    /// # Examples
    ///
    /// - Invalid data format
    /// - Serialization failure
    /// - Deserialization failure
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl ContractError {
    /// Check if the error is a client error
    ///
    /// # Returns
    ///
    /// `true` if the error is a `ClientError`, `false` otherwise
    #[must_use]
    pub fn is_client_error(&self) -> bool {
        matches!(self, Self::ClientError(_))
    }

    /// Check if the error is a program error
    ///
    /// # Returns
    ///
    /// `true` if the error is a `ProgramError`, `false` otherwise
    #[must_use]
    pub fn is_program_error(&self) -> bool {
        matches!(self, Self::ProgramError(_))
    }

    /// Check if the error is a transaction error
    ///
    /// # Returns
    ///
    /// `true` if the error is a `TransactionFailed`, `false` otherwise
    #[must_use]
    pub fn is_transaction_error(&self) -> bool {
        matches!(self, Self::TransactionFailed(_))
    }

    /// Get a user-friendly error message
    ///
    /// # Returns
    ///
    /// A user-friendly error message string
    #[must_use]
    pub fn user_message(&self) -> String {
        match self {
            Self::ClientError(msg) => format!("Client configuration error: {}", msg),
            Self::ProgramError(msg) => format!("Program execution error: {}", msg),
            Self::InvalidAccount(msg) => format!("Invalid account: {}", msg),
            Self::TransactionFailed(msg) => format!("Transaction failed: {}", msg),
            Self::NetworkError(msg) => format!("Network error: {}", msg),
            Self::SerializationError(msg) => format!("Serialization error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_error() {
        let error = ContractError::ClientError("test".to_string());
        assert!(error.is_client_error());
        assert!(!error.is_program_error());
        assert!(!error.is_transaction_error());
    }

    #[test]
    fn test_program_error() {
        let error = ContractError::ProgramError("test".to_string());
        assert!(!error.is_client_error());
        assert!(error.is_program_error());
        assert!(!error.is_transaction_error());
    }

    #[test]
    fn test_transaction_error() {
        let error = ContractError::TransactionFailed("test".to_string());
        assert!(!error.is_client_error());
        assert!(!error.is_program_error());
        assert!(error.is_transaction_error());
    }

    #[test]
    fn test_user_message() {
        let error = ContractError::ClientError("test".to_string());
        let msg = error.user_message();
        assert!(msg.contains("Client configuration error"));
        assert!(msg.contains("test"));
    }

    #[test]
    fn test_error_display() {
        let error = ContractError::ClientError("test".to_string());
        let display = format!("{}", error);
        assert_eq!(display, "Client error: test");
    }
}
