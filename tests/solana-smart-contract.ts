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

  it("update message", async () => {
    const [dataPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("data"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    await program.methods.updateMessage("Update SOL").accounts({
      data: dataPda,
      user: provider.wallet.publicKey
    }).rpc();

    const updated = await program.account.dataAccount.fetch(dataPda);
    console.log("Updated Message: ", updated.message);
  })
  
});