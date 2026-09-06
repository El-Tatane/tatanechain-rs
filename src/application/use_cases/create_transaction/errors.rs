use std::fmt;
use crate::application::services::transaction_signer::SignError;
use crate::application::services::wallet_importer::ImportWalletError;
use crate::domain::errors::transaction_error::TransactionError;

#[derive(Debug, PartialEq)]
pub enum CreateTransactionError {
    ZeroAmount,
    SenderIsRecipient,
    InvalidPrivateKey,
    SigningFailed
}

impl From<TransactionError> for CreateTransactionError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::ZeroAmount => Self::ZeroAmount,
            TransactionError::SenderIsRecipient => Self::SenderIsRecipient,
        }
    }
}

impl From<ImportWalletError> for CreateTransactionError {
    fn from(_: ImportWalletError) -> Self {
        Self::InvalidPrivateKey
    }
}

impl From<SignError> for CreateTransactionError {
    fn from(err: SignError) -> Self {
        match err {
            SignError::InvalidPrivateKey => Self::InvalidPrivateKey,
            SignError::SigningFailed => Self::SigningFailed,
        }
    }
}

impl fmt::Display for CreateTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroAmount => write!(f, "transaction amount cannot be zero"),
            Self::SenderIsRecipient => write!(f, "sender and recipient must be different addresses"),
            Self::InvalidPrivateKey => write!(f, "invalid private key: not a valid secp256k1 scalar"),
            Self::SigningFailed => write!(f, "signing failed"),
        }
    }
}

impl std::error::Error for CreateTransactionError {}