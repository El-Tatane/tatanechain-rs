use std::fmt;

use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::signature::Signature;
use crate::domain::value_objects::transaction_id::TransactionId;


pub struct CreateTransactionResponse{
    pub transaction_id: TransactionId,
    pub sender: Address,
    pub recipient: Address,
    pub amount: u64,
    pub signature: Signature,
}

impl fmt::Display for CreateTransactionResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Transaction {}", self.transaction_id)?;
        writeln!(f, "  from:   {}", self.sender)?;
        writeln!(f, "  to:     {}", self.recipient)?;
        writeln!(f, "  amount: {}", self.amount)?;
        write!(f, "  sig:    {}", self.signature)
    }
}