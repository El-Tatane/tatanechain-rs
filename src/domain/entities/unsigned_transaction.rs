use crate::domain::entities::signed_transaction::SignedTransaction;
use crate::domain::errors::transaction_error::TransactionError;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;
use crate::domain::value_objects::signature::Signature;
use crate::domain::value_objects::transaction_id::TransactionId;
use sha2::{Digest, Sha256};

pub struct UnsignedTransaction {
    pub sender: Address,
    pub recipient: Address,
    pub amount: u64,
}

impl UnsignedTransaction {
    pub fn new(sender: Address, recipient: Address, amount: u64) -> Result<Self, TransactionError> {
        if amount <= 0 {
            return Err(TransactionError::ZeroAmount);
        }
        if sender == recipient {
            return Err(TransactionError::SenderIsRecipient);
        }

        Ok(Self {
            sender: sender,
            recipient: recipient,
            amount: amount,
        })
    }
    pub fn get_id(&self)-> TransactionId{
        TransactionId::new(self.hash())
    }
    
    pub fn hash(&self) -> [u8; 32] {
        Sha256::digest(self.canonical_bytes()).into()
    }

    pub fn into_signed(self, public_key: PublicKey, signature: Signature) -> SignedTransaction {
        SignedTransaction::new(self, public_key, signature)
    }
    
    fn canonical_bytes(&self) -> [u8; 48] {
        let mut bytes = [0u8; 48];

        bytes[..20].copy_from_slice(self.sender.as_bytes());
        bytes[20..40].copy_from_slice(self.recipient.as_bytes());
        bytes[40..].copy_from_slice(&self.amount.to_be_bytes());

        bytes
    }
}
