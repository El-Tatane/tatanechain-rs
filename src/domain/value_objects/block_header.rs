use sha2::{Sha256, Digest};
use crate::domain::value_objects::block_hash::BlockHash;
use crate::domain::value_objects::merkle_root::MerkleRoot;
use crate::domain::value_objects::nonce::Nonce;
use crate::domain::value_objects::timestamp::Timestamp;

pub struct BlockHeader{
    index: u64,
    previous_hash: BlockHash,
    merkle_root: MerkleRoot,
    timestamp: Timestamp,
    nonce: Nonce,
}


impl BlockHeader {
    fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + 32 + 32 + 8 + 8);
        bytes.extend_from_slice(&self.index.to_le_bytes());
        bytes.extend_from_slice(self.previous_hash.as_bytes());
        bytes.extend_from_slice(self.merkle_root.as_bytes());
        bytes.extend_from_slice(&self.timestamp.as_secs().to_le_bytes());
        bytes.extend_from_slice(&self.nonce.as_u64().to_le_bytes());

        bytes
    }

    pub fn hash(&self) -> BlockHash {
        let digest: [u8; 32] = Sha256::digest(self.canonical_bytes()).into();
        BlockHash::new(digest)
    }
}