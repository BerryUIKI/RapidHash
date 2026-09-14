# ADR-0001: Use Rust for the Shared Core

- Status: Accepted
- Date: 2026-09-14
- Owners: RapidHash maintainers

## Context

RapidHash must stream large files, execute multiple algorithms, coordinate
cancellable concurrent work, preserve platform path semantics, and share behavior
between desktop and CLI interfaces on Windows, macOS, and Linux.

## Decision

Implement the domain model, hashing engine, manifest engine, traversal policy,
and job scheduling in stable Rust. Keep these crates independent of the desktop
framework. Prefer safe Rust and require explicit review for unsafe code.

## Alternatives Considered

- **C++:** strong performance and platform access, but a higher memory-safety
  burden and less convenient dependency tooling for this project.
- **C#/.NET:** productive and cross-platform, but less suitable for a small native
  CLI/runtime footprint and the desired Rust ecosystem integrations.
- **TypeScript/Electron only:** mature UI development, but moves high-throughput
  file processing into a less appropriate runtime and increases distribution
  size.

## Consequences

The project gains one safe, reusable implementation for CLI and desktop use and
can use mature Rust digest crates. Contributors need Rust expertise, platform
packaging still requires native toolchains, and dependency/security review
remains essential.
