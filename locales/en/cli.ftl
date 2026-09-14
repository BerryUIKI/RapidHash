# CLI messages, descriptions, and diagnostics

# Command descriptions
cli-description = RapidHash: Fast, cross-platform checksum verification and generation
cli-hash-description = Calculate cryptographic and non-cryptographic checksums for files or directories
cli-verify-description = Verify files against an existing checksum manifest
cli-compare-description = Compare a single file against an expected digest
cli-algorithms-description = List available checksum algorithms and their status
cli-completions-description = Generate shell completion scripts

# Arguments and options
cli-arg-path = Target file or directory path
cli-arg-manifest = Checksum manifest file path
cli-arg-digest = Expected hexadecimal digest
cli-arg-shell = Target shell name

cli-opt-algorithm = Checksum algorithm to use
cli-opt-format = Manifest format to use
cli-opt-output = Write output to the specified file path
cli-opt-json = Emit results as structured JSON
cli-opt-quiet = Suppress informational messages and progress
cli-opt-verbose = Enable detailed diagnostic output
cli-opt-recursive = Traverse directories recursively
cli-opt-root = Set approved root directory for manifest paths

# Output and progress summaries
cli-summary-verified = Verified { $total } files: { $matched } matched, { $mismatched } mismatched, { $failed } failed
cli-item-progress = Processing { $path } ({ $percent }%)
cli-compare-matched = The checksum for { $path } matches the expected digest.
cli-compare-mismatched = The checksum for { $path } does not match the expected digest.

# Errors and diagnostics
cli-error-file-not-found = File not found: { $path }
cli-error-permission-denied = Permission denied: { $path }
cli-error-invalid-digest = Invalid hexadecimal digest format: { $digest }
cli-error-unsupported-algorithm = Unsupported algorithm: { $algorithm }
cli-error-path-traversal = Path attempts to escape approved root: { $path }
cli-error-io = I/O error occurred while reading { $path }: { $error }
