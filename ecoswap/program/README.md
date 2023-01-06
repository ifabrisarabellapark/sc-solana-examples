# Solana Contract | Ecoswap :superhero_man: :handshake: 
This folder contains a custom designed Rust smart contract on Solana. Its logic allows for a unidirectional 1:1 SOL &rarr; ECOV tokenswap ('ECOV' is he arbitrary name of an SPL-token we previously minted on Solana).

---

### :package: Requirements
Rust and npm or yarn.

### :old_key: .env
Create an `.env` file with these variables
```bash
SOLANA_NETWORK="devnet"

# Any publickey: SOL recipient
PAYEE=<publickey_to_cashin_sol_token>

# Phantom wallet 1: SOL sender and ECOV recipient
USER_PRIVATEKEY=<secretkey>
USER_ATA=<publickey_of_recipient_associated_token_account>

# Phantom wallet 2: ECOV sender
VAULT_PRIVATEKEY=<secretkey>
VAULT_ATA=<publickey_of_sender_associated_token_account>
```

### What?
WIP


### How?
WIP
