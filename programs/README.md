# Programs

Solana programs for XForce Terminal Contracts.

## Available Programs

### batch-swap-router
Main program for batch swap execution.

See [batch-swap-router/README.md](batch-swap-router/README.md) for detailed documentation.

## Structure

```
programs/
├── batch-swap-router/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Entry point
│       ├── constants.rs        # Program constants
│       ├── errors.rs           # Error definitions
│       ├── events.rs           # Event definitions
│       ├── state.rs            # Account structures
│       ├── utils.rs            # Utilities
│       ├── security.rs         # Security functions
│       ├── swap_execution.rs   # Swap logic
│       └── instructions/       # Instruction handlers
│           ├── mod.rs
│           ├── batch_swap.rs
│           └── execute_swap.rs
```

## Adding New Programs

1. Create directory: `programs/<program-name>/`
2. Add `Cargo.toml` with Anchor dependencies
3. Create `src/lib.rs` with program module
4. Add to `Anchor.toml`
