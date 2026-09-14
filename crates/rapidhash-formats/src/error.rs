//! Manifest error definitions.

use thiserror::Error;

/// Errors that can occur during manifest parsing and formatting.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum FormatError {
    /// Syntax error on a specific line of the manifest.
    #[error("manifest syntax error on line {line}: {reason}")]
    InvalidSyntax { line: usize, reason: String },

    /// The manifest could not be read due to invalid text encoding (non-UTF-8).
    #[error("invalid character encoding in manifest: {reason}")]
    InvalidEncoding { reason: String },

    /// Could not determine algorithm from digest length or manifest context.
    #[error("ambiguous or unsupported algorithm for digest of length {length} on line {line}")]
    AmbiguousAlgorithm { line: usize, length: usize },

    /// Underlying I/O error when reading or writing manifest files.
    #[error("manifest I/O failure on '{path}': {message}")]
    Io { path: String, message: String },
}

impl FormatError {
    /// Return the stable presentation message identifier.
    pub fn stable_id(&self) -> &'static str {
        match self {
            FormatError::InvalidSyntax { .. } => "verification-status-malformed",
            FormatError::InvalidEncoding { .. } => "verification-status-malformed",
            FormatError::AmbiguousAlgorithm { .. } => "verification-status-unsupported",
            FormatError::Io { .. } => "cli-error-io",
        }
    }
}
