use crate::application::types::private_key::PrivateKey;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;
use std::fmt;


pub struct ImportedWallet{
    pub address: Address,
    pub public_key: PublicKey,
}

pub trait WalletImporter {
    fn import(&self, private_key: PrivateKey) -> Result<ImportedWallet, ImportWalletError>;
}

#[derive(Debug, PartialEq)]
pub enum ImportWalletError {
    InvalidPrivateKey,
}

impl fmt::Display for ImportWalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPrivateKey => write!(f, "invalid private key: not a valid secp256k1 scalar"),
        }
    }
}

impl std::error::Error for ImportWalletError {}