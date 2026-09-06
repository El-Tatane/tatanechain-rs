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