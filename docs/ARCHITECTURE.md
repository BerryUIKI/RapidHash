# Architecture

## 1. Context

RapidHash is a local-first desktop and CLI application. It reads untrusted files
and manifests, performs CPU-intensive calculations, and reports immutable result
records. It does not require a server component.

## 2. System Boundaries

```text
Desktop UI (React/TypeScript)
        |
        | typed Tauri commands and events
        v
Desktop adapter (Rust/Tauri) ---- Platform integrations
        |
        v
Application services
        |
        +---- Hashing engine
        +---- Manifest engine
        +---- Traversal and path policy
        +---- Job scheduler and progress
        |
        v
Operating-system file APIs

CLI adapter ----------^  (shares application services and core types)
```

The UI cannot access arbitrary files directly. File access occurs through narrow
Rust commands with Tauri capabilities scoped to the required windows and
operations.

## 3. Planned Workspace

```text
crates/rapidhash-core       domain types, hashing, traversal, scheduling
crates/rapidhash-formats    manifest parsing, validation, and serialization
crates/rapidhash-cli        command-line adapter
apps/desktop/src-tauri      Tauri application adapter
apps/desktop/src            React/TypeScript presentation layer
integrations/               optional operating-system adapters
tests/fixtures              shared compatibility fixtures
```

Dependencies point inward: platform and presentation layers may depend on core
interfaces, but core crates never depend on Tauri, React, or operating-system
integration crates.

## 4. Core Domain Model

- `AlgorithmId`: stable identifier independent of display text.
- `InputSpec`: an approved file, directory, or manifest source.
- `JobSpec`: immutable algorithms, traversal rules, and output policy.
- `JobId` and `ItemId`: opaque identifiers for event correlation.
- `Digest`: algorithm-tagged bytes; formatting is a presentation concern.
- `ItemResult`: path, stable metadata snapshot, digests, status, and diagnostics.
- `ManifestEntry`: parsed digest, path representation, and source location.
- `ProgressEvent`: monotonic counters with no correctness significance.

Public serialized structures must carry a schema version.

## 5. Execution Model

The application uses blocking, buffered sequential reads on a bounded worker
pool. Asynchronous messages connect workers to UI and CLI adapters. This avoids
assuming that asynchronous file APIs improve throughput on every local or
network file system.

The scheduler limits:

- concurrently open files;
- hashing workers;
- outstanding directory entries;
- progress-event frequency; and
- memory allocated to read buffers.

Cancellation is cooperative and checked between reads and traversal batches.
Partial digests are discarded.

## 6. Single-Pass Multi-Hashing

When several algorithms are requested for a file, one worker should feed each
algorithm from the same input buffer. Exceptions require benchmarks and an ADR,
for example when a specialized parallel algorithm provides a material benefit.

## 7. Path Model

Paths are not assumed to be valid UTF-8. Core APIs use native path types and
delay display conversion until the UI boundary. A display-safe escaped form and
a machine-readable platform representation are separate concepts.

Manifest resolution normalizes lexical components, rejects absolute paths when
the format requires relative paths, and prevents escape from the approved root.
Symbolic links are governed by an explicit job policy and never followed by
accident.

## 8. Error Model

Errors are structured by category: invalid input, unsupported input, missing,
permission denied, changed during read, I/O failure, cancelled, and internal
failure. User-facing messages are localized in the presentation layer. Internal
errors retain source chains for diagnostics without exposing secrets by default.

Localization occurs only in presentation adapters. Core errors carry stable
message identifiers and typed arguments so the desktop application and CLI can
use the shared Fluent catalogs without parsing English error text.

One failed item does not fail unrelated items. Process exit status summarizes
the complete job according to the CLI contract.

## 9. Persistence

The first release persists only settings that do not reveal file activity:
theme, locale, selected algorithms, display choices, and concurrency preferences.
Recent files and digest history are disabled unless a later ADR introduces an
explicit opt-in model.

## 10. Extensibility

Algorithms and formats are registered through compile-time descriptors. Dynamic
plugins are out of scope until a threat model and stable API exist. Platform
integrations invoke stable CLI or IPC entry points instead of loading into the
core process when practical.

## 11. Observability

Development builds may emit structured tracing. Release logs are disabled by
default, exclude file contents and full paths unless explicitly enabled, and
must be safe to share after documented redaction.

## 12. Related Decisions

- [ADR-0001: Rust for the core](decisions/0001-rust-core.md)
- [ADR-0002: Tauri desktop architecture](decisions/0002-tauri-desktop.md)
- [ADR-0003: Apache-2.0 licensing](decisions/0003-apache-2-license.md)
- [ADR-0004: Fluent-based internationalization](decisions/0004-fluent-i18n.md)
