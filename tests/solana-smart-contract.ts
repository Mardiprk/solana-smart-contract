import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {ComputeUnit} from "../target/types/compute_unit";

describe("compute_unit", ()=>{
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ComputeUnit as Program<ComputeUnit>;
  
  it("Initialize", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("YOUR TX: ", tx);
  });
});