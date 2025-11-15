# Batch Swap Router Program

A Solana program that enables batch execution of token swaps in a single transaction. This program allows users to execute multiple swaps atomically, reducing transaction fees and improving user experience.

## Overview

The Batch Swap Router is designed to address the common problem of executing multiple token swaps which would normally require multiple transactions. By batching swaps into a single transaction, users can:

- **Reduce Fees**: Pay transaction fees once instead of multiple times
- **Atomic Execution**: All swaps succeed or fail together
- **Better UX**: Execute complex swap strategies in one transaction
- **Slippage Protection**: Validate slippage for each swap
- **Fee Management**: Calculate and distribute protocol fees

## Architecture

The program is structured in a modular fashion following Anchor best practices:

```
lib.rs                    # Main program entry point
├── constants.rs          # Program constants (limits, minimums, fees)
├── errors.rs             # Error definitions
├── events.rs             # Event definitions
├── state.rs              # Account structures and state types
├── utils.rs              # Utility functions
├── swap_execution.rs     # Swap execution logic
└── instructions/         # Instruction handlers
    ├── mod.rs           # Instruction module
    ├── batch_swap.rs    # Batch swap instruction
    └── execute_swap.rs  # Single swap instruction
```

## Instructions

### `batch_swap`

Execute multiple swaps in a single transaction. This is the primary instruction that enables fee reduction by batching multiple operations.

**Features**:
- Maximum 10 swaps per batch
- Atomic execution (all or nothing)
- Comprehensive validation
- Fee calculation and tracking
- Event emission for tracking

### `execute_swap`

Execute a single token swap with slippage protection and fee calculation. This instruction performs actual token swaps between different mints.

**Features**:
- Token swap execution (different mints)
- Slippage validation
- Fee calculation and distribution
- Account validation
- Authority verification
- Event emission

## Security Considerations

- All inputs are validated before processing
- Account ownership is verified
- Amount limits prevent dust attacks
- Batch size limits prevent DoS attacks
- Atomic execution prevents partial failures
- Slippage protection prevents unfavorable swaps
- Fee calculation is transparent and auditable

## Usage

### Batch Swap

```rust
// Execute batch swap
batch_swap(ctx, vec![
    SwapParams {
        input_mint: sol_mint,
        output_mint: usdc_mint,
        amount: 1_000_000_000, // 1 SOL
        min_output_amount: 90_000_000, // 90 USDC (10% slippage)
    },
])?;
```

### Single Swap

```rust
// Execute single swap
execute_swap(
    ctx,
    1_000_000_000,  // Input amount: 1 SOL
    90_000_000,     // Min output: 90 USDC
    95_000_000,     // Expected output: 95 USDC (from Jupiter quote)
)?;
```

## Events

The program emits events for tracking and indexing:

- `BatchSwapEvent` - Emitted when a batch swap is executed
  - Contains: authority, swap_count, total_input_amount, total_protocol_fees, timestamp

- `SwapExecutedEvent` - Emitted when a single swap is executed
  - Contains: authority, input_amount, output_amount, input_mint, output_mint, protocol_fee, slippage_bps, timestamp

## Error Handling

All errors are defined in the `ErrorCode` enum and provide descriptive error messages for debugging and user feedback. Common errors include:

- `EmptySwaps` - No swaps provided in batch
- `TooManySwaps` - Batch exceeds maximum size
- `InvalidAmount` - Invalid swap amount
- `SlippageExceeded` - Slippage tolerance exceeded
- `SwapExecutionFailed` - Swap execution failed

## Integration

This program integrates with:

- **Jupiter Aggregator**: For DEX routing and swap execution (client-side)
- **SPL Token Program**: For token operations
- **System Program**: For account management

## Building

```bash
# Build the program
anchor build

# Run tests
anchor test

# Deploy (after configuring Anchor.toml)
anchor deploy
```

## License

This program is licensed under the MIT License.

