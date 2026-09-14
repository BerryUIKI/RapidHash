# CLI Specification

This document defines the intended stable behavior of the RapidHash command-line
interface. Exact option spelling may change before the first implementation PR,
but changes after beta require compatibility review.

## Command Shape

```text
rapidhash hash [OPTIONS] <PATH>...
rapidhash verify [OPTIONS] <MANIFEST>
rapidhash compare [OPTIONS] <PATH> <DIGEST>
rapidhash algorithms [--json]
rapidhash completions <SHELL>
rapidhash --version
rapidhash --help
```

`hash` is the default only when doing so cannot make input ambiguous. Scripts
should specify the subcommand.

## Output

Human-readable results go to stdout. Warnings, progress, and diagnostics go to
stderr. Progress is disabled when stderr is not an interactive terminal unless
explicitly requested. Color follows `auto`, `always`, or `never` policy.

`--json` emits one versioned JSON document or newline-delimited records when an
explicit streaming option is selected. JSON contains no localized values.

## Determinism

Given stable files and equivalent options, result order, digest casing, path
formatting, and JSON field meaning are deterministic. Directory traversal output
is sorted by a documented byte/platform order unless streaming unordered output
is explicitly requested.

## Exit Codes

| Code | Meaning |
| --- | --- |
| 0 | All requested operations completed and all verifications matched |
| 1 | At least one verification mismatch |
| 2 | Invalid command-line usage or malformed expected digest |
| 3 | At least one input was missing or unreadable |
| 4 | Manifest syntax, encoding, or safety policy failure |
| 5 | Requested algorithm or feature is unsupported |
| 130 | Interrupted by the user where the platform convention permits |
| 70 | Unexpected internal failure |

When several conditions occur, the CLI uses a documented severity precedence
rather than returning the last item's state. JSON preserves every item outcome.

## Paths and Standard Input

A lone `-` may represent standard input only for commands that document it.
Reading file lists from stdin requires an explicit option and supports a NUL
delimiter for unambiguous automation. Output intended for a manifest never mixes
with progress.

## Security

Manifest verification applies the same approved-root and symbolic-link policy as
the desktop application. The CLI never executes file names, manifest text, or
environment-provided command fragments.

## Compatibility

Shell completion scripts and JSON schemas carry versioning expectations. Human
output is not a stable parser interface; automation must use JSON or documented
manifest output.
