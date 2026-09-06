use crate::domain::value_objects::public_key::PublicKey;
use crate::domain::value_objects::signature::Signature;

pub trait TransactionVerifier {
    fn verify(
        &self,
        public_key: &PublicKey,
        message: &[u8],
        signature: &Signature,
    ) -> Result<(), VerifyError>;
}

#[derive(Debug, PartialEq)]
pub enum VerifyError {
    InvalidPublicKey,
    MalformedSignature,
    InvalidSignature,
}