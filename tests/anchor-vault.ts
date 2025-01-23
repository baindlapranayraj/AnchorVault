import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { assert } from "chai";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;

  const vault_state_pda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_state"), provider.wallet.publicKey.toBuffer()],
    program.programId
  )[0];

  const vault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_account"), vault_state_pda.toBuffer()],
    program.programId
  )[0];

  it("Is initialized!", async () => {
    try {
      await program.methods
        .initialize()
        .accountsStrict({
          systemProgram: anchor.web3.SystemProgram.programId,
          depositer: provider.wallet.publicKey,
          vaultState: vault_state_pda,
          vaultAccount: vault,
        })
        .rpc();

      const account = await program.account.vaultState.fetch(vault_state_pda);

      assert.equal(account.amountDeposited.toNumber(), 0);
      assert.equal(account.amountWithdraw.toNumber(), 0);
    } catch (e) {
      console.log(`You got error while initializing: ${e}`);
    }
  });
});
