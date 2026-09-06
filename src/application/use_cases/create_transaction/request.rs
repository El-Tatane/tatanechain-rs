use crate::application::types::private_key::PrivateKey;
use crate::domain::value_objects::address::Address;

pub struct CreateTransactionRequest{
    pub private_key: PrivateKey,
    pub recipient: Address,
    pub amount: u64,
}