use crate::application::services::wallet_generator::{GeneratedWallet, WalletGenerator};
use crate::application::services::wallet_importer::{ImportWalletError, ImportedWallet, WalletImporter};
use crate::application::types::private_key::PrivateKey;
use crate::infrastructure::crypto::k256_wallet::K256Wallet;

pub struct K256WalletFactory;

impl WalletGenerator for K256WalletFactory {
    fn generate(&self) -> GeneratedWallet {
        let wallet = K256Wallet::generate();
        GeneratedWallet {
            address: wallet.get_address(),
            public_key: wallet.public_key_compressed(),
            private_key: PrivateKey::new(wallet.to_bytes()),
        }
    }
}

impl WalletImporter for K256WalletFactory {
    fn import(&self, private_key: &PrivateKey) -> Result<ImportedWallet, ImportWalletError> {
        let wallet = K256Wallet::from_bytes(*private_key.reveal())
            .map_err(|_| ImportWalletError::InvalidPrivateKey)?;

        Ok(
            ImportedWallet{
                address: wallet.get_address(),
                public_key: wallet.public_key_compressed(),
            }
        )
    }
}