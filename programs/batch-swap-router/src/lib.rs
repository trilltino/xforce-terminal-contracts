//! # Batch Swap Router Program
//!
//! A Solana program that enables batch execution of token swaps in a single transaction.
//! This program allows users to execute multiple swaps atomically, reducing transaction
//! fees and improving user experience.
//!
//! ## Overview
//!
//! The Batch Swap Router is designed to address the common problem of executing multiple
//! token swaps which would normally require multiple transactions. By batching swaps into
//! a single transaction, users can:
//!
//! - **Reduce Fees**: Pay transaction fees once instead of multiple times
//! - **Atomic Execution**: All swaps succeed or fail together
//! - **Better UX**: Execute complex swap strategies in one transaction
//! - **Slippage Protection**: Validate slippage for each swap
//! - **Fee Management**: Calculate and distribute protocol fees
//!
//! ## Architecture
//!
//! The program is structured in a modular fashion following Anchor best practices:
//!
//! ```text
//! lib.rs                    # Main program entry point
//! ├── constants.rs          # Program constants (limits, minimums, fees)
//! ├── errors.rs             # Error definitions
//! ├── events.rs             # Event definitions
//! ├── state.rs              # Account structures and state types
//! ├── utils.rs              # Utility functions
//! ├── swap_execution.rs     # Swap execution logic
//! └── instructions/         # Instruction handlers
//!     ├── mod.rs           # Instruction module
//!     ├── batch_swap.rs    # Batch swap instruction
//!     └── execute_swap.rs  # Single swap instruction
//! ```
//!
//! ## Instructions
//!
//! ### `batch_swap`
//!
//! Execute multiple swaps in a single transaction. This is the primary instruction
//! that enables fee reduction by batching multiple operations.
//!
//! **Features**:
//! - Maximum 10 swaps per batch
//! - Atomic execution (all or nothing)
//! - Comprehensive validation
//! - Fee calculation and tracking
//! - Event emission for tracking
//!
//! ### `execute_swap`
//!
//! Execute a single token swap with slippage protection and fee calculation.
//! This instruction performs actual token swaps between different mints.
//!
//! **Features**:
//! - Token swap execution (different mints)
//! - Slippage validation
//! - Fee calculation and distribution
//! - Account validation
//! - Authority verification
//! - Event emission
//!
//! ## Security Considerations
//!
//! - All inputs are validated before processing
//! - Account ownership is verified
//! - Amount limits prevent dust attacks
//! - Batch size limits prevent DoS attacks
//! - Atomic execution prevents partial failures
//! - Slippage protection prevents unfavorable swaps
//! - Fee calculation is transparent and auditable
//!
//! ## Usage
//!
//! ### Batch Swap
//!
//! ```rust,ignore
//! // Execute batch swap
//! batch_swap(ctx, vec![
//!     SwapParams {
//!         input_mint: sol_mint,
//!         output_mint: usdc_mint,
//!         amount: 1_000_000_000, // 1 SOL
//!         min_output_amount: 90_000_000, // 90 USDC (10% slippage)
//!     },
//! ])?;
//! ```
//!
//! ### Single Swap
//!
//! ```rust,ignore
//! // Execute single swap
//! execute_swap(
//!     ctx,
//!     1_000_000_000,  // Input amount: 1 SOL
//!     90_000_000,     // Min output: 90 USDC
//!     95_000_000,     // Expected output: 95 USDC (from Jupiter quote)
//! )?;
//! ```
//!
//! ## Events
//!
//! The program emits events for tracking and indexing:
//!
//! - `BatchSwapEvent` - Emitted when a batch swap is executed
//!   - Contains: authority, swap_count, total_input_amount, total_protocol_fees, timestamp
//!
//! - `SwapExecutedEvent` - Emitted when a single swap is executed
//!   - Contains: authority, input_amount, output_amount, input_mint, output_mint,
//!     protocol_fee, slippage_bps, timestamp
//!
//! ## Error Handling
//!
//! All errors are defined in the `ErrorCode` enum and provide descriptive
//! error messages for debugging and user feedback. Common errors include:
//!
//! - `EmptySwaps` - No swaps provided in batch
//! - `TooManySwaps` - Batch exceeds maximum size
//! - `InvalidAmount` - Invalid swap amount
//! - `SlippageExceeded` - Slippage tolerance exceeded
//! - `SwapExecutionFailed` - Swap execution failed
//!
//! ## Integration
//!
//! This program integrates with:
//!
//! - **Jupiter Aggregator**: For DEX routing and swap execution (client-side)
//! - **SPL Token Program**: For token operations
//! - **System Program**: For account management
//!
//! ## Future Enhancements
//!
//! - Program-side Jupiter integration via CPI
//! - Price oracle integration
//! - Advanced routing logic
//! - Multi-hop swap optimization
//! - Fee optimization strategies
//!
//! ## License
//!
//! This program is licensed under the MIT License.

#![allow(clippy::result_large_err)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use anchor_lang::prelude::*;

// Declare the program ID
// Program ID for devnet deployment
declare_id!("HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx");

// Module declarations
pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod security;
pub mod state;
pub mod swap_execution;
pub mod utils;

// Re-export commonly used types
pub use constants::*;
pub use errors::ErrorCode;
pub use events::*;
pub use security::*;
pub use state::*;
pub use swap_execution::*;

// Instruction handlers are used directly in the instruction functions

/// Main program module
///
/// This module contains the program entry point and instruction handlers.
/// All instructions are delegated to their respective handler modules for
/// better organization and maintainability.
#[program]
pub mod batch_swap_router {
    use super::*;

    /// Execute multiple token swaps in a single transaction
    ///
    /// This instruction allows users to execute multiple swaps atomically in a
    /// single transaction, reducing fees and improving user experience.
    ///
    /// # Process Flow
    ///
    /// 1. Validate the batch size (not empty, not too large)
    /// 2. Validate each swap parameter
    /// 3. Calculate fees for all swaps
    /// 4. Validate swap parameters
    /// 5. Emit event for tracking
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context containing account information
    /// * `swaps` - Vector of swap parameters (max 10 swaps per batch)
    ///
    /// # Accounts
    ///
    /// * `authority` - The signer executing the batch swap (must sign)
    /// * `fee_recipient` - Optional fee recipient account
    /// * `token_program` - SPL Token program
    /// * `system_program` - System program for account management
    ///
    /// # Validation
    ///
    /// - Batch must not be empty
    /// - Batch size must not exceed MAX_BATCH_SIZE (10)
    /// - Each swap amount must be >= MIN_SWAP_AMOUNT (1)
    /// - Input and output mints must differ for each swap
    /// - Minimum output amount must be > 0 for each swap
    ///
    /// # Errors
    ///
    /// * `ErrorCode::EmptySwaps` - No swaps provided
    /// * `ErrorCode::TooManySwaps` - More than MAX_BATCH_SIZE swaps provided
    /// * `ErrorCode::InvalidAmount` - Invalid swap amount (zero or below minimum)
    /// * `ErrorCode::InvalidSwapPair` - Input and output mints are the same
    /// * `ErrorCode::InvalidMinOutput` - Invalid minimum output amount
    ///
    /// # Events
    ///
    /// Emits `BatchSwapEvent` on successful execution with:
    /// - Authority public key
    /// - Number of swaps executed
    /// - Total input amount
    /// - Total protocol fees
    /// - Timestamp of execution
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Execute a batch of 3 swaps
    /// batch_swap(ctx, vec![
    ///     SwapParams {
    ///         input_mint: sol_mint,
    ///         output_mint: usdc_mint,
    ///         amount: 1_000_000_000, // 1 SOL
    ///         min_output_amount: 90_000_000, // 90 USDC (10% slippage)
    ///     },
    ///     SwapParams {
    ///         input_mint: usdc_mint,
    ///         output_mint: btc_mint,
    ///         amount: 50_000_000, // 50 USDC
    ///         min_output_amount: 0_001_000_000, // 0.001 BTC
    ///     },
    /// ])?;
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// - For client-side execution: Client gets quotes from Jupiter API, constructs
    ///   transaction with Jupiter swap instructions, and includes this instruction
    ///   for validation and tracking
    /// - For program-side execution (future): Program would call Jupiter program via
    ///   CPI for each swap and validate slippage after execution
    pub fn batch_swap(ctx: Context<BatchSwap>, swaps: Vec<SwapParams>) -> Result<()> {
        instructions::batch_swap::handler(ctx, swaps)
    }

    /// Execute a single token swap
    ///
    /// This instruction performs an actual token swap with slippage protection,
    /// fee calculation, and DEX integration support. It executes swaps between
    /// different token mints with validation and fee distribution.
    ///
    /// # Process Flow
    ///
    /// 1. Validate the swap amount and parameters
    /// 2. Validate account mints differ (actual swap)
    /// 3. Verify authority owns the input account
    /// 4. Validate fee recipient (if provided)
    /// 5. Calculate protocol fees
    /// 6. Execute swap (client-side via Jupiter or program-side)
    /// 7. Validate slippage tolerance
    /// 8. Distribute fees (if fee recipient provided)
    /// 9. Emit event for tracking
    ///
    /// # Arguments
    ///
    /// * `ctx` - Context containing token accounts, mints, and authority
    /// * `amount` - Amount of input tokens to swap (in token's smallest unit)
    /// * `min_output_amount` - Minimum output amount (slippage protection)
    /// * `expected_output` - Expected output amount (from Jupiter quote, client-provided)
    ///
    /// # Accounts
    ///
    /// * `authority` - The signer executing the swap (must sign, must own input account)
    /// * `input_token_account` - Input token account (tokens swapped from)
    /// * `output_token_account` - Output token account (tokens received)
    /// * `input_mint` - Input token mint
    /// * `output_mint` - Output token mint
    /// * `fee_recipient` - Optional fee recipient account
    /// * `token_program` - SPL Token program
    /// * `system_program` - System program
    ///
    /// # Validation
    ///
    /// - Amount must be >= MIN_SWAP_AMOUNT (1)
    /// - Input and output accounts must have different mints
    /// - Authority must be the owner of the input token account
    /// - Slippage must be within tolerance (MAX_SLIPPAGE_BPS)
    /// - Output must meet minimum requirement
    ///
    /// # Errors
    ///
    /// * `ErrorCode::InvalidAmount` - Amount is zero or below minimum
    /// * `ErrorCode::InvalidSwapPair` - Input and output mints are the same
    /// * `ErrorCode::InvalidAuthority` - Authority doesn't own input account
    /// * `ErrorCode::SlippageExceeded` - Actual output < min_output_amount
    /// * `ErrorCode::SwapExecutionFailed` - Swap execution failed
    /// * `ErrorCode::InvalidFeeRecipient` - Invalid fee recipient account
    ///
    /// # Events
    ///
    /// Emits `SwapExecutedEvent` on successful execution with:
    /// - Authority public key
    /// - Input and output amounts
    /// - Input and output mints
    /// - Protocol fee
    /// - Slippage in basis points
    /// - Timestamp of execution
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Swap 1000 tokens from mint A to mint B
    /// // Expected output: 950 tokens (from Jupiter quote)
    /// // Minimum output: 900 tokens (5% slippage tolerance)
    /// execute_swap(ctx, 1000, 900, 950)?;
    /// ```
    ///
    /// # Security Notes
    ///
    /// - Authority must sign the transaction
    /// - Authority must own the input token account
    /// - Slippage protection prevents unfavorable swaps
    /// - Fees are calculated and distributed transparently
    /// - Swap execution integrates with Jupiter/DEX aggregators
    ///
    /// # Implementation Notes
    ///
    /// - For client-side execution: Client includes Jupiter swap instructions in the
    ///   same transaction, and this instruction validates the results
    /// - For program-side execution (future): Program would call Jupiter program via
    ///   CPI to execute the swap
    pub fn execute_swap(
        ctx: Context<ExecuteSwap>,
        amount: u64,
        min_output_amount: u64,
        expected_output: u64,
    ) -> Result<()> {
        instructions::execute_swap::handler(ctx, amount, min_output_amount, expected_output)
    }
}
