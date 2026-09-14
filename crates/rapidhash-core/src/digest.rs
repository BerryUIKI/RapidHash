//! Digest representation and formatting.

use crate::algorithm::AlgorithmId;
use crate::error::CoreError;
use std::fmt;

/// An algorithm-tagged digest containing computed raw bytes.
#[derive(Debug, Clone, Eq)]
pub struct Digest {
    algorithm: AlgorithmId,
    bytes: Vec<u8>,
}

impl Digest {
    /// Create a new digest for the given algorithm from raw bytes.
    pub fn new(algorithm: AlgorithmId, bytes: Vec<u8>) -> Result<Self, CoreError> {
        let expected_len = algorithm.descriptor().digest_length_bytes;
        if bytes.len() != expected_len {
            return Err(CoreError::InvalidDigestLength {
                algorithm: algorithm.as_str().to_string(),
                expected: expected_len,
                actual: bytes.len(),
            });
        }
        Ok(Self { algorithm, bytes })
    }

    /// Return the algorithm associated with this digest.
    #[inline]
    pub fn algorithm(&self) -> AlgorithmId {
        self.algorithm
    }

    /// Return a reference to the raw digest bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume the digest and return the underlying byte buffer.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Format the digest bytes as a lowercase hexadecimal string.
    pub fn to_hex_lowercase(&self) -> String {
        let mut string = String::with_capacity(self.bytes.len() * 2);
        for &byte in &self.bytes {
            use std::fmt::Write;
            let _ = write!(string, "{byte:02x}");
        }
        string
    }

    /// Format the digest bytes as an uppercase hexadecimal string.
    pub fn to_hex_uppercase(&self) -> String {
        let mut string = String::with_capacity(self.bytes.len() * 2);
        for &byte in &self.bytes {
            use std::fmt::Write;
            let _ = write!(string, "{byte:02X}");
        }
        string
    }

    /// Format the digest using the algorithm's canonical casing convention.
    #[inline]
    pub fn to_canonical_hex(&self) -> String {
        if self.algorithm.descriptor().prefers_uppercase {
            self.to_hex_uppercase()
        } else {
            self.to_hex_lowercase()
        }
    }

    /// Parse a hexadecimal string for a specific algorithm.
    ///
    /// Accepts uppercase, lowercase, and mixed-case hex characters.
    pub fn from_hex(algorithm: AlgorithmId, hex: &str) -> Result<Self, CoreError> {
        let trimmed = hex.trim();
        let expected_chars = algorithm.descriptor().digest_length_bytes * 2;
        if trimmed.len() != expected_chars {
            return Err(CoreError::InvalidDigestHex {
                digest: hex.to_string(),
                reason: format!(
                    "expected {expected_chars} hex characters, found {}",
                    trimmed.len()
                ),
            });
        }

        let mut bytes = Vec::with_capacity(algorithm.descriptor().digest_length_bytes);
        for chunk in trimmed.as_bytes().chunks_exact(2) {
            let high = hex_val(chunk[0]).ok_or_else(|| CoreError::InvalidDigestHex {
                digest: hex.to_string(),
                reason: format!("invalid character '{}'", chunk[0] as char),
            })?;
            let low = hex_val(chunk[1]).ok_or_else(|| CoreError::InvalidDigestHex {
                digest: hex.to_string(),
                reason: format!("invalid character '{}'", chunk[1] as char),
            })?;
            bytes.push((high << 4) | low);
        }

        Self::new(algorithm, bytes)
    }

    /// Constant-time comparison between two byte slices.
    ///
    /// Prevents timing attacks when verifying expected digests.
    #[inline]
    pub fn constant_time_eq(&self, other: &Self) -> bool {
        if self.algorithm != other.algorithm || self.bytes.len() != other.bytes.len() {
            return false;
        }
        let mut diff = 0u8;
        for (a, b) in self.bytes.iter().zip(other.bytes.iter()) {
            diff |= a ^ b;
        }
        diff == 0
    }
}

fn hex_val(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

impl PartialEq for Digest {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.constant_time_eq(other)
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical_hex())
    }
}
