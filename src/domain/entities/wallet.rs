use k256::{
    PublicKey,
    ecdsa::{SigningKey, VerifyingKey, Signature, signature::Signer, signature::Verifier},
};
use elliptic_curve::Generate;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use crate::domain::value_objects::address::Address;

pub struct Wallet {
    signing_key: SigningKey,
}

impl Wallet {
    pub fn new(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.signing_key.verifying_key().into()
    }

    pub fn get_address(&self) -> Address {
        Address::from_public_key(self.signing_key.verifying_key())
    }

    pub fn verify(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
        VerifyingKey::from(public_key)
            .verify(message, signature)
            .is_ok()
    }

    pub fn generate() -> Self {
        Self::new(SigningKey::generate())
    }

    pub fn from_seed(seed: [u8; 32]) -> Self {
        let mut rng = ChaCha20Rng::from_seed(seed);
        Self::new(SigningKey::generate_from_rng(&mut rng))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::ecdsa::{VerifyingKey, signature::Verifier};
    use crate::test_helpers::wallet_fixture::WalletBuilder;

    fn verifying_key(wallet: &Wallet) -> VerifyingKey {
        wallet.signing_key.verifying_key().clone()
    }

    #[test]
    fn sign_returns_signature_verifiable_by_own_key() {
        let wallet = WalletBuilder::new().build();
        let message = b"hello blockchain";

        let signature = wallet.sign(message);

        verifying_key(&wallet)
            .verify(message, &signature)
            .expect("signature should be valid");
    }

    #[test]
    fn sign_produces_unique_signature_per_message() {
        let wallet = WalletBuilder::new().build();
        let sig1 = wallet.sign(b"message one");
        let sig2 = wallet.sign(b"message two");
        assert_ne!(sig1.to_bytes(), sig2.to_bytes());
    }

    #[test]
    fn sign_is_deterministic() {
        let wallet = WalletBuilder::new().build();

        let sig1 = wallet.sign(b"same message");
        let sig2 = wallet.sign(b"same message");
        assert_eq!(sig1.to_bytes(), sig2.to_bytes());
    }

    #[test]
    fn verify_rejects_signature_for_tampered_message() {
        let wallet = WalletBuilder::new().build();
        let signature = wallet.sign(b"correct message");

        let result = verifying_key(&wallet).verify(b"wrong message", &signature);
        assert!(result.is_err());
    }

    #[test]
    fn verify_accepts_own_signature() {
        let message = b"correct message";
        let wallet = WalletBuilder::new().build();
        let signature = wallet.sign(message);
        let public_key = wallet.get_public_key();

        assert!(Wallet::verify(&public_key, message, &signature));
    }

    #[test]
    fn verify_rejects_signature_from_different_wallet() {
        let message = b"correct message";
        let wallet = WalletBuilder::new().with_seed([1u8; 32]).build();
        let other_wallet = WalletBuilder::new().with_seed([9u8; 32]).build();
        let signature = other_wallet.sign(message);

        let public_key = wallet.get_public_key();
        assert!(!Wallet::verify(&public_key, message, &signature));
    }

    #[test]
    fn get_address_is_deterministic() {
        let wallet = WalletBuilder::new().build();
        assert_eq!(wallet.get_address().as_bytes(), wallet.get_address().as_bytes());
    }

    #[test]
    fn get_address_differs_for_different_wallets() {
        let wallet = WalletBuilder::new().with_seed([1u8; 32]).build();
        let other_wallet = WalletBuilder::new().with_seed([2u8; 32]).build();
        assert_ne!(wallet.get_address().as_bytes(), other_wallet.get_address().as_bytes());
    }

    #[test]
    fn get_address_is_20_bytes() {
        let wallet = WalletBuilder::new().build();
        assert_eq!(wallet.get_address().as_bytes().len(), 20);
    }
}
