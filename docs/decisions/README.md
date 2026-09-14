# Architecture Decision Records

Architecture Decision Records (ADRs) document durable decisions that affect
interfaces, security, compatibility, licensing, or project structure.

## Process

1. Copy `0000-template.md` to the next four-digit number and a short slug.
2. Set the status to Proposed and open a focused pull request.
3. Record context, decision, alternatives, consequences, and references.
4. After approval, set the status to Accepted and merge it before dependent work.
5. Never rewrite an accepted decision to hide history. Supersede it with a new
   ADR and link both records.

## Index

- [ADR-0001: Use Rust for the shared core](0001-rust-core.md)
- [ADR-0002: Use Tauri 2 for the desktop application](0002-tauri-desktop.md)
- [ADR-0003: License the project under Apache-2.0](0003-apache-2-license.md)
