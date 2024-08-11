use solana_sdk::signature::{Keypair, Signer};

fn main() {
    // Генерація нової пари ключів
    let keypair = Keypair::new();

    // Отримання публічного та приватного ключів
    let public_key = keypair.pubkey();
    let private_key = keypair.to_bytes();

    println!("Публічний ключ: {}", public_key);
    println!("Приватний ключ: {:?}", private_key);
}
