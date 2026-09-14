# User Experience Specification

## 1. Experience Goals

RapidHash should feel calm during long-running work, make verification outcomes
impossible to misread, and expose advanced choices without requiring them for a
simple one-file check.

## 2. Information Architecture

The desktop application has four primary destinations:

1. **Calculate** for files and directories.
2. **Verify** for pasted digests and manifests.
3. **Generate** for checksum manifests.
4. **Settings** for algorithms, behavior, appearance, and diagnostics.

The current queue and its progress remain accessible from Calculate, Verify, and
Generate. Navigation must not cancel work.

## 3. Calculate Flow

- The empty state offers drop, file picker, and directory picker actions.
- Adding items creates visible queued rows immediately.
- Algorithm selection applies to newly created jobs; changing it never mutates an
  active job silently.
- Each row exposes file name, parent path, size, status, progress, algorithms,
  results, and a contextual action menu.
- Digest text uses a readable monospace font and supports selection.
- Copy actions announce success non-modally.

## 4. Verification Language

Status must use icon, text, and color together:

- **Match:** calculated bytes equal the expected digest.
- **Mismatch:** calculation succeeded but bytes differ.
- **Missing:** a manifest target does not exist.
- **Unreadable:** the target exists but cannot be read.
- **Malformed:** the expected entry cannot be interpreted safely.
- **Unsupported:** the requested algorithm or record is recognized but unavailable.
- **Cancelled:** the user stopped processing.

Never describe a matching checksum as proof that a file is safe or authentic.

## 5. Destructive and Risky Actions

RapidHash does not edit input files. Manifest overwrite requires explicit
confirmation and uses an atomic replace strategy where supported. External
lookups, if ever added, must display exactly what data will leave the device.

## 6. Progress

- Report byte progress for regular files with known size.
- Report indeterminate progress during directory discovery.
- Separate discovery from hashing counts.
- Throttle visual updates to avoid consuming meaningful hashing capacity.
- Show aggregate throughput as informational, not as a guarantee.
- Acknowledge cancellation immediately and distinguish it from completion.

## 7. Large Result Sets

The table uses virtualization, stable row identities, keyboard selection, and
incremental filtering. Export acts on the complete filtered set, not only
currently rendered rows. Sorting never changes generated manifest order unless
the user explicitly selects that ordering.

## 8. Errors

An error message contains a short summary, the affected item, a suggested next
action when known, and expandable technical detail. File paths are selectable.
Repeated equivalent errors may be grouped without hiding their count.

## 9. Settings

Settings use safe defaults and clearly label legacy algorithms. The application
provides a reset action and documents which settings are persisted. Advanced
concurrency values are bounded and offer an automatic option.

## 10. First-Run Experience

The first run provides a concise explanation of calculation versus verification,
states that processing is local, and presents a direct drop target. It does not
require an account, telemetry consent, or network access.
