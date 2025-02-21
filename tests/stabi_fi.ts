import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StabiFi } from "../target/types/stabi_fi";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import { assert } from "chai";

describe("stabi_fi", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.StabiFi as Program<StabiFi>;

  const authority = new Keypair();

  const [configAccountPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [mintAuthorityPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  async function airdrop(
    publicKey: PublicKey,
    amount: number = 0.05 * LAMPORTS_PER_SOL
  ) {
    const tx = await program.provider.connection.requestAirdrop(
      publicKey,
      amount
    );

    const blockHashInfo =
      await program.provider.connection.getLatestBlockhash();

    await program.provider.connection.confirmTransaction({
      blockhash: blockHashInfo.blockhash,
      lastValidBlockHeight: blockHashInfo.lastValidBlockHeight,
      signature: tx,
    });
  }

  it("Initializes the Config", async () => {
    await airdrop(authority.publicKey);
    const liquidationThreshold = new BN(50);
    const liquidationBonus = new BN(10);
    const minHealthFactor = new BN(1);

    const initConfigTx = await program.methods
      .initialize(liquidationThreshold, liquidationBonus, minHealthFactor)
      .accounts({ 
        authority: authority.publicKey,
      })
      .signers([authority])
      .rpc();

    console.dir({ initConfigTx }, { depth: Infinity });

    const configAccount = await program.account.config.fetch(configAccountPDA);

    console.log({authority: authority.publicKey, configAuthority: configAccount.authority.toString()})

    console.log("Config Account:", configAccount);

    assert.equal(configAccount.authority.toString(), authority.publicKey.toString());
    assert.equal(configAccount.mintAccount.toString(), mintAuthorityPDA.toString());
    assert.equal(configAccount.liquidationThreshold.toNumber(), liquidationThreshold.toNumber());
    assert.equal(configAccount.liquidationBonus.toNumber(), liquidationBonus.toNumber());
    assert.equal(configAccount.minHealthFactor.toNumber(), minHealthFactor.toNumber());
  });
});
