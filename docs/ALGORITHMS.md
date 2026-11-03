# Algorithm Policy

## 1. Categories

RapidHash presents algorithms by intended use rather than implying equal
security properties.

| Category | Algorithms | Guidance |
| --- | --- | --- |
| Modern cryptographic | `sha256`, `sha512`, `sha384`, `sha3-512`, `sha3-256`, `blake2b-512`, `blake2s-256`, `blake3` | Preferred when a publisher supports them |
| Legacy cryptographic | `md5`, `sha1` | Compatibility and accidental-corruption checks only |
| Non-cryptographic | `crc32`, `crc32c`, `xxh3`, `xxh64`, `xxh32` | Fast integrity checks; not adversarial security |

The interface must warn that a checksum does not authenticate its source. A
digest obtained from the same compromised location as a file does not establish
trust.

## 2. Supported Matrix

| ID | Output bytes | Hex characters | Implementation | Notes |
| --- | --- | --- | --- | --- |
| `sha256` | 32 | 64 | `sha2` (RustCrypto) | FIPS 180-4 |
| `sha512` | 64 | 128 | `sha2` (RustCrypto) | FIPS 180-4 |
| `sha384` | 48 | 96 | `sha2` (RustCrypto) | FIPS 180-4 |
| `sha3-512` | 64 | 128 | `sha3` (RustCrypto) | FIPS 202 |
| `sha3-256` | 32 | 64 | `sha3` (RustCrypto) | FIPS 202 |
| `blake2b-512` | 64 | 128 | `blake2` (RustCrypto) | RFC 7693, unkeyed |
| `blake2s-256` | 32 | 64 | `blake2` (RustCrypto) | RFC 7693, unkeyed |
| `blake3` | 32 | 64 | `blake3` | Default output length |
| `crc32` | 4 | 8 | `crc32fast` | IEEE 802.3; uppercase in SFV output |
| `crc32c` | 4 | 8 | `crc32c` | Castagnoli polynomial |
| `xxh3` | 8 | 16 | `xxhash-rust` | XXH3-64, zero seed |
| `xxh64` | 8 | 16 | `xxhash-rust` | Zero seed |
| `xxh32` | 4 | 8 | `xxhash-rust` | Zero seed |
| `md5` | 16 | 32 | `md-5` (RustCrypto) | RFC 1321, legacy |
| `sha1` | 20 | 40 | `sha1` (RustCrypto) | FIPS 180-4, legacy |

CRC and xxHash digests are serialized **big-endian**, so the hexadecimal text
matches the canonical numeric value of the algorithm. Only `crc32` prefers
uppercase hexadecimal outside manifests; every other algorithm formats lowercase
hex by default, while parsers accept either case.

## 3. Stable Identifiers

Algorithm identifiers in APIs, manifests, settings, and JSON are lowercase
ASCII tokens such as `sha256`, `sha3-256`, `blake2b-512`, `xxh64`, and `crc32`.
Display names are localized separately. Renaming display text must not change
serialized IDs.

Parsers additionally accept common aliases (`sha-256`, `sha-1`, `sha3256`,
`blake2b512`, `xxhash64`, `xxh3-64`, and similar) case-insensitively. Aliases
are input conveniences; canonical identifiers are the ones listed in Section 2.

## 4. Implementation Selection

Implementations come from actively maintained Rust crates or narrowly reviewed
project code with compatible licensing. All crates listed in Section 2 are
MIT, Apache-2.0, or dual-licensed and are compatible with Apache-2.0
distribution. Native CPU acceleration may be used when runtime detection is safe
and results remain identical.

Each dependency requires:

- an SPDX-compatible license review;
- upstream activity and security review;
- published or authoritative test vectors;
- streaming operation;
- support for tier-one platforms; and
- a benchmark against reasonable alternatives.

## 5. Correctness Testing

Every algorithm must pass:

- empty, short, block-boundary, and multi-block vectors;
- incremental updates using varied chunk boundaries;
- large streamed inputs;
- independent implementation comparison;
- upper- and lowercase parsing where textual digests are accepted; and
- deterministic formatting tests.

The vector suite in `crates/rapidhash-core/tests/vectors.rs` covers the empty
input and `"abc"` for cryptographic algorithms, the `"123456789"` check value
for CRC algorithms, and published xxHash values for a zero seed. Streaming and
multi-hash consistency tests iterate every registered algorithm.

## 6. Defaults

The recommended calculation set remains SHA-256 and BLAKE3. Legacy and
non-cryptographic algorithms remain available but are not silently selected for
security-oriented verification.

## 7. Adding or Removing Algorithms

Adding an algorithm requires documentation, vectors, dependency review, UI
classification, and format compatibility analysis. Removing a serialized
algorithm identifier is a breaking change and requires an ADR plus a migration
period.
