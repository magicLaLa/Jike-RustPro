use std::{collections::HashMap, env::current_dir, fs};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    error::BlockchainError,
    utils::{deserialize, serialize},
    Wallet,
};

pub const WALLET_FILE: &str = "wallet.dat";

#[derive(Serialize, Deserialize)]
pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallets {
    pub fn new() -> Result<Self, BlockchainError> {
        let wallets = Self::load_wallet_from_file();
        wallets
    }

    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        self.save_wallet_to_file().unwrap();
        address
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
      self.wallets.get(address)
    }

    pub fn get_addresses(&self) -> Vec<&String> {
      self.wallets.keys().collect()
    }

    pub fn save_wallet_to_file(&self) -> Result<(), BlockchainError> {
        let path = current_dir()
            .unwrap()
            .join(format!("other/blockchain_rust/{}", WALLET_FILE));
        let walletts_ser = serialize(&self)?;
        fs::write(path, &walletts_ser).unwrap();
        Ok(())
    }

    pub fn load_wallet_from_file() -> Result<Self, BlockchainError> {
        let path = current_dir()
            .unwrap()
            .join(format!("other/blockchain_rust/{}", WALLET_FILE));
        info!("Wallet path: {:?}", path);

        if !path.exists() {
            let wallets = Wallets {
                wallets: HashMap::new(),
            };
            return Ok(wallets);
        }

        let wallets_ser = fs::read(&path).unwrap();
        let wallets = deserialize(&wallets_ser);
        wallets
    }
}
