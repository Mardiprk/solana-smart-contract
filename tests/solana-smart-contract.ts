import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {SolanaSmartContract} from '../target/types/solana_smart_contract';

describe("solana-smart-contract", () =>{
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaSmartContract as Program<SolanaSmartContract>;

  it("initialize the test", async () =>{
    const dataAccount = anchor.web3.Keypair.generate();

    await program.methods.initialize("Hello, SOLANA").accounts({
      data: dataAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([dataAccount]).rpc();

    const account = await program.account.dataAccount.fetch(dataAccount.publicKey);
    console.log("Stored Message: ", account.message);
  });
});