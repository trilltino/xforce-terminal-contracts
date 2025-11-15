//! # Client Security Module
//!
//! This module contains security utilities and validation functions for the client library.
//! It provides input validation, parameter checking, and security assertions to protect
//! against common vulnerabilities when interacting with Solana programs.
//!
//! ## Security Features
//!
//! - **Input Validation**: Validate all user inputs before sending transactions
//! - **Parameter Validation**: Check parameters for valid ranges and values
//! - **Address Validation**: Verify public keys are valid and not default/null
//! - **Amount Validation**: Ensure amounts are within safe bounds
//! - **Slippage Validation**: Validate slippage tolerances
//!
//! ## Usage
//!
//! ```rust,ignore
//! use xforce_terminal_contracts_client::security::*;
//!
//! // Validate swap parameters before sending transaction
//! validate_swap_params(&swap_params)?;
//!
//! // Validate public key
//! assert_valid_pubkey(&pubkey)?;
//!
//! // Validate amount
//! assert_valid_amount(amount, min_amount, max_amount)?;
//! ```

use solana_sdk::pubkey::Pubkey;
use crate::error::ContractError;
use crate::types::SwapParams;

/// Validate that a public key is not the default/null key
///
/// # Arguments
///
/// * `pubkey` - The public key to validate
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if valid, error otherwise
///
/// # Errors
///
/// Returns `ContractError::InvalidAccount` if the key is default/null
pub fn assert_valid_pubkey(pubkey: &Pubkey) -> Result<(), ContractError> {
    if *pubkey == Pubkey::default() {
        return Err(ContractError::InvalidAccount(
            "Public key cannot be default/null".to_string(),
        ));
    }
    Ok(())
}

/// Validate that an amount is within valid bounds
///
/// # Arguments
///
/// * `amount` - The amount to validate
/// * `min_amount` - The minimum allowed amount
/// * `max_amount` - The maximum allowed amount (optional)
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if valid, error otherwise
///
/// # Errors
///
/// Returns `ContractError::InvalidAccount` if amount is out of bounds
pub fn assert_valid_amount(
    amount: u64,
    min_amount: u64,
    max_amount: Option<u64>,
) -> Result<(), ContractError> {
    if amount < min_amount {
        return Err(ContractError::InvalidAccount(format!(
            "Amount {} is below minimum {}",
            amount, min_amount
        )));
    }

    if let Some(max) = max_amount {
        if amount > max {
            return Err(ContractError::InvalidAccount(format!(
                "Amount {} exceeds maximum {}",
                amount, max
            )));
        }
    }

    Ok(())
}

/// Validate that two public keys are different
///
/// # Arguments
///
/// * `key1` - First public key
/// * `key2` - Second public key
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if keys are different, error otherwise
///
/// # Errors
///
/// Returns `ContractError::InvalidAccount` if keys are the same
pub fn assert_different_pubkeys(key1: &Pubkey, key2: &Pubkey) -> Result<(), ContractError> {
    if key1 == key2 {
        return Err(ContractError::InvalidAccount(
            "Public keys must be different".to_string(),
        ));
    }
    Ok(())
}

/// Validate swap parameters
///
/// # Arguments
///
/// * `params` - The swap parameters to validate
/// * `min_amount` - The minimum allowed swap amount
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if valid, error otherwise
///
/// # Errors
///
/// Returns `ContractError::InvalidAccount` if parameters are invalid
pub fn validate_swap_params(params: &SwapParams, min_amount: u64) -> Result<(), ContractError> {
    // Validate input mint
    assert_valid_pubkey(&params.input_mint)?;

    // Validate output mint
    assert_valid_pubkey(&params.output_mint)?;

    // Validate mints are different
    assert_different_pubkeys(&params.input_mint, &params.output_mint)?;

    // Validate amount
    assert_valid_amount(params.amount, min_amount, None)?;

    // Validate minimum output
    if params.min_output_amount == 0 {
        return Err(ContractError::InvalidAccount(
            "Minimum output amount must be greater than 0".to_string(),
        ));
    }

    Ok(())
}

/// Validate slippage tolerance
///
/// # Arguments
///
/// * `slippage_bps` - The slippage tolerance in basis points
/// * `max_slippage_bps` - The maximum allowed slippage tolerance
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if valid, error otherwise
///
/// # Errors
///
/// Returns `ContractError::InvalidAccount` if slippage is too high
pub fn assert_valid_slippage(
    slippage_bps: u64,
    max_slippage_bps: u64,
) -> Result<(), ContractError> {
    if slippage_bps > max_slippage_bps {
        return Err(ContractError::InvalidAccount(format!(
            "Slippage {} bps exceeds maximum {} bps",
            slippage_bps, max_slippage_bps
        )));
    }
    Ok(())
}

/// Calculate slippage in basis points
///
/// # Arguments
///
/// * `expected` - The expected amount
/// * `actual` - The actual amount received
///
/// # Returns
///
/// * `Option<u64>` - The slippage in basis points, or None if calculation fails
pub fn calculate_slippage_bps(expected: u64, actual: u64) -> Option<u64> {
    if expected == 0 {
        return None;
    }

    if actual >= expected {
        return Some(0);
    }

    let difference = expected.checked_sub(actual)?;
    let slippage_bps = (difference as u128)
        .checked_mul(10000)?
        .checked_div(expected as u128)?;

    u64::try_from(slippage_bps).ok()
}

/// Validate batch size
///
/// # Arguments
///
/// * `batch_size` - The batch size to validate
/// * `max_batch_size` - The maximum allowed batch size
///
/// # Returns
///
/// * `Result<()>` - Returns Ok if valid, error otherwise
///
/// # Errors
///
/// Returns `ContractError::InvalidAccount` if batch size is invalid
pub fn assert_valid_batch_size(batch_size: usize, max_batch_size: usize) -> Result<(), ContractError> {
    if batch_size == 0 {
        return Err(ContractError::InvalidAccount(
            "Batch size cannot be zero".to_string(),
        ));
    }

    if batch_size > max_batch_size {
        return Err(ContractError::InvalidAccount(format!(
            "Batch size {} exceeds maximum {}",
            batch_size, max_batch_size
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_valid_pubkey() {
        let valid_key = Pubkey::new_unique();
        assert!(assert_valid_pubkey(&valid_key).is_ok());

        let default_key = Pubkey::default();
        assert!(assert_valid_pubkey(&default_key).is_err());
    }

    #[test]
    fn test_assert_valid_amount() {
        assert!(assert_valid_amount(100, 1, None).is_ok());
        assert!(assert_valid_amount(100, 1, Some(200)).is_ok());
        assert!(assert_valid_amount(100, 1, Some(50)).is_err());
        assert!(assert_valid_amount(0, 1, None).is_err());
    }

    #[test]
    fn test_assert_different_pubkeys() {
        let key1 = Pubkey::new_unique();
        let key2 = Pubkey::new_unique();
        assert!(assert_different_pubkeys(&key1, &key2).is_ok());
        assert!(assert_different_pubkeys(&key1, &key1).is_err());
    }

    #[test]
    fn test_calculate_slippage_bps() {
        assert_eq!(calculate_slippage_bps(100, 95), Some(500)); // 5% slippage
        assert_eq!(calculate_slippage_bps(100, 100), Some(0)); // No slippage
        assert_eq!(calculate_slippage_bps(100, 105), Some(0)); // Better than expected
        assert_eq!(calculate_slippage_bps(0, 100), None); // Division by zero
    }
}

