import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {ComputeUnit} from "../target/types/compute_unit";

describe("compute_unit", ()=>{
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ComputeUnit as Program<ComputeUnit>;
  
  it("Initialize", async () => {
    const seeds = [];

    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    console.log("The storage account address is", myStorage.toBase58());

    await program.methods.initialize().accounts({ myStorage: myStorage}).rpc();
  });
});