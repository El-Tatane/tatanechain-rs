use std::fmt;

pub struct PrivateKey([u8; 32]);

impl PrivateKey {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn reveal(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrivateKey(***)")
    }
}