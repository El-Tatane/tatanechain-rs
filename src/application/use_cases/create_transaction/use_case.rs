use crate::application::services::transaction_signer::TransactionSigner;
use crate::application::services::wallet_importer::WalletImporter;
use crate::application::use_cases::create_transaction::errors::CreateTransactionError;
use crate::application::use_cases::create_transaction::{CreateTransactionRequest as Request, CreateTransactionResponse as Response};
use crate::domain::entities::unsigned_transaction::UnsignedTransaction;


pub struct CreateTransactionUseCase<I: WalletImporter, S: TransactionSigner>{
    importer: I,
    signer: S,
}

impl<I: WalletImporter, S: TransactionSigner> CreateTransactionUseCase<I, S>{
    pub fn new(importer: I, signer: S) -> Self{
        Self{ importer, signer }
    }

    pub fn execute(&self, request: Request) -> Result<Response, CreateTransactionError>{
        let wallet = self.importer.import(&request.private_key)?;

        let unsigned_transaction = UnsignedTransaction::new(
            wallet.address,
            request.recipient,
            request.amount,
        )?;

        let signature = self.signer.sign(&request.private_key, &unsigned_transaction.hash())?;
        let signed_transaction = unsigned_transaction.into_signed(wallet.public_key, signature);

        Ok(
            Response{
                transaction_id: signed_transaction.payload.get_id(),
                sender: signed_transaction.payload.sender,
                recipient: signed_transaction.payload.recipient,
                amount: signed_transaction.payload.amount,
                signature: signed_transaction.signature,
        })
    }
}