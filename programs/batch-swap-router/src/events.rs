//! # Event Definitions
//!
//! This module contains all event types emitted by the batch swap router program.
//! Events are emitted on-chain and can be indexed and queried by off-chain services.
//!
//! ## Events
//!
//! - `BatchSwapEvent`: Emitted when a batch swap is executed
//! - `SwapExecutedEvent`: Emitted when a single swap is executed
//!
//! ## Event Indexing
//!
//! Events can be indexed by:
//! - Block explorers (Solana Explorer, Solscan)
//! - Indexing services (Helius, QuickNode)
//! - Custom indexers
//!
//! ## Event Structure
//!
//! Each event contains:
//! - Relevant data about the operation
//! - Timestamp of execution
//! - Authority who executed the operation
//! - Other operation-specific data

use anchor_lang::prelude::*;

/// Event emitted when a batch swap is executed
///
/// This event is emitted after a successful batch swap execution. It contains
/// information about the batch swap that can be used for tracking, indexing,
/// and analytics.
///
/// # Event Data
///
/// * `authority` - The public key of the authority who executed the batch swap
/// * `swap_count` - The number of swaps executed in this batch
/// * `timestamp` - The Unix timestamp when the batch swap was executed
///
/// # Usage
///
/// This event can be used to:
/// - Track batch swap executions
/// - Analyze swap patterns
/// - Calculate statistics (average batch size, etc.)
/// - Monitor program usage
///
/// # Indexing
///
/// Events can be indexed by:
/// - Block explorers
/// - Indexing services
/// - Custom indexers
///
/// # Example
///
/// ```rust,ignore
/// // Event is automatically emitted after successful batch swap
/// emit!(BatchSwapEvent {
///     authority: authority.key(),
///     swap_count: swaps.len() as u8,
///     timestamp: clock.unix_timestamp,
/// });
/// ```
///
/// # Event Data Details
///
/// ## authority
///
/// The public key of the account that executed the batch swap. This is useful
/// for tracking which users are using the batch swap functionality.
///
/// ## swap_count
///
/// The number of swaps executed in this batch. This is useful for:
/// - Analyzing batch sizes
/// - Understanding usage patterns
/// - Calculating average batch sizes
///
/// ## timestamp
///
/// The Unix timestamp when the batch swap was executed. This is useful for:
/// - Time-based analysis
/// - Tracking execution times
/// - Monitoring program activity over time
///
#[event]
pub struct BatchSwapEvent {
    /// The public key of the authority who executed the batch swap
    pub authority: Pubkey,
    
    /// The number of swaps executed in this batch
    pub swap_count: u8,
    
    /// Total input amount across all swaps
    pub total_input_amount: u64,
    
    /// Total protocol fees collected
    pub total_protocol_fees: u64,
    
    /// The Unix timestamp when the batch swap was executed
    pub timestamp: i64,
}

/// Event emitted when a single swap is executed
///
/// This event is emitted after a successful single swap execution. It contains
/// information about the swap that can be used for tracking, indexing, and analytics.
///
/// # Event Data
///
/// * `authority` - The public key of the authority who executed the swap
/// * `amount` - The amount of tokens transferred
/// * `from` - The public key of the source token account
/// * `to` - The public key of the destination token account
/// * `timestamp` - The Unix timestamp when the swap was executed
///
/// # Usage
///
/// This event can be used to:
/// - Track swap executions
/// - Analyze swap patterns
/// - Calculate statistics (total volume, etc.)
/// - Monitor program usage
///
/// # Indexing
///
/// Events can be indexed by:
/// - Block explorers
/// - Indexing services
/// - Custom indexers
///
/// # Example
///
/// ```rust,ignore
/// // Event is automatically emitted after successful swap
/// emit!(SwapExecutedEvent {
///     authority: authority.key(),
///     amount: amount,
///     from: from.key(),
///     to: to.key(),
///     timestamp: clock.unix_timestamp,
/// });
/// ```
///
/// # Event Data Details
///
/// ## authority
///
/// The public key of the account that executed the swap. This is useful
/// for tracking which users are using the swap functionality.
///
/// ## amount
///
/// The amount of tokens transferred in this swap. This is expressed in
/// the token's smallest unit (e.g., lamports for SOL).
///
/// ## from
///
/// The public key of the source token account. This is the account that
/// tokens were transferred from.
///
/// ## to
///
/// The public key of the destination token account. This is the account
/// that tokens were transferred to.
///
/// ## timestamp
///
/// The Unix timestamp when the swap was executed. This is useful for
/// time-based analysis and tracking.
///
#[event]
pub struct SwapExecutedEvent {
    /// The public key of the authority who executed the swap
    pub authority: Pubkey,
    
    /// Input token amount
    pub input_amount: u64,
    
    /// Output token amount received
    pub output_amount: u64,
    
    /// Input token mint
    pub input_mint: Pubkey,
    
    /// Output token mint
    pub output_mint: Pubkey,
    
    /// Protocol fee charged
    pub protocol_fee: u64,
    
    /// Slippage in basis points
    pub slippage_bps: u64,
    
    /// The Unix timestamp when the swap was executed
    pub timestamp: i64,
}


