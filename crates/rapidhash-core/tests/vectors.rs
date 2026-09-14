use hex_literal::hex;
use rapidhash_core::{AlgorithmId, Digest, MultiHasher};

#[test]
fn sha256_standard_vectors() {
    // Empty vector
    let hasher = AlgorithmId::Sha256.hasher();
    let digest = hasher.finalize();
    assert_eq!(
        digest.as_bytes(),
        &hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    );
    assert_eq!(
        digest.to_hex_lowercase(),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );

    // NIST vector: "abc"
    let mut hasher = AlgorithmId::Sha256.hasher();
    hasher.update(b"abc");
    let digest = hasher.finalize();
    assert_eq!(
        digest.as_bytes(),
        &hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
    );
    assert_eq!(
        digest.to_hex_lowercase(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn blake3_standard_vectors() {
    // Empty vector
    let hasher = AlgorithmId::Blake3.hasher();
    let digest = hasher.finalize();
    assert_eq!(
        digest.as_bytes(),
        &hex!("af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262")
    );
    assert_eq!(
        digest.to_hex_lowercase(),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );

    // Official BLAKE3 vector: "abc"
    let mut hasher = AlgorithmId::Blake3.hasher();
    hasher.update(b"abc");
    let digest = hasher.finalize();
    assert_eq!(
        digest.as_bytes(),
        &hex!("6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85")
    );
    assert_eq!(
        digest.to_hex_lowercase(),
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"
    );
}

#[test]
fn crc32_standard_vectors() {
    // Empty input: 0x00000000
    let hasher = AlgorithmId::Crc32.hasher();
    let digest = hasher.finalize();
    assert_eq!(digest.as_bytes(), &[0x00, 0x00, 0x00, 0x00]);
    assert_eq!(digest.to_hex_uppercase(), "00000000");

    // Standard vector: "123456789" -> 0xCBF43926
    let mut hasher = AlgorithmId::Crc32.hasher();
    hasher.update(b"123456789");
    let digest = hasher.finalize();
    assert_eq!(digest.as_bytes(), &[0xCB, 0xF4, 0x39, 0x26]);
    assert_eq!(digest.to_hex_uppercase(), "CBF43926");
    assert_eq!(digest.to_canonical_hex(), "CBF43926");
}

#[test]
fn digest_hex_parsing_and_case_insensitivity() {
    let parsed_upper =
        Digest::from_hex(AlgorithmId::Crc32, "CBF43926").expect("valid uppercase hex");
    let parsed_lower =
        Digest::from_hex(AlgorithmId::Crc32, "cbf43926").expect("valid lowercase hex");
    assert_eq!(parsed_upper, parsed_lower);
    assert_eq!(parsed_upper.as_bytes(), &[0xCB, 0xF4, 0x39, 0x26]);

    // Invalid length
    assert!(Digest::from_hex(AlgorithmId::Crc32, "CBF4392").is_err());
    // Invalid characters
    assert!(Digest::from_hex(AlgorithmId::Crc32, "CBF4392Z").is_err());
}

#[test]
fn incremental_vs_oneshot_consistency() {
    let test_data = b"The quick brown fox jumps over the lazy dog. RapidHash stream testing.";

    for &algo in AlgorithmId::ALL {
        // One shot
        let mut h1 = algo.hasher();
        h1.update(test_data);
        let d1 = h1.finalize();

        // Varied chunk sizes
        let mut h2 = algo.hasher();
        for chunk in test_data.chunks(7) {
            h2.update(chunk);
        }
        let d2 = h2.finalize();

        assert_eq!(d1, d2, "Chunk boundary variation failed for {algo}");
    }
}

#[test]
fn multihasher_single_pass_equals_independent_hashers() {
    let test_data = b"Single-pass multi-hashing verification across all supported algorithms.";

    let mut multi = MultiHasher::new(AlgorithmId::ALL);
    multi.update(test_data);
    let multi_results = multi.finalize();

    for &algo in AlgorithmId::ALL {
        let mut independent = algo.hasher();
        independent.update(test_data);
        let independent_digest = independent.finalize();

        assert_eq!(
            multi_results.get(&algo),
            Some(&independent_digest),
            "MultiHasher output mismatch for {algo}"
        );
    }
}

#[test]
fn multihasher_reader_streaming() {
    let test_data = b"Stream reader test with small buffer size.";
    let mut multi = MultiHasher::new(AlgorithmId::ALL);
    let bytes_read = multi
        .update_from_reader(&test_data[..], 8)
        .expect("reader should succeed");
    assert_eq!(bytes_read, test_data.len() as u64);

    let multi_results = multi.finalize();
    assert_eq!(multi_results.len(), AlgorithmId::ALL.len());
}
