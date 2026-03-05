# Tests

Integration tests for XForce Terminal Contracts.

## Files

- `batch-swap-router.ts` - TypeScript integration tests

## Running Tests

Using Anchor test framework:

```bash
anchor test
```

## Test Coverage

- Batch swap execution
- Single swap execution
- Error handling
- Account validation
- Fee calculation

## Writing Tests

Tests use the Anchor TypeScript framework:

```typescript
it("Executes batch swap", async () => {
    const swaps = [{
        inputMint: solMint,
        outputMint: usdcMint,
        amount: new BN(1_000_000_000),
        minOutputAmount: new BN(90_000_000),
    }];
    
    await program.methods
        .batchSwap(swaps)
        .accounts({...})
        .rpc();
});
```
