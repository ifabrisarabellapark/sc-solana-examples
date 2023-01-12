# Solana Program | EcoSwap :evergreen_tree:
This directory contains a Solana Rust smart contract (program) that performs a unidirectional 1:1 SOL &rarr; ECOV tokenswap. The program does 2 actions concurrently:
 - transfer SOL from George's wallet(user) address to Bbox cash-in account
 - transfer ECOV from ECOV liquidity pool (ATA) to George's ATA
> 'ECOV' is the arbitrary name of an SPL-token we previously minted on Solana.
> '[ATA](https://spl.solana.com/associated-token-account)' stands for Associated Token Account, a special type of [PDAs](https://solanacookbook.com/core-concepts/pdas.html#facts) that holds SPL-tokens
---

### :package: Requirements
Rust and npm or yarn.

### :unlock: .env
Create an `.env` file with these variables
```bash
SOLANA_NETWORK=devnet  #for devnet

# Any publickey: SOL recipient
PAYEE=<publickey_to_cashin_sol_token>

# Phantom wallet 1: SOL sender and ECOV recipient
USER_PRIVATEKEY=<recipient_secretkey>
USER_ATA=<publickey_of_recipient_associated_token_account>

# Phantom wallet 2: ECOV sender
VAULT_PRIVATEKEY=<sender_secretkey>
VAULT_ATA=<publickey_of_sender_associated_token_account>
```

### :bulb: About
The program handles the logic to perform a 1:1 swap between two tokens: <br/>
- sends 1 SOL (Solana's native token) 
- gets 1 ECOV (custom mint SPL-token)

In Solana, an account can hold at most one token. I.e., in Solana, no account can possibly hold both native tokens (SOL) and SPL-tokens (ECOV). Rather, every account is created since inception to be able to accommodate only a specific SPL-token mint or SOL but not both. Also, the only way to transfer tokens around in Solana, is by invoking Solana's native programs ([System Program](https://docs.solana.com/developing/runtime-facilities/programs), [Token Program](https://spl.solana.com/token), [Associated Token Account Program](https://spl.solana.com/associated-token-account),...). So, our custom-designed program will be performing 2 cross-program-invocations, invoking 2 transfer functions declared respectively in Solana's System Program and Token Program.

### :dna: How
How did we manage to transfer different token types in one unified transaction?
- Performing 2 cross-program-invocations: the first calls on the System Program to transfer SOL, the second calls on the Token Program to transfer SPL-tokens, namely ECOV in our case
- Using two signers in our TypeScript client to sign `sendAndConfirmTransaction` (`main.ts`). The first signer is the SOL sender who authorizes the withdrawal of SOL from their wallet. The second signer is the ECOV sender (an ATA) that authorizes the transfer of ECOV out of the sender's ATA into the receiver's ATA
- The ECOV receiver must already own an active ATA for ECOV to be transferred into. ATAs are token accounts that are deterministically derived from the user's public key and the token-mint-account.

### :racing_car: Run
If you haven't deployed your program to devnet yet, do it now. Compile the .rs code and deploy it to blockchain, running
```bash
npm run reset-and-build
```

The above command will return the programId of your freshly deployed program. Save it. Next, in *another* terminal run the command below. It'll trigger an event listener that prints log events whenever someone interacts with your programId
```bash
solana logs | grep "3q6Pn9Uu4zf8xPbRxVJHipxdvhn1YzvCrjo7xYogN8Mi invoke" -A 15             
```
> Replace `3q6Pn9Uu4zf8xPbRxVJHipxdvhn1YzvCrjo7xYogN8Mi` with your programId and replace `15` with the max number of lines you want the listener to print out.

Now that the program is deployed, go ahead and swap tokens
```bash
npm run simulation                                
```
This triggers a command inside _cicd/cicd.sh to run the .ts client file and simulate a transaction on Solana devnet. Check your [Phantom](https://chrome.google.com/webstore/detail/phantom/bfnaelmomeimhlpmgjnjophhpkkoljpa?hl=en) wallet and [Solscan](https://solscan.io/) to view the transaction.