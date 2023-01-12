
# Solana Program | SolTransfer :chipmunk:

 - What? This is an on-chain program on Solana, which interacts with Solana's native System Program<br/>
 - Why? Transfer an amount of SOL token from account A to account B.<br/>
 - How? The on-chain program is built in Rust, whereas the actual transfer is executed through a TypeScript frontend script.

---

### :unlock: .env
Create an `.env` file with these variables
```bash
SOLANA_NETWORK=devnet
PATH_SENDER_WALLET=<path_to_json_file_containing_system_wallet_keys>
PATH_RECEIVER_WALLET=<path_to_json_file_containing_system_wallet_keys>
```

> example of a valid path is <br/>
> PATH_SENDER_WALLET=`/Users/irenefabris/Documents/GitHub/sc-solana-examples/accounts/bathsheba.json`


### :gear: Deploy
Prerequisites: create 4 Solana File System Wallets. We'll save all wallets in the same folder, called `accounts`, so start by generating an empty `accounts` folder. This prerequisite is already satisfied by the `accounts` folder present in the root directory of this Git repo. If you don't have file system wallets yet, create them now, running

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