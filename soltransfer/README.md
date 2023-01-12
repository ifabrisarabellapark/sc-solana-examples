
# SolTransfer Program :chipmunk:

 - What? This is an on-chain program on Solana, which interacts with Solana's native System Program<br/>
 - Why? Transfer an amount of SOL token from account A to account B.<br/>
 - How? The on-chain program is built in Rust, whereas the actual transfer is executed through a TypeScript frontend script.

---

### Deploy
Prerequisites: create 4 Solana File System Wallets. We'll save all wallets in the same folder, called `accounts`, so start by generating an empty `accounts` folder. Then enter the parent directory of the `accounts` folder and from there run

```bash
solana-keygen new --no-bip39-passphrase -o accounts/gigi.json
solana airdrop 2 X000Y0uRacCoUnTAddrEss000pAYXj5NyiEjz6MZWXi
solana account X000Y0uRacCoUnTAddrEss000pAYXj5NyiEjz6MZWXi
```

Next, deploy to Solana **devnet**. But first... ensure that the account you'll be using to sign the deploy transaction has enough balance, if not airdrop some devnet SOL tokens to it. Then run
```
solana config set --url https://api.devnet.solana.com                       # set network to devnet
npm run reset-and-build                                                     # clean up mess and deploy
```

Now check the deployment was successful. Navigate to [solscan.io](https://solscan.io/?cluster=devnet) and search your program entering its id. Then run in terminal
```bash
solana program show --programs                                              # print program specs
solana-keygen verify <PROGRAM_ID> _dist/program/program-keypair.json        # verify the program address against your keypair file (should return 'success')
```