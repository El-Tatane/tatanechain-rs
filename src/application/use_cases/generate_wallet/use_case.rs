use crate::application::services::wallet_generator::WalletGenerator;
use crate::application::use_cases::generate_wallet::GenerateWalletResponse;

pub struct GenerateWalletUseCase<G: WalletGenerator> {
    generator: G,
}

impl<G: WalletGenerator> GenerateWalletUseCase<G> {
    pub fn new(generator: G) -> Self {
        Self { generator }
    }

    pub fn execute(&self) -> GenerateWalletResponse {
        let generated = self.generator.generate();

        GenerateWalletResponse {
            address: generated.address,
            public_key: generated.public_key,
            private_key: generated.private_key
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::application::use_cases::generate_wallet::GenerateWalletUseCase;
    use crate::domain::value_objects::address::Address;
    use crate::domain::value_objects::public_key::PublicKey;
    use crate::test_helpers::fake_wallet_generator::FakeGenerator;

    #[test]
    fn execute_returns_wallet_from_generator() {
        let use_case = GenerateWalletUseCase::new(FakeGenerator);

        let response = use_case.execute();

        let mut expected_pk = [2u8; 33];
        expected_pk[0] = 0x02;
        assert_eq!(response.public_key, PublicKey::new(expected_pk));
        assert_eq!(response.private_key.reveal(), &[1u8; 32]);
        assert_eq!(response.address, Address::from_public_key(&response.public_key));
    }
}