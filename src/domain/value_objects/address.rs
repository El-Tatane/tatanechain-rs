use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use crate::domain::value_objects::public_key::PublicKey;
use std::fmt;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address([u8; 20]);

impl Address{
    pub fn from_public_key(public_key: &PublicKey) -> Self {
        let sha = Sha256::digest(public_key.as_bytes());
        let hash160 = Ripemd160::digest(sha);
        Self(hash160.into())
    }

    pub fn as_bytes(&self) -> &[u8; 20] { &self.0 }

}


impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::public_key::PublicKey;

    fn make_public_key(seed: u8) -> PublicKey {
        let mut bytes = [seed; 33];
        bytes[0] = 0x02; // préfixe SEC1 valide
        PublicKey::new(bytes)
    }

    #[test]
    fn address_is_20_bytes() {
        let key = make_public_key(1);
        assert_eq!(Address::from_public_key(&key).as_bytes().len(), 20);
    }

    #[test]
    fn address_is_deterministic() {
        let key = make_public_key(1);
        let addr1 = Address::from_public_key(&key);
        let addr2 = Address::from_public_key(&key);
        assert_eq!(addr1.as_bytes(), addr2.as_bytes());
    }

    #[test]
    fn different_keys_produce_different_addresses() {
        let addr1 = Address::from_public_key(&make_public_key(1));
        let addr2 = Address::from_public_key(&make_public_key(2));
        assert_ne!(addr1.as_bytes(), addr2.as_bytes());
    }

    #[test]
    fn address_is_not_all_zeros() {
        let addr = Address::from_public_key(&make_public_key(1));
        assert_ne!(addr.as_bytes(), &[0u8; 20]);
    }
}