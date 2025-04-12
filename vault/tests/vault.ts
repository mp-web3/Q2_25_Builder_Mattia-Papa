import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const vaultProgram = anchor.workspace.Vault as Program<Vault>;
  const user = provider.wallet;

  // Generate PDA addresses for vault and vault state
  const [vaultPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    vaultProgram.programId
  );

  const [vaultStatePDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), user.publicKey.toBuffer()],
    vaultProgram.programId
  );

  it("Initializes the vault", async () => {
    // Initialize the vault
    const tx = await vaultProgram.methods
      .initialize()
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Initialization transaction signature:", tx);

    // Fetch the vault state to verify it was created
    const vaultState = await vaultProgram.account.vaultState.fetch(vaultStatePDA);

    // Verify the vault state account was created with correct data
    expect(vaultState).to.not.be.null;
    // These bump checks verify the PDAs were created with correct seeds
    expect(vaultState.vaultBump).to.be.greaterThan(0);
    expect(vaultState.stateBump).to.be.greaterThan(0);
  });

  it("Deposits funds into the vault", async () => {
    // Amount to deposit (in lamports)
    const depositAmount = new anchor.BN(1_000_000_000); // 1 SOL

    // Get initial balances
    const initialUserBalance = await provider.connection.getBalance(user.publicKey);
    const initialVaultBalance = await provider.connection.getBalance(vaultPDA);

    try {
      // Create deposit instruction
      const tx = await vaultProgram.methods
        .deposit(depositAmount)
        .accounts({
          user: user.publicKey,
          vaultState: vaultStatePDA,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      console.log("Deposit transaction signature:", tx);

      // Get updated balances
      const updatedUserBalance = await provider.connection.getBalance(user.publicKey);
      const updatedVaultBalance = await provider.connection.getBalance(vaultPDA);

      // Verify balances changed correctly (accounting for transaction fees)
      expect(initialUserBalance).to.be.greaterThan(updatedUserBalance);
      expect(updatedVaultBalance).to.equal(initialVaultBalance + depositAmount.toNumber());

      // Specifically verify the vault received exactly the deposit amount
      expect(updatedVaultBalance - initialVaultBalance).to.equal(depositAmount.toNumber());
    } catch (error) {
      console.error("Error during deposit:", error);
      // If there are logs available, print them for debugging
      if (error.logs) {
        console.error("Transaction logs:", error.logs);
      }
      throw error;
    }
  });

  it("Withdraw funds from the vault", async () => {
    const initialUserBalance = await provider.connection.getBalance(user.publicKey);
    const initialVaultBalance = await provider.connection.getBalance(vaultPDA);

    const withdrawalAmount = initialVaultBalance;

    try {
      // Create Withdraw instructions
      const tx = await vaultProgram.methods
        .withdraw(new anchor.BN(withdrawalAmount))
        .accounts({
          user: user.publicKey,
          vaultState: vaultStatePDA,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();

      console.log("Withdrawal transaction signature: ", tx);

      const updatedUserBalance = await provider.connection.getBalance(user.publicKey);
      const updatedVaultBalance = await provider.connection.getBalance(vaultPDA);

      expect(updatedUserBalance).to.be.greaterThan(initialUserBalance);
      expect(updatedVaultBalance).to.equal(0);

    } catch (error) {
      console.error("Error during withdrawal:", error);
      // If there are logs available, print them for debugging
      if (error.logs) {
        console.error("Transaction logs:", error.logs);
      }
      throw error;
    }
  });

  it("Close the vault", async () => {

    const tx = await vaultProgram.methods.closeVault().accounts({
      user: user.publicKey,
      vaultState: vaultStatePDA,
      vault: vaultPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    })

    
  })
});
