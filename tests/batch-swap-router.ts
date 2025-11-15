import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BatchSwapRouter } from "../target/types/batch_swap_router";
import { 
  PublicKey, 
  Keypair, 
  SystemProgram,
  LAMPORTS_PER_SOL 
} from "@solana/web3.js";
import { 
  TOKEN_PROGRAM_ID, 
  getOrCreateAssociatedTokenAccount,
  createMint,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";

describe("batch-swap-router", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const program = anchor.workspace.BatchSwapRouter as Program<BatchSwapRouter>;

  // Test accounts
  let authority: Keypair;
  let user: Keypair;
  let mintA: PublicKey;
  let mintB: PublicKey;
  let tokenAccountA: PublicKey;
  let tokenAccountB: PublicKey;
  let tokenAccountAUser: PublicKey;
  let tokenAccountBUser: PublicKey;

  before(async () => {
    // Create test keypairs
    authority = Keypair.generate();
    user = Keypair.generate();

    // Airdrop SOL to test accounts
    await provider.connection.requestAirdrop(
      authority.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      user.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    // Wait for airdrops to confirm
    await new Promise((resolve) => setTimeout(resolve, 1000));

    // Create test mints
    mintA = await createMint(
      provider.connection,
      authority,
      authority.publicKey,
      null,
      9 // 9 decimals
    );

    mintB = await createMint(
      provider.connection,
      authority,
      authority.publicKey,
      null,
      6 // 6 decimals
    );

    // Create token accounts for authority
    tokenAccountA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      authority,
      mintA,
      authority.publicKey
    ).then((account) => account.address);

    tokenAccountB = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      authority,
      mintB,
      authority.publicKey
    ).then((account) => account.address);

    // Create token accounts for user
    tokenAccountAUser = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mintA,
      user.publicKey
    ).then((account) => account.address);

    tokenAccountBUser = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mintB,
      user.publicKey
    ).then((account) => account.address);

    // Mint tokens to user for testing
    await mintTo(
      provider.connection,
      authority,
      mintA,
      tokenAccountAUser,
      authority,
      1000 * 10 ** 9 // 1000 tokens with 9 decimals
    );

    await mintTo(
      provider.connection,
      authority,
      mintB,
      tokenAccountBUser,
      authority,
      1000 * 10 ** 6 // 1000 tokens with 6 decimals
    );
  });

  describe("batch_swap", () => {
    it("Executes batch swap with valid parameters", async () => {
      const swaps = [
        {
          inputMint: mintA,
          outputMint: mintB,
          amount: new anchor.BN(100 * 10 ** 9), // 100 tokens
          minOutputAmount: new anchor.BN(90 * 10 ** 6), // 90 tokens minimum
        },
      ];

      const tx = await program.methods
        .batchSwap(swaps)
        .accounts({
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      console.log("Batch swap transaction:", tx);

      // Verify transaction was successful
      const transaction = await provider.connection.getTransaction(tx, {
        commitment: "confirmed",
      });
      expect(transaction?.meta?.err).to.be.null;
    });

    it("Fails with empty swaps array", async () => {
      try {
        await program.methods
          .batchSwap([])
          .accounts({
            authority: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([authority])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (err) {
        expect(err.error?.errorMessage).to.equal("Empty swaps array");
      }
    });

    it("Fails with too many swaps", async () => {
      // Create 11 swaps (max is 10)
      const swaps = Array.from({ length: 11 }, () => ({
        inputMint: mintA,
        outputMint: mintB,
        amount: new anchor.BN(100 * 10 ** 9),
        minOutputAmount: new anchor.BN(90 * 10 ** 6),
      }));

      try {
        await program.methods
          .batchSwap(swaps)
          .accounts({
            authority: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([authority])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (err) {
        expect(err.error?.errorMessage).to.equal(
          "Too many swaps (max 10 per batch)"
        );
      }
    });

    it("Fails with zero amount", async () => {
      const swaps = [
        {
          inputMint: mintA,
          outputMint: mintB,
          amount: new anchor.BN(0),
          minOutputAmount: new anchor.BN(90 * 10 ** 6),
        },
      ];

      try {
        await program.methods
          .batchSwap(swaps)
          .accounts({
            authority: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([authority])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (err) {
        expect(err.error?.errorMessage).to.equal(
          "Invalid swap amount (must be > 0)"
        );
      }
    });

    it("Fails with same input and output mint", async () => {
      const swaps = [
        {
          inputMint: mintA,
          outputMint: mintA, // Same mint
          amount: new anchor.BN(100 * 10 ** 9),
          minOutputAmount: new anchor.BN(90 * 10 ** 6),
        },
      ];

      try {
        await program.methods
          .batchSwap(swaps)
          .accounts({
            authority: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([authority])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (err) {
        expect(err.error?.errorMessage).to.equal(
          "Invalid swap pair (input and output mints must differ)"
        );
      }
    });

    it("Fails with zero min output amount", async () => {
      const swaps = [
        {
          inputMint: mintA,
          outputMint: mintB,
          amount: new anchor.BN(100 * 10 ** 9),
          minOutputAmount: new anchor.BN(0),
        },
      ];

      try {
        await program.methods
          .batchSwap(swaps)
          .accounts({
            authority: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([authority])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (err) {
        expect(err.error?.errorMessage).to.equal("Invalid minimum output amount");
      }
    });

    it("Executes batch swap with multiple swaps", async () => {
      const swaps = [
        {
          inputMint: mintA,
          outputMint: mintB,
          amount: new anchor.BN(50 * 10 ** 9),
          minOutputAmount: new anchor.BN(45 * 10 ** 6),
        },
        {
          inputMint: mintB,
          outputMint: mintA,
          amount: new anchor.BN(30 * 10 ** 6),
          minOutputAmount: new anchor.BN(25 * 10 ** 9),
        },
      ];

      const tx = await program.methods
        .batchSwap(swaps)
        .accounts({
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      console.log("Multi-swap batch transaction:", tx);

      const transaction = await provider.connection.getTransaction(tx, {
        commitment: "confirmed",
      });
      expect(transaction?.meta?.err).to.be.null;
    });
  });

  describe("execute_swap", () => {
    it("Executes swap with valid parameters", async () => {
      const amount = new anchor.BN(100 * 10 ** 9); // 100 tokens

      // Get initial balances
      const fromAccountBefore = await getAccount(
        provider.connection,
        tokenAccountAUser
      );
      const toAccountBefore = await getAccount(
        provider.connection,
        tokenAccountA
      );

      const tx = await program.methods
        .executeSwap(amount)
        .accounts({
          authority: user.publicKey,
          from: tokenAccountAUser,
          to: tokenAccountA,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();

      console.log("Execute swap transaction:", tx);

      // Verify transaction was successful
      const transaction = await provider.connection.getTransaction(tx, {
        commitment: "confirmed",
      });
      expect(transaction?.meta?.err).to.be.null;

      // Verify balances changed
      const fromAccountAfter = await getAccount(
        provider.connection,
        tokenAccountAUser
      );
      const toAccountAfter = await getAccount(
        provider.connection,
        tokenAccountA
      );

      expect(fromAccountAfter.amount.toString()).to.equal(
        (BigInt(fromAccountBefore.amount.toString()) - BigInt(amount.toString())).toString()
      );
      expect(toAccountAfter.amount.toString()).to.equal(
        (BigInt(toAccountBefore.amount.toString()) + BigInt(amount.toString())).toString()
      );
    });

    it("Fails with zero amount", async () => {
      try {
        await program.methods
          .executeSwap(new anchor.BN(0))
          .accounts({
            authority: user.publicKey,
            from: tokenAccountAUser,
            to: tokenAccountA,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([user])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (err) {
        expect(err.error?.errorMessage).to.equal(
          "Invalid swap amount (must be > 0)"
        );
      }
    });

    it("Fails with mismatched account mints", async () => {
      try {
        await program.methods
          .executeSwap(new anchor.BN(100 * 10 ** 9))
          .accounts({
            authority: user.publicKey,
            from: tokenAccountAUser, // Mint A
            to: tokenAccountB, // Mint B - mismatch!
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([user])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (err) {
        // This will fail at the Anchor account validation level
        expect(err).to.exist;
      }
    });

    it("Fails when authority doesn't own source account", async () => {
      // Try to use authority's account but sign with user
      try {
        await program.methods
          .executeSwap(new anchor.BN(100 * 10 ** 9))
          .accounts({
            authority: user.publicKey, // User is signing
            from: tokenAccountA, // But this is authority's account
            to: tokenAccountAUser,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([user])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (err) {
        expect(err.error?.errorMessage).to.equal(
          "Invalid authority (must be token account owner)"
        );
      }
    });
  });
});

