# Solana Contract | Event Emitter :satellite:
A Solana Rust smart contract (or program) that emits log events, says "helloworld", and counts the number of helloworld-calls.

---

## :package: Requirements
Set up your development environment to write a Rust smart contract on Solana. Install:

 - node.js & npm
 - latest [Rust](https://rustup.rs/) stable <br/>
   (if Rust is already installed, upgrade it via `rustup upgrade`)
 - latest [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

To be able to (a) build and (b) deploy your smart contract you'll need to have in your root directory:

 - `Cargo.toml` file listing all rust crates dependencies
 - `package.json` file with the script commands to run on bash


## :gear: Build
```bash
# Sanity Checks
solana --version                                            
solana-keygen --version                                             # ensure keygen is installed
solana config get                                                   # ensure devnet path exists

# Build contract
cargo clean                                                         # clean up rust files
solana config set --url https://api.devnet.solana.com               # declare path to devnet
solana-keygen pubkey prompt://                                      # (optional) derive a pubkey from a seed phrase and a passphrase
npm install                                                         # install npm dependencies
npm run <SCRIPT_TO_BUILD_SBF>                                       # run a build script contained in package.json
solana-keygen new -o /Users/irenefabris/.config/solana/id.json      # generate new keypair and save it locally
solana airdrop 1 <RECIPIENT_ACCOUNT_ADDRESS>                        # airdrop devnet SOL to a wallet address
solana balance <ACCOUNT_ADDRESS>                                    # check account balance

# Deploy to blockchain
solana program deploy <PROGRAM_FILEPATH>                            # deploy to location of the program's shared object (the program binary .so)
solana balance <ACCOUNT_ADDRESS>  
solana program show <PROGRAM_ID>
```

> Example of a `PROGRAM_FILEPATH`=dist/program/helloworld.so



## :bangbang: Logging
During the program execution both the runtime and the program itself are logging statuses and error messages. Export the recommended log mask to format log messages emitted by the program:
```bash
export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=info,solana_bpf_loader=debug,solana_rbpf=debug
```


## :test_tube: Tests
Replace `project_name` with the name of your Rust project, specified in Cargo.toml
```bash
cargo test --package <PROJECT_NAME> -- --nocapture
```