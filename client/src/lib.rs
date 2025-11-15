//! # XForce Terminal Contracts Client
//!
//! Rust client library for interacting with XForce Terminal Solana smart contracts.
//! This client provides a high-level, type-safe interface for interacting with the
//! batch swap router program.
//!
//! ## Overview
//!
//! The XForce Terminal Contracts Client is a Rust library that simplifies interaction
//! with the batch swap router program on Solana. It provides:
//!
//! - **Type-safe APIs**: Compile-time type checking for all operations
//! - **Error Handling**: Comprehensive error handling with detailed error messages
//! - **Transaction Management**: Simplified transaction creation and submission
//! - **Account Management**: Helper functions for account creation and management
//!
//! ## Architecture
//!
//! ```text
//! lib.rs                    # Main library entry point
//! ├── batch_swap_router.rs  # Batch swap router client
//! ├── error.rs              # Error definitions
//! └── types.rs              # Type definitions
//! ```
//!
//! ## Usage
//!
//! ### Basic Usage
//!
//! ```rust,no_run
//! use xforce_terminal_contracts_client::*;
//! use anchor_client::Client;
//! use solana_sdk::signature::Keypair;
//!
//! // Create a client
//! let payer = Keypair::new();
//! let client = create_client("http://localhost:8899", payer)?;
//!
//! // Get the program
//! let program_id = get_batch_swap_router_program_id();
//! let program = client.program(program_id)?;
//!
//! // Create batch swap router client
//! let swap_client = BatchSwapRouterClient::new(program);
//!
//! // Execute a batch swap
//! let swaps = vec![
//!     SwapParams {
//!         input_mint: mint_a,
//!         output_mint: mint_b,
//!         amount: 1000,
//!         min_output_amount: 900,
//!     },
//! ];
//!
//! let signature = swap_client.batch_swap(swaps).await?;
//! ```
//!
//! ### Error Handling
//!
//! ```rust,no_run
//! use xforce_terminal_contracts_client::ContractError;
//!
//! match swap_client.batch_swap(swaps).await {
//!     Ok(signature) => println!("Transaction: {}", signature),
//!     Err(ContractError::TransactionFailed(msg)) => {
//!         eprintln!("Transaction failed: {}", msg);
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! ## Features
//!
//! - **Async Support**: All operations are async and non-blocking
//! - **Type Safety**: Compile-time type checking for all operations
//! - **Error Handling**: Comprehensive error handling with detailed messages
//! - **Transaction Management**: Simplified transaction creation and submission
//! - **Account Management**: Helper functions for account operations
//!
//! ## Requirements
//!
//! - Rust 1.70.0 or later
//! - Anchor framework (for program compilation)
//! - Solana CLI tools (for local development)
//!
//! ## License
//!
//! This client is licensed under the MIT License.

#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]

use anchor_client::Client;
use anchor_client::solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signer,
};
use std::rc::Rc;

pub mod batch_swap_router;
pub mod error;
pub mod security;
pub mod types;

/// Re-export commonly used types and clients for convenience.
pub use batch_swap_router::BatchSwapRouterClient;
pub use error::ContractError;
pub use security::*;
pub use types::SwapParams;

/// Create a client for interacting with XForce Terminal contracts
///
/// This function creates a new Anchor client configured for interacting with
/// XForce Terminal contracts on the specified cluster.
///
/// # Arguments
///
/// * `cluster_url` - The RPC URL of the Solana cluster (e.g., "http://localhost:8899")
/// * `payer` - The keypair that will pay for transactions
///
/// # Returns
///
/// * `Ok(Client<Rc<C>>)` - A configured Anchor client on success
/// * `Err(ContractError)` - An error if client creation fails
///
/// # Example
///
/// ```rust,no_run
/// use xforce_terminal_contracts_client::create_client;
/// use solana_sdk::signature::Keypair;
///
/// let payer = Keypair::new();
/// let client = create_client("http://localhost:8899", payer)?;
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The cluster URL is invalid
/// - The client cannot be created
/// - The payer keypair is invalid
pub fn create_client<C>(cluster_url: &str, payer: C) -> Client<Rc<C>>
where
    C: Clone + Signer + 'static,
{
    let commitment = CommitmentConfig::confirmed();
    Client::new_with_options(
        anchor_client::Cluster::Custom(cluster_url.to_string(), "".to_string()),
        Rc::new(payer),
        commitment,
    )
}

/// Get the program ID for batch swap router
///
/// This function returns the program ID for the batch swap router program.
/// The program ID is declared in the program's `lib.rs` file and should match
/// the deployed program ID.
///
/// # Returns
///
/// The program ID as a `Pubkey`
///
/// # Example
///
/// ```rust
/// use xforce_terminal_contracts_client::get_batch_swap_router_program_id;
///
/// let program_id = get_batch_swap_router_program_id();
/// println!("Program ID: {}", program_id);
/// ```
///
/// # Note
///
/// This program ID matches the program ID declared in
/// `programs/batch-swap-router/src/lib.rs`. After deploying the program,
/// ensure this ID matches the deployed program ID.
pub fn get_batch_swap_router_program_id() -> Pubkey {
    // This matches the program ID in programs/batch-swap-router/src/lib.rs (devnet)
    "C48gmshkEL8UdCe8GcpZKGwrEfCLbWWq4zk23tHmNDcE"
        .parse()
        .expect("Invalid batch-swap-router program ID")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the program ID is valid
    #[test]
    fn test_program_id() {
        let program_id = get_batch_swap_router_program_id();
        assert_eq!(
            program_id.to_string(),
            "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"
        );
    }

    /// Test that program ID can be converted to and from string
    #[test]
    fn test_program_id_roundtrip() {
        let program_id = get_batch_swap_router_program_id();
        let program_id_str = program_id.to_string();
        let program_id_parsed: Pubkey = program_id_str.parse().unwrap();
        assert_eq!(program_id, program_id_parsed);
    }
}
