import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { assert } from "chai";
import { Airdrop } from "../target/types/airdrop";

describe("airdrop", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Airdrop as Program<Airdrop>;
  const provider = anchor.getProvider();
  const { connection } = provider;

  let globalVault;
  let globalVaultWallet;
  it("Is initialized!", async () => {
    const signer = provider.publicKey;
    const [vault] = await PublicKey.findProgramAddress(
      [Buffer.from("vault"), signer.toBuffer()],
      program.programId
    );
    globalVault = vault;
    const [vault_wallet] = await PublicKey.findProgramAddress(
      [Buffer.from("vault_wallet"), signer.toBuffer(), vault.toBuffer()],

      program.programId
    );
    globalVaultWallet = vault_wallet;
    //await connection.requestAirdrop(signer, 10 * LAMPORTS_PER_SOL);
    //console.log(await connection.getBalance(signer));
    const sig = await connection.requestAirdrop(
      vault_wallet,
      10 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(sig);
    console.log((await connection.getBalance(vault_wallet)) / LAMPORTS_PER_SOL);
    const amount = new BN(1 * LAMPORTS_PER_SOL);
    // Add your test here.
    const tx = await program.methods
      .initialize(amount)
      .accounts({
        owner: signer,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    // Confirm transaction
    await connection.confirmTransaction(tx);

    // Fetch the created account
    const vaultAccount = await program.account.vault.fetch(vault);
    const vaultWalletBalance = await connection.getBalance(vault_wallet);
    console.log(vaultWalletBalance / LAMPORTS_PER_SOL);
    assert(vaultAccount.owner.equals(signer));
    assert(vaultAccount.fundedAmount.eq(amount));
    assert(new BN(vaultWalletBalance / LAMPORTS_PER_SOL).eq(new BN(11)));
  });

  it("airdropped!", async () => {
    const airdropAmount = new BN(1 * LAMPORTS_PER_SOL);
    const toWallet = new Keypair().publicKey;
    console.log(toWallet.toBase58());
    // Add your test here.
    const airdropTx = await program.methods
      .airdrop(airdropAmount)
      .accounts({
        owner: provider.publicKey,
        toWallet,
      })
      .rpc();
    console.log("Your airdrop transaction signature", airdropTx);
    await connection.confirmTransaction(airdropTx);
    const vaultAccount = await program.account.vault.fetch(globalVault);
    const vaultWalletBalance = await connection.getBalance(globalVaultWallet);
    const toWalletBalance = await connection.getBalance(toWallet);
    console.log(vaultWalletBalance / LAMPORTS_PER_SOL);
    assert(vaultAccount.airdroppedAmount.eq(airdropAmount));
    assert(new BN(vaultWalletBalance / LAMPORTS_PER_SOL).eq(new BN(10)));
    assert(new BN(toWalletBalance).eq(airdropAmount));
  });
});
