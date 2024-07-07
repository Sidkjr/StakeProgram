import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakeProgram } from "../target/types/stake_program";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("stake_program", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.StakeProgram as Program<StakeProgram>;


  let tokenMint = await PublicKey.findProgramAddressSync([Buffer.from("token-mint")], program.programId)
  let mintAuthority = await PublicKey.findProgramAddressSync([Buffer.from("token-authority")], program.programId)
  let poolAuthority = await PublicKey.findProgramAddressSync([Buffer.from("pool-authority")], program.programId)
  let stateAccount = await PublicKey.findProgramAddressSync([Buffer.from("state-account")],program.programId)
  let poolTokenAccount = await Keypair.generate()
  let userTokenAccount = await Keypair.generate()

  it("Create Mint ", async () => {
    const tx = await program.methods.initializeMint(10)
    .accounts({
      tokenMint: tokenMint[0],
      tokenAuthority: mintAuthority[0],
      payer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc();
    console.log("Mint tx: ", tx);
  });

  it("Create Pool ", async () => {
    const tx = await program.methods.initializePool()
    .accounts({
      tokenMint: tokenMint[0],
      poolAuthority: poolAuthority[0],
      poolTokenAccount: poolTokenAccount.publicKey,
      payer: provider.wallet.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([poolTokenAccount])
    .rpc();
    console.log("Pool tx: ", tx);
  });

  it("Airdrop tokens", async () => {
    const tx = await program.methods.airdrop(new anchor.BN(12))
    .accounts({
      tokenMint: tokenMint[0],
      tokenAuthority: mintAuthority[0],
      user: provider.wallet.publicKey,
      userTokenAccount: userTokenAccount.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([userTokenAccount])
    .rpc()
    console.log("Airdrop tx:", tx)
  })

  it("Staking tokens", async () => {
    const tx = await program.methods.stake(new anchor.BN(5))
    .accounts({
      tokenMint: tokenMint[0],
      poolAuthority: poolAuthority[0],
      user: provider.wallet.publicKey,
      userTokenAccount: userTokenAccount.publicKey,
      userStateAccount: stateAccount[0],
      poolTokenAccount: poolTokenAccount.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc()
    console.log("Stake tx: ", tx)
  })

  it("Unstaking tokens", async () => {
    const tx = await program.methods.unstake(new anchor.BN(5))
    .accounts({
      tokenMint: tokenMint[0],
      poolAuthority: poolAuthority[0],
      user: provider.wallet.publicKey,
      userTokenAccount: userTokenAccount.publicKey,
      userStateAccount: stateAccount[0],
      poolTokenAccount: poolTokenAccount.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    })
    .signers([])
    .rpc()
    console.log("Unstake tx: ", tx)
  })
});
