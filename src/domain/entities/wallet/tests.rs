use super::*;
use k256::ecdsa::{VerifyingKey, signature::Verifier};
use crate::test_helpers::wallet_fixture::WalletBuilder;

fn verifying_key(wallet: &Wallet) -> VerifyingKey {
    SigningKey::from(&wallet.secret_key).verifying_key().clone()
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

    assert!(wallet.verify(message, &signature));
}

#[test]
fn verify_rejects_signature_from_different_wallet() {
    let message = b"correct message";
    let wallet = WalletBuilder::new().with_seed([1u8; 32]).build();
    let other_wallet = WalletBuilder::new().with_seed([9u8; 32]).build();
    let signature = other_wallet.sign(message);

    assert!(!wallet.verify(message, &signature));
}
