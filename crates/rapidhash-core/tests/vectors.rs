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

#[test]
fn sha512_standard_vectors() {
    let digest = AlgorithmId::Sha512.hasher().finalize();
    assert_eq!(
        digest.to_hex_lowercase(),
        "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
         47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
    );

    let mut hasher = AlgorithmId::Sha512.hasher();
    hasher.update(b"abc");
    assert_eq!(
        hasher.finalize().to_hex_lowercase(),
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
         2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
    );
}

#[test]
fn sha384_standard_vectors() {
    let digest = AlgorithmId::Sha384.hasher().finalize();
    assert_eq!(
        digest.to_hex_lowercase(),
        "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da\
         274edebfe76f65fbd51ad2f14898b95b"
    );

    let mut hasher = AlgorithmId::Sha384.hasher();
    hasher.update(b"abc");
    assert_eq!(
        hasher.finalize().to_hex_lowercase(),
        "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed\
         8086072ba1e7cc2358baeca134c825a7"
    );
}

#[test]
fn sha3_256_standard_vectors() {
    let digest = AlgorithmId::Sha3_256.hasher().finalize();
    assert_eq!(
        digest.to_hex_lowercase(),
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
    );

    let mut hasher = AlgorithmId::Sha3_256.hasher();
    hasher.update(b"abc");
    assert_eq!(
        hasher.finalize().to_hex_lowercase(),
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
    );
}

#[test]
fn sha3_512_standard_vectors() {
    let digest = AlgorithmId::Sha3_512.hasher().finalize();
    assert_eq!(
        digest.to_hex_lowercase(),
        "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a6\
         15b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"
    );

    let mut hasher = AlgorithmId::Sha3_512.hasher();
    hasher.update(b"abc");
    assert_eq!(
        hasher.finalize().to_hex_lowercase(),
        "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e\
         10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"
    );
}

#[test]
fn blake2b_standard_vectors() {
    let digest = AlgorithmId::Blake2b512.hasher().finalize();
    assert_eq!(
        digest.to_hex_lowercase(),
        "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419\
         d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce"
    );

    let mut hasher = AlgorithmId::Blake2b512.hasher();
    hasher.update(b"abc");
    assert_eq!(
        hasher.finalize().to_hex_lowercase(),
        "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d1\
         7d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923"
    );
}

#[test]
fn blake2s_standard_vectors() {
    let digest = AlgorithmId::Blake2s256.hasher().finalize();
    assert_eq!(
        digest.to_hex_lowercase(),
        "69217a3079908094e11121d042354a7c1f55b6482ca1a51e1b250dfd1ed0eef9"
    );

    let mut hasher = AlgorithmId::Blake2s256.hasher();
    hasher.update(b"abc");
    assert_eq!(
        hasher.finalize().to_hex_lowercase(),
        "508c5e8c327c14e2e1a72ba34eeb452f37458b209ed63a294d999b4c86675982"
    );
}

#[test]
fn legacy_standard_vectors() {
    let md5_empty = AlgorithmId::Md5.hasher().finalize();
    assert_eq!(
        md5_empty.to_hex_lowercase(),
        "d41d8cd98f00b204e9800998ecf8427e"
    );

    let mut md5 = AlgorithmId::Md5.hasher();
    md5.update(b"abc");
    assert_eq!(
        md5.finalize().to_hex_lowercase(),
        "900150983cd24fb0d6963f7d28e17f72"
    );

    let sha1_empty = AlgorithmId::Sha1.hasher().finalize();
    assert_eq!(
        sha1_empty.to_hex_lowercase(),
        "da39a3ee5e6b4b0d3255bfef95601890afd80709"
    );

    let mut sha1 = AlgorithmId::Sha1.hasher();
    sha1.update(b"abc");
    assert_eq!(
        sha1.finalize().to_hex_lowercase(),
        "a9993e364706816aba3e25717850c26c9cd0d89d"
    );
}

#[test]
fn crc32c_standard_vector() {
    let empty = AlgorithmId::Crc32c.hasher().finalize();
    assert_eq!(empty.as_bytes(), &[0x00, 0x00, 0x00, 0x00]);

    // CRC32C (Castagnoli) of "123456789" is 0xE3069283.
    let mut hasher = AlgorithmId::Crc32c.hasher();
    hasher.update(b"123456789");
    let digest = hasher.finalize();
    assert_eq!(digest.as_bytes(), &hex!("e3069283"));
    assert_eq!(digest.to_hex_lowercase(), "e3069283");
}

#[test]
fn xxhash_standard_vectors() {
    let xxh32_empty = AlgorithmId::Xxh32.hasher().finalize();
    assert_eq!(xxh32_empty.to_hex_lowercase(), "02cc5d05");

    let mut xxh32 = AlgorithmId::Xxh32.hasher();
    xxh32.update(b"abc");
    assert_eq!(xxh32.finalize().to_hex_lowercase(), "32d153ff");

    let xxh64_empty = AlgorithmId::Xxh64.hasher().finalize();
    assert_eq!(xxh64_empty.to_hex_lowercase(), "ef46db3751d8e999");

    let mut xxh64 = AlgorithmId::Xxh64.hasher();
    xxh64.update(b"abc");
    assert_eq!(xxh64.finalize().to_hex_lowercase(), "44bc2cf5ad770999");

    let xxh3_empty = AlgorithmId::Xxh3.hasher().finalize();
    assert_eq!(xxh3_empty.to_hex_lowercase(), "2d06800538d394c2");
}

#[test]
fn algorithm_identifiers_round_trip() {
    for &algorithm in AlgorithmId::ALL {
        let id = algorithm.as_str();
        assert_eq!(id, id.to_ascii_lowercase());
        assert_eq!(
            id.parse::<AlgorithmId>().expect("canonical id must parse"),
            algorithm
        );
    }

    assert_eq!(
        "SHA-256".parse::<AlgorithmId>().unwrap(),
        AlgorithmId::Sha256
    );
    assert_eq!("sha-1".parse::<AlgorithmId>().unwrap(), AlgorithmId::Sha1);
    assert_eq!(
        "sha3256".parse::<AlgorithmId>().unwrap(),
        AlgorithmId::Sha3_256
    );
    assert_eq!(
        "blake2b512".parse::<AlgorithmId>().unwrap(),
        AlgorithmId::Blake2b512
    );
    assert_eq!(
        "xxhash64".parse::<AlgorithmId>().unwrap(),
        AlgorithmId::Xxh64
    );
    assert!("sha1024".parse::<AlgorithmId>().is_err());
}

#[test]
fn every_algorithm_matches_its_descriptor_length() {
    for &algorithm in AlgorithmId::ALL {
        let descriptor = algorithm.descriptor();
        let digest = algorithm.hasher().finalize();
        assert_eq!(
            digest.as_bytes().len(),
            descriptor.digest_length_bytes,
            "descriptor length mismatch for {algorithm}"
        );
        let hex = digest.to_hex_lowercase();
        assert_eq!(hex.len(), descriptor.digest_length_bytes * 2);
        assert_eq!(
            Digest::from_hex(algorithm, &hex).expect("computed hex must parse"),
            digest
        );
    }
}
