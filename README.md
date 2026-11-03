# RapidHash

RapidHash is a fast, safe, and cross-platform checksum application for Windows,
macOS, and Linux. It combines a reusable Rust hashing engine, a command-line
interface, and a Tauri desktop application.

> [!IMPORTANT]
> RapidHash is pre-alpha. The Rust core, CLI, and desktop application are under
> active development on `dev`; no production release has been published yet.

## Goals

- Calculate checksums for files and directory trees without loading entire files
  into memory.
- Verify pasted digests and common checksum manifests.
- Generate portable, Unicode-safe checksum manifests.
- Provide a responsive desktop experience and a scriptable CLI backed by the
  same Rust core.
- Behave consistently across Windows, macOS, and Linux.
- Remain private by default: files and checksums stay on the local machine unless
  the user explicitly chooses an external integration.

## Supported Algorithms

RapidHash calculates the following checksums:

- **Modern cryptographic:** SHA-256, SHA-384, SHA-512, SHA3-256, SHA3-512,
  BLAKE2b-512, BLAKE2s-256, and BLAKE3.
- **Legacy cryptographic:** MD5 and SHA-1, clearly marked as unsuitable for
  security uses.
- **Non-cryptographic:** CRC32, CRC32C, XXH3, XXH64, and XXH32 for fast
  integrity checks.

See [Algorithm Policy](docs/ALGORITHMS.md) for identifiers, digest lengths, and
hex formatting rules.

## Interfaces

- **Desktop:** Tauri 2 with a React and TypeScript user interface.
- **CLI:** a native Rust executable suitable for shells and automation.
- **Platform integrations:** optional file-manager actions implemented separately
  for each operating system; Windows Explorer context menu integration is
  available today.

The project architecture is described in [Architecture](docs/ARCHITECTURE.md),
and the delivery sequence is maintained in the [Roadmap](ROADMAP.md).

## Project Status

RapidHash is currently **pre-alpha**. The hashing engine, CLI, manifest
workflows, and desktop application are implemented and under active review on
`dev`; the stable release process and remaining platform integrations are still
in progress.

## Documentation

- [Product requirements](docs/PRODUCT_REQUIREMENTS.md)
- [Architecture](docs/ARCHITECTURE.md)
- [User experience specification](docs/UX_SPECIFICATION.md)
- [Checksum formats](docs/CHECKSUM_FORMATS.md)
- [CLI specification](docs/CLI_SPECIFICATION.md)
- [File traversal policy](docs/FILE_TRAVERSAL.md)
- [Supported platforms](docs/SUPPORTED_PLATFORMS.md)
- [Building and development](docs/BUILDING.md)
- [Testing strategy](docs/TESTING.md)
- [Security model](docs/SECURITY_MODEL.md)
- [Privacy policy](docs/PRIVACY.md)
- [Localization](docs/LOCALIZATION.md)
- [Accessibility](docs/ACCESSIBILITY.md)
- [Performance policy](docs/PERFORMANCE.md)
- [Dependency policy](docs/DEPENDENCY_POLICY.md)
- [Release process](docs/RELEASING.md)
- [Branching policy](docs/BRANCHING.md)
- [Architecture decision records](docs/decisions/README.md)

## Contributing

Contributions are welcome. Read
[CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md), the [Code of Conduct](CODE_OF_CONDUCT.md),
and the [Security Policy](SECURITY.md) before opening an issue or pull request.

## Inspiration and Independence

RapidHash is inspired by the workflows of OpenHashTab and RapidCRC Unicode, but
it is an independent implementation. Contributors must not copy source code,
assets, text, or reverse-engineered implementation details from those projects.
See the clean-room requirements in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

RapidHash is licensed under the [Apache License, Version 2.0](LICENSE).
Unless explicitly stated otherwise, contributions are accepted under the same
terms.
