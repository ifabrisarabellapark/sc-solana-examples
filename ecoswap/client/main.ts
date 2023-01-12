import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import {
    TOKEN_PROGRAM_ID,
  } from "@solana/spl-token";
  
import path from 'path';
import { readFileSync } from "fs";
import * as bs58 from 'bs58';
import * as dotenv from 'dotenv';
dotenv.config();


/**
 * HELPER FUNC
 */
function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(
      Buffer.from(JSON.parse(readFileSync(path, "utf-8")))
  )
}
function createPublicKeyFromStr(address: string): PublicKey {
  return new PublicKey(address)
}


/**
 * VARS
 */
const PAYEE = process.env.PAYEE as string;
const USER_ATA = process.env.USER_ATA as string;
const VAULT_ATA = process.env.VAULT_ATA as string;
const USER_PRIVATEKEY = process.env.USER_PRIVATEKEY as string;
const VAULT_PRIVATEKEY = process.env.VAULT_PRIVATEKEY as string;
const SOLANA_NETWORK = process.env.SOLANA_NETWORK;
const TOKEN_TRANSFER_AMOUNT = 1;
const lo = require("buffer-layout");

let user: Keypair;
let vault: Keypair;
let user_ata: PublicKey;
let vault_ata: PublicKey;
let payee: PublicKey;
let connection: Connection;
let programKeypair: Keypair;
let programId: PublicKey;


/**
 * SWAP SOL-> ECOV
 * Send x-SOL and recieve x-ECOV
 * This function passes instructions to the Solana Program
 * we custom-wrote and deployed earlier
 */
async function swapSOLforECOV(
      user: Keypair,
      vault: Keypair,
      user_ata: PublicKey,
      vault_ata: PublicKey,
      payee: PublicKey,
      connection: Connection,
) {
  let data = Buffer.alloc(8)
  lo.ns64("value").encode(TOKEN_TRANSFER_AMOUNT, data);

  let ins = new TransactionInstruction({
    programId: programId,
    data: data,
  keys: [
      {
        isSigner: true,
        isWritable: true,
        pubkey: user.publicKey, //keypair
      },
      {
        isSigner: true,
        isWritable: true,
        pubkey: vault.publicKey, //keypair
      },
      {
        isSigner: false,
        isWritable: true,
        pubkey: user_ata, //pubkey (ATA)
      },
      {
        isSigner: false,
        isWritable: true,
        pubkey: vault_ata, //pubkey (ATA)
      },
      {
        isSigner: false,
        isWritable: true,
        pubkey: payee, //pubkey
      },
      {
        isSigner: false,
        isWritable: false,
        pubkey: TOKEN_PROGRAM_ID, //pubkey
      },
      {
        isSigner: false,
        isWritable: false,
        pubkey: SystemProgram.programId, //pubkey
      },
    ],
  })

  // send instructions to Solana program
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(ins),
    [user, vault]
  );
}


/**
 * MAIN
 */
async function main() {
  // Connect to blockchain
  connection = new Connection(
    `https://api.${SOLANA_NETWORK}.solana.com`, 'confirmed'
  );
  
  // Get the programId of the deployed on-chain Solana program
  // that our client will be sending intstructions to
  programKeypair = createKeypairFromFile(
    path.join(
        path.resolve(__dirname, '../_dist/program'),
        'program-keypair.json'
    )
  );
  programId = programKeypair.publicKey;

  // Accounts
  user = Keypair.fromSecretKey(bs58.decode(USER_PRIVATEKEY));
  vault = Keypair.fromSecretKey(bs58.decode(VAULT_PRIVATEKEY));
  user_ata = createPublicKeyFromStr(USER_ATA);
  vault_ata = createPublicKeyFromStr(VAULT_ATA);
  payee = createPublicKeyFromStr(PAYEE);

  // Swap tokens
  await swapSOLforECOV(
    user,
    vault,
    user_ata,
    vault_ata, 
    payee,
    connection,
  );
  console.log("Token transfer succeeded!");
}

// Execute main
main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);