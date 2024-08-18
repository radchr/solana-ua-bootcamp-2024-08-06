// This uses "@metaplex-foundation/mpl-token-metadata@2" to create tokens
import "dotenv/config";
import {
  getKeypairFromEnvironment,
  getExplorerLink,
} from "@solana-developers/helpers";
import {
  Connection,
  clusterApiUrl,
  PublicKey,
  Keypair,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

// Yes, createCreate! We're making an instruction for createMetadataV3...
import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";

// const user = getKeypairFromEnvironment("SECRET_KEY"); /// - не заработал

let privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) {
  console.log("Add SECRET_KEY to .env!");
  process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const user = Keypair.fromSecretKey(asArray);
//////////////////////////////////////////////////////////

const connection = new Connection(clusterApiUrl("devnet"));

console.log(
  `🔑 We've loaded our keypair securely, using an env file! Our public key is: ${user.publicKey.toBase58()}`
);

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"  // адреса програми -- метадата - з нею буде взаємодія
  );
  
  // Subtitute in your token mint account from create-token-mint.ts
  const tokenMintAccount = new PublicKey(
    "GznEVQAhRYJPMYTNGrJdcpb3kH56jjjWi8khmNCopsrg"  // адресса токена
  );
  
  const metadataData = {
    name: "Solana UA Bootcamp 2024 v.2",
    symbol: "UASB-2",
    // Arweave / IPFS / Pinata etc link using metaplex standard for off-chain data
    uri: "https://arweave.net/1234",
    sellerFeeBasisPoints: 0,
    creators: null,
    collection: null,
    uses: null,
  };

  const metadataPDAAndBump = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      tokenMintAccount.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );
  
  const metadataPDA = metadataPDAAndBump[0];
  
  const transaction = new Transaction();
  
  const createMetadataAccountInstruction =
    createCreateMetadataAccountV3Instruction(
      {
        metadata: metadataPDA,
        mint: tokenMintAccount,
        mintAuthority: user.publicKey,
        payer: user.publicKey,
        updateAuthority: user.publicKey,
      },
      {
        createMetadataAccountArgsV3: {
          collectionDetails: null,
          data: metadataData,
          isMutable: true,
        },
      }
    );
  
  transaction.add(createMetadataAccountInstruction);

  await sendAndConfirmTransaction(
    connection,
    transaction,
    [user]
  );
  
  const tokenMintLink = getExplorerLink(
    "address",
    tokenMintAccount.toString(),
    "devnet"
  );
  
  console.log(`✅ Look at the token mint again: ${tokenMintLink}!`);
  