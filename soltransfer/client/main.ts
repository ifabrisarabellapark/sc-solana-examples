import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { readFileSync } from "fs";
import path from 'path';
import * as dotenv from 'dotenv';
dotenv.config();


/**
 * VARS
 */
let tamar: Keypair;
let rahab: Keypair;
let ruth: Keypair;
let bathsheba: Keypair;
let programId: PublicKey;
let programKeypair: Keypair;
let connection: Connection;

// Conversion: 1 Lamport = 1 nano SOL
const AIRDROP_AMOUNT = 1000000000; // Lamports
const TRANSFER_AMOUNT = 1000000000; // Lamports
const SOLANA_NETWORK = process.env.SOLANA_NETWORK;
const PATH_SENDER_WALLET = process.env.PATH_SENDER_WALLET as string;
const PATH_RECEIVER_WALLET = process.env.PATH_RECEIVER_WALLET as string;
const lo = require("buffer-layout");


/**
 * HELPER FUNC
 */
function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(readFileSync(path, "utf-8")))
    )
}


/**
 * SEND SOL
 * Send lamports (i.e. nano SOL) using the Rust program
 * Interact with the program by sending the proper instructions
 */
async function sendLamports(from: Keypair, to: PublicKey, amount: number) {
    let data = Buffer.alloc(8)
    lo.ns64("value").encode(amount, data);

    let ins = new TransactionInstruction({
        keys: [
            {pubkey: from.publicKey, isSigner: true, isWritable: true},
            {pubkey: to, isSigner: false, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
        programId: programId,
        data: data,
    })

    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(ins),
        [from]
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

    // Account addresses
    rahab = createKeypairFromFile(PATH_RECEIVER_WALLET);
    bathsheba = createKeypairFromFile(PATH_SENDER_WALLET);


    // Airdrop some lamports to Bathsheba
    await connection.confirmTransaction(
        await connection.requestAirdrop(
            bathsheba.publicKey,
            AIRDROP_AMOUNT,
        )
    );

    // Bathsheba sends some SOL to Rahab
    console.log(`Bathsheba sends ${TRANSFER_AMOUNT} Lamports to Rahab`);
    console.log(`Bathsheba's pubkey = ${bathsheba.publicKey}`);
    console.log(`Rahab's pubkey = ${rahab.publicKey}`);
    await sendLamports(bathsheba, rahab.publicKey, TRANSFER_AMOUNT);
}


main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
);