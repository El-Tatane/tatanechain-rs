pub struct BlockHash([u8; 32]);

impl BlockHash{
    pub fn new(block_hash: [u8; 32]) -> Self {
        Self(block_hash)
    }
}