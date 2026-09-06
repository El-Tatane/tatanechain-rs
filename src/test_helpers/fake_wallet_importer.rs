use crate::application::services::wallet_importer::{
    ImportWalletError, ImportedWallet, WalletImporter,
};
use crate::application::types::private_key::PrivateKey;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;

pub struct FakeImporter {
    pub should_fail: bool,
}

impl WalletImporter for FakeImporter {
    fn import(&self, _private_key: &PrivateKey) -> Result<ImportedWallet, ImportWalletError> {
        if self.should_fail {
            return Err(ImportWalletError::InvalidPrivateKey);
        }

        let mut pk = [2u8; 33];
        pk[0] = 0x02;
        let public_key = PublicKey::new(pk);

        Ok(ImportedWallet {
            address: Address::from_public_key(&public_key),
            public_key,
        })
    }
}