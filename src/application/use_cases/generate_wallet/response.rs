use crate::application::types::private_key::PrivateKey;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;

pub struct GenerateWalletResponse{
    pub address : Address,
    pub public_key : PublicKey,
    pub private_key: PrivateKey,
}