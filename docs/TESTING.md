# Testing Strategy

## Objectives

Testing establishes digest correctness, safe path handling, format compatibility,
bounded resource use, responsive cancellation, stable interfaces, and accessible
cross-platform behavior.

## Test Layers

### Unit Tests

Cover algorithm adapters, digest parsing, encoding decisions, path policy,
serialization, error mapping, and scheduling state transitions.

### Property Tests

Generate chunk boundaries, path components, digest casing, line endings, and
manifest records. Important invariants include one-shot versus incremental hash
equality, serialization round trips where defined, and containment after path
resolution.

### Vector and Compatibility Tests

Store small, license-compatible fixtures with documented provenance. Compare
results with published vectors and independent tools. Fixtures cover empty data,
binary data, Unicode names, non-UTF-8 paths where supported, spaces, separators,
BOMs, malformed lines, duplicates, and mixed valid/invalid manifests.

### Integration Tests

Exercise traversal, cancellation, changing files, permission errors, symbolic
links, large sparse inputs, CLI exit codes, JSON schemas, and Tauri command
boundaries.

### End-to-End Tests

Automate critical desktop flows where platform tooling permits. Manual release
checklists cover drag-and-drop, native pickers, clipboard, keyboard navigation,
screen readers, packaging, upgrade, and uninstall behavior.

### Fuzzing

Continuously fuzz manifest parsers, path conversion, digest decoding, and IPC
deserialization. Every discovered crash receives a minimized regression test.

### Performance Tests

Benchmarks follow [PERFORMANCE.md](PERFORMANCE.md). Performance changes do not
replace correctness tests and ordinary CI should avoid flaky absolute thresholds.

## CI Matrix

Pull requests run formatting, linting, unit tests, documentation checks, and
dependency policy checks. Platform builds run on Windows, macOS, and Linux.
Expensive fuzzing, packaging, and extended tests run on schedules or release
candidates.

## Test Data Rules

- Do not commit personal or confidential files.
- Keep ordinary fixtures small.
- Generate large files deterministically during tests.
- Document the origin and license of external fixtures.
- Avoid assertions dependent on locale, wall-clock time, directory enumeration
  order, or machine-specific paths.

## Release Gate

A release requires a green protected commit, tier-one package smoke tests,
algorithm and format compatibility suites, dependency review, and resolution or
explicit documentation of known release-blocking defects.
