

use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

hello

use std::{env, io, str::FromStr};
fn main() {
    // Завантаження змінних середовища з файлу .env
sfsfs    dotenv().ok();
    // Підключення до devnet
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client       = RpcClient::new(rpc_url);

    
    // Отримання приватного ключа з .env файлу
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
    let sender = Keypair::from_base58_string(&private_key);
// Запит адреси отримувача від користувача
    println!("Введіть адресу отримувача:");
    let mut recipient_address = String::new();
    io::stdin()
        .read_line(&mut recipient_address)
        .expect("Не вдалося прочитати рядок");
    let recipient_address = recipient_address.trim();

    // Перевірка та конвертація адреси отримувача
    let recipient = match solana_sdk::pubkey::Pubkey::from_str(recipient_address) {
        Ok(pubkey) => pubkey, Err(_) => { println!("Неправильний формат адреси отримувача"); return;
        }
    };

    // Сума для відправлення (10 SOL)
    let amount = 10_000_000_000; // 10 SOL in lamports

    // Створення транзакції
    let instruction = system_instruction::transfer(&sender.pubkey(), &recipient, amount);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&sender.pubkey()),
        &[&sender],
    );

    // Відправлення транзакції
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Транзакція відправлена успішно. Підпис: {}", signature),
 
    // Сума для відправлення (10 SOL)
    let amount = 10_000_000_000; // 10 SOL in lamports

    // Створення транзакції
    let instruction = system_instruction::transfer(&sender.pubkey(), &recipient, amount);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&sender.pubkey()),
        &[&sender],
    );

    // Відправлення транзакції
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Транзакція відправлена успішно. Підпис: {}", signature),
 
    // Сума для відправлення (10 SOL)
    let amount = 10_000_000_000; // 10 SOL in lamports

    // Створення транзакції
    let instruction = system_instruction::transfer(&sender.pubkey(), &recipient, amount);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&sender.pubkey()),
        &[&sender],
    );

    // Відправлення транзакції
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Транзакція відправлена успішно. Підпис: {}", signature),
 
    // Сума для відправлення (10 SOL)
    let amount = 10_000_000_000; // 10 SOL in lamports

    // Створення транзакції
    let instruction = system_instruction::transfer(&sender.pubkey(), &recipient, amount);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&sender.pubkey()),
        &[&sender],
    );

    // Відправлення транзакції
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Транзакція відправлена успішно. Підпис: {}", signature),
 
    // Сума для відправлення (10 SOL)
    let amount = 10_000_000_000; // 10 SOL in lamports

    // Створення транзакції
    let instruction = system_instruction::transfer(&sender.pubkey(), &recipient, amount);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&sender.pubkey()),
        &[&sender],
    );

    // Відправлення транзакції
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Транзакція відправлена успішно. Підпис: {}", signature),
 
    // Сума для відправлення (10 SOL)
    let amount = 10_000_000_000; // 10 SOL in lamports

    // Створення транзакції
    let instruction = system_instruction::transfer(&sender.pubkey(), &recipient, amount);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&sender.pubkey()),
        &[&sender],
    );

    // Відправлення транзакції
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Транзакція відправлена успішно. Підпис: {}", signature),
        Err(e) => println!("Помилка при відправленні транзакції: {}", e),
    }
}
