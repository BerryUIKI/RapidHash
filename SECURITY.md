# Security Policy

## Supported Versions

RapidHash has not published a stable release. During pre-alpha development, only
the latest commit on `main` is eligible for security fixes. After the first
stable release, this table will identify supported release lines.

## Reporting a Vulnerability

Use GitHub private vulnerability reporting when it is enabled for this
repository. If that option is unavailable, contact the repository owner
privately through GitHub before sharing technical details.

Do not disclose suspected vulnerabilities in a public issue, discussion, pull
request, social-media post, or checksum fixture.

Include, when possible:

- affected version or commit;
- operating system and architecture;
- reproduction steps or a minimal proof of concept;
- security impact and required attacker capabilities;
- suggested mitigation; and
- whether the issue has been disclosed elsewhere.

The maintainers will acknowledge a valid report as soon as practical, coordinate
a fix and disclosure timeline with the reporter, and credit the reporter unless
anonymity is requested. No guaranteed response or remediation deadline is made
while the project is maintained by volunteers.

## Scope

Security-sensitive areas include manifest path traversal, symbolic-link handling,
unbounded resource consumption, malformed input, unsafe Rust, platform
integration, updater integrity, dependency compromise, and unintended network
transmission. See [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md).
