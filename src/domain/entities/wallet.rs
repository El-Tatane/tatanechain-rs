use k256::{
    SecretKey,
    PublicKey,
    ecdsa::{SigningKey, VerifyingKey, Signature, signature::Signer, signature::Verifier},
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

    pub fn get_public_key(&self) -> PublicKey{
        self.secret_key.public_key()
    }

    pub fn verify(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
        let verifying_key = VerifyingKey::from(public_key);

        verifying_key.verify(message, signature).is_ok()
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