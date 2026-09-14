# ADR-0004: Use Fluent for Shared Internationalization

- Status: Accepted
- Date: 2026-09-14
- Owners: RapidHash maintainers

## Context

RapidHash has two user-facing adapters: a Rust CLI and a React/TypeScript desktop
application. Both require pluralization, variables, locale fallback, translator
context, accessible labels, and safe rendering of untrusted paths. Duplicating
messages in unrelated Rust and frontend formats would increase drift and review
cost.

## Decision

Use Mozilla Fluent (`.ftl`) as the canonical message catalog format for both the
CLI and desktop application. Store locale resources in the repository-level
`locales/<bcp47-tag>/` hierarchy, partitioned as follows:

- `common.ftl` for genuinely shared verification states and domain terms;
- `cli.ftl` for terminal help, summaries, and diagnostics; and
- `desktop.ftl` for controls, dialogs, menus, and accessibility labels.

English is the complete fallback catalog; Simplified Chinese is included from
the first executable scaffold.

Core crates expose stable error and event identifiers with typed arguments and do
not localize prose. Presentation adapters load catalogs, negotiate locale tags,
format messages, and isolate untrusted bidirectional text. Machine-readable JSON,
manifest formats, algorithm identifiers, and tracing fields remain invariant.

CI validates catalog syntax, identifier and variable parity, English completeness,
and the absence of hard-coded user-facing text where practical. Untrusted paths
are directionally isolated in terminal messages and rendered in `<bdi>` elements
in the desktop interface.

## Alternatives Considered

- **Separate i18next JSON and Rust catalogs:** mature per ecosystem, but duplicates
  messages and permits adapters to drift.
- **ICU MessageFormat:** broadly understood, but would require separate runtime
  integration and offers less natural translator context for this project.
- **Compile-time Rust-only localization:** strong CLI integration, but forces the
  frontend through a custom bridge for every presentation message.
- **English-only CLI:** simpler initially, but violates the product-wide i18n
  requirement and creates a costly future migration.

## Consequences

The project gains one expressive translation system for both interfaces and can
establish message stability before UI work. Partitioning prevents terminal layout
constraints from distorting desktop messages. Both Rust and TypeScript need
Fluent runtime dependencies, build tooling must package the shared resources,
and CI must detect catalog drift. Locale negotiation and formatting remain
adapter responsibilities rather than core hashing concerns.
