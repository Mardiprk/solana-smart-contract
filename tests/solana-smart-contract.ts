import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {Sysvar} from "../target/types/sysvar";

describe("sysvar", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sysvar as Program<Sysvar>;

  it("Initialize", async () =>{
    const tx = await program.methods.initialize().rpc();
    console.log("Your Transaction signature: {}", tx);
  });
});