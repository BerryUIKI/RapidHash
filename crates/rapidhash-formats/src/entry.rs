//! Manifest entry representation.

use rapidhash_core::Digest;

/// Format category of a checksum manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestFormat {
    /// GNU coreutils style single-algorithm manifest (<digest><space><space><path>).
    Gnu,
    /// Simple File Verification format (<path><space><8-digit CRC32>).
    Sfv,
}

/// A parsed record from a checksum manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEntry {
    /// Target path string as extracted from the manifest.
    pub path_str: String,
    /// Parsed digest value and associated algorithm.
    pub digest: Digest,
    /// 1-based source line number in the manifest.
    pub line_number: usize,
}

impl ManifestEntry {
    /// Create a new manifest entry record.
    pub fn new(path_str: impl Into<String>, digest: Digest, line_number: usize) -> Self {
        Self {
            path_str: path_str.into(),
            digest,
            line_number,
        }
    }
}
