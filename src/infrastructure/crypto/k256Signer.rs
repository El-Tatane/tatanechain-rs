use k256::ecdsa::signature::hazmat::PrehashSigner;
use k256::ecdsa::{Signature as EcdsaSignature, SigningKey};

use crate::application::services::transaction_signer::{SignError, TransactionSigner};
use crate::application::types::private_key::PrivateKey;
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