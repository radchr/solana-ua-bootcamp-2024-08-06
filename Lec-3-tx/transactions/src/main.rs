use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig,
};
use std::env;
use dotenv::dotenv;

fn main() {
    // Завантаження змінних середовища з файлу .env
    dotenv().ok();

    // Отримання секретного ключа з змінної середовища
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_array).expect("Invalid keypair");

    // Створення з'єднання з devnet
    let connection = RpcClient::new("https://api.devnet.solana.com");

    println!("🔑 Our public key is: {}", sender.pubkey());

    // // Визначення публічного ключа отримувача
    // let recipient = Pubkey::from_str("2uX7PASnp9DgrG2Zynroho5S2xohZFGL9TVRPrk1D7q9").expect("Invalid recipient public key");
    // println!("💸 Attempting to send 0.01 SOL to {}...", recipient);

    // // Створення інструкції для переказу
    // let send_sol_instruction = system_instruction::transfer(
    //     &sender.pubkey(),
    //     &recipient,
    //     (0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL) as u64,
    // );

    // // Створення транзакції
    // let mut transaction = Transaction::new_with_payer(
    //     &[send_sol_instruction],
    //     Some(&sender.pubkey()),
    // );

    // // Підписання транзакції
    // let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get recent blockhash");
    // transaction.sign(&[&sender], recent_blockhash);

    // // Відправка та підтвердження транзакції
    // let signature = connection.send_and_confirm_transaction_with_spinner_and_config(
    //     &transaction,
    //     RpcSendTransactionConfig {
    //         skip_preflight: true,
    //         ..RpcSendTransactionConfig::default()
    //     },
    // ).expect("Failed to send transaction");

    // println!("✅ Transaction confirmed, signature: {}!", signature);
}
