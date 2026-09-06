use crate::application::services::wallet_importer::{ImportWalletError, WalletImporter};
use crate::application::use_cases::import_wallet_from_private_key::{ImportWalletFromPrivateKeyRequest as Request, ImportWalletFromPrivateKeyResponse as Response};

pub struct ImportWalletFromPrivateKeyUseCase<I: WalletImporter>{
    importer: I
}

impl<I: WalletImporter> ImportWalletFromPrivateKeyUseCase<I>{
    pub fn new(importer: I) -> Self{
        Self{ importer }
    }

    pub fn execute(&self, request: Request)-> Result<Response, ImportWalletError>{
        let imported = self.importer.import(&request.private_key)?;

        Ok(Response{
            address: imported.address,
            public_key: imported.public_key,
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::application::types::private_key::PrivateKey;
    use crate::domain::value_objects::address::Address;
    use crate::test_helpers::fake_wallet_importer::FakeImporter;
    use super::*;


    #[test]
    fn execute_returns_wallet_on_valid_key() {
        let use_case = ImportWalletFromPrivateKeyUseCase::new(FakeImporter { should_fail: false });

        let response = use_case
            .execute(Request { private_key: PrivateKey::new([1u8; 32]) })
            .unwrap();

        assert_eq!(response.address, Address::from_public_key(&response.public_key));
    }

    #[test]
    fn execute_propagates_importer_error() {
        let use_case = ImportWalletFromPrivateKeyUseCase::new(FakeImporter { should_fail: true });

        let result = use_case.execute(Request { private_key: PrivateKey::new([1u8; 32]) });

        assert_eq!(result.unwrap_err(), ImportWalletError::InvalidPrivateKey);
    }
}