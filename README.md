# XForce Terminal Contracts

Solana smart contracts for the XForce Terminal platform. Provides batch token swap functionality with security and efficiency optimizations.

## Overview

The XForce Terminal Contracts enable atomic batch execution of token swaps on Solana. By bundling multiple swaps into a single transaction, users save on fees and ensure all operations succeed or fail together.

## Program

### Batch Swap Router

Execute multiple swaps atomically in one transaction.

**Key Features:**
- Batch up to 10 swaps per transaction
- Jupiter aggregator integration for optimal routing
- Slippage protection with configurable minimums
- Protocol fee management
- Atomic execution guarantees

**Devnet Deployment:**
- Program ID: `HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx`
- Explorer: https://explorer.solana.com/address/HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx?cluster=devnet

## Project Structure

```
xforce-terminal-contracts/
├── programs/
│   └── batch-swap-router/    # Anchor program
├── client/                     # Rust client library
├── examples/                   # Usage examples
├── tests/                      # Integration tests
└── docs/                       # Documentation
```

## Quick Start

### Prerequisites
- Rust 1.70+
- Solana CLI 1.17+
- Anchor 0.30+

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Deploy

```bash
# Devnet
anchor deploy --provider.cluster devnet

# Mainnet (requires keypair with SOL)
anchor deploy --provider.cluster mainnet
```

## Usage

### Rust Client

```rust
use xforce_terminal_contracts_client::BatchSwapRouterClient;

let client = BatchSwapRouterClient::new(
    program_id,
    payer,
    rpc_client,
);

let swaps = vec![SwapParams {
    input_mint: sol_mint,
    output_mint: usdc_mint,
    amount: 1_000_000_000,      // 1 SOL
    min_output_amount: 90_000_000,  // Min 90 USDC
}];

let signature = client.batch_swap(swaps).await?;
```

### TypeScript Client

```typescript
import { BatchSwapRouter } from './client';

const router = new BatchSwapRouter(program, provider);

const swaps = [{
    inputMint: SOL_MINT,
    outputMint: USDC_MINT,
    amount: new BN(1_000_000_000),
    minOutputAmount: new BN(90_000_000),
}];

const tx = await router.batchSwap(swaps);
await provider.sendAndConfirm(tx);
```

## Architecture

### Instructions

1. **`batch_swap`** - Execute multiple swaps in one transaction
   - Maximum 10 swaps per batch
   - Atomic execution
   - Fee calculation per swap

2. **`execute_swap`** - Execute single swap
   - Token swap execution
   - Slippage protection
   - Fee distribution

### Security

- Input validation on all parameters
- Account ownership verification
- Amount limits prevent dust attacks
- Batch size limits prevent DoS
- Slippage protection
- Atomic execution

## Documentation

- [Program README](programs/batch-swap-router/README.md) - Detailed program documentation
- [Client README](client/README.md) - Client library documentation
- [Security](docs/SECURITY.md) - Security considerations
- [MVP Summary](docs/MVP_SUMMARY.md) - Project overview

## License

MIT - See [LICENSE](LICENSE) for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests for new features
4. Ensure all tests pass
5. Submit a pull request
