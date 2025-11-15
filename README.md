# XForce Terminal - Batch Swap Router Contracts

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![FOSS](https://img.shields.io/badge/FOSS-Yes-brightgreen.svg)](LICENSE)
[![Solana](https://img.shields.io/badge/Solana-Devnet-purple.svg)](https://explorer.solana.com/address/HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx?cluster=devnet)
[![Anchor](https://img.shields.io/badge/Anchor-0.32.1-blue.svg)](https://www.anchor-lang.com/)

**A production-grade, open-source Solana smart contract for batch token swaps with Jupiter integration**

## 🌟 Overview

XForce Terminal is a **top-tier, fully open-source** Solana DeFi trading platform. This repository contains the smart contracts powering the batch swap functionality, enabling users to execute multiple token swaps atomically in a single transaction.

### Key Highlights

- ✅ **100% Open Source** - MIT Licensed, free for everyone
- ✅ **Production Ready** - Deployed on Solana Devnet
- ✅ **Jupiter Integration** - Best price routing across all DEXes
- ✅ **Batch Swaps** - Execute up to 10 swaps in one transaction
- ✅ **Slippage Protection** - Built-in validation and safety checks
- ✅ **Fee Management** - Transparent protocol fee calculation
- ✅ **Event Emission** - Full on-chain tracking and indexing

## 🚀 Features

### Batch Swap Router Program

- **Atomic Execution**: All swaps succeed or fail together
- **Fee Reduction**: Pay transaction fees once for multiple swaps
- **Slippage Protection**: Validate minimum output amounts
- **Jupiter Integration**: Automatic best-price routing
- **Event Tracking**: Emit events for analytics and indexing
- **Security First**: Comprehensive validation and safe math operations

## 📋 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              XForce Terminal Application                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   egui UI    │  │  Backend API │  │  Wallet SDK   │     │
│  │  (Desktop)   │  │   (Axum)     │  │  (Signing)   │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
└─────────┼──────────────────┼──────────────────┼─────────────┘
          │                  │                  │
          └──────────────────┼──────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│         Batch Swap Router Program (This Repo)               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  • batch_swap instruction                           │   │
│  │  • execute_swap instruction                         │   │
│  │  • Slippage validation                              │   │
│  │  • Fee calculation                                  │   │
│  │  • Event emission                                   │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────┬───────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────┐
│              Jupiter Aggregator                              │
│  • Multi-DEX routing                                         │
│  • Best price discovery                                      │
│  • Swap execution                                            │
└─────────┬───────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────┐
│              Solana Network (Devnet/Mainnet)                │
└─────────────────────────────────────────────────────────────┘
```

## 🏗️ Project Structure

```
xforce-terminal-contracts/
├── programs/
│   └── batch-swap-router/      # Main Solana program
│       ├── src/
│       │   ├── lib.rs          # Program entry point
│       │   ├── instructions/   # Instruction handlers
│       │   ├── state.rs        # Account structures
│       │   ├── security.rs     # Security validations
│       │   └── ...
│       └── Cargo.toml
├── client/                      # Rust client library
│   └── src/
│       └── batch_swap_router.rs
├── tests/                       # Integration tests
├── docs/                        # Documentation
│   ├── MVP_SUMMARY.md
│   └── SECURITY.md
└── README.md                   # This file
```

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v3.0+)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.32.1)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/xforce-terminal-contracts.git
cd xforce-terminal-contracts

# Install dependencies
npm install

# Build the program
anchor build

# Run tests
anchor test
```

### Deployment

```bash
# Deploy to Devnet
anchor deploy --provider.cluster devnet

# Or use Solana CLI directly
solana program deploy target/deploy/batch_swap_router.so \
  --program-id target/deploy/batch_swap_router-keypair.json \
  --url devnet
```

### Program ID

**Devnet**: `HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx`

[View on Solana Explorer](https://explorer.solana.com/address/HS63bw1V1qTM5uWf92q3uaFdqogrc4SN9qUJSR8aqBMx?cluster=devnet)

## 💻 Usage

### Using the Client Library

```rust
use xforce_terminal_contracts_client::BatchSwapRouterClient;
use xforce_terminal_contracts_client::SwapParams;

// Create client
let client = BatchSwapRouterClient::new(program);

// Execute batch swap
let swaps = vec![
    SwapParams {
        input_mint: sol_mint,
        output_mint: usdc_mint,
        amount: 1_000_000_000,      // 1 SOL
        min_output_amount: 90_000_000, // 90 USDC (10% slippage)
    },
];

let signature = client.batch_swap(swaps)?;
```

## 🔒 Security

- ✅ Comprehensive input validation
- ✅ Safe math operations (no overflow/underflow)
- ✅ Slippage protection
- ✅ Account ownership verification
- ✅ Atomic execution guarantees
- ✅ Fee calculation transparency

See [docs/SECURITY.md](docs/SECURITY.md) for detailed security documentation.

## 📚 Documentation

- **[Security Documentation](docs/SECURITY.md)** - Security considerations and best practices
- **[MVP Summary](docs/MVP_SUMMARY.md)** - Feature overview and roadmap

## 🤝 Contributing

We welcome contributions! This is a **fully open-source** project.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

**Free and Open Source Software** - Use it, modify it, distribute it, contribute to it!

## 🙏 Acknowledgments

- [Jupiter Aggregator](https://jup.ag/) - For providing the best DEX routing
- [Anchor Framework](https://www.anchor-lang.com/) - For the amazing Solana development framework
- [Solana Foundation](https://solana.org/) - For building the best blockchain for DeFi
- All contributors and the open-source community

## ⭐ Star History

If you find this project useful, please consider giving it a star! ⭐

---

<div align="center">

**Built with ❤️ by the XForce Terminal Team**

*Free and Open Source Software for the Solana Ecosystem*

</div>

