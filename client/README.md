# XForce Terminal Contracts Client

Rust client library for interacting with XForce Terminal Solana smart contracts. This client provides a high-level, type-safe interface for interacting with the batch swap router program.

## Overview

The XForce Terminal Contracts Client is a Rust library that simplifies interaction with the batch swap router program on Solana. It provides:

- **Type-safe APIs**: Compile-time type checking for all operations
- **Error Handling**: Comprehensive error handling with detailed error messages
- **Transaction Management**: Simplified transaction creation and submission
- **Account Management**: Helper functions for account creation and management

## Architecture

```
lib.rs                    # Main library entry point
├── batch_swap_router.rs  # Batch swap router client
├── error.rs              # Error definitions
└── types.rs              # Type definitions
```

## Usage

### Basic Usage

```rust
use xforce_terminal_contracts_client::*;
use anchor_client::Client;
use solana_sdk::signature::Keypair;

// Create a client
let payer = Keypair::new();
let client = create_client("http://localhost:8899", payer)?;

// Get the program
let program_id = get_batch_swap_router_program_id();
let program = client.program(program_id)?;

// Create batch swap router client
let swap_client = BatchSwapRouterClient::new(program);

// Execute a batch swap
let swaps = vec![
    SwapParams {
        input_mint: mint_a,
        output_mint: mint_b,
        amount: 1000,
        min_output_amount: 900,
    },
];

let signature = swap_client.batch_swap(swaps)?;
```

### Error Handling

```rust
use xforce_terminal_contracts_client::ContractError;

match swap_client.batch_swap(swaps) {
    Ok(signature) => println!("Transaction: {}", signature),
    Err(ContractError::TransactionFailed(msg)) => {
        eprintln!("Transaction failed: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Features

- **Type Safety**: Compile-time type checking for all operations
- **Error Handling**: Comprehensive error handling with detailed messages
- **Transaction Management**: Simplified transaction creation and submission
- **Account Management**: Helper functions for account operations

## Requirements

- Rust 1.70.0 or later
- Anchor framework (for program compilation)
- Solana CLI tools (for local development)

## Building

```bash
# Build the client
cargo build

# Run tests
cargo test

# Build documentation
cargo doc --open
```

## License

This client is licensed under the MIT License.

