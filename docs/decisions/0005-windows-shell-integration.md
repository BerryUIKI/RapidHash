# ADR-0005: Windows Explorer Context Menu Integration

- Status: Accepted
- Date: 2026-11-02
- Owners: RapidHash maintainers

## Context

Desktop checksum users frequently initiate hashing operations directly from the operating system's file manager (Windows Explorer). RapidHash needs a secure, lightweight, and responsive mechanism to launch or activate the application when users right-click files, folders, or directory whitespace.

## Decision

Implement Windows Explorer integration using per-user (`HKEY_CURRENT_USER\Software\Classes`) static registry shell verbs combined with single-instance process IPC (`tauri-plugin-single-instance`) and an explicit runtime Settings toggle:

1. **Registry Scope**: Register verbs under `HKCU\Software\Classes\*\shell\RapidHash`, `Directory\shell\RapidHash`, and `Directory\Background\shell\RapidHash`.
2. **Elevation Avoidance**: Avoid `HKLM` (all-users) registration so that the feature requires zero UAC administrative elevation and operates safely within the default per-user installation model.
3. **Multi-Selection Model**: Configure `MultiSelectModel="Player"` to allow Explorer to pass multiple selected files in one batch rather than spawning separate processes.
4. **Process Lifecycle & IPC**: Utilize `tauri-plugin-single-instance` to intercept secondary launches, bring the main window forward (`unminimize`, `show`, `set_focus`), and stream incoming paths to the React frontend via typed events without race conditions.
5. **Shell Refresh**: Issue `SHChangeNotify(SHCNE_ASSOCCHANGED, ...)` immediately upon enabling or disabling the setting so that Explorer updates without requiring a system restart or logoff.

## Alternatives Considered

- **In-process COM Shell Extension (`IContextMenu` / `IExplorerCommand` DLL)**:
  Provides first-level context menu placement on Windows 11, but requires native C++/Rust COM in-process DLL injection, complex MSIX/Sparse packaging, code signing identities, and presents stability and crash risks to the Explorer host process.
- **System-wide `HKLM` Shell Registration**:
  Requires UAC elevation during configuration, which creates cross-integrity UIPI communication blocks between elevated and non-elevated instances.
- **Headless CLI invocation from Explorer**:
  Flashes console windows and fails to present progress or results in the rich desktop user interface.

## Consequences

- The context menu operates without elevation and can be toggled cleanly on and off from the application Settings.
- On Windows 11, static registry verbs appear under "Show more options" (`Shift + F10`) rather than the modern primary menu; this is documented in user-facing strings and UX guides.
- Additional platform integrations (macOS Finder Quick Actions and Linux file-manager scripts) will follow similar decoupled IPC routing patterns.
