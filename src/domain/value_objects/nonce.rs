#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nonce(u64);

impl Nonce {
    pub const ZERO: Nonce = Nonce(0);

    pub fn new(nonce: u64) -> Self {
        Self(nonce)
    }

    pub fn increment(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}