import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Mycalculatordapp } from "../target/types/mycalculatordapp";
import { assert } from "chai";

const { SystemProgram } = anchor.web3;

describe("mycalculatordapp", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Mycalculatordapp as Program<Mycalculatordapp>;

  it("Is initialized!", async () => {
    // Add your test here.
    await program.rpc.create("Welcome to Solana", {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [calculator],    
    });

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting === "Welcome to Solana");
  });
});
