# Building and Development

This document defines the intended development workflow. Commands will be made
executable when the implementation scaffold is introduced.

## Planned Prerequisites

- a stable Rust toolchain managed by `rustup`;
- Node.js active LTS and Corepack;
- platform requirements documented by Tauri 2;
- Git; and
- platform packaging or signing tools only when producing release artifacts.

The repository will pin or declare minimum versions once the first scaffold is
merged. Do not assume globally installed prerelease toolchains.

## Planned Layout and Commands

The workspace will expose repository-level commands for:

```text
format          format Rust, TypeScript, and documentation
lint            run Rust and frontend static analysis
test            run unit and integration tests
desktop-dev     launch the desktop application for development
build           build release binaries for the current platform
docs-check      validate Markdown and links
```

Exact commands will be added to this document with the scaffold. Contributors
should not need to discover separate package-manager invocations for routine
validation.

## Platform Notes

### Windows

Use the MSVC Rust target, Microsoft C++ Build Tools required by Tauri, and a
supported WebView2 runtime.

### macOS

Install Xcode Command Line Tools. Packaging and notarization require Apple
credentials that are not needed for ordinary development.

### Linux

Install the WebKitGTK and desktop integration development packages specified by
Tauri for the distribution. Exact package names vary by distribution.

## Secrets

Never place signing certificates, tokens, passwords, or private keys in the
repository or ordinary `.env` files. Release secrets belong in the approved CI
secret store with least-privilege access.

## Reproducibility

Commit lockfiles and avoid build steps that fetch undeclared artifacts. Release
builds record toolchain and dependency versions. Platform signing can make final
packages nondeterministic; unsigned payload reproducibility should still be
measured where practical.
