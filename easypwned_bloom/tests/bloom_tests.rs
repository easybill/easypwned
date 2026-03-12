use std::io::Write;

use bloomfilter::Bloom;
use easypwned_bloom::bloom::{bloom_get, EasyBloom};

fn create_test_bloom() -> EasyBloom {
    let seed = [1u8; 32];
    let mut bloom = Bloom::new_for_fp_rate_with_seed(100, 0.01, &seed).unwrap();
    bloom.set("AABBCCDD".as_bytes());
    bloom.set("11223344".as_bytes());
    bloom
}

#[test]
fn test_bloom_roundtrip_bytes() {
    let bloom = create_test_bloom();

    assert!(bloom.check(b"AABBCCDD"));
    assert!(bloom.check(b"11223344"));
    assert!(!bloom.check(b"XXXXXXXX"));

    let bytes = bloom.to_bytes();
    let restored = Bloom::<[u8]>::from_bytes(bytes).unwrap();

    assert!(restored.check(b"AABBCCDD"));
    assert!(restored.check(b"11223344"));
    assert!(!restored.check(b"XXXXXXXX"));
}

#[test]
fn test_bloom_roundtrip_slice() {
    let bloom = create_test_bloom();

    let slice = bloom.as_slice();
    let restored = Bloom::<[u8]>::from_slice(slice).unwrap();

    assert!(restored.check(b"AABBCCDD"));
    assert!(restored.check(b"11223344"));
    assert!(!restored.check(b"XXXXXXXX"));
}

#[test]
fn test_bloom_get_from_file() {
    let bloom = create_test_bloom();
    let bytes = bloom.to_bytes();

    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.write_all(&bytes).unwrap();
    tmpfile.flush().unwrap();

    let loaded = bloom_get(tmpfile.path().to_str().unwrap()).unwrap();

    assert!(loaded.check(b"AABBCCDD"));
    assert!(loaded.check(b"11223344"));
    assert!(!loaded.check(b"XXXXXXXX"));
}

#[test]
fn test_bloom_get_nonexistent_file() {
    let result = bloom_get("/tmp/nonexistent_bloom_file_12345.bloom");
    assert!(result.is_err());
}

#[test]
fn test_bloom_get_invalid_data() {
    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    tmpfile.write_all(b"this is not a bloom filter").unwrap();
    tmpfile.flush().unwrap();

    let result = bloom_get(tmpfile.path().to_str().unwrap());
    assert!(result.is_err());
}
