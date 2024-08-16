import { Keypair } from "@solana/web3.js";
import { writeFile } from 'fs/promises';   // потрібно інсталювати - npm i @types/node
import { join } from 'path';

// функція для запису даних у файл
async function writeToFileAsync(filename: string, data: string) {
  const filePath = join(__dirname, filename);
  await writeFile(filePath, data, { flag: 'w' }); // 'w' - перезаписує файл, якщо він існує
  if (filename === ".env") {console.log(`Secret key written to file: ${filePath}`);}
  else if (filename === "config.ts") {console.log(`Public key written to file: ${filePath}`);}
  else {console.log(`Data written to file: ${filePath}`);}
}

const keypair = Keypair.generate();

// Приклад використання
writeToFileAsync('.env', `SECRET_KEY = "[${keypair.secretKey}]"`);
writeToFileAsync('config.ts', `export const PUBLIC_KEY = "${keypair.publicKey.toBase58()}"`);
