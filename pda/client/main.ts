import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    sendAndConfirmTransaction,
} from "@solana/web3.js";
import { readFileSync } from "fs";
import path from 'path';
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

/**
 * VARS
 */
const SOLANA_NETWORK = process.env.SOLANA_NETWORK;

let connection: Connection;
let programKeypair: Keypair;
let programId: PublicKey;
let signer: Keypair;


/**
 * DERIVE PDA
 */
async function derivePDA(signer: Keypair, programId: PublicKey) {
    const [pda, bump] = await PublicKey.findProgramAddress(
        [Buffer.from("BalloonBox-"), Buffer.from("escrow")],
        programId
    );
    console.log(`PDA Pubkey: ${pda.toString()}`);
    console.log(`PDA Bump: ${bump.toString()}`);

    let data = Buffer.from(Uint8Array.of(bump));
    let ins = new TransactionInstruction({
        keys: [
            {pubkey: signer.publicKey, isSigner: true, isWritable: true},
            {pubkey: pda, isSigner: false, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
        programId: programId,
        data: data,
    })

    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(ins),
        [signer]
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
            path.resolve(__dirname, '../dist/program'),
            'pda-keypair.json'
        )
    );
    programId = programKeypair.publicKey;
    signer = createKeypairFromFile(
        "/Users/irenefabris/Documents/GitHub/ecoverse-dao/pda/accounts/bathsheba.json"
    );
    console.log(`ProgramId = ${programId.toString()}`);
    console.log(`Signer = ${signer.publicKey.toString()}`);

    await derivePDA(signer, programId);
    console.log("Created PDA successfully");
}


main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
);