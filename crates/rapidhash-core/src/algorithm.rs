//! Checksum algorithm models, descriptors, and streaming hash engine.

use crate::digest::Digest;
use crate::error::CoreError;
use blake2::{Blake2b512, Blake2s256};
use blake3::Hasher as Blake3HasherState;
use crc32fast::Hasher as Crc32HasherState;
use md5::Md5;
use sha1::Sha1;
use sha2::{Digest as Sha2DigestTrait, Sha256 as Sha256HasherState, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_512};
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;
use xxhash_rust::xxh3::Xxh3;
use xxhash_rust::xxh32::Xxh32;
use xxhash_rust::xxh64::Xxh64;

/// Category of an algorithm according to ALGORITHMS.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmCategory {
    /// Preferred modern algorithms for authentication and integrity (e.g. SHA-256, BLAKE3).
    ModernCryptographic,
    /// Historical algorithms for compatibility only (e.g. MD5, SHA-1).
    LegacyCryptographic,
    /// Fast integrity check algorithms not suitable for adversarial security (e.g. CRC32).
    NonCryptographic,
}

/// Metadata and policy descriptors for an algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlgorithmDescriptor {
    /// Canonical lowercase ASCII token (e.g. "sha256").
    pub id_str: &'static str,
    /// Output digest size in bytes.
    pub digest_length_bytes: usize,
    /// High-level category.
    pub category: AlgorithmCategory,
    /// Whether this algorithm is recommended for new hashes.
    pub is_recommended: bool,
    /// Whether standard formatting prefers uppercase hexadecimal (e.g. SFV CRC32).
    pub prefers_uppercase: bool,
}

/// Identifiers for supported checksum algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlgorithmId {
    /// SHA-256 (FIPS 180-4).
    Sha256,
    /// BLAKE3 (cryptographic tree hash).
    Blake3,
    /// SHA-512 (FIPS 180-4).
    Sha512,
    /// SHA-384 (FIPS 180-4).
    Sha384,
    /// SHA3-512 (FIPS 202).
    Sha3_512,
    /// SHA3-256 (FIPS 202).
    Sha3_256,
    /// BLAKE2b with a 512-bit digest (RFC 7693).
    Blake2b512,
    /// BLAKE2s with a 256-bit digest (RFC 7693).
    Blake2s256,
    /// CRC32 (IEEE 802.3 / ISO 3309).
    Crc32,
    /// CRC32C (Castagnoli polynomial).
    Crc32c,
    /// XXH3-64 (xxHash specification).
    Xxh3,
    /// XXH64 (xxHash specification).
    Xxh64,
    /// XXH32 (xxHash specification).
    Xxh32,
    /// MD5 (RFC 1321, legacy compatibility only).
    Md5,
    /// SHA-1 (FIPS 180-4, legacy compatibility only).
    Sha1,
}

impl AlgorithmId {
    /// Return the static descriptor for this algorithm.
    pub const fn descriptor(self) -> AlgorithmDescriptor {
        match self {
            AlgorithmId::Sha256 => AlgorithmDescriptor {
                id_str: "sha256",
                digest_length_bytes: 32,
                category: AlgorithmCategory::ModernCryptographic,
                is_recommended: true,
                prefers_uppercase: false,
            },
            AlgorithmId::Blake3 => AlgorithmDescriptor {
                id_str: "blake3",
                digest_length_bytes: 32,
                category: AlgorithmCategory::ModernCryptographic,
                is_recommended: true,
                prefers_uppercase: false,
            },
            AlgorithmId::Sha512 => AlgorithmDescriptor {
                id_str: "sha512",
                digest_length_bytes: 64,
                category: AlgorithmCategory::ModernCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Sha384 => AlgorithmDescriptor {
                id_str: "sha384",
                digest_length_bytes: 48,
                category: AlgorithmCategory::ModernCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Sha3_512 => AlgorithmDescriptor {
                id_str: "sha3-512",
                digest_length_bytes: 64,
                category: AlgorithmCategory::ModernCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Sha3_256 => AlgorithmDescriptor {
                id_str: "sha3-256",
                digest_length_bytes: 32,
                category: AlgorithmCategory::ModernCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Blake2b512 => AlgorithmDescriptor {
                id_str: "blake2b-512",
                digest_length_bytes: 64,
                category: AlgorithmCategory::ModernCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Blake2s256 => AlgorithmDescriptor {
                id_str: "blake2s-256",
                digest_length_bytes: 32,
                category: AlgorithmCategory::ModernCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Crc32 => AlgorithmDescriptor {
                id_str: "crc32",
                digest_length_bytes: 4,
                category: AlgorithmCategory::NonCryptographic,
                is_recommended: false,
                prefers_uppercase: true,
            },
            AlgorithmId::Crc32c => AlgorithmDescriptor {
                id_str: "crc32c",
                digest_length_bytes: 4,
                category: AlgorithmCategory::NonCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Xxh3 => AlgorithmDescriptor {
                id_str: "xxh3",
                digest_length_bytes: 8,
                category: AlgorithmCategory::NonCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Xxh64 => AlgorithmDescriptor {
                id_str: "xxh64",
                digest_length_bytes: 8,
                category: AlgorithmCategory::NonCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Xxh32 => AlgorithmDescriptor {
                id_str: "xxh32",
                digest_length_bytes: 4,
                category: AlgorithmCategory::NonCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Md5 => AlgorithmDescriptor {
                id_str: "md5",
                digest_length_bytes: 16,
                category: AlgorithmCategory::LegacyCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
            AlgorithmId::Sha1 => AlgorithmDescriptor {
                id_str: "sha1",
                digest_length_bytes: 20,
                category: AlgorithmCategory::LegacyCryptographic,
                is_recommended: false,
                prefers_uppercase: false,
            },
        }
    }

    /// Return all supported algorithms in display order.
    pub const ALL: &'static [AlgorithmId] = &[
        AlgorithmId::Sha256,
        AlgorithmId::Blake3,
        AlgorithmId::Sha512,
        AlgorithmId::Sha384,
        AlgorithmId::Sha3_512,
        AlgorithmId::Sha3_256,
        AlgorithmId::Blake2b512,
        AlgorithmId::Blake2s256,
        AlgorithmId::Crc32,
        AlgorithmId::Crc32c,
        AlgorithmId::Xxh3,
        AlgorithmId::Xxh64,
        AlgorithmId::Xxh32,
        AlgorithmId::Md5,
        AlgorithmId::Sha1,
    ];

    /// Return the canonical lowercase identifier string.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        self.descriptor().id_str
    }

    /// Instantiate a new streaming hasher for this algorithm.
    pub fn hasher(self) -> Box<dyn Hasher> {
        match self {
            AlgorithmId::Sha256 => Box::new(Sha256Hasher::new()),
            AlgorithmId::Blake3 => Box::new(Blake3Hasher::new()),
            AlgorithmId::Sha512 => Box::new(CryptoHasher::<Sha512>::new(AlgorithmId::Sha512)),
            AlgorithmId::Sha384 => Box::new(CryptoHasher::<Sha384>::new(AlgorithmId::Sha384)),
            AlgorithmId::Sha3_512 => Box::new(CryptoHasher::<Sha3_512>::new(AlgorithmId::Sha3_512)),
            AlgorithmId::Sha3_256 => Box::new(CryptoHasher::<Sha3_256>::new(AlgorithmId::Sha3_256)),
            AlgorithmId::Blake2b512 => {
                Box::new(CryptoHasher::<Blake2b512>::new(AlgorithmId::Blake2b512))
            }
            AlgorithmId::Blake2s256 => {
                Box::new(CryptoHasher::<Blake2s256>::new(AlgorithmId::Blake2s256))
            }
            AlgorithmId::Crc32 => Box::new(Crc32Hasher::new()),
            AlgorithmId::Crc32c => Box::new(Crc32cHasher::new()),
            AlgorithmId::Xxh3 => Box::new(Xxh3Hasher::new()),
            AlgorithmId::Xxh64 => Box::new(Xxh64Hasher::new()),
            AlgorithmId::Xxh32 => Box::new(Xxh32Hasher::new()),
            AlgorithmId::Md5 => Box::new(CryptoHasher::<Md5>::new(AlgorithmId::Md5)),
            AlgorithmId::Sha1 => Box::new(CryptoHasher::<Sha1>::new(AlgorithmId::Sha1)),
        }
    }
}

impl fmt::Display for AlgorithmId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for AlgorithmId {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "sha256" | "sha-256" => Ok(AlgorithmId::Sha256),
            "blake3" => Ok(AlgorithmId::Blake3),
            "sha512" | "sha-512" => Ok(AlgorithmId::Sha512),
            "sha384" | "sha-384" => Ok(AlgorithmId::Sha384),
            "sha3-512" | "sha3_512" | "sha3512" | "sha-3-512" => Ok(AlgorithmId::Sha3_512),
            "sha3-256" | "sha3_256" | "sha3256" | "sha-3-256" => Ok(AlgorithmId::Sha3_256),
            "blake2b-512" | "blake2b512" | "blake2b" => Ok(AlgorithmId::Blake2b512),
            "blake2s-256" | "blake2s256" | "blake2s" => Ok(AlgorithmId::Blake2s256),
            "crc32" => Ok(AlgorithmId::Crc32),
            "crc32c" | "crc32-castagnoli" => Ok(AlgorithmId::Crc32c),
            "xxh3" | "xxh3-64" | "xxhash3" | "xxhash-3" => Ok(AlgorithmId::Xxh3),
            "xxh64" | "xxhash64" | "xxhash-64" => Ok(AlgorithmId::Xxh64),
            "xxh32" | "xxhash32" | "xxhash-32" => Ok(AlgorithmId::Xxh32),
            "md5" => Ok(AlgorithmId::Md5),
            "sha1" | "sha-1" => Ok(AlgorithmId::Sha1),
            other => Err(CoreError::UnsupportedAlgorithm {
                algorithm: other.to_string(),
            }),
        }
    }
}

/// Abstract streaming hasher interface.
pub trait Hasher: Send {
    /// Return the algorithm associated with this hasher.
    fn algorithm(&self) -> AlgorithmId;

    /// Update the hash state with incoming input bytes.
    fn update(&mut self, data: &[u8]);

    /// Finalize the hash state and produce the final Digest.
    fn finalize(self: Box<Self>) -> Digest;
}

/// Streaming SHA-256 hasher.
pub struct Sha256Hasher {
    state: Sha256HasherState,
}

impl Sha256Hasher {
    /// Create a new SHA-256 hasher instance.
    pub fn new() -> Self {
        Self {
            state: Sha256HasherState::new(),
        }
    }
}

impl Default for Sha256Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Sha256Hasher {
    fn algorithm(&self) -> AlgorithmId {
        AlgorithmId::Sha256
    }

    fn update(&mut self, data: &[u8]) {
        Sha2DigestTrait::update(&mut self.state, data);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let output = self.state.finalize();
        Digest::new(AlgorithmId::Sha256, output.to_vec())
            .expect("SHA-256 output length is always valid")
    }
}

/// Streaming BLAKE3 hasher.
pub struct Blake3Hasher {
    state: Blake3HasherState,
}

impl Blake3Hasher {
    /// Create a new BLAKE3 hasher instance.
    pub fn new() -> Self {
        Self {
            state: Blake3HasherState::new(),
        }
    }
}

impl Default for Blake3Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Blake3Hasher {
    fn algorithm(&self) -> AlgorithmId {
        AlgorithmId::Blake3
    }

    fn update(&mut self, data: &[u8]) {
        self.state.update(data);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let output = self.state.finalize();
        Digest::new(AlgorithmId::Blake3, output.as_bytes().to_vec())
            .expect("BLAKE3 output length is always valid")
    }
}

/// Streaming CRC32 hasher.
pub struct Crc32Hasher {
    state: Crc32HasherState,
}

impl Crc32Hasher {
    /// Create a new CRC32 hasher instance.
    pub fn new() -> Self {
        Self {
            state: Crc32HasherState::new(),
        }
    }
}

impl Default for Crc32Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Crc32Hasher {
    fn algorithm(&self) -> AlgorithmId {
        AlgorithmId::Crc32
    }

    fn update(&mut self, data: &[u8]) {
        self.state.update(data);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let crc = self.state.finalize();
        Digest::new(AlgorithmId::Crc32, crc.to_be_bytes().to_vec())
            .expect("CRC32 output length is always valid")
    }
}

/// Streaming hasher backed by a RustCrypto fixed-output digest.
pub struct CryptoHasher<D> {
    state: D,
    algorithm: AlgorithmId,
}

impl<D: Sha2DigestTrait + Send + 'static> CryptoHasher<D> {
    /// Create a new hasher state for the given algorithm.
    pub fn new(algorithm: AlgorithmId) -> Self {
        Self {
            state: D::new(),
            algorithm,
        }
    }
}

impl<D: Sha2DigestTrait + Send + 'static> Hasher for CryptoHasher<D> {
    fn algorithm(&self) -> AlgorithmId {
        self.algorithm
    }

    fn update(&mut self, data: &[u8]) {
        Sha2DigestTrait::update(&mut self.state, data);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let output = self.state.finalize();
        Digest::new(self.algorithm, output.to_vec())
            .expect("RustCrypto digest output length matches the descriptor")
    }
}

/// Streaming CRC32C (Castagnoli) hasher.
pub struct Crc32cHasher {
    crc: u32,
}

impl Crc32cHasher {
    /// Create a new CRC32C hasher instance.
    pub fn new() -> Self {
        Self { crc: 0 }
    }
}

impl Default for Crc32cHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Crc32cHasher {
    fn algorithm(&self) -> AlgorithmId {
        AlgorithmId::Crc32c
    }

    fn update(&mut self, data: &[u8]) {
        self.crc = crc32c::crc32c_append(self.crc, data);
    }

    fn finalize(self: Box<Self>) -> Digest {
        Digest::new(AlgorithmId::Crc32c, self.crc.to_be_bytes().to_vec())
            .expect("CRC32C output length is always valid")
    }
}

/// Streaming XXH32 hasher.
pub struct Xxh32Hasher {
    state: Xxh32,
}

impl Xxh32Hasher {
    /// Create a new XXH32 hasher instance with a zero seed.
    pub fn new() -> Self {
        Self {
            state: Xxh32::new(0),
        }
    }
}

impl Default for Xxh32Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Xxh32Hasher {
    fn algorithm(&self) -> AlgorithmId {
        AlgorithmId::Xxh32
    }

    fn update(&mut self, data: &[u8]) {
        self.state.update(data);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let value = self.state.digest();
        Digest::new(AlgorithmId::Xxh32, value.to_be_bytes().to_vec())
            .expect("XXH32 output length is always valid")
    }
}

/// Streaming XXH64 hasher.
pub struct Xxh64Hasher {
    state: Xxh64,
}

impl Xxh64Hasher {
    /// Create a new XXH64 hasher instance with a zero seed.
    pub fn new() -> Self {
        Self {
            state: Xxh64::new(0),
        }
    }
}

impl Default for Xxh64Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Xxh64Hasher {
    fn algorithm(&self) -> AlgorithmId {
        AlgorithmId::Xxh64
    }

    fn update(&mut self, data: &[u8]) {
        self.state.update(data);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let value = self.state.digest();
        Digest::new(AlgorithmId::Xxh64, value.to_be_bytes().to_vec())
            .expect("XXH64 output length is always valid")
    }
}

/// Streaming XXH3-64 hasher.
pub struct Xxh3Hasher {
    state: Xxh3,
}

impl Xxh3Hasher {
    /// Create a new XXH3 hasher instance with a zero seed.
    pub fn new() -> Self {
        Self { state: Xxh3::new() }
    }
}

impl Default for Xxh3Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for Xxh3Hasher {
    fn algorithm(&self) -> AlgorithmId {
        AlgorithmId::Xxh3
    }

    fn update(&mut self, data: &[u8]) {
        self.state.update(data);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let value = self.state.digest();
        Digest::new(AlgorithmId::Xxh3, value.to_be_bytes().to_vec())
            .expect("XXH3 output length is always valid")
    }
}

/// Single-pass multi-hasher feeding multiple algorithms from one buffer.
///
/// Implements Section 6 of ARCHITECTURE.md.
pub struct MultiHasher {
    hashers: Vec<Box<dyn Hasher>>,
}

impl MultiHasher {
    /// Create a new MultiHasher for the specified set of algorithms.
    pub fn new(algorithms: &[AlgorithmId]) -> Self {
        let hashers = algorithms.iter().map(|&algo| algo.hasher()).collect();
        Self { hashers }
    }

    /// Update all managed algorithms with the provided byte slice.
    pub fn update(&mut self, data: &[u8]) {
        for hasher in &mut self.hashers {
            hasher.update(data);
        }
    }

    /// Read through a reader to completion in chunks and update all algorithms.
    pub fn update_from_reader<R: Read>(
        &mut self,
        mut reader: R,
        buffer_size: usize,
    ) -> io::Result<u64> {
        let mut buffer = vec![0u8; buffer_size.max(4096)];
        let mut total_bytes = 0u64;
        loop {
            let read = reader.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            self.update(&buffer[..read]);
            total_bytes += read as u64;
        }
        Ok(total_bytes)
    }

    /// Finalize all algorithms and return calculated digests.
    pub fn finalize(self) -> std::collections::BTreeMap<AlgorithmId, Digest> {
        let mut results = std::collections::BTreeMap::new();
        for hasher in self.hashers {
            let algo = hasher.algorithm();
            let digest = hasher.finalize();
            results.insert(algo, digest);
        }
        results
    }
}
