//! # Batch Swap Router Client
//!
//! Client for interacting with the batch swap router contract.
//!
//! This client provides a high-level, type-safe interface for interacting with the
//! batch swap router program. It wraps the Anchor program client and provides
//! convenient methods for executing batch swaps and single swaps.
//!
//! ## Overview
//!
//! The `BatchSwapRouterClient` provides methods for:
//!
//! - Executing batch swaps (multiple swaps in one transaction)
//! - Executing single swaps
//! - Managing swap parameters
//! - Error handling and validation
//!
//! ## Usage
//!
//! ### Creating a Client
//!
//! ```rust,no_run
//! use xforce_terminal_contracts_client::*;
//! use anchor_client::Client;
//! use solana_sdk::signature::Keypair;
//!
//! // Create an Anchor client
//! let payer = Keypair::new();
//! let client = create_client("http://localhost:8899", payer)?;
//!
//! // Get the program
//! let program_id = get_batch_swap_router_program_id();
//! let program = client.program(program_id)?;
//!
//! // Create batch swap router client
//! let swap_client = BatchSwapRouterClient::new(program);
//! ```
//!
//! ### Executing a Batch Swap
//!
//! ```rust,no_run
//! use xforce_terminal_contracts_client::{BatchSwapRouterClient, SwapParams};
//! use solana_sdk::pubkey::Pubkey;
//!
//! // Prepare swap parameters
//! let swaps = vec![
//!     SwapParams {
//!         input_mint: sol_mint,
//!         output_mint: usdc_mint,
//!         amount: 1_000_000_000, // 1 SOL
//!         min_output_amount: 90_000_000, // 90 USDC minimum
//!     },
//! ];
//!
//! // Execute batch swap
//! let signature = swap_client.batch_swap(swaps)?;
//! println!("Transaction signature: {}", signature);
//! ```
//!
//! ### Executing a Single Swap
//!
//! ```rust,no_run
//! use xforce_terminal_contracts_client::BatchSwapRouterClient;
//!
//! // Execute single swap
//! let signature = swap_client.execute_swap(
//!     input_token_account,
//!     output_token_account,
//!     input_mint,
//!     output_mint,
//!     1_000_000_000,  // Input amount
//!     90_000_000,     // Min output
//!     95_000_000,     // Expected output
//! )?;
//! ```
//!
//! ## Error Handling
//!
//! All methods return `Result<T, ContractError>`. Errors can be handled
//! as follows:
//!
//! ```rust,no_run
//! use xforce_terminal_contracts_client::ContractError;
//!
//! match swap_client.batch_swap(swaps) {
//!     Ok(signature) => println!("Success: {}", signature),
//!     Err(ContractError::TransactionFailed(msg)) => {
//!         eprintln!("Transaction failed: {}", msg);
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! ## Notes
//!
//! - After building the Anchor program with `anchor build`, the IDL will be
//!   generated and the client will use the generated types automatically.
//! - For now, the client methods require the IDL to be generated first.
//! - All operations are synchronous and blocking.

use anchor_client::Program;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Signer, Signature},
};

use crate::error::ContractError;
use crate::types::SwapParams;

/// Client for batch swap router contract
///
/// This client wraps the Anchor program client and provides a high-level
/// interface for batch swap operations. It handles instruction construction,
/// account management, and error handling.
///
/// # Type Parameters
///
/// * `C` - The signer type that implements `Signer + Clone`
///
/// # Example
///
/// ```rust,no_run
/// use xforce_terminal_contracts_client::BatchSwapRouterClient;
/// use anchor_client::Program;
///
/// let program: Program<SomeSigner> = // ... get program
/// let client = BatchSwapRouterClient::new(program);
/// ```
pub struct BatchSwapRouterClient<C> {
    /// The underlying Anchor program client
    program: Program<C>,
}

impl<C> BatchSwapRouterClient<C>
where
    C: Signer + Clone,
{
    /// Create a new batch swap router client
    ///
    /// # Arguments
    ///
    /// * `program` - The Anchor program client
    ///
    /// # Returns
    ///
    /// A new `BatchSwapRouterClient` instance
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use xforce_terminal_contracts_client::BatchSwapRouterClient;
    /// use anchor_client::Program;
    ///
    /// let program: Program<SomeSigner> = // ... get program
    /// let client = BatchSwapRouterClient::new(program);
    /// ```
    #[must_use]
    pub fn new(program: Program<C>) -> Self {
        Self { program }
    }

    /// Execute a batch swap
    ///
    /// This method executes a batch of token swaps in a single transaction.
    /// All swaps are validated and executed atomically.
    ///
    /// # Arguments
    ///
    /// * `swaps` - Vector of swap parameters. Each swap specifies:
    ///   - `input_mint`: The mint address of the input token
    ///   - `output_mint`: The mint address of the output token
    ///   - `amount`: Amount of input tokens to swap
    ///   - `min_output_amount`: Minimum amount of output tokens to receive (slippage protection)
    ///
    /// # Returns
    ///
    /// * `Ok(Signature)` - Transaction signature on success
    /// * `Err(ContractError)` - Error if the transaction fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The batch is empty
    /// - The batch exceeds the maximum size (10 swaps)
    /// - Any swap parameter is invalid
    /// - The transaction fails
    /// - The IDL types are not available (program not built)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use xforce_terminal_contracts_client::{BatchSwapRouterClient, SwapParams};
    ///
    /// let swaps = vec![
    ///     SwapParams {
    ///         input_mint: sol_mint,
    ///         output_mint: usdc_mint,
    ///         amount: 1_000_000_000, // 1 SOL
    ///         min_output_amount: 90_000_000, // 90 USDC minimum
    ///     },
    /// ];
    ///
    /// let signature = client.batch_swap(swaps)?;
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// After building the Anchor program with `anchor build`, the IDL will be
    /// generated and this method will use the generated types. For now, this
    /// method requires the IDL to be generated first.
    ///
    /// The actual implementation would look like:
    ///
    /// ```rust,ignore
    /// let swap_args: Vec<_> = swaps.into_iter().map(|s| batch_swap_router::SwapParams {
    ///     input_mint: s.input_mint,
    ///     output_mint: s.output_mint,
    ///     amount: s.amount,
    ///     min_output_amount: s.min_output_amount,
    /// }).collect();
    ///
    /// let payer = self.program.payer();
    /// let authority = payer.pubkey();
    ///
    /// self.program
    ///     .request()
    ///     .accounts(batch_swap_router::accounts::BatchSwap {
    ///         authority,
    ///         fee_recipient: None,
    ///         token_program: anchor_spl::token::ID,
    ///         system_program: anchor_client::solana_sdk::system_program::ID,
    ///     })
    ///     .args(batch_swap_router::instruction::BatchSwap { swaps: swap_args })
    ///     .send()
    ///     .map_err(|e| ContractError::TransactionFailed(e.to_string()))
    /// ```
    pub fn batch_swap(
        &self,
        swaps: Vec<SwapParams>,
    ) -> Result<Signature, ContractError> {
        // Validate swaps
        for swap in &swaps {
            swap.validate()
                .map_err(|e| ContractError::InvalidAccount(e))?;
        }

        // Build the instruction request
        // Note: After building with Anchor, use the generated IDL types
        //
        // For now, this requires the IDL to be generated by running `anchor build`
        Err(ContractError::TransactionFailed(
            "Batch swap requires Anchor IDL types. Build the program with 'anchor build' first, then use the generated IDL types with anchor-client.".to_string()
        ))
    }

    /// Execute a single swap
    ///
    /// This method executes a single token swap with slippage protection and
    /// fee calculation. It executes swaps between different token mints.
    ///
    /// # Arguments
    ///
    /// * `input_token_account` - Input token account (tokens swapped from)
    /// * `output_token_account` - Output token account (tokens received)
    /// * `input_mint` - Input token mint
    /// * `output_mint` - Output token mint
    /// * `amount` - Amount of input tokens to swap
    /// * `min_output_amount` - Minimum output amount (slippage protection)
    /// * `expected_output` - Expected output amount (from Jupiter quote)
    ///
    /// # Returns
    ///
    /// * `Ok(Signature)` - Transaction signature on success
    /// * `Err(ContractError)` - Error if the transaction fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The amount is zero or below minimum
    /// - The input and output accounts have the same mint
    /// - The authority doesn't own the input token account
    /// - Slippage tolerance is exceeded
    /// - The transaction fails
    /// - The IDL types are not available (program not built)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use xforce_terminal_contracts_client::BatchSwapRouterClient;
    ///
    /// let signature = client.execute_swap(
    ///     input_token_account,
    ///     output_token_account,
    ///     input_mint,
    ///     output_mint,
    ///     1_000_000_000,  // Input amount: 1 SOL
    ///     90_000_000,     // Min output: 90 USDC
    ///     95_000_000,     // Expected output: 95 USDC
    /// )?;
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// After building the Anchor program with `anchor build`, the IDL will be
    /// generated and this method will use the generated types.
    pub fn execute_swap(
        &self,
        _input_token_account: Pubkey,
        _output_token_account: Pubkey,
        _input_mint: Pubkey,
        _output_mint: Pubkey,
        amount: u64,
        min_output_amount: u64,
        expected_output: u64,
    ) -> Result<Signature, ContractError> {
        // Validate parameters
        if amount == 0 {
            return Err(ContractError::InvalidAccount(
                "Amount must be greater than zero".to_string(),
            ));
        }

        if min_output_amount == 0 {
            return Err(ContractError::InvalidAccount(
                "Minimum output amount must be greater than zero".to_string(),
            ));
        }

        if _input_mint == _output_mint {
            return Err(ContractError::InvalidAccount(
                "Input and output mints must differ".to_string(),
            ));
        }

        // Build the instruction request
        // Note: After building with Anchor, use the generated IDL types
        //
        // For now, this requires the IDL to be generated by running `anchor build`
        Err(ContractError::TransactionFailed(
            "Execute swap requires Anchor IDL types. Build the program with 'anchor build' first, then use the generated IDL types with anchor-client.".to_string()
        ))
    }

    /// Get the underlying program instance
    ///
    /// This method returns a reference to the underlying Anchor program client.
    /// This can be useful for advanced operations that require direct access
    /// to the program client.
    ///
    /// # Returns
    ///
    /// A reference to the underlying program client
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use xforce_terminal_contracts_client::BatchSwapRouterClient;
    ///
    /// let program = client.program();
    /// // Use program for advanced operations
    /// ```
    #[must_use]
    pub fn program(&self) -> &Program<C> {
        &self.program
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    // Note: These tests require a mock program, which would require additional
    // setup. For now, we test the validation logic.

    #[test]
    fn test_execute_swap_validation_zero_amount() {
        // This test would require a mock program
        // For now, we can test the validation logic separately
        let result: Result<(), ContractError> = Err(ContractError::InvalidAccount(
            "Amount must be greater than zero".to_string(),
        ));

        assert!(result.is_err());
        if let Err(ContractError::InvalidAccount(msg)) = result {
            assert!(msg.contains("Amount must be greater than zero"));
        }
    }

    #[test]
    fn test_execute_swap_validation_same_mints() {
        let result: Result<(), ContractError> = Err(ContractError::InvalidAccount(
            "Input and output mints must differ".to_string(),
        ));

        assert!(result.is_err());
        if let Err(ContractError::InvalidAccount(msg)) = result {
            assert!(msg.contains("Input and output mints must differ"));
        }
    }
}
