use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::public_key::PublicKey;
use elliptic_curve::Generate;
use k256::elliptic_curve::sec1::ToSec1Point;
use k256::{
    PublicKey as k256PublicKey,
    ecdsa::{Signature, SigningKey, VerifyingKey, signature::Signer, signature::Verifier},
};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

pub struct K256Wallet {
    signing_key: SigningKey,
}


impl K256Wallet {
    pub fn new(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn get_public_key(&self) -> k256PublicKey {
        self.signing_key.verifying_key().into()
    }

    pub fn public_key_compressed(&self) -> PublicKey {
        let bytes: [u8; 33] = self.get_public_key()
            .to_sec1_point(true)
            .as_bytes()
            .try_into()
            .expect("compressed SEC1 point is always 33 bytes");

        PublicKey::new(bytes)
    }

    pub fn get_address(&self) -> Address {
        Address::from_public_key(&self.public_key_compressed())
    }

    pub fn verify(public_key: &k256PublicKey, message: &[u8], signature: &Signature) -> bool {
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

    pub fn to_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes().into()
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self::new(SigningKey::from_bytes(&bytes.into()).expect("invalid private key bytes"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::k256_wallet_builder::K256WalletBuilder;
    use k256::ecdsa::{VerifyingKey, signature::Verifier};

    fn verifying_key(wallet: &K256Wallet) -> VerifyingKey {
        wallet.signing_key.verifying_key().clone()
    }

    #[test]
    fn sign_returns_signature_verifiable_by_own_key() {
        let wallet = K256WalletBuilder::new().build();
        let message = b"hello blockchain";

        let signature = wallet.sign(message);

        verifying_key(&wallet)
            .verify(message, &signature)
            .expect("signature should be valid");
    }

    #[test]
    fn sign_produces_unique_signature_per_message() {
        let wallet = K256WalletBuilder::new().build();
        let sig1 = wallet.sign(b"message one");
        let sig2 = wallet.sign(b"message two");
        assert_ne!(sig1.to_bytes(), sig2.to_bytes());
    }

    #[test]
    fn sign_is_deterministic() {
        let wallet = K256WalletBuilder::new().build();

        let sig1 = wallet.sign(b"same message");
        let sig2 = wallet.sign(b"same message");
        assert_eq!(sig1.to_bytes(), sig2.to_bytes());
    }

    #[test]
    fn verify_rejects_signature_for_tampered_message() {
        let wallet = K256WalletBuilder::new().build();
        let signature = wallet.sign(b"correct message");

        let result = verifying_key(&wallet).verify(b"wrong message", &signature);
        assert!(result.is_err());
    }

    #[test]
    fn verify_accepts_own_signature() {
        let message = b"correct message";
        let wallet = K256WalletBuilder::new().build();
        let signature = wallet.sign(message);
        let public_key = wallet.get_public_key();

        assert!(K256Wallet::verify(&public_key, message, &signature));
    }

    #[test]
    fn verify_rejects_signature_from_different_wallet() {
        let message = b"correct message";
        let wallet = K256WalletBuilder::new().with_seed([1u8; 32]).build();
        let other_wallet = K256WalletBuilder::new().with_seed([9u8; 32]).build();
        let signature = other_wallet.sign(message);

        let public_key = wallet.get_public_key();
        assert!(!K256Wallet::verify(&public_key, message, &signature));
    }

    #[test]
    fn get_address_is_deterministic() {
        let wallet = K256WalletBuilder::new().build();
        assert_eq!(wallet.get_address().as_bytes(), wallet.get_address().as_bytes());
    }

    #[test]
    fn get_address_differs_for_different_wallets() {
        let wallet = K256WalletBuilder::new().with_seed([1u8; 32]).build();
        let other_wallet = K256WalletBuilder::new().with_seed([2u8; 32]).build();
        assert_ne!(wallet.get_address().as_bytes(), other_wallet.get_address().as_bytes());
    }

    #[test]
    fn get_address_is_20_bytes() {
        let wallet = K256WalletBuilder::new().build();
        assert_eq!(wallet.get_address().as_bytes().len(), 20);
    }

    #[test]
    fn from_bytes_to_bytes_roundtrip() {
        let bytes = [1u8; 32];
        let wallet = K256Wallet::from_bytes(bytes);

        assert_eq!(bytes, wallet.to_bytes());
    }

    #[test]
    #[should_panic]
    fn from_bytes_rejects_zero_key() {
        K256Wallet::from_bytes([0u8; 32]);
    }

    #[test]
    fn to_bytes_from_bytes_roundtrip() {
        let wallet = K256Wallet::generate();
        let restored = K256Wallet::from_bytes(wallet.to_bytes());

        assert_eq!(wallet.to_bytes(), restored.to_bytes());
    }

}
