# Product Requirements

## 1. Purpose

RapidHash enables people to calculate, compare, generate, and verify file
checksums through a consistent desktop application and command-line interface on
Windows, macOS, and Linux.

## 2. Product Principles

1. **Correct before fast.** A fast incorrect digest is a critical defect.
2. **Local by default.** File contents and digests are not transmitted.
3. **Cross-platform by design.** The shared core defines behavior; platform
   integrations remain optional adapters.
4. **Explicit trust.** The UI distinguishes integrity checks from cryptographic
   authenticity and identifies legacy algorithms.
5. **Responsive at scale.** Long-running work is observable and cancellable.
6. **Safe with hostile input.** Paths and manifests are always untrusted.

## 3. Target Users

- People validating downloaded files against a publisher-provided digest.
- Release engineers generating checksum manifests.
- Archivists detecting accidental file corruption.
- Developers and administrators automating verification with a CLI.
- Users migrating from platform-specific checksum utilities.

## 4. Primary Workflows

### 4.1 Calculate

The user selects or drops files and directories, selects one or more algorithms,
and receives digests with progress, throughput, and per-item status.

### 4.2 Compare

The user pastes a digest or drops text containing a recognizable digest. The
application compares it without case sensitivity and reports match, mismatch, or
ambiguous input. The original text is never executed or interpreted as a path
unless the user explicitly opens it as a manifest.

### 4.3 Verify a Manifest

The user opens a checksum manifest and selects or confirms its root directory.
RapidHash parses supported records, confines relative paths to the approved
root, computes required algorithms, and reports matched, mismatched, missing,
unreadable, unsupported, and malformed entries separately.

### 4.4 Generate a Manifest

The user selects files, an algorithm, path style, output encoding, and line
ending. RapidHash writes deterministic output without silently overwriting an
existing file.

### 4.5 Automate

The user invokes the CLI and receives stable exit codes plus human-readable or
JSON output.

## 5. Functional Requirements

### Core

- Stream files with bounded memory use.
- Calculate multiple requested algorithms in one read pass when practical.
- Traverse directories deterministically and support explicit exclusions.
- Limit concurrency and remain responsive during cancellation.
- Detect files that change during calculation and report the result as unstable.
- Preserve raw operating-system paths internally without lossy conversion.

### Desktop

- Select and drag files or directories.
- Add work to a queue without blocking the UI.
- Show overall and per-item progress when size is known.
- Pause queue admission and cancel active work.
- Sort, filter, select, and copy results.
- Explain errors with actionable messages while retaining technical details.
- Restore non-sensitive preferences, but not file history by default.

### CLI

- Support explicit algorithms, recursive traversal, manifests, and verification.
- Write ordinary output to stdout and diagnostics to stderr.
- Provide deterministic JSON with a versioned schema.
- Support cancellation through standard process signals.
- Never use color when output is not a terminal unless explicitly requested.

### Manifests

- Support the formats in [CHECKSUM_FORMATS.md](CHECKSUM_FORMATS.md).
- Detect supported Unicode encodings conservatively.
- Preserve file-name bytes or platform path semantics where the format permits.
- Require explicit confirmation before accepting paths outside a selected root.

## 6. Non-Functional Requirements

- Correctness is tested with published vectors and independent tools.
- Memory usage is bounded by worker count and buffer policy, not total input size.
- The core has no network dependency.
- The desktop application remains interactive under sustained hashing load.
- Accessibility meets [ACCESSIBILITY.md](ACCESSIBILITY.md).
- Release artifacts are reproducible where the platform toolchain permits.

## 7. Out of Scope for the First Stable Release

- Proving publisher identity or replacing digital signatures.
- Malware classification or automatic reputation lookup.
- Repairing corrupted files.
- Remote cloud storage providers.
- Mobile platforms.
- A general-purpose file manager.

## 8. Success Criteria for 1.0

- Supported algorithm results match all project vectors on all tier-one targets.
- All required manifest compatibility fixtures pass.
- A user can calculate and verify checksums without consulting documentation.
- Cancellation completes within the defined responsiveness budget.
- No known critical or high-severity security defect remains open.
- Install, update, and uninstall paths are documented and tested per platform.

## 9. Open Product Questions

Open questions must be resolved in issues or ADRs before they become release
commitments:

- Whether file-manager integrations ship in 1.0 or a later feature release.
- Whether a local, opt-in history is valuable enough to justify its privacy cost.
- Which Linux packaging formats qualify as tier one after maintainer testing.
