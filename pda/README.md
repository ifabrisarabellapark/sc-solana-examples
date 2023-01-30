# PDAs :octopus: :telephone:

A program Derived Address (PDA) in Solana is an account with only a public key (no private key) and owned by a program - just like any other Solana account. Why do we care? PDAs are the foundation of Cross-Program Invocation and what allows for the composability of dapps in the Solana blockchain. Through PDAs, programs can programmatically sign for certain addresses without needing a private key. I sometimes think of PDAs as escrow accounts or high-level managers of lower-level accounts.

---
### About
This Repo contains:
 - a `.rs` Rust smart contract with the logic to derive a PDA on chain
 - a `.ts` TypeScript Client to interact with the smart contract and actually derive the PDA
 - a `.sh` file to run in batch multiple shell commands and easily create a PDA



### Create a PDA
Run in terminal
```bash
solana config set --url https://api.devnet.solana.com 
solana config set --keypair /Users/irenefabris/.config/solana/id.json    # set the path to whichever keypair you want to use to sign the blockchain transaction
solana config get                                                        # check your current solana configurations: PRC should be devnet and Keypair path should be your chosen keypair
```

Before continuing, ensure you configured as Solana default wallet your desired one. Run the two following commands. If your configuration is correct, these two commands should return the same public key
```bash
solana-keygen pubkey /Users/irenefabris/.config/solana/id.json           # use the same path you used to run 'solana config set --keypair ...'
solana address
```

Now, ensure you are inside this Repo and run in terminal the following. This will cargo clean the repo, compile the Rust course code into bytecode, and deploy the contract to Solana devnet. The output will be `ProgramId` of the deployed contract.
```bash
npm run reset-and-build
```

Now, save the ProgramId and run the TypeScript client to create a fresh PDA. The command will return as feedback the PDA's pubkey, seeds, and bump.
```bash
npm run simulation
```

> :warning: :no_entry: Inefficiency in the code to be aware of: the seeds used to derive the PDA are currently **hard coded** both in the Rust contract and in the TypeScript client, as per official [Solana Cookbook](https://solanacookbook.com/references/programs.html#how-to-create-a-pda) docs. This seed is also present in the Rust contract for _ecoswap_, which in turn, calls the PDA to transfer around ECOV tokens. The PDA seeds must match up in **ALL** three spots (and are coded this way). Iff the seeds match up, will the cross-program-invocation run seamlessly. So, ensure seeds match up!

