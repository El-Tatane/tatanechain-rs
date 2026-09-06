pub struct Timestamp(u64);

impl Timestamp {
    pub fn new(seconds: u64) -> Self {
        Self(seconds)
    }

    pub fn as_secs(&self) -> u64 {
        self.0
    }
}
