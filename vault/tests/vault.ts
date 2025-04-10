import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  // Configure the client to use the local cluster.

  let provider;
  let vaultProgram;

  beforeEach(async () => {
    provider = anchor.setProvider(anchor.AnchorProvider.env());
    vaultProgram = anchor.workspace.Vault as Program<Vault>;
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await vaultProgram.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
