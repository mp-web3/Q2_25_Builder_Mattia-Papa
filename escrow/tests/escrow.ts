import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const escrowProgram = anchor.workspace.Escrow as Program<Escrow>;
  const user = provider.wallet;

  // Generate PDA Addresses

  it("Initializes Escrow and Deposit", async () => {
    // Add your test here.
  });
});
