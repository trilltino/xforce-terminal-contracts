//! # Utility Functions
//!
//! This module contains utility functions used throughout the batch swap router program.
//! These functions provide common functionality that can be reused across different
//! parts of the program.
//!
//! ## Utility Functions
//!
//! Currently, this module is a placeholder for future utility functions. As the program
//! evolves, utility functions can be added here for:
//!
//! - Address validation
//! - Amount calculations
//! - Slippage calculations
//! - Fee calculations
//! - Price calculations
//! - Format conversions
//!
//! ## Future Enhancements
//!
//! - Add address validation utilities
//! - Add amount conversion utilities
//! - Add slippage calculation utilities
//! - Add fee calculation utilities
//! - Add price calculation utilities

use anchor_lang::prelude::*;

// This module is currently a placeholder for future utility functions.
// As the program evolves, utility functions can be added here.

/// Validate a public key address
///
/// This function validates that a public key is not the default/null public key.
/// In Solana, the Pubkey type already ensures the address is a valid 32-byte array,
/// so we only need to check for the default/null address.
///
/// # Arguments
///
/// * `address` - The public key to validate
///
/// # Returns
///
/// * `bool` - Returns `true` if the address is valid, `false` otherwise
///
/// # Example
///
/// ```rust,ignore
/// let valid_key = Pubkey::new_unique();
/// assert!(is_valid_address(&valid_key));
///
/// let default_key = Pubkey::default();
/// assert!(!is_valid_address(&default_key));
/// ```
pub fn is_valid_address(address: &Pubkey) -> bool {
    // Check that the address is not the default/null public key
    // The default key is all zeros, which is invalid for actual usage
    *address != Pubkey::default()
}

/// Calculate slippage percentage
///
/// This function calculates the slippage percentage between expected and actual amounts.
/// Slippage is calculated as: ((expected - actual) / expected) * 10000 (basis points)
///
/// # Arguments
///
/// * `expected` - The expected amount
/// * `actual` - The actual amount received
///
/// # Returns
///
/// * `Option<u64>` - Returns the slippage percentage (in basis points), or `None` if calculation fails
///
/// # Basis Points
///
/// - 1 basis point = 0.01%
/// - 100 basis points = 1%
/// - 10000 basis points = 100%
///
/// # Edge Cases
///
/// - Returns `None` if expected is 0 (division by zero)
/// - Returns `None` if actual > expected (negative slippage, which is positive)
/// - Returns 0 if expected == actual (no slippage)
///
/// # Example
///
/// ```rust,ignore
/// // Expected 100, got 95 -> 5% slippage = 500 basis points
/// let slippage = calculate_slippage(100, 95);
/// assert_eq!(slippage, Some(500));
///
/// // Expected 1000, got 990 -> 1% slippage = 100 basis points
/// let slippage = calculate_slippage(1000, 990);
/// assert_eq!(slippage, Some(100));
/// ```
pub fn calculate_slippage(expected: u64, actual: u64) -> Option<u64> {
    // Handle edge cases
    if expected == 0 {
        // Division by zero - cannot calculate slippage
        return None;
    }
    
    if actual >= expected {
        // No slippage or negative slippage (better than expected)
        return Some(0);
    }
    
    // Calculate slippage: ((expected - actual) / expected) * 10000
    // We use checked arithmetic to prevent overflow
    let difference = expected.checked_sub(actual)?;
    
    // Multiply by 10000 first to maintain precision, then divide
    // This avoids floating point arithmetic and maintains integer precision
    let slippage_bps = (difference as u128)
        .checked_mul(10000)?
        .checked_div(expected as u128)?;
    
    // Convert back to u64 (slippage_bps should always fit in u64 since it's at most 10000)
    u64::try_from(slippage_bps).ok()
}

/// Validate slippage tolerance
///
/// This function validates that the actual amount received is within the
/// acceptable slippage tolerance. It calculates the minimum acceptable amount
/// based on the tolerance and compares it to the actual amount.
///
/// # Arguments
///
/// * `expected` - The expected amount
/// * `actual` - The actual amount received
/// * `tolerance_bps` - The slippage tolerance in basis points (1 basis point = 0.01%)
///
/// # Returns
///
/// * `bool` - Returns `true` if slippage is within tolerance, `false` otherwise
///
/// # Tolerance Calculation
///
/// The minimum acceptable amount is calculated as:
/// `min_amount = expected * (10000 - tolerance_bps) / 10000`
///
/// # Edge Cases
///
/// - Returns `true` if expected is 0 (cannot validate)
/// - Returns `true` if actual >= expected (better than expected)
/// - Returns `false` if tolerance_bps >= 10000 (100% tolerance is invalid)
///
/// # Example
///
/// ```rust,ignore
/// // Expected 100, tolerance 5% (500 bps), actual 96 -> within tolerance
/// assert!(is_slippage_acceptable(100, 96, 500));
///
/// // Expected 100, tolerance 5% (500 bps), actual 94 -> outside tolerance
/// assert!(!is_slippage_acceptable(100, 94, 500));
///
/// // Expected 1000, tolerance 1% (100 bps), actual 995 -> within tolerance
/// assert!(is_slippage_acceptable(1000, 995, 100));
/// ```
pub fn is_slippage_acceptable(expected: u64, actual: u64, tolerance_bps: u64) -> bool {
    // Handle edge cases
    if expected == 0 {
        // Cannot validate zero expected amount
        return true;
    }
    
    if actual >= expected {
        // Better than expected - always acceptable
        return true;
    }
    
    if tolerance_bps >= 10000 {
        // Invalid tolerance (>= 100%)
        return false;
    }
    
    // Calculate minimum acceptable amount
    // min_amount = expected * (10000 - tolerance_bps) / 10000
    let multiplier = match 10000u64.checked_sub(tolerance_bps) {
        Some(m) => m,
        None => return false,
    };
    
    let min_amount = match (expected as u128)
        .checked_mul(multiplier as u128)
        .and_then(|v| v.checked_div(10000u128))
    {
        Some(amount) => amount,
        None => return false,
    };
    
    // Compare actual to minimum acceptable amount
    actual >= min_amount as u64
}


