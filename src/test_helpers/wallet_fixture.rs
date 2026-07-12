use crate::domain::entities::wallet::Wallet;


#[derive(Default)]
pub struct WalletBuilder {
    seed: [u8; 32],
}

impl WalletBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(mut self, seed: [u8; 32]) -> Self {
        self.seed = seed;
        self
    }

    pub fn build(self) -> Wallet {
        Wallet::from_seed(self.seed)
    }
}
