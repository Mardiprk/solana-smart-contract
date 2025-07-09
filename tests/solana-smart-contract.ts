import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {Day14} from "../target/types/day14";

describe("Day14", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Day14 as Program<Day14>;

  it("signer checker", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        signer: program.provider.publicKey,
      })
      .rpc();
    console.log("he Signer: ", program.provider.publicKey.toBase58());
    console.log("TX Signature: ", tx);
  });
});
  