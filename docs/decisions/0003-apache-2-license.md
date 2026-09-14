# ADR-0003: License the Project Under Apache-2.0

- Status: Accepted
- Date: 2026-09-14
- Owners: RapidHash maintainers

## Context

RapidHash is intended to be an open-source project with clear permissions for
use, modification, and distribution. It also needs an explicit patent grant and
a clean licensing boundary from GPL-licensed reference applications.

## Decision

License RapidHash under the Apache License, Version 2.0. Accept contributions
under the same terms unless a file clearly states an approved compatible license.
Maintain a clean-room rule prohibiting copied code and assets from OpenHashTab,
RapidCRC Unicode, or other incompatible sources.

## Alternatives Considered

- **MIT:** simple and permissive, but lacks Apache-2.0's explicit patent grant.
- **MIT OR Apache-2.0:** common in Rust libraries, but the project owner selected
  one unambiguous project-wide license.
- **GPL:** compatible with studying and modifying GPL code under its terms, but
  does not match the selected permissive distribution model.

## Consequences

Users receive broad rights plus an explicit patent license and notice
requirements. Dependency review must ensure distributed components are license
compatible. Reference projects influence product goals only, not implementation.
