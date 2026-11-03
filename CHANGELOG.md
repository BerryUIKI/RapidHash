# Changelog

All notable changes to RapidHash will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and
the project intends to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
after its public API and release process stabilize.

## [Unreleased]

### Added

- Rust core hashing engine with streaming I/O, cancellation, deterministic
  traversal, and path-confined manifest resolution.
- Checksum algorithms: SHA-256, SHA-384, SHA-512, SHA3-256, SHA3-512,
  BLAKE2b-512, BLAKE2s-256, BLAKE3, MD5, SHA-1, CRC32, CRC32C, XXH3, XXH64, and
  XXH32, validated against published vectors.
- Command-line interface with `hash`, `verify`, `compare`, and `algorithms`
  subcommands, structured JSON output, and localized presentation.
- GNU-style and SFV manifest parsing and serialization with UTF-8, UTF-8 with
  BOM, and UTF-16 LE input tolerance.
- Tauri 2 desktop application with React and TypeScript, drag-and-drop and file
  picker input, CRC-into-filename, manifest export, and checksum verification.
- Optional Windows Explorer context menu integration using per-user registry
  verbs, single-instance path routing, and an in-app settings toggle.
- Mozilla Fluent catalogs for English and Simplified Chinese with structural
  parity checks.
- Initial project documentation, governance, licensing, and architecture records.

### Changed

- Desktop settings now cover CRC filename formatting and Windows Explorer
  integration alongside language and appearance.

[Unreleased]: https://github.com/BerryUIKI/RapidHash/commits/main
