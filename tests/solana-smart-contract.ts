import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSmartContract } from "../target/types/solana_smart_contract";

describe("solana-smart-contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaSmartContract as Program<SolanaSmartContract>;

  it("Initialize with a message", async () =>{
    const dataAccount = anchor.web3.Keypair.generate();

    await program.methods.initialize("Hello, SOL").accounts({

    });
  });
});