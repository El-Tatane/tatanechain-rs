use sha2::{Sha256, Digest};
use crate::domain::value_objects::transaction_id::TransactionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MerkleRoot([u8; 32]);

impl MerkleRoot{
    pub fn new(merkle_root: [u8; 32]) -> Self {
        Self(merkle_root)
    }
    pub fn as_bytes(&self) -> &[u8; 32] { &self.0 }

    pub fn from_transaction_ids(ids: &[TransactionId]) -> Self {
        if ids.is_empty() {
            return MerkleRoot([0u8; 32]);
        }

        let mut level: Vec<[u8; 32]> = ids.iter().map(|id| *id.as_bytes()).collect();

        while level.len() > 1 {
            level = level
                .chunks(2)
                .map(|pair| {
                    let mut hasher = Sha256::new();
                    hasher.update(pair[0]);
                    hasher.update(pair.get(1).unwrap_or(&pair[0]));
                    hasher.finalize().into()
                })
                .collect();
        }

        MerkleRoot(level[0])
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::transaction_id::TransactionId;
    use sha2::{Digest, Sha256};

    // --- Helpers ---

    fn tid(byte: u8) -> TransactionId {
        TransactionId::new([byte; 32])
    }

    fn pair(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(a);
        h.update(b);
        h.finalize().into()
    }

    fn hex(bytes: &[u8; 32]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }


    #[test]
    fn empty_list_returns_zero_root_by_convention() {
        let root = MerkleRoot::from_transaction_ids(&[]);
        assert_eq!(root.as_bytes(), &[0u8; 32]);
    }

    #[test]
    fn single_leaf_root_is_the_leaf_itself() {
        let a = tid(0xAA);
        let root = MerkleRoot::from_transaction_ids(&[a]);
        assert_eq!(root.as_bytes(), a.as_bytes());
    }


    #[test]
    fn two_leaves_root_is_hash_of_concatenation() {
        let (a, b) = (tid(0x01), tid(0x02));
        let expected = pair(a.as_bytes(), b.as_bytes());
        let root = MerkleRoot::from_transaction_ids(&[a, b]);
        assert_eq!(root.as_bytes(), &expected);
    }

    #[test]
    fn four_leaves_build_two_levels() {
        let (a, b, c, d) = (tid(1), tid(2), tid(3), tid(4));
        let ab = pair(a.as_bytes(), b.as_bytes());
        let cd = pair(c.as_bytes(), d.as_bytes());
        let expected = pair(&ab, &cd);
        let root = MerkleRoot::from_transaction_ids(&[a, b, c, d]);
        assert_eq!(root.as_bytes(), &expected);
    }

    #[test]
    fn odd_count_duplicates_last_leaf() {
        let (a, b, c) = (tid(1), tid(2), tid(3));
        let ab = pair(a.as_bytes(), b.as_bytes());
        let cc = pair(c.as_bytes(), c.as_bytes());
        let expected = pair(&ab, &cc);
        let root = MerkleRoot::from_transaction_ids(&[a, b, c]);
        assert_eq!(root.as_bytes(), &expected);
    }


    #[test]
    fn golden_root_two_leaves() {
        let root = MerkleRoot::from_transaction_ids(&[tid(1), tid(2)]);
        assert_eq!(
            hex(root.as_bytes()),
            "f818afd37a6dc3bc92fb44731011277006db4efa6e9023cd7468c02335d22a4d"
        );
    }

    #[test]
    fn golden_root_three_leaves() {
        let root = MerkleRoot::from_transaction_ids(&[tid(1), tid(2), tid(3)]);
        assert_eq!(
            hex(root.as_bytes()),
            "831e18b32b5392c031f24c715086821d7532fcb6cac0bb815a1a647990cff261"
        );
    }

    #[test]
    fn golden_root_four_leaves() {
        let root = MerkleRoot::from_transaction_ids(&[tid(1), tid(2), tid(3), tid(4)]);
        assert_eq!(
            hex(root.as_bytes()),
            "2c0c4083be2badf7c9f9046d8730d21e034c1ce50f519c166d7605848b17b0d5"
        );
    }

    #[test]
    fn golden_root_five_leaves() {
        let root = MerkleRoot::from_transaction_ids(&[tid(1), tid(2), tid(3), tid(4), tid(5)]);
        assert_eq!(
            hex(root.as_bytes()),
            "f632ed650b4b2c467228719df716e74232b7293f997e481489bd172e73473a77"
        );
    }

    #[test]
    fn is_deterministic() {
        let ids = [tid(1), tid(2), tid(3), tid(4), tid(5)];
        let r1 = MerkleRoot::from_transaction_ids(&ids);
        let r2 = MerkleRoot::from_transaction_ids(&ids);
        assert_eq!(r1, r2);
    }

    #[test]
    fn is_order_sensitive() {
        let (a, b) = (tid(1), tid(2));
        let r_ab = MerkleRoot::from_transaction_ids(&[a, b]);
        let r_ba = MerkleRoot::from_transaction_ids(&[b, a]);
        assert_ne!(r_ab, r_ba);
    }

    #[test]
    fn changing_one_leaf_changes_the_root() {
        let original = [tid(1), tid(2), tid(3), tid(4)];
        let mut tampered = original;
        tampered[2] = tid(0xFF);
        assert_ne!(
            MerkleRoot::from_transaction_ids(&original),
            MerkleRoot::from_transaction_ids(&tampered)
        );
    }

    #[test]
    fn root_never_equals_a_non_trivial_leaf() {
        let ids = [tid(1), tid(2), tid(3)];
        let root = MerkleRoot::from_transaction_ids(&ids);
        for id in &ids {
            assert_ne!(root.as_bytes(), id.as_bytes());
        }
    }


    #[test]
    fn documents_known_duplication_collision() {
        let (a, b, c) = (tid(1), tid(2), tid(3));
        let r3 = MerkleRoot::from_transaction_ids(&[a, b, c]);
        let r4 = MerkleRoot::from_transaction_ids(&[a, b, c, c]);
        assert_eq!(r3, r4);
    }
}