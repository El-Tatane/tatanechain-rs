use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;

#[derive(Debug)]
pub struct ImportWalletFromPrivateKeyResponse {
    pub address: Address,
    pub public_key: PublicKey,
}