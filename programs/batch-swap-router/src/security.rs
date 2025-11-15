//! # Security Module
//!
//! This module contains security utilities and validation functions for the batch swap router program.
//! It provides safe math operations, account validation, and security assertions to protect against
//! common vulnerabilities in Solana programs.
//!
//! ## Security Features
//!
//! - **Safe Math**: Overflow/underflow protection for all arithmetic operations
//! - **Account Validation**: Comprehensive account ownership and state validation
//! - **Input Validation**: Parameter validation to prevent malicious inputs
//! - **Access Control**: Signer and authority validation
//! - **Reentrancy Protection**: Patterns to prevent reentrancy attacks
//!
//! ## Common Vulnerabilities Addressed
//!
//! - Integer overflow/underflow
//! - Account ownership attacks
//! - Invalid account data
//! - Unauthorized access
//! - Slippage attacks
//! - Dust attacks
//! - DoS attacks
//!
//! ## Usage
//!
//! ```rust,ignore
//! use crate::security::*;
//!
//! // Safe math operations
//! let result = amount.safe_add(other_amount)?;
//! let fee = amount.safe_mul(PROTOCOL_FEE_BPS)?.safe_div(10000)?;
//!
//! // Account validation
//! assert_signer(&account_info)?;
//! assert_owned_by(&account_info, &expected_owner)?;
//! assert_token_account(&token_account, &expected_mint)?;
//! ```

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::errors::ErrorCode;

// ============================================================================
// Safe Math Operations
// ============================================================================

/// Safe math trait for overflow/underflow protection
///
/// This trait provides safe arithmetic operations that return errors on overflow
/// or underflow, preventing integer overflow/underflow vulnerabilities.
pub trait SafeMath {
    /// Safe addition with overflow check
    fn safe_add(self, rhs: Self) -> Result<Self>
    where
        Self: Sized;

    /// Safe subtraction with underflow check
    fn safe_sub(self, rhs: Self) -> Result<Self>
    where
        Self: Sized;

    /// Safe multiplication with overflow check
    fn safe_mul(self, rhs: Self) -> Result<Self>
    where
        Self: Sized;

    /// Safe division with zero check
    fn safe_div(self, rhs: Self) -> Result<Self>
    where
        Self: Sized;
}

impl SafeMath for u64 {
    fn safe_add(self, rhs: Self) -> Result<Self> {
        self.checked_add(rhs)
            .ok_or_else(|| ErrorCode::MathOverflow.into())
    }

    fn safe_sub(self, rhs: Self) -> Result<Self> {
        self.checked_sub(rhs)
            .ok_or_else(|| ErrorCode::MathOverflow.into())
    }

    fn safe_mul(self, rhs: Self) -> Result<Self> {
        self.checked_mul(rhs)
            .ok_or_else(|| ErrorCode::MathOverflow.into())
    }

    fn safe_div(self, rhs: Self) -> Result<Self> {
        if rhs == 0 {
            return Err(ErrorCode::MathOverflow.into());
        }
        self.checked_div(rhs)
            .ok_or_else(|| ErrorCode::MathOverflow.into())
    }
}

impl SafeMath for u128 {
    fn safe_add(self, rhs: Self) -> Result<Self> {
        self.checked_add(rhs)
            .ok_or_else(|| ErrorCode::MathOverflow.into())
    }

    fn safe_sub(self, rhs: Self) -> Result<Self> {
        self.checked_sub(rhs)
            .ok_or_else(|| ErrorCode::MathOverflow.into())
    }

    fn safe_mul(self, rhs: Self) -> Result<Self> {
        self.checked_mul(rhs)
            .ok_or_else(|| ErrorCode::MathOverflow.into())
    }

    fn safe_div(self, rhs: Self) -> Result<Self> {
        if rhs == 0 {
            return Err(ErrorCode::MathOverflow.into());
        }
        self.checked_div(rhs)
            .ok_or_else(|| ErrorCode::MathOverflow.into())
    }
}

// ============================================================================
// Account Validation
// ============================================================================

/// Assert that an account is a signer
///
/// # Arguments
///
/// * `account_info` - The account info to check
///
/// # Errors
///
/// Returns `ErrorCode::InvalidAuthority` if the account is not a signer
pub fn assert_signer(account_info: &AccountInfo) -> Result<()> {
    require!(account_info.is_signer, ErrorCode::InvalidAuthority);
    Ok(())
}

/// Assert that an account is owned by a specific program
///
/// # Arguments
///
/// * `account_info` - The account info to check
/// * `expected_owner` - The expected owner program ID
///
/// # Errors
///
/// Returns `ErrorCode::InvalidAccount` if ownership doesn't match
pub fn assert_owned_by(account_info: &AccountInfo, expected_owner: &Pubkey) -> Result<()> {
    require!(
        account_info.owner == expected_owner,
        ErrorCode::InvalidAccount
    );
    Ok(())
}

/// Assert that two public keys are equal
///
/// # Arguments
///
/// * `key1` - First public key
/// * `key2` - Second public key
///
/// # Errors
///
/// Returns `ErrorCode::InvalidAccount` if keys don't match
pub fn assert_keys_equal(key1: &Pubkey, key2: &Pubkey) -> Result<()> {
    require!(key1 == key2, ErrorCode::InvalidAccount);
    Ok(())
}

/// Assert that a token account has the expected mint
///
/// # Arguments
///
/// * `token_account` - The token account to check
/// * `expected_mint` - The expected mint address
///
/// # Errors
///
/// Returns `ErrorCode::InvalidAccount` if mint doesn't match
pub fn assert_token_account_mint(
    token_account: &Account<TokenAccount>,
    expected_mint: &Pubkey,
) -> Result<()> {
    require!(
        token_account.mint == *expected_mint,
        ErrorCode::InvalidAccount
    );
    Ok(())
}

/// Assert that a token account is owned by a specific authority
///
/// # Arguments
///
/// * `token_account` - The token account to check
/// * `expected_owner` - The expected owner address
///
/// # Errors
///
/// Returns `ErrorCode::InvalidAuthority` if ownership doesn't match
pub fn assert_token_account_owner(
    token_account: &Account<TokenAccount>,
    expected_owner: &Pubkey,
) -> Result<()> {
    require!(
        token_account.owner == *expected_owner,
        ErrorCode::InvalidAuthority
    );
    Ok(())
}

/// Assert that a public key is not the default/null key
///
/// # Arguments
///
/// * `key` - The public key to check
///
/// # Errors
///
/// Returns `ErrorCode::InvalidAccount` if key is default
pub fn assert_not_default(key: &Pubkey) -> Result<()> {
    require!(*key != Pubkey::default(), ErrorCode::InvalidAccount);
    Ok(())
}

/// Assert that an account has sufficient balance
///
/// # Arguments
///
/// * `account_info` - The account info to check
/// * `min_balance` - The minimum required balance
///
/// # Errors
///
/// Returns `ErrorCode::InsufficientFunds` if balance is insufficient
pub fn assert_sufficient_balance(account_info: &AccountInfo, min_balance: u64) -> Result<()> {
    require!(
        account_info.lamports() >= min_balance,
        ErrorCode::InsufficientFunds
    );
    Ok(())
}

/// Assert that a token account has sufficient balance
///
/// # Arguments
///
/// * `token_account` - The token account to check
/// * `min_amount` - The minimum required token amount
///
/// # Errors
///
/// Returns `ErrorCode::InsufficientFunds` if balance is insufficient
pub fn assert_sufficient_token_balance(
    token_account: &Account<TokenAccount>,
    min_amount: u64,
) -> Result<()> {
    require!(
        token_account.amount >= min_amount,
        ErrorCode::InsufficientFunds
    );
    Ok(())
}

// ============================================================================
// Input Validation
// ============================================================================

/// Assert that an amount is within valid bounds
///
/// # Arguments
///
/// * `amount` - The amount to validate
/// * `min_amount` - The minimum allowed amount
/// * `max_amount` - The maximum allowed amount (optional)
///
/// # Errors
///
/// Returns `ErrorCode::InvalidAmount` if amount is out of bounds
pub fn assert_amount_in_bounds(
    amount: u64,
    min_amount: u64,
    max_amount: Option<u64>,
) -> Result<()> {
    require!(amount >= min_amount, ErrorCode::InvalidAmount);

    if let Some(max) = max_amount {
        require!(amount <= max, ErrorCode::InvalidAmount);
    }

    Ok(())
}

/// Assert that a slippage tolerance is valid
///
/// # Arguments
///
/// * `slippage_bps` - The slippage tolerance in basis points
/// * `max_slippage_bps` - The maximum allowed slippage tolerance
///
/// # Errors
///
/// Returns `ErrorCode::SlippageExceeded` if slippage is too high
pub fn assert_valid_slippage(slippage_bps: u64, max_slippage_bps: u64) -> Result<()> {
    require!(
        slippage_bps <= max_slippage_bps,
        ErrorCode::SlippageExceeded
    );
    Ok(())
}

/// Assert that two mints are different
///
/// # Arguments
///
/// * `mint1` - First mint address
/// * `mint2` - Second mint address
///
/// # Errors
///
/// Returns `ErrorCode::InvalidSwapPair` if mints are the same
pub fn assert_different_mints(mint1: &Pubkey, mint2: &Pubkey) -> Result<()> {
    require!(mint1 != mint2, ErrorCode::InvalidSwapPair);
    Ok(())
}

// ============================================================================
// Security Helpers
// ============================================================================

/// Calculate fee with safe math
///
/// # Arguments
///
/// * `amount` - The amount to calculate fee for
/// * `fee_bps` - The fee in basis points
///
/// # Returns
///
/// * `Result<u64>` - The calculated fee amount
///
/// # Errors
///
/// Returns `ErrorCode::MathOverflow` if calculation overflows
pub fn calculate_fee_safe(amount: u64, fee_bps: u64) -> Result<u64> {
    // Convert to u128 for intermediate calculation to avoid overflow
    let amount_u128 = amount as u128;
    let fee_bps_u128 = fee_bps as u128;

    // Calculate fee: (amount * fee_bps) / 10000
    let fee = amount_u128
        .safe_mul(fee_bps_u128)?
        .safe_div(10000u128)?;

    // Convert back to u64
    u64::try_from(fee).map_err(|_| ErrorCode::MathOverflow.into())
}

/// Validate that actual output meets minimum requirement with safe math
///
/// # Arguments
///
/// * `actual_output` - The actual output amount received
/// * `min_output` - The minimum required output amount
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if validation passes
///
/// # Errors
///
/// Returns `ErrorCode::SlippageExceeded` if actual output is less than minimum
pub fn validate_min_output(actual_output: u64, min_output: u64) -> Result<()> {
    require!(
        actual_output >= min_output,
        ErrorCode::SlippageExceeded
    );
    Ok(())
}

/// Calculate amount after fee with safe math
///
/// # Arguments
///
/// * `amount` - The original amount
/// * `fee` - The fee to subtract
///
/// # Returns
///
/// * `Result<u64>` - The amount after fee
///
/// # Errors
///
/// Returns `ErrorCode::MathOverflow` if calculation underflows
pub fn amount_after_fee(amount: u64, fee: u64) -> Result<u64> {
    amount.safe_sub(fee)
}

/// Validate that amount after fee is sufficient
///
/// # Arguments
///
/// * `amount` - The original amount
/// * `fee` - The fee to subtract
/// * `min_amount` - The minimum required amount after fee
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if validation passes
///
/// # Errors
///
/// Returns `ErrorCode::InsufficientOutput` if amount after fee is insufficient
pub fn validate_amount_after_fee(amount: u64, fee: u64, min_amount: u64) -> Result<()> {
    let amount_after = amount_after_fee(amount, fee)?;
    require!(amount_after >= min_amount, ErrorCode::InsufficientOutput);
    Ok(())
}

