//! Checksum algorithm models, descriptors, and streaming hash engine.

use crate::digest::Digest;
use crate::error::CoreError;
use blake3::Hasher as Blake3HasherState;
use crc32fast::Hasher as Crc32HasherState;
use sha2::{Digest as Sha2DigestTrait, Sha256 as Sha256HasherState};
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

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
    /// CRC32 (IEEE 802.3 / ISO 3309).
    Crc32,
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
            AlgorithmId::Crc32 => AlgorithmDescriptor {
                id_str: "crc32",
                digest_length_bytes: 4,
                category: AlgorithmCategory::NonCryptographic,
                is_recommended: false,
                prefers_uppercase: true,
            },
        }
    }

    /// Return all supported algorithms in the initial scaffold.
    pub const ALL: &'static [AlgorithmId] =
        &[AlgorithmId::Sha256, AlgorithmId::Blake3, AlgorithmId::Crc32];

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
            AlgorithmId::Crc32 => Box::new(Crc32Hasher::new()),
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
            "crc32" => Ok(AlgorithmId::Crc32),
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
