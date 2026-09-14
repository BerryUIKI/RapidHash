# Algorithm Policy

## 1. Categories

RapidHash presents algorithms by intended use rather than implying equal
security properties.

| Category | Planned algorithms | Guidance |
| --- | --- | --- |
| Modern cryptographic | SHA-256, SHA-384, SHA-512, SHA-3, BLAKE2, BLAKE3 | Preferred when a publisher supports them |
| Legacy cryptographic | MD5, SHA-1 | Compatibility and accidental-corruption checks only |
| Non-cryptographic | CRC32, CRC32C, xxHash32, xxHash64, XXH3 | Fast integrity checks; not adversarial security |

The interface must warn that a checksum does not authenticate its source. A
digest obtained from the same compromised location as a file does not establish
trust.

## 2. Stable Identifiers

Algorithm identifiers in APIs, manifests, settings, and JSON are lowercase
ASCII tokens such as `sha256`, `sha512`, `blake3`, and `crc32`. Display names are
localized separately. Renaming display text must not change serialized IDs.

## 3. Implementation Selection

Implementations should come from actively maintained Rust crates or narrowly
reviewed project code with compatible licensing. Native CPU acceleration may be
used when runtime detection is safe and results remain identical.

Each dependency requires:

- an SPDX-compatible license review;
- upstream activity and security review;
- published or authoritative test vectors;
- streaming operation;
- support for tier-one platforms; and
- a benchmark against reasonable alternatives.

## 4. Correctness Testing

Every algorithm must pass:

- empty, short, block-boundary, and multi-block vectors;
- incremental updates using varied chunk boundaries;
- large streamed inputs;
- independent implementation comparison;
- upper- and lowercase parsing where textual digests are accepted; and
- deterministic formatting tests.

## 5. Defaults

The initial recommended calculation set is SHA-256 and BLAKE3. Legacy and
non-cryptographic algorithms remain available but are not silently selected for
security-oriented verification.

## 6. Adding or Removing Algorithms

Adding an algorithm requires documentation, vectors, dependency review, UI
classification, and format compatibility analysis. Removing a serialized
algorithm identifier is a breaking change and requires an ADR plus a migration
period.
