//! Exit codes conforming to Section "Exit Codes" of CLI_SPECIFICATION.md.

/// Exit codes for the RapidHash CLI executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(i32)]
pub enum ExitCode {
    /// 0: All requested operations completed and all verifications matched.
    Success = 0,
    /// 1: At least one verification mismatch.
    VerificationMismatch = 1,
    /// 2: Invalid command-line usage or malformed expected digest.
    InvalidUsage = 2,
    /// 3: At least one input was missing or unreadable.
    InputUnreadable = 3,
    /// 4: Manifest syntax, encoding, or safety policy failure.
    ManifestFailure = 4,
    /// 5: Requested algorithm or feature is unsupported.
    UnsupportedFeature = 5,
    /// 70: Unexpected internal failure.
    InternalFailure = 70,
    /// 130: Interrupted by the user.
    Interrupted = 130,
}

impl ExitCode {
    /// Convert to the process exit code integer.
    #[inline]
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
