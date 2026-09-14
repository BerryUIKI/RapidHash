# Common domain terms, verification states, and shared messages

app-name = RapidHash

# Verification states
verification-status-match = Match
    .description = The calculated checksum matches the expected digest exactly.
verification-status-mismatch = Mismatch
    .description = The calculated checksum differs from the expected digest.
verification-status-missing = Missing
    .description = The target file does not exist.
verification-status-unreadable = Unreadable
    .description = The target file exists but could not be read.
verification-status-malformed = Malformed
    .description = The checksum entry or manifest record is invalid.
verification-status-unsupported = Unsupported
    .description = The requested algorithm or format is not supported.
verification-status-cancelled = Cancelled
    .description = The operation was stopped by the user.

# Common actions and controls
action-cancel = Cancel
action-retry = Retry
action-copy = Copy
action-close = Close

# Common notices and security guidance
notice-checksum-not-authenticity = A matching checksum verifies integrity, not safety or authenticity.
