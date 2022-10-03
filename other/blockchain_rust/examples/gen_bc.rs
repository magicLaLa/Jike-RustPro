use std::env::current_dir;
use std::sync::Arc;

use blockchain_rust_part::{Blockchain, SledDb, UTXOSet, Wallets};

fn main() {
    tracing_subscriber::fmt().init();

    let mut wallets = Wallets::new().unwrap();
    let genesis_addr = wallets.create_wallet();
    println!("==> genesis address: {}", genesis_addr);

    let path = current_dir().unwrap().join("other/blockchain_rust/data");
    let storage = Arc::new(SledDb::new(path));

    let bc = Blockchain::new(storage.clone());
    let utxos = UTXOSet::new(storage);
    utxos.reindex(&bc).unwrap();

    bc.blocks_info();
}
