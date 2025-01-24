import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { assert } from "chai";
import { BN } from "bn.js";

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

  it("Is initialized Vault 🥳! ", async () => {
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

  it("Deposite the Lamports", async () => {
    try {
      let lamp = new BN(1000000000);

      await program.methods
        .deposite(lamp)
        .accountsStrict({
          depositer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          vaultAccount: vault,
          vaultState: vault_state_pda,
        })
        .rpc();

      let accounts = await program.account.vaultState.fetch(vault_state_pda);

      let { amountDeposited, amountWithdraw } = accounts;

      // assert.equal(amountDeposited,lamp);
      // assert.equal(amountWithdraw.toNumber(),0);
      console.log(`The amount is: ${amountDeposited.toNumber()}`);
      console.log(`The amount withdraw is: ${amountWithdraw.toNumber()}`);
    } catch (e) {
      console.log(`You got error for depositing the lamports: ${e}`);
    }
  });

  it("Withdraw Amount!", async () => {
    try {
      let amountWithDraw = new BN(1);

      console.log("Withdrawing the amount.....");

      await program.methods
        .withDrawAmmount(amountWithDraw)
        .accountsStrict({
          depositer: provider.wallet.publicKey,
          vaultState: vault_state_pda,
          vaultAccount: vault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      let account = await program.account.vaultState.fetch(vault_state_pda);
      let { amountDeposited, amountWithdraw } = account;

      console.log(`The new deposite amount is: ${amountDeposited}`);
      console.log(`The new withdraw amount is: ${amountWithDraw}`);
    } catch (error) {
      console.error(
        `You got an error while trying to withdraw some amount: ${error}`
      );
    }
  });

  it("Withdraw All Lamports!", async () => {
    try {
      await program.methods
        .withDrawAll()
        .accountsStrict({
          depositer: provider.wallet.publicKey,
          systemAccount: anchor.web3.SystemProgram.programId,
          vaultState: vault_state_pda,
          vaultAccount: vault,
        })
        .rpc();

      let vaultStateAccount = await program.account.vaultState.fetchNullable(
        vault_state_pda
      );
      assert.equal(vaultStateAccount, null);
    } catch (error) {
      console.error(`You got an error while trying to withdraw all ${error}`);
    }
  });
});
