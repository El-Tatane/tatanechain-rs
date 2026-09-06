use crate::domain::entities::signed_transaction::SignedTransaction;
use crate::domain::value_objects::block_hash::BlockHash;
use crate::domain::value_objects::block_header::BlockHeader;
use crate::domain::value_objects::merkle_root::MerkleRoot;
use crate::domain::value_objects::transaction_id::TransactionId;

pub struct Block{
    header: BlockHeader,
    transactions: Vec<SignedTransaction>
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<SignedTransaction>) -> Self{
        Self { header, transactions }
    }

    pub fn hash(&self) -> BlockHash{
        self.header.hash()
    }

    pub fn compute_merkle_root(transactions: Vec<SignedTransaction>) -> MerkleRoot{
        let transaction_ids: Vec<TransactionId> = transactions.iter()
            .map(|transaction| transaction.payload.get_id())
            .collect();

        MerkleRoot::from_transaction_ids(&transaction_ids)
    }

}