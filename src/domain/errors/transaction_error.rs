#[derive(Debug, PartialEq)]
pub enum TransactionError {
    ZeroAmount,
    SenderIsRecipient,
}