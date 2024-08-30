import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiRollbackExample } from "../target/types/cpi_rollback_example";
import { CallerDummy } from "../target/types/caller_dummy";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("cpi-rollback-example", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .CpiRollbackExample as Program<CpiRollbackExample>;
  const callerDummyProgram = anchor.workspace
    .CallerDummy as Program<CallerDummy>;

  it("Is initialized!", async () => {
    // Create keypairs for sender and fallback
    const sender = Keypair.generate();
    const fallback = Keypair.generate();
    const recipient = Keypair.generate();

    // Airdrop 10 SOL to sender
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        sender.publicKey,
        1 * LAMPORTS_PER_SOL
      )
    );

    // Airdrop 20 SOL to fallback
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        fallback.publicKey,
        20 * LAMPORTS_PER_SOL
      )
    );

    try {
      // Invoke the fallback instruction
      const tx = await program.methods
        .fallback()
        .accounts({
          sender: sender.publicKey,
          recipient: recipient.publicKey,
          fallback: fallback.publicKey,
        })
        .signers([sender, fallback])
        .rpc({ skipPreflight: true });
    } catch (err) {
      console.error("Transaction failed:", err);
      // Get final balances
      const finalFallbackBalance = await provider.connection.getBalance(
        fallback.publicKey
      );
      const finalRecipientBalance = await provider.connection.getBalance(
        recipient.publicKey
      );

      assert.equal(finalFallbackBalance, 20 * LAMPORTS_PER_SOL);
      assert.equal(finalRecipientBalance, 0);
    }
  });
});
