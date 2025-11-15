//! # Type Definitions
//!
//! This module contains type definitions used throughout the client library.
//! These types provide a type-safe interface for interacting with the batch
//! swap router program.

use solana_sdk::pubkey::Pubkey;

/// Parameters for a single swap operation
///
/// This structure contains all parameters needed to execute a single swap
/// within a batch or as a standalone operation.
///
/// # Fields
///
/// * `input_mint` - The mint address of the input token (token being swapped from)
/// * `output_mint` - The mint address of the output token (token being swapped to)
/// * `amount` - Amount of input tokens to swap (in token's smallest unit)
/// * `min_output_amount` - Minimum output amount (slippage protection)
///
/// # Example
///
/// ```rust
/// use xforce_terminal_contracts_client::SwapParams;
/// use solana_sdk::pubkey::Pubkey;
///
/// let swap = SwapParams {
///     input_mint: Pubkey::new_unique(),
///     output_mint: Pubkey::new_unique(),
///     amount: 1_000_000_000, // 1 SOL (in lamports)
///     min_output_amount: 90_000_000, // 90 USDC minimum (10% slippage)
/// };
/// ```
///
/// # Validation
///
/// - `input_mint` must differ from `output_mint`
/// - `amount` must be >= 1 (MIN_SWAP_AMOUNT)
/// - `min_output_amount` must be > 0
///
/// # Slippage Protection
///
/// The `min_output_amount` field provides slippage protection. It specifies
/// the minimum amount of output tokens that must be received for the swap
/// to succeed. If the actual output is less than this amount, the swap will
/// fail.
///
/// # Units
///
/// All amounts are expressed in the token's smallest unit:
/// - SOL: lamports (1 SOL = 1,000,000,000 lamports)
/// - USDC: micro-USDC (1 USDC = 1,000,000 micro-USDC)
/// - Other tokens: depends on token decimals
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwapParams {
    /// Input token mint (source token)
    ///
    /// This is the mint address of the token being swapped from.
    /// Must be a valid token mint address on Solana.
    pub input_mint: Pubkey,

    /// Output token mint (destination token)
    ///
    /// This is the mint address of the token being swapped to.
    /// Must be a valid token mint address on Solana.
    /// Must differ from `input_mint`.
    pub output_mint: Pubkey,

    /// Amount of input tokens to swap
    ///
    /// This is the amount of input tokens to swap, expressed in the token's
    /// smallest unit (e.g., lamports for SOL, or the token's decimal base).
    ///
    /// # Constraints
    ///
    /// - Must be >= 1 (MIN_SWAP_AMOUNT)
    /// - Should be economically meaningful (not dust)
    /// - Must not exceed account balance
    pub amount: u64,

    /// Minimum output amount (for slippage protection)
    ///
    /// This is the minimum amount of output tokens that must be received
    /// for the swap to succeed. If the actual output is less than this amount,
    /// the swap will fail.
    ///
    /// # Constraints
    ///
    /// - Must be > 0
    /// - Should account for expected slippage
    /// - Should be expressed in output token's smallest unit
    ///
    /// # Example
    ///
    /// If swapping 1 SOL for USDC:
    /// - Expected output: 100 USDC
    /// - Slippage tolerance: 5%
    /// - `min_output_amount`: 95 USDC (95% of expected)
    pub min_output_amount: u64,
}

impl SwapParams {
    /// Create a new `SwapParams` instance
    ///
    /// # Arguments
    ///
    /// * `input_mint` - Input token mint
    /// * `output_mint` - Output token mint
    /// * `amount` - Amount of input tokens to swap
    /// * `min_output_amount` - Minimum output amount
    ///
    /// # Returns
    ///
    /// A new `SwapParams` instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use xforce_terminal_contracts_client::SwapParams;
    /// use solana_sdk::pubkey::Pubkey;
    ///
    /// let swap = SwapParams::new(
    ///     Pubkey::new_unique(),
    ///     Pubkey::new_unique(),
    ///     1_000_000_000,
    ///     90_000_000,
    /// );
    /// ```
    #[must_use]
    pub fn new(
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount: u64,
        min_output_amount: u64,
    ) -> Self {
        Self {
            input_mint,
            output_mint,
            amount,
            min_output_amount,
        }
    }

    /// Validate swap parameters
    ///
    /// This function validates that the swap parameters are valid.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If all parameters are valid
    /// * `Err(String)` - If any parameter is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use xforce_terminal_contracts_client::SwapParams;
    /// use solana_sdk::pubkey::Pubkey;
    ///
    /// let swap = SwapParams::new(
    ///     Pubkey::new_unique(),
    ///     Pubkey::new_unique(),
    ///     1_000_000_000,
    ///     90_000_000,
    /// );
    ///
    /// if let Err(e) = swap.validate() {
    ///     eprintln!("Invalid swap parameters: {}", e);
    /// }
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.input_mint == self.output_mint {
            return Err("Input and output mints must differ".to_string());
        }

        if self.amount == 0 {
            return Err("Amount must be greater than zero".to_string());
        }

        if self.min_output_amount == 0 {
            return Err("Minimum output amount must be greater than zero".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_params_new() {
        let input_mint = Pubkey::new_unique();
        let output_mint = Pubkey::new_unique();
        let amount = 1_000_000_000;
        let min_output = 90_000_000;

        let swap = SwapParams::new(input_mint, output_mint, amount, min_output);

        assert_eq!(swap.input_mint, input_mint);
        assert_eq!(swap.output_mint, output_mint);
        assert_eq!(swap.amount, amount);
        assert_eq!(swap.min_output_amount, min_output);
    }

    #[test]
    fn test_swap_params_validate_success() {
        let swap = SwapParams::new(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            1_000_000_000,
            90_000_000,
        );

        assert!(swap.validate().is_ok());
    }

    #[test]
    fn test_swap_params_validate_same_mints() {
        let mint = Pubkey::new_unique();
        let swap = SwapParams::new(mint, mint, 1_000_000_000, 90_000_000);

        assert!(swap.validate().is_err());
    }

    #[test]
    fn test_swap_params_validate_zero_amount() {
        let swap = SwapParams::new(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            0,
            90_000_000,
        );

        assert!(swap.validate().is_err());
    }

    #[test]
    fn test_swap_params_validate_zero_min_output() {
        let swap = SwapParams::new(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            1_000_000_000,
            0,
        );

        assert!(swap.validate().is_err());
    }
}

