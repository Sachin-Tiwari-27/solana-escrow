import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { assert } from "chai";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.SolanaEscrow as Program;

const initializer = anchor.web3.Keypair.generate();
const taker = anchor.web3.Keypair.generate();

let mint = null;
let initializerDepositAccount = null;
let initializerReceiveAccount = null;
let takerReceiveAccount = null;

async function getTokenBalance(
  provider: anchor.AnchorProvider,
  tokenAccount: anchor.web3.PublicKey
): Promise<number> {
  const accountInfo = await provider.connection.getTokenAccountBalance(tokenAccount);
  return Number(accountInfo.value.amount);
}

describe("solana-escrow", () => {
  it("Creates mint and token accounts", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(initializer.publicKey, 2e9),
      "confirmed"
    );

    mint = await createMint(
      provider.connection,
      initializer,
      initializer.publicKey,
      null,
      6
    );

    initializerDepositAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      initializer,
      mint,
      initializer.publicKey
    );

    initializerReceiveAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      initializer,
      mint,
      initializer.publicKey
    );

    takerReceiveAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      taker,
      mint,
      taker.publicKey
    );

    await mintTo(
      provider.connection,
      initializer,
      mint,
      initializerDepositAccount.address,
      initializer,
      2_000_000 // Mint more tokens for multiple tests
    );
  });

  it("Initializes the escrow", async () => {
    const [escrowPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("escrow"), initializer.publicKey.toBuffer()],
      program.programId
    );

    const [vaultPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), escrowPDA.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeEscrow(new anchor.BN(1_000_000))
      .accounts({
        initializer: initializer.publicKey,
        initializerDepositTokenAccount: initializerDepositAccount.address,
        initializerReceiveTokenAccount: initializerReceiveAccount.address,
        escrowAccount: escrowPDA,
        vault: vaultPDA,
        mint: mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([initializer])
      .rpc();
  });

  it("Deposits tokens", async () => {
    const [escrowPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("escrow"), initializer.publicKey.toBuffer()],
      program.programId
    );
    const [vaultPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), escrowPDA.toBuffer()],
      program.programId
    );

    const preVault = await getTokenBalance(provider, vaultPDA);

    await program.methods
      .deposit()
      .accounts({
        initializer: initializer.publicKey,
        initializerDepositTokenAccount: initializerDepositAccount.address,
        escrowAccount: escrowPDA,
        vault: vaultPDA,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([initializer])
      .rpc();

    const postVault = await getTokenBalance(provider, vaultPDA);
    assert.equal(postVault - preVault, 1_000_000);
  });

  it("Completes the escrow", async () => {
    const [escrowPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("escrow"), initializer.publicKey.toBuffer()],
      program.programId
    );
    const [vaultPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), escrowPDA.toBuffer()],
      program.programId
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(taker.publicKey, 1e9),
      "confirmed"
    );

    const preTaker = await getTokenBalance(provider, takerReceiveAccount.address);

    await program.methods
      .complete()
      .accounts({
        taker: taker.publicKey,
        takerReceiveTokenAccount: takerReceiveAccount.address,
        escrowAccount: escrowPDA,
        vault: vaultPDA,
        initializerReceiveTokenAccount: initializerReceiveAccount.address,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([taker])
      .rpc();

    const postTaker = await getTokenBalance(provider, takerReceiveAccount.address);
    assert.equal(postTaker - preTaker, 1_000_000);
  });

  it("Cancels escrow (separate escrow instance)", async () => {
    // Create a new keypair for a separate escrow test
    const cancelInitializer = anchor.web3.Keypair.generate();
    
    // Airdrop to the new initializer
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(cancelInitializer.publicKey, 2e9),
      "confirmed"
    );

    // Create token accounts for the new initializer
    const cancelInitializerDepositAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      cancelInitializer,
      mint,
      cancelInitializer.publicKey
    );

    const cancelInitializerReceiveAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      cancelInitializer,
      mint,
      cancelInitializer.publicKey
    );

    // Mint tokens to the new initializer
    await mintTo(
      provider.connection,
      initializer, // Original mint authority
      mint,
      cancelInitializerDepositAccount.address,
      initializer,
      1_000_000
    );

    // Create PDAs for the new escrow
    const [cancelEscrowPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("escrow"), cancelInitializer.publicKey.toBuffer()],
      program.programId
    );

    const [cancelVaultPDA] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), cancelEscrowPDA.toBuffer()],
      program.programId
    );

    // Initialize the new escrow
    await program.methods
      .initializeEscrow(new anchor.BN(1_000_000))
      .accounts({
        initializer: cancelInitializer.publicKey,
        initializerDepositTokenAccount: cancelInitializerDepositAccount.address,
        initializerReceiveTokenAccount: cancelInitializerReceiveAccount.address,
        escrowAccount: cancelEscrowPDA,
        vault: cancelVaultPDA,
        mint: mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([cancelInitializer])
      .rpc();

    // Deposit tokens
    await program.methods
      .deposit()
      .accounts({
        initializer: cancelInitializer.publicKey,
        initializerDepositTokenAccount: cancelInitializerDepositAccount.address,
        escrowAccount: cancelEscrowPDA,
        vault: cancelVaultPDA,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([cancelInitializer])
      .rpc();

    const preBuyer = await getTokenBalance(provider, cancelInitializerReceiveAccount.address);

    // Cancel the escrow
    await program.methods
      .cancel()
      .accounts({
        initializer: cancelInitializer.publicKey,
        initializerReceiveTokenAccount: cancelInitializerReceiveAccount.address,
        escrowAccount: cancelEscrowPDA,
        vault: cancelVaultPDA,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([cancelInitializer])
      .rpc();

    const postBuyer = await getTokenBalance(provider, cancelInitializerReceiveAccount.address);
    assert.equal(postBuyer - preBuyer, 1_000_000);
  });
});
