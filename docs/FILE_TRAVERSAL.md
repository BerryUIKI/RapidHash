# File Traversal Policy

## Objectives

Traversal must be deterministic, cancellable, bounded, transparent about links,
and safe on file systems with different path and identity semantics.

## Defaults

- Process explicitly selected regular files.
- Recurse into explicitly selected directories.
- Do not follow directory symbolic links, junctions, aliases, or mount-point-like
  redirects by default.
- Do not cross file-system boundaries when a future option requests confinement.
- Include hidden files unless the user selects an exclusion policy.
- Report permission failures without abandoning unrelated paths.

## Ordering

Discovery and calculation may execute concurrently, but stable output is ordered
by a documented normalized relative-path key. An explicit unordered streaming
mode may trade determinism for lower latency in CLI workflows.

## Identity and Duplicates

The same underlying file may be reachable through several paths. Default behavior
reports each explicitly selected path but avoids accidental recursive cycles.
Future deduplication options must state whether identity uses canonical paths,
file IDs, or content and must not silently change manifest semantics.

## Changed Files

Capture available identity, size, and modification metadata before and after
reading. If a relevant field changes, return an unstable result. Do not retry
indefinitely; an explicit bounded retry policy may be offered.

## Special Files

Devices, sockets, named pipes, and similar non-regular files are rejected by
default because reads may block, mutate state, or never terminate. Supporting a
special file requires explicit CLI intent and a documented byte/time limit.

## Exclusions

Exclusion patterns operate on normalized relative display paths with documented
separator and case rules. A preview or diagnostic explains why an item was
excluded. Ignore-file compatibility is deferred until its exact semantics are
specified.

## Limits

Traversal enforces configurable bounds for discovered entries, pending work,
open directories, depth, and diagnostic volume. Hitting a limit produces a
visible incomplete outcome, never silent truncation.
