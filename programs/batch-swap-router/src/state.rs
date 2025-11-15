//! # Account State and Structures
//!
//! This module contains all account structures and state types used by the
//! batch swap router program. These structures define the accounts required
//! for each instruction and the data types used throughout the program.
//!
//! ## Account Structures
//!
//! - `BatchSwap`: Accounts required for batch swap instruction
//! - `ExecuteSwap`: Accounts required for execute swap instruction
//!
//! ## Data Types
//!
//! - `SwapParams`: Parameters for a single swap operation

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

/// Account structure for batch swap instruction
///
/// This structure defines all accounts required to execute a batch swap.
/// For program-side execution, this includes token accounts for each swap.
///
/// # Accounts
///
/// * `authority` - The signer executing the batch swap
///   - Must be mutable (may need to pay fees)
///   - Must sign the transaction
///   - Must own all input token accounts
///
/// * `fee_recipient` - Optional fee recipient account
///   - Receives protocol fees from swaps
///   - If not provided, fees are not collected
///
/// * `token_program` - SPL Token program
///   - Required for token operations
///
/// * `system_program` - System program for account management
///   - Required for any account operations
///
/// # Security
///
/// - Authority must sign (enforced by `Signer` constraint)
/// - Token account ownership is validated in instruction
/// - All accounts are validated before swap execution
#[derive(Accounts)]
pub struct BatchSwap<'info> {
    /// The authority (signer) executing the batch swap
    ///
    /// This account must:
    /// - Sign the transaction
    /// - Have sufficient SOL to pay transaction fees
    /// - Own all input token accounts for the swaps
    ///
    /// The authority is the user who wants to execute the batch swap.
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Fee recipient account
    ///
    /// This account receives protocol fees from swaps.
    /// CHECK: Validated in instruction if provided (must be owned by token program)
    #[account(mut)]
    pub fee_recipient: UncheckedAccount<'info>,
    
    /// SPL Token program
    ///
    /// Required for token operations during swaps.
    pub token_program: Program<'info, Token>,
    
    /// System program for account management
    ///
    /// Required for any account operations. This is the standard Solana
    /// system program that handles account creation, transfers, etc.
    pub system_program: Program<'info, System>,
}

/// Account structure for execute swap instruction
///
/// This structure defines all accounts required to execute a single swap.
/// This now supports actual token swaps (different mints) via DEX integration.
///
/// # Accounts
///
/// * `authority` - The signer executing the swap
///   - Must be mutable (may need to pay fees)
///   - Must sign the transaction
///   - Must own the input token account
///
/// * `input_token_account` - Input token account (source)
///   - Must be mutable (tokens will be swapped from here)
///   - Must be a valid SPL token account
///   - Must be owned by the authority
///
/// * `output_token_account` - Output token account (destination)
///   - Must be mutable (tokens will be received here)
///   - Must be a valid SPL token account
///   - Must have a different mint than input account
///
/// * `input_mint` - Input token mint
///   - Mint of the input token
///
/// * `output_mint` - Output token mint
///   - Mint of the output token
///   - Must differ from input_mint
///
/// * `fee_recipient` - Optional fee recipient account
///   - Receives protocol fees
///
/// * `token_program` - SPL Token program
///   - Required for token operations
///
/// # Security
///
/// - Authority must sign (enforced by `Signer` constraint)
/// - Input account ownership is validated
/// - Mint validation ensures different tokens
/// - Slippage protection via min_output_amount parameter
#[derive(Accounts)]
pub struct ExecuteSwap<'info> {
    /// The authority (signer) executing the swap
    ///
    /// This account must:
    /// - Sign the transaction
    /// - Own the input token account
    /// - Have sufficient SOL to pay transaction fees
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// Input token account (source - tokens swapped from)
    ///
    /// This account:
    /// - Must be a valid SPL token account
    /// - Must be owned by the authority
    /// - Must contain sufficient tokens for the swap
    #[account(mut)]
    pub input_token_account: Account<'info, TokenAccount>,
    
    /// Output token account (destination - tokens received)
    ///
    /// This account:
    /// - Must be a valid SPL token account
    /// - Must have a different mint than input account
    /// - Will receive the swapped tokens
    #[account(mut)]
    pub output_token_account: Account<'info, TokenAccount>,
    
    /// Input token mint
    ///
    /// Mint of the input token being swapped.
    /// Used for validation and routing.
    /// CHECK: Validated in instruction
    pub input_mint: AccountInfo<'info>,
    
    /// Output token mint
    ///
    /// Mint of the output token being received.
    /// Must differ from input_mint.
    /// CHECK: Validated in instruction
    pub output_mint: AccountInfo<'info>,
    
    /// Fee recipient account
    ///
    /// Receives protocol fees from the swap.
    /// CHECK: Validated in instruction if provided (must be owned by token program)
    #[account(mut)]
    pub fee_recipient: UncheckedAccount<'info>,
    
    /// SPL Token program
    ///
    /// Required for token operations during the swap.
    pub token_program: Program<'info, Token>,
    
    /// System program
    ///
    /// Required for account operations.
    pub system_program: Program<'info, System>,
}

/// Parameters for a single swap operation
///
/// This structure contains all parameters needed to execute a single swap
/// within a batch. Each swap in a batch will have its own `SwapParams`.
///
/// # Fields
///
/// * `input_mint` - The mint (token type) of the input token
///   - This identifies what token is being swapped from
///   - Must be a valid token mint address
///   - Must differ from `output_mint` (validated in instruction)
///
/// * `output_mint` - The mint (token type) of the output token
///   - This identifies what token is being swapped to
///   - Must be a valid token mint address
///   - Must differ from `input_mint` (validated in instruction)
///
/// * `amount` - Amount of input tokens to swap
///   - Expressed in the token's smallest unit (e.g., lamports for SOL)
///   - Must be >= MIN_SWAP_AMOUNT (1) (validated in instruction)
///   - Should be economically meaningful (not dust)
///
/// * `min_output_amount` - Minimum output amount (slippage protection)
///   - Expressed in the output token's smallest unit
///   - The swap will fail if the output is less than this amount
///   - Must be > 0 (validated in instruction)
///   - Should account for slippage (e.g., 5% slippage tolerance)
///
/// # Example
///
/// ```rust,ignore
/// SwapParams {
///     input_mint: sol_mint,        // SOL mint address
///     output_mint: usdc_mint,      // USDC mint address
///     amount: 1_000_000_000,       // 1 SOL (in lamports)
///     min_output_amount: 90_000_000, // 90 USDC (10% slippage tolerance)
/// }
/// ```
///
/// # Validation
///
/// The following validations are performed:
/// - `amount` >= MIN_SWAP_AMOUNT
/// - `min_output_amount` > 0
/// - `input_mint` != `output_mint`
///
/// # Security Considerations
///
/// - `min_output_amount` provides slippage protection
/// - `amount` must be validated to prevent attacks
/// - Mints must be validated to prevent invalid swaps
///
/// # Future Enhancements
///
/// - Could add deadline for swap execution
/// - Could add route information (which DEX to use)
/// - Could add fee preferences
/// - Could add price oracle information
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SwapParams {
    /// Input token mint (source token)
    ///
    /// This is the mint address of the token being swapped from.
    /// Must be a valid token mint address on Solana.
    ///
    /// # Example
    ///
    /// - SOL: `So11111111111111111111111111111111111111112`
    /// - USDC: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
    /// - Custom token: Any valid SPL token mint address
    pub input_mint: Pubkey,
    
    /// Output token mint (destination token)
    ///
    /// This is the mint address of the token being swapped to.
    /// Must be a valid token mint address on Solana.
    /// Must differ from `input_mint`.
    ///
    /// # Example
    ///
    /// - SOL: `So11111111111111111111111111111111111111112`
    /// - USDC: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
    /// - Custom token: Any valid SPL token mint address
    pub output_mint: Pubkey,
    
    /// Amount of input tokens to swap
    ///
    /// This is the amount of input tokens to swap, expressed in the token's
    /// smallest unit (e.g., lamports for SOL, or the token's decimal base).
    ///
    /// # Example
    ///
    /// - 1 SOL = 1_000_000_000 lamports
    /// - 1 USDC = 1_000_000 (6 decimals)
    /// - 1 Custom token = depends on token decimals
    ///
    /// # Constraints
    ///
    /// - Must be >= MIN_SWAP_AMOUNT (1)
    /// - Should be economically meaningful (not dust)
    /// - Must not exceed account balance
    pub amount: u64,
    
    /// Minimum output amount (for slippage protection)
    ///
    /// This is the minimum amount of output tokens that must be received
    /// for the swap to succeed. If the actual output is less than this amount,
    /// the swap will fail.
    ///
    /// This provides slippage protection, ensuring users don't receive less
    /// than expected due to price movements or liquidity issues.
    ///
    /// # Example
    ///
    /// If swapping 1 SOL for USDC:
    /// - Expected output: 100 USDC
    /// - Slippage tolerance: 5%
    /// - `min_output_amount`: 95 USDC (95% of expected)
    ///
    /// # Constraints
    ///
    /// - Must be > 0
    /// - Should account for expected slippage
    /// - Should be expressed in output token's smallest unit
    ///
    /// # Security
    ///
    /// - Prevents receiving less than expected
    /// - Protects against price manipulation
    /// - Protects against liquidity issues
    pub min_output_amount: u64,
}


