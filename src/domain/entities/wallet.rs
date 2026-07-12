use k256::{
    SecretKey,
    ecdsa::{SigningKey, Signature, signature::Signer, signature::Verifier},
};
use elliptic_curve::Generate;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

pub struct Wallet {
    secret_key: SecretKey,
}

impl Wallet {
    pub fn new(secret_key: SecretKey) -> Self {
        Self { secret_key }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        let signing_key = SigningKey::from(&self.secret_key);
        signing_key.sign(message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        let signing_key = SigningKey::from(&self.secret_key);
        let result = signing_key.verifying_key().verify(message, signature);

        !result.is_err()
    }

    pub fn generate() -> Self {
        Self::new(SecretKey::generate())
    }

    pub fn from_seed(seed: [u8; 32]) -> Self {
        let mut rng = ChaCha20Rng::from_seed(seed);
        Self::new(SecretKey::generate_from_rng(&mut rng))
    }
}

#[cfg(test)]
mod tests;