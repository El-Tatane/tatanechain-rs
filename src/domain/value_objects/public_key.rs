use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PublicKey([u8; 33]);


impl PublicKey {
    pub fn new(bytes: [u8; 33]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 33] {
        &self.0
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}