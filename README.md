# XForce Terminal - Batch Swap Router Contracts

Production-grade Solana smart contracts for batch token swaps with Jupiter integration.

## Overview

Batch swap router that enables users to execute multiple token swaps atomically in a single transaction, reducing fees and improving user experience.

## Features

- Batch swap execution (up to 10 swaps per transaction)
- Jupiter integration for best-price routing
- Slippage protection and validation
- Fee management and distribution
- Event emission for tracking
- Comprehensive security validations

## Program ID

**Devnet**: `HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx`

[View on Solana Explorer](https://explorer.solana.com/address/HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx?cluster=devnet)

## Quick Start

### Prerequisites

- Rust (latest stable)
- Solana CLI (v3.0+)
- Anchor (v0.32.1)

### Installation

```bash
git clone https://github.com/trilltino/xforce-terminal-contracts.git
cd xforce-terminal-contracts
npm install
anchor build
anchor test
```

### Deployment

```bash
anchor deploy --provider.cluster devnet
```

## Usage

### Batch Swap

Execute multiple swaps in a single transaction:

```rust
use xforce_terminal_contracts_client::BatchSwapRouterClient;
use xforce_terminal_contracts_client::SwapParams;

let swaps = vec![
    SwapParams {
        input_mint: sol_mint,
        output_mint: usdc_mint,
        amount: 1_000_000_000,
        min_output_amount: 90_000_000,
    },
];

let signature = client.batch_swap(swaps)?;
```

## Architecture

- **Program**: Solana smart contract (Anchor framework)
- **Client Library**: Rust client for integration
- **Jupiter Integration**: Multi-DEX routing for optimal prices
- **Backend API**: Transaction building and validation

## Documentation

- [Security Documentation](docs/SECURITY.md)
- [MVP Summary](docs/MVP_SUMMARY.md)

## License

MIT License - Free and Open Source Software

See [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome. Fork the repository, create a feature branch, and submit a pull request.

## Acknowledgments

- Jupiter Aggregator for DEX routing
- Anchor Framework for Solana development
- Solana Foundation
