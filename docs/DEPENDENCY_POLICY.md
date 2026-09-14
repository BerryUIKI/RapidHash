# Dependency Policy

## Selection Criteria

A dependency must provide material value compared with a small maintainable
implementation and must be compatible with Apache-2.0 distribution. Review:

- license and transitive licenses;
- maintenance activity and ownership;
- security history and advisories;
- supported targets and minimum Rust version;
- build scripts, native code, and network behavior;
- transitive dependency cost;
- correctness evidence and test vectors; and
- replacement or removal difficulty.

Copyleft dependencies are not accepted into distributed artifacts without an
explicit licensing ADR and maintainer approval. Tooling used only for development
is reviewed separately but must still be legally and operationally acceptable.

## Versioning

Commit Rust and frontend lockfiles. Use compatible version ranges in manifests
while lockfiles make application builds deterministic. Security updates may be
expedited but still require tests and review.

## Automation

Automated dependency pull requests must be small and grouped only when packages
are operationally coupled. CI actions must be pinned to immutable commit hashes
before the workflow is made a required check.

## Auditing

The project intends to automate vulnerability, license, duplicate-version, and
unused-dependency checks. Release candidates require a human review of new native
code, build scripts, and high-risk transitive changes.

## Vendoring and Forks

Vendoring or maintaining a fork requires documented provenance, update procedure,
local modifications, and ownership. It is reserved for reliability, security,
or platform support that cannot be achieved upstream.
