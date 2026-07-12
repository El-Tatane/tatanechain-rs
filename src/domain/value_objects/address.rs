use k256::ecdsa::VerifyingKey;
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;

pub struct Address([u8; 20]);

impl Address{
    pub fn from_public_key(key: &VerifyingKey) -> Self {
        let pub_bytes = key.to_sec1_bytes();
        let sha = Sha256::digest(&pub_bytes);
        let hash160 = Ripemd160::digest(sha);
        Self(hash160.into())
    }

    pub fn as_bytes(&self) -> &[u8; 20] { &self.0 }
}

#[cfg(test)]
mod tests;