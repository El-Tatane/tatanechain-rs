use crate::application::types::private_key::PrivateKey;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;

pub struct GeneratedWallet {
    pub address: Address,
    pub public_key: PublicKey,
    pub private_key: PrivateKey,
}

pub trait WalletGenerator {
    fn generate(&self) -> GeneratedWallet;
}