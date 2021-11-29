const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

//const fs = require('fs');
const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = await anchor.workspace.Mysolanagif;
  console.log("💾 Solana GIF program:", program);
  const baseAccount = anchor.web3.Keypair.generate();
  //program = programForUser(owner);
  //fs.writeFileSync('./keypair.json', JSON.stringify(baseAccount))
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('gf cnt', account.totalGifs.toString())
	
  // Call add_gif!
  await program.rpc.addGif("gif link",{
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });
  
  // Get the account again to see what changed.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('gf count', account.totalGifs.toString())
  console.log(account.gifList)

  await program.rpc.voteGif("gif link",{  
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('gif votes', account.gifList[0].votes.toString())
  // console.log('sol', account.sol.toString())
  
  
      //popupState.to it's the gif poster public key that I'm trying to convert to AccountInfo
  //const to_account_info = await provider.connection.getAccountInfo(popupState.to);
  await program.rpc.sendSol(
    new anchor.BN(1),
    {
    accounts: {
      from: provider.wallet.publicKey,
      to: "9hqqMGMfG44L2R1a1osDgQRWKYt4YuegfUB6rUSaXrv8",
      systemProgram: SystemProgram.programId,
    },
  });
  // account = await program.account.baseAccount.fetch(baseAccount.publicKey);
 // console.log('sol', account.sol.toString())

}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();