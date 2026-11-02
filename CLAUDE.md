# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview & Architecture

RapidHash is a fast, cross-platform checksum verification and calculation utility implemented as a Rust workspace with a Tauri 2 desktop GUI and a standalone CLI.

### Workspace Structure

- `crates/rapidhash-core`: Core hashing engine, streaming I/O with bounded memory, cancellation tokens, algorithm definitions (`AlgorithmId`, `Digest`), deterministic path traversal (`traversal`), path confinement security against directory traversal escapes (`path_policy`), and CRC filename embedding/extraction (`filename_crc`).
- `crates/rapidhash-formats`: Checksum manifest parsing and serialization. Supports GNU-style manifests (`*sum`) and SFV manifests with UTF-8, UTF-8 BOM, and UTF-16LE encoding tolerance, securely confining relative paths against an approved root.
- `crates/rapidhash-i18n`: Mozilla Fluent (`.ftl`) catalogs and localization negotiation. Contains structural parity tests (`catalog_parity.rs`) ensuring exact 1:1 message, attribute, and argument symmetry between `locales/en/` and `locales/zh-CN/`.
- `crates/rapidhash-cli`: Command-line executable providing `hash`, `verify`, `compare`, and `algorithms` subcommands with human-friendly and structured JSON outputs.
- `apps/desktop/src-tauri`: Desktop backend adapter exposing Tauri commands (`calculate_hashes`, `verify_manifest_file`, `write_crc_to_filename`, `save_manifest_file`, etc.) connecting the core Rust engine to the frontend.
- `apps/desktop`: Frontend application built with React 19, TypeScript, Tailwind CSS, and Vite.

---

## Development & Testing Commands

### Rust Workspace

- **Build all workspace crates:**
  ```bash
  cargo build --workspace
  ```

- **Run all unit & integration tests:**
  ```bash
  cargo test --workspace
  ```

- **Run tests for a single crate:**
  ```bash
  cargo test -p rapidhash-core
  cargo test -p rapidhash-formats
  cargo test -p rapidhash-cli
  cargo test -p rapidhash-i18n
  ```

- **Run a specific test suite or test by name:**
  ```bash
  # Integration test targets
  cargo test --test vectors
  cargo test --test traversal_security
  cargo test --test manifest_parsing
  cargo test --test catalog_parity
  cargo test --test cli_e2e

  # Specific test function filter
  cargo test <test_function_name>
  # Example:
  cargo test filename_crc::tests::test_insert_crc
  ```

- **Check code formatting:**
  ```bash
  cargo fmt --all --check
  ```

- **Format Rust code:**
  ```bash
  cargo fmt --all
  ```

- **Run Clippy (strict mode required by CI):**
  ```bash
  cargo clippy --workspace --all-targets -- -D warnings
  ```

### Desktop Frontend (`apps/desktop`)

- **Install frontend dependencies:**
  ```bash
  cd apps/desktop && pnpm install --frozen-lockfile
  ```

- **Typecheck and build frontend bundle:**
  ```bash
  cd apps/desktop && pnpm build
  ```

- **Run Vite frontend development server:**
  ```bash
  cd apps/desktop && pnpm dev
  ```

- **Launch full Tauri desktop app in development mode:**
  ```bash
  cd apps/desktop && pnpm tauri dev
  ```

---

## Branching & Contribution Workflow

RapidHash strictly enforces a two-tier protected branch model governed by [AGENTS.md](AGENTS.md):

```text
[Topic Branches] ──(PR)──> dev (Integration) ──(Milestone PR)──> main (Production)
```

1. **Always branch off `dev`**:
   - `git checkout dev && git pull origin dev && git checkout -b <type>/<description>`
   - Topic branch naming: `<type>/<short-description>` (e.g. `feat/hash-export`, `fix/crc-spacing`).
2. **Never target `main` directly** for feature or bugfix PRs. All PRs target `dev`.
3. **Conventional Commits (Strictly in English)**:
   - Format: `<type>(<scope>): <summary in English>` (`feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`).
   - Always commit non-interactively without GPG prompts: `git commit --no-gpg-sign -m "..."`.
4. **Mandatory Verification Gates Before Every PR**:
   Every PR targeting `dev` or `main` must pass all four gates with zero errors and zero warnings:
   - `cargo test --workspace`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo fmt --all --check`
   - `cd apps/desktop && pnpm build`

---

## Key Development Rules & Invariants

- **Clean-Room Boundary**: RapidHash is licensed under Apache-2.0 and inspired by OpenHashTab and RapidCRC Unicode. Never copy, adapt, or decompile GPL code from those projects.
- **Path Confinement**: All file paths from user input or manifests must be verified through `rapidhash_core::path_policy::resolve_manifest_path` to prevent directory traversal and symlink escapes.
- **Localization Parity**:
  - `locales/en/` is canonical.
  - Any key added to `en` must simultaneously be added to `zh-CN` with identical identifiers, attributes, and variables.
  - Keep fallback strings synchronized in `apps/desktop/src/i18n.ts` and `apps/desktop/src-tauri/src/lib.rs`.
