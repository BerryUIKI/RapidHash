# Privacy

## Default Behavior

RapidHash is local-first. The hashing core, desktop application, and CLI do not
upload file contents, file names, paths, checksums, manifest contents, or usage
data. No account is required.

## Stored Data

The initial release may store non-sensitive preferences such as locale, theme,
selected algorithms, table layout, and concurrency settings. It does not store
recent file paths, generated digests, or job history by default.

The settings screen must identify the storage location and provide a reset
action. Uninstall behavior is documented per package because operating systems
differ in whether user settings are retained.

## Logs

Release logging is off by default. When diagnostic logging is enabled, logs must
avoid file contents and should redact or shorten full paths. The user reviews any
diagnostic bundle before sharing it.

## Network Access

Initial releases require no network access during ordinary use. Package managers
or optional update mechanisms may access their configured distribution service,
but this is separate from checksum calculation.

Any future online service must be opt-in, document the exact transmitted fields,
define retention and operator identity, and undergo a security and privacy ADR.

## Sensitive Digests

A digest can identify known content and should not automatically be treated as
anonymous. RapidHash therefore does not transmit digests without explicit user
action.
