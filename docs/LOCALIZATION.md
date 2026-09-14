# Localization

## Canonical Language

English is the canonical language for project development and source strings.
The initial user-interface locales are English (`en`) and Simplified Chinese
(`zh-CN`). Additional locales depend on maintained translations and review.

## Message Design

- Use stable message identifiers rather than English text as keys.
- Keep interpolation typed and named.
- Do not concatenate translated fragments.
- Support plural rules through the localization library.
- Keep technical tokens, algorithm IDs, paths, and digests out of translated
  grammar when possible.
- Provide translator comments for security-sensitive or ambiguous strings.

## Formatting

Dates, numbers, byte sizes, and rates use locale-aware formatting. Machine
formats, CLI JSON, algorithm identifiers, and manifest serialization remain
locale-independent.

## Fallback

Missing translations fall back to English and generate a development diagnostic.
A locale must pass key-completeness validation before release. Incorrect or stale
security messaging blocks that locale from shipping.

## Bidirectional and Untrusted Text

File paths and manifest text may contain bidirectional control characters. The UI
must isolate such content, make controls visible or escaped where helpful, and
never allow it to reorder trusted labels or verification status.

## Contributions

Translation pull requests must identify the locale and reviewer competence.
Automated translation may assist drafting but requires human review before merge.
