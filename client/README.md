# Client

Rust client library for interacting with XForce Terminal Contracts.

## Features

- Type-safe program interactions
- Account building helpers
- Transaction construction
- Error handling

## Modules

### lib.rs
Main client interface and entry point.

### batch_swap_router.rs
Batch swap router program interactions.

### types.rs
Client-specific types and structures.

### error.rs
Client error types.

### security.rs
Security verification helpers.

## Usage

```rust
use xforce_terminal_contracts_client::BatchSwapRouterClient;

// Initialize client
let client = BatchSwapRouterClient::new(
    program_id,
    payer_keypair,
    rpc_client,
)?;

// Build swap parameters
let swaps = vec![SwapParams {
    input_mint: sol_mint,
    output_mint: usdc_mint,
    amount: 1_000_000_000,
    min_output_amount: 90_000_000,
}];

// Execute batch swap
let signature = client.batch_swap(swaps).await?;
println!("Transaction: {}", signature);
```

## Dependencies

- `solana-client`: RPC client
- `solana-sdk`: Core types
- `anchor-client`: Anchor framework
- `spl-token`: SPL token program

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```
