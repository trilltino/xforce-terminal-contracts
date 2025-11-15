//! # Program Constants
//!
//! This module contains all constants used throughout the batch swap router program.
//! These constants define limits, minimums, and other configuration values that
//! affect the program's behavior.
//!
//! ## Constants
//!
//! - `MAX_BATCH_SIZE`: Maximum number of swaps allowed in a single batch
//! - `MIN_SWAP_AMOUNT`: Minimum swap amount to prevent dust attacks

/// Maximum number of swaps allowed in a single batch transaction
///
/// This limit prevents:
/// - DoS attacks through excessive computation
/// - Transaction size limits
/// - Excessive compute unit usage
///
/// **Current Value**: 10 swaps per batch
///
/// This limit balances functionality with security and performance.
/// Increasing this limit would:
/// - Allow more swaps per transaction (better fee savings)
/// - Increase compute unit usage (higher risk of hitting limits)
/// - Increase transaction size (may hit size limits)
///
/// # Rationale
///
/// - 10 swaps is enough for most use cases (portfolio rebalancing, etc.)
/// - Keeps compute units well below Solana's limits
/// - Allows for significant fee savings (1 transaction vs 10)
///
/// # Future Considerations
///
/// - Could be made configurable per program
/// - Could be adjusted based on network conditions
/// - Could be different for different instruction types
pub const MAX_BATCH_SIZE: usize = 10;

/// Minimum swap amount to prevent dust attacks
///
/// This minimum prevents:
/// - Dust attacks (spam transactions with tiny amounts)
/// - Economic attacks (exploiting rounding errors)
/// - Unnecessary computation for trivial amounts
///
/// **Current Value**: 1 token unit (smallest unit)
///
/// This is the absolute minimum - in practice, users should use
/// amounts that are economically meaningful (e.g., at least $1 worth).
///
/// # Rationale
///
/// - 1 unit is the smallest possible amount
/// - Prevents zero-amount swaps
/// - Prevents dust attacks
/// - Allows for maximum flexibility
///
/// # Security Considerations
///
/// - Prevents zero-amount swaps that could be used for attacks
/// - Prevents dust attacks that could spam the network
/// - Prevents economic attacks exploiting rounding errors
///
/// # Future Considerations
///
/// - Could be made configurable per token (different tokens have different decimals)
/// - Could be adjusted based on token value
/// - Could be different for different instruction types
pub const MIN_SWAP_AMOUNT: u64 = 1;

/// Program name for logging and identification
pub const PROGRAM_NAME: &str = "batch-swap-router";

/// Program version
pub const PROGRAM_VERSION: &str = "0.1.0";

/// Protocol fee basis points (100 = 1%)
/// 
/// This fee is charged on each swap and distributed to the protocol treasury.
/// The fee is calculated as a percentage of the swap amount.
///
/// **Current Value**: 30 basis points (0.3%)
///
/// # Fee Calculation
///
/// Fee = (swap_amount * PROTOCOL_FEE_BPS) / 10000
///
/// Example:
/// - Swap amount: 1000 tokens
/// - Fee BPS: 30
/// - Fee: (1000 * 30) / 10000 = 3 tokens
pub const PROTOCOL_FEE_BPS: u64 = 30;

/// Maximum slippage tolerance in basis points (500 = 5%)
///
/// This is a safety limit to prevent excessive slippage tolerance.
/// Users can set lower tolerance, but not higher than this maximum.
///
/// **Current Value**: 500 basis points (5%)
pub const MAX_SLIPPAGE_BPS: u64 = 500;

/// Jupiter program ID (v6)
///
/// This is the program ID for Jupiter aggregator v6.
/// Used for executing swaps via CPI.
///
/// **Program ID**: `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4`
pub const JUPITER_PROGRAM_ID: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";


