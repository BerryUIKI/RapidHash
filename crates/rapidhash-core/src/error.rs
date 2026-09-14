//! Core domain error types with presentation-stable message identifiers.

use thiserror::Error;

/// Structured domain errors emitted by rapidhash-core operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// An unknown or unsupported algorithm string was specified.
    #[error("unsupported algorithm: {algorithm}")]
    UnsupportedAlgorithm { algorithm: String },

    /// The provided byte slice does not match the required length for the algorithm.
    #[error(
        "invalid digest length for algorithm '{algorithm}': expected {expected}, got {actual}"
    )]
    InvalidDigestLength {
        algorithm: String,
        expected: usize,
        actual: usize,
    },

    /// The provided string could not be parsed as a hexadecimal digest.
    #[error("invalid hexadecimal digest '{digest}': {reason}")]
    InvalidDigestHex { digest: String, reason: String },

    /// A path was outside or escaped the approved root directory.
    #[error("path attempts to escape approved root: {path}")]
    PathEscape { path: String },

    /// File or resource was not found.
    #[error("file not found: {path}")]
    NotFound { path: String },

    /// Permission denied when accessing a file or path.
    #[error("permission denied: {path}")]
    PermissionDenied { path: String },

    /// An I/O error occurred with an explanatory context.
    #[error("I/O failure on '{path}': {message}")]
    Io { path: String, message: String },

    /// The operation was cancelled cooperatively by the user or supervisor.
    #[error("operation cancelled")]
    Cancelled,
}

impl CoreError {
    /// Stable message identifier corresponding to Fluent catalog entries.
    pub fn stable_id(&self) -> &'static str {
        match self {
            CoreError::UnsupportedAlgorithm { .. } => "cli-error-unsupported-algorithm",
            CoreError::InvalidDigestLength { .. } | CoreError::InvalidDigestHex { .. } => {
                "cli-error-invalid-digest"
            }
            CoreError::PathEscape { .. } => "cli-error-path-traversal",
            CoreError::NotFound { .. } => "cli-error-file-not-found",
            CoreError::PermissionDenied { .. } => "cli-error-permission-denied",
            CoreError::Io { .. } => "cli-error-io",
            CoreError::Cancelled => "verification-status-cancelled",
        }
    }
}
