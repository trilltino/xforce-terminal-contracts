//! # Instruction Handlers Module
//!
//! This module contains all instruction handlers for the batch swap router program.
//! Each instruction has its own module with a handler function that contains the
//! instruction logic.
//!
//! ## Module Structure
//!
//! Each instruction handler is in its own module:
//!
//! - [`batch_swap`] - Batch swap instruction handler
//! - [`execute_swap`] - Single swap instruction handler
//!
//! ## Handler Pattern
//!
//! All instruction handlers follow this pattern:
//!
//! 1. **Validate Inputs**: Check that all inputs are valid and within limits
//! 2. **Validate Accounts**: Verify account ownership, types, and relationships
//! 3. **Execute Operation**: Perform the swap operation (client-side or program-side)
//! 4. **Validate Results**: Check slippage, fees, and output amounts
//! 5. **Emit Events**: Emit events for tracking and indexing
//! 6. **Return Success**: Return `Ok(())` on success
//!
//! ## Error Handling
//!
//! All handlers use the [`ErrorCode`] enum for error handling. Errors are returned
//! using Anchor's `Result<T>` type, which automatically reverts the transaction
//! on error.
//!
//! ## Events
//!
//! Handlers emit events for tracking and indexing:
//!
//! - [`BatchSwapEvent`] - Emitted by `batch_swap` handler
//! - [`SwapExecutedEvent`] - Emitted by `execute_swap` handler
//!
//! ## Usage
//!
//! Instruction handlers are called automatically by Anchor when the corresponding
//! instruction is invoked. They should not be called directly from other parts
//! of the program.
//!
//! [`ErrorCode`]: crate::errors::ErrorCode
//! [`BatchSwapEvent`]: crate::events::BatchSwapEvent
//! [`SwapExecutedEvent`]: crate::events::SwapExecutedEvent

pub mod batch_swap;
pub mod execute_swap;

// Re-export handlers for convenience
pub use batch_swap::handler as batch_swap_handler;
pub use execute_swap::handler as execute_swap_handler;
