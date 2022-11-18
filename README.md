# Solana Contract | Event Listener :satellite:
A Solana Rust smart contract (or program) that emits log events, listens to emitted events, and performs an action whenever events got intercepted

---

### Requirements
Set up your development environment to write a Rust smart contract on Solana. Install:

 - latest [Rust](https://rustup.rs/) stable <br/>
   (if Rust is already installed, upgrade it via `rustup upgrade`)
 - latest [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)


### How to Build
```bash
cargo build
```

### Logging
During the program execution both the runtime and the program itself are logging statuses and error messages. Export the recommended log mask to format log messages emitted by the program:
```bash
export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=info,solana_bpf_loader=debug,solana_rbpf=debug
```
