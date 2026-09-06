use crate::domain::value_objects::public_key::PublicKey;
use crate::domain::value_objects::signature::Signature;
use crate::domain::value_objects::unsigned_transaction::UnsignedTransaction;

pub struct SignedTransaction {
    pub payload: UnsignedTransaction,
    pub public_key: PublicKey,
    pub signature: Signature,
}

impl SignedTransaction {
    pub fn new(payload: UnsignedTransaction, public_key: PublicKey, signature: Signature) -> Self {
        Self {
            payload,
            public_key,
            signature,
        }
    }
}
