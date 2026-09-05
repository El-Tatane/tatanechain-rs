use crate::infrastructure::crypto::k256_wallet::K256Wallet;


#[derive(Default)]
pub struct K256WalletBuilder {
    seed: [u8; 32],
}

impl K256WalletBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(mut self, seed: [u8; 32]) -> Self {
        self.seed = seed;
        self
    }

    pub fn build(self) -> K256Wallet {
        K256Wallet::from_seed(self.seed)
    }
}
