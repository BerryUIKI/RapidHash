# Security Model

## 1. Security Goals

- Calculate the requested digest over the intended bytes.
- Prevent manifests from accessing files outside an approved root by default.
- Avoid executing or rendering untrusted file content.
- Bound CPU, memory, file-descriptor, and UI resource consumption.
- Preserve release and dependency integrity.
- Keep file contents, paths, and digests local unless explicitly authorized.

## 2. Non-Goals

RapidHash does not establish file authorship, publisher identity, malware safety,
or the trustworthiness of an expected digest. It is not a digital-signature
system, antivirus product, or forensic chain-of-custody tool.

## 3. Trust Boundaries

Untrusted inputs include files, directories, manifests, paths, command-line
arguments, drag-and-drop payloads, localization resources, settings, environment
variables, IPC messages, and dependency/build outputs.

The frontend is less trusted than the Rust core for file access. Tauri commands
validate all parameters and capabilities permit only required operations.

## 4. Principal Threats and Controls

### Manifest Path Traversal

Resolve paths against an explicitly approved root, reject disallowed absolute
paths, normalize lexical components, and verify containment. Tests include mixed
separators, encoded characters, Unicode confusables, drive prefixes, UNC paths,
and symbolic-link boundaries.

### Time-of-Check to Time-of-Use Changes

Record metadata before and after reading. If relevant identity, size, or
modification data changes, mark the result unstable rather than presenting it as
an ordinary verified result. Stronger handle-based identity checks may be added
per platform.

### Resource Exhaustion

Bound traversal queues, open handles, workers, buffers, manifest record size,
line length, entry count, and progress frequency. Cancellation is available for
all potentially long operations.

### Malformed Input

Parsers avoid recursion proportional to attacker-controlled depth, unchecked
integer conversions, implicit lossy decoding, and panic-based error handling.
Fuzz manifest parsers and serialized command boundaries.

### Symbolic Links and Cycles

Do not follow directory links by default. If enabled, track directory identity,
detect cycles, disclose cross-root behavior, and retain the approved-root policy.

### Frontend Injection

Render paths and errors as text, never HTML. Use a restrictive Content Security
Policy and no remote scripts. Keep Tauri command and filesystem capabilities
minimal.

### Supply Chain

Commit lockfiles, review dependency changes, automate vulnerability and license
scanning, minimize build-script dependencies, and pin CI actions to immutable
revisions before release.

### Release Integrity

Build on controlled CI runners, protect release tags, publish checksums and
provenance, and sign/notarize artifacts where platforms support it. Signing keys
must not be stored in the repository.

## 5. Unsafe Rust

Unsafe Rust is denied by default in project crates. An exception requires a
documented safety invariant, a narrow wrapper, tests, an ADR when architectural,
and explicit reviewer approval.

## 6. Network Policy

Core hashing and manifest verification perform no network access. Update checks
or reputation services require a separate opt-in design, privacy review, threat
model update, and visible UI disclosure.

## 7. Security Testing

- Unit and property tests for path policy and parsers.
- Coverage-guided fuzzing of manifest inputs and IPC serialization.
- Dependency vulnerability and license checks.
- Static linting with warnings denied in CI.
- Manual platform integration and packaging review.
- Adversarial fixtures for large, cyclic, changing, and permission-denied trees.
