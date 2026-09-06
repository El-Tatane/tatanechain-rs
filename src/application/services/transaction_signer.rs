use crate::application::types::private_key::PrivateKey;
use crate::domain::value_objects::signature::Signature;


pub trait TransactionSigner{
    fn sign(&self, private_key: &PrivateKey, message: &[u8]) -> Result<Signature, SignError>;
}



#[derive(Debug, PartialEq)]
pub enum SignError{
    InvalidPrivateKey,
    SigningFailed
}