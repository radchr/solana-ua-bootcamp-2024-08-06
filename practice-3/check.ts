import "dotenv/config";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";


import { Keypair } from "@solana/web3.js";

const secretKey = Uint8Array.from(JSON.parse(process.env["SECRET_KEY"]!))
const keypair = Keypair.fromSecretKey(secretKey);

console.log(
  `✅ Loaded keypair securely! Public key is: ${keypair.publicKey.toBase58()}`
);

const pubKey = keypair.publicKey.toBase58();

const connection = new Connection(clusterApiUrl("devnet"));

console.log(`⚡️ Connected to devnet`);

const publicKey = new PublicKey(pubKey);

const balanceInLamports = await connection.getBalance(publicKey);

const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

console.log(
  `💰 Finished! The balance for the wallet at address ${publicKey} is ${balanceInSOL}!`
);
