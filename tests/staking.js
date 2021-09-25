const anchor = require("@project-serum/anchor");
const assert = require("assert");
const { BN } = require("bn.js");

const { SystemProgram } = anchor.web3;

describe("Test staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();

  const program = anchor.workspace.Staking;

  it("Init", async () => {
    // Add your test here.
    await program.rpc.init(provider.wallet.publicKey, new BN(200), {
      accounts: {
        token: alice.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [alice],
    });
    let aliceData = await program.account.token.fetch(alice.publicKey);
    assert.ok(aliceData.amount.toNumber() === 200);
  });

  it("Deposit", async () => {
    await program.rpc.deposit(new BN(200), {
      accounts: {
        token: alice.publicKey,
        authority: provider.wallet.publicKey,
      },
    });

    let aliceData = await program.account.token.fetch(alice.publicKey);
    assert.ok(aliceData.amount.toNumber() === 400);
  });

  it("Withdraw", async () => {
    await program.rpc.withdraw(new BN(100), {
      accounts: {
        token: alice.publicKey,
        authority: provider.wallet.publicKey,
      },
    });
    let aliceData = await program.account.token.fetch(alice.publicKey);
    assert.ok(aliceData.amount.toNumber() === 300);
  });
});
