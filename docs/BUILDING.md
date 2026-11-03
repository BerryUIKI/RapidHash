# Building and Development

This document describes the development workflow for the implemented workspace.

## Prerequisites

- a stable Rust toolchain managed by `rustup`;
- Node.js active LTS and Corepack;
- platform requirements documented by Tauri 2;
- Git; and
- platform packaging or signing tools only when producing release artifacts.

The workspace declares Rust 1.85 as its minimum supported toolchain. Do not
assume globally installed prerelease toolchains.

## Layout and Validation Commands

The workspace provides the following validation commands:

```shell
# Run all tests, including catalog parity and syntax validation:
cargo test --workspace

# Run tests specifically for the i18n crate:
cargo test -p rapidhash-i18n

# Lint all workspace targets with warnings denied:
cargo clippy --workspace --all-targets -- -D warnings

# Check code formatting without making changes:
cargo fmt --all --check

# Format code across the workspace:
cargo fmt --all
```

Frontend checks run from `apps/desktop` with pnpm:

```shell
pnpm install --frozen-lockfile
pnpm build
```

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
