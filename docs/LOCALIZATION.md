# Localization

## Canonical Language

English is the canonical language for project development and source strings.
The initial user-interface locales are English (`en`) and Simplified Chinese
(`zh-CN`). Additional locales depend on maintained translations and review.

RapidHash uses Mozilla Fluent resource files as its shared message format. The
desktop application and CLI consume the same locale catalog wherever their
messages have the same meaning. Catalogs live under:

```text
locales/
├─ en/
│  ├─ common.ftl
│  ├─ cli.ftl
│  └─ desktop.ftl
└─ zh-CN/
   ├─ common.ftl
   ├─ cli.ftl
   └─ desktop.ftl
```

`common.ftl` contains shared verification states and domain terms. `cli.ftl`
contains terminal help, summaries, and diagnostics. `desktop.ftl` contains
controls, dialogs, menus, and accessibility labels. Messages are shared only
when their meaning and presentation constraints are genuinely identical.

English is the complete fallback catalog. Every other locale may override it but
must pass release-level completeness and syntax checks before it is advertised as
supported.

## Architecture Boundary

Core crates return structured error categories, identifiers, and typed values;
they do not construct localized sentences. The CLI and desktop adapters resolve
message identifiers at the presentation boundary. Stable API tokens, JSON keys,
algorithm identifiers, log field names, and manifest output are never localized.

User-facing strings must not be hard-coded in Rust, TypeScript, JSX, or platform
integration code. Exceptions are limited to non-user-visible test fixtures and
must be obvious from context.

## Message Design

- Use stable message identifiers rather than English text as keys.
- Keep interpolation typed and named.
- Do not concatenate translated fragments.
- Support plural rules through the localization library.
- Keep technical tokens, algorithm IDs, paths, and digests out of translated
  grammar when possible.
- Provide translator comments for security-sensitive or ambiguous strings.
- Use Fluent attributes for related labels, descriptions, and accessibility text
  when this improves translator context.
- Keep message identifiers semantic and stable, for example
  `verification-status-match`, rather than tied to an English phrase.

## Formatting

Dates, numbers, byte sizes, and rates use locale-aware formatting. Machine
formats, CLI JSON, algorithm identifiers, and manifest serialization remain
locale-independent.

## Fallback

Missing translations fall back to English and generate a development diagnostic.
A locale must pass key-completeness validation before release. Incorrect or stale
security messaging blocks that locale from shipping.

Locale selection follows this precedence:

1. an explicit CLI `--locale` option or saved desktop preference;
2. the operating-system locale;
3. the `LANGUAGE`, `LC_ALL`, or `LANG` environment on supported Unix systems;
4. English.

Unsupported or malformed locale tags fall back safely and produce a diagnostic
without preventing checksum operations.

## Bidirectional and Untrusted Text

File paths and manifest text may contain bidirectional control characters. The UI
must isolate such content, make controls visible or escaped where helpful, and
never allow it to reorder trusted labels or verification status.

## Contributions

Translation pull requests must identify the locale and reviewer competence.
Automated translation may assist drafting but requires human review before merge.

All locale files must preserve variables, selectors, and message identifiers.
CI checks Fluent syntax, missing English keys, unknown keys, variable parity, and
duplicate identifiers. Translation-only pull requests must not change application
behavior.
