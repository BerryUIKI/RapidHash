//! Job and verification result models.

use crate::algorithm::AlgorithmId;
use crate::digest::Digest;
use crate::error::CoreError;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Verification outcome according to UX specification Section 4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationStatus {
    /// Calculated bytes equal the expected digest.
    Match,
    /// Calculation succeeded but bytes differ.
    Mismatch,
    /// A manifest target does not exist.
    Missing,
    /// The target exists but cannot be read.
    Unreadable,
    /// The expected entry or manifest record cannot be interpreted safely.
    Malformed,
    /// The requested algorithm or record is recognized but unavailable.
    Unsupported,
    /// The user or scheduler stopped processing.
    Cancelled,
}

impl VerificationStatus {
    /// Return the stable Fluent catalog identifier for this status.
    pub fn stable_id(&self) -> &'static str {
        match self {
            VerificationStatus::Match => "verification-status-match",
            VerificationStatus::Mismatch => "verification-status-mismatch",
            VerificationStatus::Missing => "verification-status-missing",
            VerificationStatus::Unreadable => "verification-status-unreadable",
            VerificationStatus::Malformed => "verification-status-malformed",
            VerificationStatus::Unsupported => "verification-status-unsupported",
            VerificationStatus::Cancelled => "verification-status-cancelled",
        }
    }

    /// Whether this status indicates an operational success (digest matched).
    #[inline]
    pub fn is_success(&self) -> bool {
        matches!(self, VerificationStatus::Match)
    }
}

/// Result record for an individual item (file) processed by RapidHash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemResult {
    /// Path of the processed item.
    pub path: PathBuf,
    /// Size of the processed item in bytes, if determined.
    pub size_bytes: Option<u64>,
    /// Calculated digests indexed by algorithm.
    pub digests: BTreeMap<AlgorithmId, Digest>,
    /// Verification status, if this was part of a verification/comparison operation.
    pub status: Option<VerificationStatus>,
    /// Error details, if an error occurred for this item.
    pub error: Option<CoreError>,
}

impl ItemResult {
    /// Create a new successful calculation record.
    pub fn new_success(
        path: PathBuf,
        size_bytes: Option<u64>,
        digests: BTreeMap<AlgorithmId, Digest>,
    ) -> Self {
        Self {
            path,
            size_bytes,
            digests,
            status: None,
            error: None,
        }
    }

    /// Create an item record that failed with an error.
    pub fn new_failure(
        path: PathBuf,
        size_bytes: Option<u64>,
        status: VerificationStatus,
        error: CoreError,
    ) -> Self {
        Self {
            path,
            size_bytes,
            digests: BTreeMap::new(),
            status: Some(status),
            error: Some(error),
        }
    }
}
