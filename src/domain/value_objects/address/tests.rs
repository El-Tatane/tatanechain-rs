use super::*;
use k256::ecdsa::SigningKey;
use elliptic_curve::Generate;

fn make_verifying_key(seed: u8) -> k256::ecdsa::VerifyingKey {
    use rand_chacha::ChaCha20Rng;
    use rand_core::SeedableRng;
    let mut rng = ChaCha20Rng::from_seed([seed; 32]);
    *SigningKey::generate_from_rng(&mut rng).verifying_key()
}

#[test]
fn address_is_20_bytes() {
    let key = make_verifying_key(1);
    assert_eq!(Address::from_public_key(&key).as_bytes().len(), 20);
}

#[test]
fn address_is_deterministic() {
    let key = make_verifying_key(1);
    let addr1 = Address::from_public_key(&key);
    let addr2 = Address::from_public_key(&key);
    assert_eq!(addr1.as_bytes(), addr2.as_bytes());
}

#[test]
fn different_keys_produce_different_addresses() {
    let key1 = make_verifying_key(1);
    let key2 = make_verifying_key(2);
    let addr1 = Address::from_public_key(&key1);
    let addr2 = Address::from_public_key(&key2);
    assert_ne!(addr1.as_bytes(), addr2.as_bytes());
}

#[test]
fn address_is_not_all_zeros() {
    let key = make_verifying_key(1);
    let addr = Address::from_public_key(&key);
    assert_ne!(addr.as_bytes(), &[0u8; 20]);
}
