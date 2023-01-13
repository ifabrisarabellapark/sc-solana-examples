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
import { readFileSync } from "fs";
import path from 'path';
import * as bs58 from 'bs58';
import * as dotenv from 'dotenv';
dotenv.config();


/**
 * VARS
 */
const SOLANA_NETWORK = process.env.SOLANA_NETWORK;
const TOKEN_TRANSFER_AMOUNT = process.env.TOKEN_TRANSFER_AMOUNT;
const TOKEN_MINT = process.env.TOKEN_MINT as string;
const SENDER_ATA = process.env.SENDER_ATA as string;
const SENDER_SECRETKEY = process.env.SENDER_SECRETKEY as string;
const RECEIVER = process.env.RECEIVER as string;
const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: PublicKey = new PublicKey(
    'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
);
const lo = require("buffer-layout");


let connection: Connection;
let programKeypair: Keypair;
let programId: PublicKey;

let token_mint: PublicKey;
let sender_ata: PublicKey;
let sender: Keypair;
let receiver: PublicKey;
let receiver_ata: PublicKey;


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

async function findAssociatedTokenAddress(
    receiver: PublicKey,
    token_mint: PublicKey
): Promise<PublicKey> {
    return (await PublicKey.findProgramAddress(
        [
            receiver.toBuffer(),
            TOKEN_PROGRAM_ID.toBuffer(),
            token_mint.toBuffer(),
        ],
        SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
    ))[0];
}


/**
 * SEND SPL-TOKEN
 * Interact with the Solana TokenProgram to execute the transfer
 */
async function SPLTransfer(sender_ata: PublicKey, sender: Keypair, receiver_ata: PublicKey) {
    let data = Buffer.alloc(8)
    lo.ns64("value").encode(TOKEN_TRANSFER_AMOUNT, data);

    let ins = new TransactionInstruction({
        programId: programId,
        data: data,
        keys: [
          {
            isSigner: false,
            isWritable: true,
            pubkey: sender_ata,
          },
          {
            isSigner: true,
            isWritable: true,
            pubkey: sender.publicKey,
          },
          {
            isSigner: false,
            isWritable: false,
            pubkey: TOKEN_PROGRAM_ID,
          },
          {
            isSigner: false,
            isWritable: true,
            pubkey: receiver_ata,
          }
        ],
    })

    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(ins),
        [sender]
    );
}


/**
 * MAIN
 */
async function main() {
    connection = new Connection(
        `https://api.${SOLANA_NETWORK}.solana.com`, 'confirmed'
    );

    programKeypair = createKeypairFromFile(
        path.join(
            path.resolve(__dirname, '../_dist/program'),
            'program-keypair.json'
        )
    );
    programId = programKeypair.publicKey;


    // Read in accounts from .env
    sender = Keypair.fromSecretKey(bs58.decode(SENDER_SECRETKEY));
    sender_ata = createPublicKeyFromStr(SENDER_ATA);
    token_mint = createPublicKeyFromStr(TOKEN_MINT);
    receiver = createPublicKeyFromStr(RECEIVER);

    receiver_ata = await findAssociatedTokenAddress(receiver, token_mint);
    console.log("Sender's ATA =", sender_ata);
    console.log("Receiver's ATA =", receiver_ata);

    // Transfer SPL-tokens
    await SPLTransfer(sender_ata, sender, receiver_ata);
    console.log("Your transfer succeeded!");
}


main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
);