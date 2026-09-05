use crate::application::services::wallet_generator::{GeneratedWallet, WalletGenerator};
use crate::application::types::private_key::PrivateKey;
use crate::infrastructure::crypto::k256_wallet::K256Wallet;

pub struct K256WalletGenerator;

impl WalletGenerator for K256WalletGenerator {
    fn generate(&self) -> GeneratedWallet {
        let wallet = K256Wallet::generate();
        GeneratedWallet {
            address: wallet.get_address(),
            public_key: wallet.public_key_compressed(),
            private_key: PrivateKey::new(wallet.to_bytes()),
        }
    }
}