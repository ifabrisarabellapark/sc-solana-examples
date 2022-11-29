# Solana Contract | Calculator :heavy_plus_sign: :heavy_division_sign:
Construct your own calculator  in the form of a Solana Rust smart contract (or program). The calculator is able to perform these math operations: addition, multiplication, subtraction, division.

---

### :package: Requirements
Set up your SOlana Smart contract dev env. Install

 - node.js & npm
 - latest *stable* [Rust](https://rustup.rs/) <br/>
   (upgrade via `rustup upgrade`)
 - latest [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) <br/>
   (v.1.14.7 or later)
 - latest [Anchor](https://www.anchor-lang.com/) framerowk (optional) <br/>
   (installation guide [here](https://book.anchor-lang.com/getting_started/installation.html?search=#anchor)
 - latest [Trdelník](https://github.com/Ackee-Blockchain/trdelnik) <br/>
   (`Solana`/`Anchor`/`Trdelnik CLI` version must [coincide](https://github.com/Ackee-Blockchain/trdelnik#supported-versions), or your code won't run)
 - Rust nightly [toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html)

> :warning: Trdelnik has a strong dependency on the nightly toolchain of Rust. To install it, run `rustup toolchain install nightly-aarch64-apple-darwin`. If nightly is missing, the **.program_client/src/lib.rs** file will NOT auto-generate.


### Usage - Anchor
Coming soon


### Usage - Trdelnik
```bash
trdelnik init              #initialize
trdelnik test              #Rust test
trdelnik --help            
```