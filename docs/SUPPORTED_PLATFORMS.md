# Supported Platforms

## Support Tiers

- **Tier 1:** built and tested for every release; regressions block release.
- **Tier 2:** built in CI when practical; maintained on a best-effort basis.
- **Tier 3:** community-supported; compilation or runtime support is not assured.

## Initial Desktop Targets

| Platform | Architecture | Planned tier | Package |
| --- | --- | --- | --- |
| Windows 10/11 | x86-64 | 1 | NSIS, with MSI evaluated later |
| Windows 11 | ARM64 | 2 until hardware CI exists | NSIS |
| macOS | Apple Silicon | 1 | signed and notarized DMG |
| macOS | Intel | 2 | signed and notarized DMG |
| Ubuntu LTS | x86-64 | 1 | AppImage and deb |
| Other modern Linux distributions | x86-64 | 2 or 3 | AppImage or community package |

Exact minimum versions will be fixed before the first beta based on Tauri,
WebView, Rust toolchain, and signing requirements.

## WebView Dependencies

The Tauri desktop interface uses the system webview. Rendering and accessibility
must be tested on WebView2 for Windows, WKWebView for macOS, and WebKitGTK for
Linux. The application must detect an unavailable runtime and provide an
actionable installation message where packaging cannot include it.

## File-System Expectations

Tests cover local file systems and representative removable media. Network file
systems are supported on a best-effort basis because locking, metadata, identity,
latency, and cancellation semantics vary. Case sensitivity is never inferred
solely from the operating-system name.

## Platform Integrations

File-manager actions are optional features:

- Windows Explorer context entry;
- macOS Finder service or Quick Action; and
- explicitly supported Linux file-manager actions.

There is no cross-platform promise of a property-page extension. Platform
integration failure must not prevent the desktop app or CLI from operating.

## Mobile and Web

Android, iOS, and browser-only builds are out of scope for 1.0. Their storage and
sandbox models require separate product and security decisions.
