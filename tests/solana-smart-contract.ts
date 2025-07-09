import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {ComputeUnit} from "../target/types/compute_unit";

describe("compute_unit", ()=>{
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.ComputeUnit as Program<ComputeUnit>;

  const defaultKeypair = new anchor.web3.PublicKey(
    "5NpeRLioZMTEZ6pcAtCvH3LQ9RaYmBBSu48Y4EXmkuXg"
  );

  it("initialized", async () =>{

    let bal_before = await program.provider.connection.getBalance(defaultKeypair);
    console.log("Bal. before: ", bal_before);

    let tx = await program.methods.initialize().rpc();

    let bal_after = await program.provider.connection.getBalance(defaultKeypair);
    console.log("Bal. After: ", bal_after);

    console.log("Difference:", BigInt(bal_before.toString()) - BigInt(bal_after.toString())); 
  });
});

//checking the CU for SOLANA Programs and it came out 5000 Lamports for this single instructions
//which is the baseline transaction fee