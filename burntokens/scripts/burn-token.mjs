import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { burn, getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import bs58 from 'bs58';
import { config } from 'dotenv';
config();


/**
 * VARS
 */
// const fs = require('fs');
const PAYER = process.env.PAYER;
const RPC_URL = process.env.RPC_URL;
const TOKEN_MINT = process.env.TOKEN_MINT;
const AMOUNT_OF_TOKENS = process.env.AMOUNT_OF_TOKENS;

const tokenMint = createPublicKeyFromStr(TOKEN_MINT);
const payer = Keypair.fromSecretKey(bs58.decode(PAYER));


/**
 * HELPER FUNC
 */
function createPublicKeyFromStr(address) {
    return new PublicKey(address)
  }

async function tokenBurn() {
    const connection = new Connection(
        RPC_URL,
        {
            commitment: 'confirmed',
            disableRetryOnRateLimit: false,
            confirmTransactionInitialTimeout: 120000
        },
      );

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        tokenMint,
        payer.publicKey
    );
  
    const signature = await burn(
        connection,
        payer,
        tokenAccount.address,
        tokenMint,
        payer.publicKey,
        AMOUNT_OF_TOKENS * 9, // tokenDecimals of the tokenMint
        []
    );
    console.log('signature', signature);
    return signature;
}

tokenBurn();