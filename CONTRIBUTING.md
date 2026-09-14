# Contributing to RapidHash

Thank you for helping build RapidHash. This document defines the contribution
contract for source code, documentation, design assets, tests, and releases.

## Before You Start

1. Search existing issues and discussions.
2. For a bug, provide a minimal reproduction and platform details.
3. For a new feature or public API change, open a proposal before implementation.
4. Read the architecture and security documentation relevant to your change.

Small documentation corrections and narrowly scoped fixes do not require a
proposal.

## Language

English is the canonical project language for source code, identifiers, commit
messages, issues, pull requests, review comments, and documentation. Translated
user-interface resources are welcome and are governed by
[docs/LOCALIZATION.md](docs/LOCALIZATION.md).

## Development Workflow

1. Fork or clone the repository.
2. Create a branch following [docs/BRANCHING.md](docs/BRANCHING.md).
3. Make one coherent change per pull request.
4. Add or update tests and documentation.
5. Run formatting, linting, tests, and relevant platform checks.
6. Open a pull request using the repository template.

Direct pushes to `main` are prohibited after repository bootstrap.

## Commit Messages

Use Conventional Commits:

```text
<type>(optional-scope): <imperative summary>
```

Allowed types are `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`,
`ci`, `chore`, and `revert`.

Examples:

```text
feat(core): add streaming SHA-256 calculation
fix(formats): reject manifest paths outside the selected root
docs: define Linux packaging support
```

Use `!` and a `BREAKING CHANGE:` footer for incompatible changes. Keep the
subject concise, do not end it with a period, and explain motivation in the body
when it is not obvious from the diff.

## Pull Request Requirements

A pull request must:

- describe the user-visible or engineering outcome;
- link relevant issues or decisions;
- state how the change was tested;
- disclose platform-specific limitations;
- update user-facing and technical documentation when behavior changes;
- contain no unrelated generated files or drive-by refactors;
- pass all required checks;
- receive the approvals required by branch protection; and
- resolve all review conversations before merge.

Squash merging is preferred. The final commit title must follow the commit
message convention.

## Clean-Room Requirement

OpenHashTab and RapidCRC Unicode are product references, not source-code
dependencies. Do not copy or translate their source code, resource files,
documentation, icons, names, screenshots, or distinctive visual assets into
RapidHash. Do not submit code derived from GPL-covered implementation details
unless the maintainers have explicitly approved a compatible licensing change.

Contributors may describe externally observable behavior and implement it from
public specifications. New third-party code must satisfy
[docs/DEPENDENCY_POLICY.md](docs/DEPENDENCY_POLICY.md).

## Code Quality Expectations

- Prefer safe Rust. Any `unsafe` block requires a documented invariant, focused
  tests, and reviewer attention.
- Do not perform network access in hashing or verification paths.
- Treat file names and manifest contents as untrusted input.
- Keep the core independent of Tauri and frontend frameworks.
- Avoid platform conditionals outside defined integration boundaries.
- Preserve cancellation, bounded memory use, and deterministic output.

## Reporting Security Problems

Do not open public issues for suspected vulnerabilities. Follow
[SECURITY.md](SECURITY.md).

## Licensing Contributions

By submitting a contribution, you represent that you have the right to license
it and agree that it is provided under Apache-2.0 terms, unless the file clearly
states otherwise.
