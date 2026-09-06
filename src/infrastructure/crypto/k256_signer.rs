use k256::ecdsa::signature::hazmat::{PrehashSigner, PrehashVerifier};
use k256::ecdsa::{Signature as EcdsaSignature, SigningKey, VerifyingKey};

use crate::application::services::transaction_signer::{SignError, TransactionSigner};
use crate::application::services::transaction_verifier::{TransactionVerifier, VerifyError};
use crate::application::types::private_key::PrivateKey;
use crate::domain::value_objects::public_key::PublicKey;
use crate::domain::value_objects::signature::Signature;

pub struct K256Signer;

impl TransactionSigner for K256Signer {
    fn sign(&self, private_key: &PrivateKey, message: &[u8]) -> Result<Signature, SignError> {
        let signing_key = SigningKey::from_slice(private_key.reveal())
            .map_err(|_| SignError::InvalidPrivateKey)?;

        let ecdsa_sig: EcdsaSignature = signing_key
            .sign_prehash(message)
            .map_err(|_| SignError::SigningFailed)?;

        Ok(Signature::new(ecdsa_sig.to_bytes().into()))
    }
}

impl TransactionVerifier for K256Signer {
    fn verify(
        &self,
        public_key: &PublicKey,
        message: &[u8],
        signature: &Signature,
    ) -> Result<(), VerifyError> {
        let verifying_key = VerifyingKey::from_sec1_bytes(public_key.as_bytes())
            .map_err(|_| VerifyError::InvalidPublicKey)?;

        let ecdsa_sig = EcdsaSignature::from_slice(signature.as_bytes())
            .map_err(|_| VerifyError::MalformedSignature)?;

        verifying_key
            .verify_prehash(message, &ecdsa_sig)
            .map_err(|_| VerifyError::InvalidSignature)
    }
}

#[cfg(test)]
mod tests {
    use crate::application::services::wallet_generator::WalletGenerator;
    use crate::infrastructure::crypto::k256_wallet_factory::K256WalletFactory;
    use super::*;

    #[test]
    fn sign_then_verify_roundtrip() {
        let signer = K256Signer;
        let factory = K256WalletFactory;
        let wallet = factory.generate();
        let digest = [42u8; 32];

        let sig = signer.sign(&wallet.private_key, &digest).unwrap();
        assert!(signer.verify(&wallet.public_key, &digest, &sig).is_ok());

        let other_digest = [43u8; 32];
        assert_eq!(
            signer.verify(&wallet.public_key, &other_digest, &sig),
            Err(VerifyError::InvalidSignature)
        );
    }
}