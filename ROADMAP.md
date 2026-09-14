# Roadmap

The roadmap communicates intent rather than a delivery guarantee. Priorities may
change after design validation, benchmarks, security review, or contributor
feedback.

## Phase 0: Foundation

- Establish governance, licensing, contribution, branch, and security policies.
- Record architecture decisions.
- Define product, compatibility, format, accessibility, and testing contracts.
- Protect `main` and require reviewed pull requests.

## Phase 1: Hashing Core and CLI

- Create the Rust workspace and core domain model.
- Implement bounded streaming file reads and cancellation.
- Add CRC32, CRC32C, MD5, SHA-1, SHA-256, SHA-384, SHA-512, and BLAKE3.
- Add deterministic CLI output and machine-readable JSON output.
- Validate algorithms with published vectors and independent implementations.

## Phase 2: Desktop MVP

- Create the Tauri 2 and React/TypeScript desktop application.
- Support file and directory selection and drag-and-drop.
- Display queued, active, completed, cancelled, and failed tasks.
- Copy digests and verify a pasted digest.
- Support English and Simplified Chinese UI resources.
- Meet keyboard and high-contrast accessibility requirements.

## Phase 3: Manifest Workflows

- Parse and write SFV and GNU-style checksum manifests.
- Add UTF-8, UTF-8 with BOM, and UTF-16 LE input compatibility.
- Securely resolve relative paths within a selected root.
- Report matched, mismatched, missing, unreadable, and malformed entries.

## Phase 4: Advanced Algorithms and Scale

- Add SHA-3, BLAKE2, xxHash32, xxHash64, and XXH3.
- Tune concurrency for SSDs, hard drives, and network file systems.
- Add large-directory filtering, sorting, and export.
- Publish repeatable performance baselines.

## Phase 5: Platform Integration and Distribution

- Add optional Windows Explorer actions.
- Add a macOS Finder service or Quick Action.
- Add supported Linux file-manager actions where maintainable.
- Ship signed or verifiable platform-native packages.
- Document update channels and release provenance.

## Deferred

- Mobile applications.
- Cloud file storage integrations.
- Automatic malware or reputation lookups.
- A persistent history database.
- A plugin system.

Deferred items require separate privacy, security, and maintenance review.
