use crate::application::services::wallet_generator::{GeneratedWallet, WalletGenerator};
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;
use crate::application::types::private_key::PrivateKey;

pub struct FakeGenerator;

impl WalletGenerator for FakeGenerator {
    fn generate(&self) -> GeneratedWallet {
        let mut pk = [2u8; 33];
        pk[0] = 0x02;
        GeneratedWallet {
            address: Address::from_public_key(&PublicKey::new(pk)),
            public_key: PublicKey::new(pk),
            private_key: PrivateKey::new([1u8; 32]),
        }
    }
}