# Solana Programs 101 | Templates :hatched_chick:
This Repo contains a handful of Solana smart contracts (programs). Use them for inspiration / boilerplates and build more complex smart contracts upon them. The featured templares are:

---

 - **calculator** <br/>
   a math calculator as a Rust smart contract (ft. [Anchor](https://www.anchor-lang.com/) & [Trdelnik](https://github.com/Ackee-Blockchain/trdelnik)) <br/>
   devnet program_id=`7Gjakg7gAYgCRr7p5RvuGoS1G5EMQxu9U61Swokse5bj`

 - **ecoswap** <br/>
   unidirectional 1:1 SOL &rarr; ECOV tokenswap (Rust program & TypeScript client) <br/>
   devnet program_id=`3q6Pn9Uu4zf8xPbRxVJHipxdvhn1YzvCrjo7xYogN8Mi`

 - **helloworld-counter** <br/>
   print 'helloword' & emits log events (built 100% in Rust) <br/>
   devnet program_id=`5BPGqGMBL3ZpEw4F7gFeZPWYHbXeUuLtsKhug9W1vykJ`

- **pda** <br/>
   WIP! Create a Program Derived Account, aka PDA (Rust program & TypeScript client) <br/>
   devnet program_id=``

- **soltransfer** <br/>
   transfer SOL from wallet A to wallet B (Rust program & TypeScript client) <br/>
   devnet program_id=``

- **spltransfer** <br/>
   transfer SPL-tokens from wallet A to wallet B (Rust program & TypeScript client) <br/>
   devnet program_id=``

 - **tictactoe** <br/>
   play tic-tac-toe game (ft. [Anchor](https://www.anchor-lang.com/) & [Trdelnik](https://github.com/Ackee-Blockchain/trdelnik)) <br/>
   devnet program_id=`CzgE1QHdwsigEvPMJXtNHNgEmjKBVhZn3sPHRrU7yKga`


> :warning: All the above programs are coded in Rust native, expect for `calculator` and `tictactoe` which were built with the Anchor framework.