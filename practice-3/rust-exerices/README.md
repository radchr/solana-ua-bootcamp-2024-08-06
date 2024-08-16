# Рішення практичного завдання 3 на Rust

```rust
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

fn main() {
    // Підключення до devnet
    let rpc_url = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(rpc_url.to_string());

    // Ваша адреса (публічний ключ)
    let address = "EH81TNMJE5yiivm4XfUqZpTQ8vVnhUbZd3Bfrocmdajc";
    let pubkey = Pubkey::from_str(address).unwrap();

    // Отримання балансу
    match rpc_client.get_balance(&pubkey) {
        Ok(balance) => println!("Баланс: {} лампортів, це {} SOL", balance, balance as f64/1000000000.0),
        Err(err) => eprintln!("Помилка: {}", err),
    }
    
}
```

Запустити

```sh
cargo run
```
