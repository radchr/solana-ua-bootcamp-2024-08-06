import "dotenv/config";
// import { PUBLIC_KEY } from './config';
import { Keypair } from "@solana/web3.js";

const secKeyStr = process.env["SECRET_KEY"];
// console.log(secKeyStr);

const secKeyUArr = Uint8Array.from(JSON.parse(process.env["SECRET_KEY"]!));

console.log(`Secret Key: ${secKeyUArr}`);
// console.log(`Public Key: ${PUBLIC_KEY}`);

const keypair = Keypair.fromSecretKey(secKeyUArr);

console.log(`Public Key: ${keypair.publicKey.toBase58()}`);
