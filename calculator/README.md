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
 - latest [Anchor](https://www.anchor-lang.com/) framerowk - optional <br/>
   (installation guide [here](https://book.anchor-lang.com/getting_started/installation.html?search=#anchor) )
 - latest [Trdelník](https://github.com/Ackee-Blockchain/trdelnik) <br/>
   (`Solana`/`Anchor`/`Trdelnik CLI` version must [coincide](https://github.com/Ackee-Blockchain/trdelnik#supported-versions), or your code won't run)
 - Rust nightly [toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html) <br/>
   (Trdelnik depends on it)


### :mag_right: About 
- [Anchor](https://www.anchor-lang.com/) is a smart contract develoment framework for Solana <br/>
  (Anchor is Rust based, but it only supports unit tests in js/ts)
- [Trdelnik](https://github.com/Ackee-Blockchain/trdelnik) is a Rust testing framework for Solana programs written in Anchor <br/>
  (for a 100% Rust development on Solana)


### :anchor: Anchor
Solana has three main clusters: `mainnet-beta` (real funds), `devnet` (for developers), and `testnet` (mostly for validators). Devnet is a more realistic environment to test your application than localnet. Let's deploy on devnet:

 - obviously, you must first create a new Anchor project, running
```bash
anchor init <MY_WORKSPACE_NAME>
```

 - build the smart contract (compiling source code into bytecode) and save your program keypair in target/deploy. Keep this keypair secret. You can reuse it on all clusters.
```bash
anchor build
```     

 - display the keypair's public key, next cipy it into your declare_id! macro BOTH in lib.rs AND Anchor.toml
```bash
anchor keys list 
```                                 

 - yes, run it again! It'll include the new program id in the binary
```bash
anchor build    
```

 - now, change the *provider.cluster* variable in Anchor.toml to *devnet*
```bash
anchor deploy
```

 - run you Typescript/Javascript unit tests, if any. Otherwise, skip this step
 ```bash
 anchor test
 ```


### :pretzel: Trdelnik
> :warning: Trdelnik has a strong dependency on the nightly toolchain of Rust. To install it, run `rustup toolchain install nightly-aarch64-apple-darwin`. If nightly is missing, the **.program_client/src/lib.rs** file will NOT auto-generate. If you haven't installed Rust's nightly toolchain yet, do that **now**.

```bash
trdelnik init              #initialize
trdelnik test              #Rust test
trdelnik --help            
```