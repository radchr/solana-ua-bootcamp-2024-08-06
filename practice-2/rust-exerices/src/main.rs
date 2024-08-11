use solana_sdk::signature::{Keypair, Signer};
use std::fs::File;
use std::io::Write;
use dotenv::dotenv;
use std::env;

fn main() {
    // Генерація нової пари ключів
    let keypair = Keypair::new();

    // Отримання публічного та приватного ключів
    let public_key = keypair.pubkey();
    let private_key = keypair.to_bytes();

    // Запис приватного ключа в .env файл
    let mut file = File::create(".env").expect("Не вдалося створити файл .env");
    writeln!(file, "PRIVATE_KEY=\"{:?}\"", private_key).expect("Не вдалося записати в файл .env");

    println!("Публічний ключ: {}", public_key);

    // Завантаження приватного ключа з .env файлу
    dotenv().ok();
    let private_key_str = env::var("PRIVATE_KEY").expect("Не вдалося знайти PRIVATE_KEY в .env файлі");
    let private_key_bytes: Vec<u8> = private_key_str[1..private_key_str.len()-1]
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect();

    let loaded_keypair = Keypair::from_bytes(&private_key_bytes).expect("Не вдалося створити Keypair з приватного ключа");
    println!("Завантажений публічний ключ: {}", loaded_keypair.pubkey());
}
