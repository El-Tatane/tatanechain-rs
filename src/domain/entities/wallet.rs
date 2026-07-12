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
mod tests;
