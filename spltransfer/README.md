
# Solana Program | SPL-token Transfer :dollar:

This Repo contains the Rust codebase of an on-chain Solana Program and its corresponding TypeScript client to perform an spl-token transfer.

---

### :unlock: .env
Create an `.env` file with these variables
```bash
SOLANA_NETWORK=devnet
TOKEN_TRANSFER_AMOUNT=[float]
TOKEN_MINT=<token_mint_account>
SENDER_ATA=<sender_associated_token_account>
SENDER_SECRETEKEY=<secretkey_of_token_sender>
RECEIVER=<publickey_of_receiver>
```