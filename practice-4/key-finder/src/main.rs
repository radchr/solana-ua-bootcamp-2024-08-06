use solana_sdk::signature::{Keypair, Signer};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let prefix = "tT"; // Задайте ваш префікс тут
    let found = Arc::new(AtomicBool::new(false));

    (0..num_cpus::get()).into_par_iter().for_each(|_| {
        while !found.load(Ordering::Relaxed) {
            let keypair = Keypair::new();
            let pubkey = keypair.pubkey();
            let pubkey_str = pubkey.to_string();

            if pubkey_str.starts_with(prefix) {
                if !found.swap(true, Ordering::Relaxed) {
                    println!("Знайдено відповідний ключ!");
                    println!("Публічний ключ: {}", pubkey_str);
                    println!("Приватний ключ: {:?}", keypair.to_bytes());
                }
                break;
            }
        }
    });
}











// use solana_sdk::signature::{Keypair, Signer};


// fn main() {
//     let prefix = "r0"; // Задайте ваш префікс тут
//     let mut attempts = 0;

//     loop {
//         let keypair = Keypair::new();
//         let pubkey = keypair.pubkey();
//         let pubkey_str = pubkey.to_string();

//         if pubkey_str.starts_with(prefix) {
//             println!("Знайдено відповідний ключ!");
//             println!("Публічний ключ: {}", pubkey_str);
//             println!("Приватний ключ: {:?}", keypair.to_bytes());
//             break;
//         }

//         attempts += 1;
//         if attempts % 1000 == 0 {
//             println!("Спроб: {}", attempts);
//         }
//     }
// }
