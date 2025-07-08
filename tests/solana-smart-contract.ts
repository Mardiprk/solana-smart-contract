import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {Sysvar} from "../target/types/sysvar";

describe("sysvar", () => {
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.Sysvar as Program<Sysvar>;

  it("initializes and read events", async () => {
    const txSig = await program.methods.initialize().rpc();
    console.log("Tx Signature:", txSig);

    const tx = await provider.connection.getTransaction(txSig,{
      commitment:"confirmed",
      maxSupportedTransactionVersion: 0,
    });

    const logs = tx?.meta?.logMessages ?? [];
    console.log("Program Logs:");
    logs.forEach(log => console.log(log));
  });
});