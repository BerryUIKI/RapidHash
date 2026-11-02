//! Windows Explorer shell context menu integration.
//!
//! Provides registry management for adding RapidHash to the context menu of
//! files (*), directories (Directory), and folder whitespace (Directory\Background)
//! under HKEY_CURRENT_USER without requiring administrative privileges.

use serde::{Deserialize, Serialize};

/// Shell integration status information for frontend presentation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShellIntegrationStatusDto {
    pub supported: bool,
    pub enabled: bool,
    pub current_exe: String,
    pub error: Option<String>,
}

/// Extracts clean file/directory paths from an argument list.
///
/// Handles arguments passed directly or following an `--open` flag, stripping
/// quotes and omitting flags.
pub fn extract_paths_from_argv(argv: &[String]) -> Vec<String> {
    if argv.is_empty() {
        return Vec::new();
    }

    let mut paths = Vec::new();
    let mut after_open = false;

    // Skip the first argument if it looks like the executable itself
    let args_to_process =
        if argv.len() > 1 && (argv[0].ends_with(".exe") || argv[0].contains("rapidhash")) {
            &argv[1..]
        } else {
            argv
        };

    for arg in args_to_process {
        let trimmed = arg.trim().trim_matches('"').trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "--open" {
            after_open = true;
            continue;
        }

        if after_open {
            if !trimmed.starts_with("--") {
                paths.push(trimmed.to_string());
            }
        } else if !trimmed.starts_with('-') && !trimmed.starts_with('/') {
            paths.push(trimmed.to_string());
        }
    }

    // Deduplicate while preserving order
    let mut seen = std::collections::HashSet::new();
    paths.retain(|p| seen.insert(p.clone()));

    paths
}

#[cfg(windows)]
mod win_ffi {
    use std::ffi::c_void;

    pub const SHCNE_ASSOCCHANGED: i32 = 0x0800_0000;
    pub const SHCNF_IDLIST: u32 = 0x0000;

    #[link(name = "shell32")]
    extern "system" {
        pub fn SHChangeNotify(
            w_event_id: i32,
            u_flags: u32,
            dw_item1: *const c_void,
            dw_item2: *const c_void,
        );
    }
}

#[cfg(windows)]
pub fn get_shell_integration_status() -> ShellIntegrationStatusDto {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let current_exe = match std::env::current_exe() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(e) => {
            return ShellIntegrationStatusDto {
                supported: true,
                enabled: false,
                current_exe: String::new(),
                error: Some(format!("Failed to determine current executable path: {e}")),
            };
        }
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let command_key_path = r"Software\Classes\*\shell\RapidHash\command";
    let enabled = match hkcu.open_subkey(command_key_path) {
        Ok(key) => {
            let cmd_val: Result<String, _> = key.get_value("");
            cmd_val.is_ok()
        }
        Err(_) => false,
    };

    ShellIntegrationStatusDto {
        supported: true,
        enabled,
        current_exe,
        error: None,
    }
}

#[cfg(not(windows))]
pub fn get_shell_integration_status() -> ShellIntegrationStatusDto {
    ShellIntegrationStatusDto {
        supported: false,
        enabled: false,
        current_exe: String::new(),
        error: None,
    }
}

#[cfg(windows)]
pub fn set_shell_integration_enabled(enabled: bool) -> Result<ShellIntegrationStatusDto, String> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS};
    use winreg::RegKey;

    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to determine current executable path: {e}"))?;
    let exe_path_str = current_exe.to_string_lossy().to_string();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let verb_keys = [
        (
            r"Software\Classes\*\shell\RapidHash",
            format!("\"{}\" --open \"%1\"", exe_path_str),
            true,
        ),
        (
            r"Software\Classes\Directory\shell\RapidHash",
            format!("\"{}\" --open \"%1\"", exe_path_str),
            true,
        ),
        (
            r"Software\Classes\Directory\Background\shell\RapidHash",
            format!("\"{}\" --open \"%V\"", exe_path_str),
            false,
        ),
    ];

    if enabled {
        let icon_str = format!("\"{}\",0", exe_path_str);

        for (key_path, command_str, is_multi) in &verb_keys {
            let (verb_key, _) = hkcu
                .create_subkey_with_flags(key_path, KEY_ALL_ACCESS)
                .map_err(|e| format!("Failed to create registry key {key_path}: {e}"))?;

            verb_key
                .set_value("", &"RapidHash")
                .map_err(|e| format!("Failed to set default display name for {key_path}: {e}"))?;
            verb_key
                .set_value("Icon", &icon_str)
                .map_err(|e| format!("Failed to set Icon for {key_path}: {e}"))?;

            if *is_multi {
                verb_key
                    .set_value("MultiSelectModel", &"Player")
                    .map_err(|e| format!("Failed to set MultiSelectModel for {key_path}: {e}"))?;
            }

            let (cmd_key, _) = verb_key
                .create_subkey_with_flags("command", KEY_ALL_ACCESS)
                .map_err(|e| format!("Failed to create command key for {key_path}: {e}"))?;

            cmd_key
                .set_value("", command_str)
                .map_err(|e| format!("Failed to set command value for {key_path}: {e}"))?;
        }
    } else {
        for (key_path, _, _) in &verb_keys {
            let _ = hkcu.delete_subkey_all(key_path);
        }
    }

    // Notify Windows Explorer of the association change
    unsafe {
        win_ffi::SHChangeNotify(
            win_ffi::SHCNE_ASSOCCHANGED,
            win_ffi::SHCNF_IDLIST,
            std::ptr::null(),
            std::ptr::null(),
        );
    }

    Ok(get_shell_integration_status())
}

#[cfg(not(windows))]
pub fn set_shell_integration_enabled(_enabled: bool) -> Result<ShellIntegrationStatusDto, String> {
    Err("Windows Explorer integration is only supported on Windows".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_paths_from_argv_empty() {
        let empty: Vec<String> = Vec::new();
        assert!(extract_paths_from_argv(&empty).is_empty());
    }

    #[test]
    fn test_extract_paths_from_argv_with_open_flag() {
        let argv = vec![
            "RapidHash.exe".to_string(),
            "--open".to_string(),
            "\"C:\\test\\file 1.txt\"".to_string(),
            "D:\\images\\photo.png".to_string(),
        ];
        let paths = extract_paths_from_argv(&argv);
        assert_eq!(
            paths,
            vec![
                "C:\\test\\file 1.txt".to_string(),
                "D:\\images\\photo.png".to_string()
            ]
        );
    }

    #[test]
    fn test_extract_paths_from_argv_direct_files() {
        let argv = vec![
            "C:\\Program Files\\RapidHash\\RapidHash.exe".to_string(),
            "C:\\data\\sample.bin".to_string(),
        ];
        let paths = extract_paths_from_argv(&argv);
        assert_eq!(paths, vec!["C:\\data\\sample.bin".to_string()]);
    }

    #[test]
    fn test_extract_paths_deduplication() {
        let argv = vec![
            "rapidhash".to_string(),
            "--open".to_string(),
            "C:\\file.txt".to_string(),
            "C:\\file.txt".to_string(),
        ];
        let paths = extract_paths_from_argv(&argv);
        assert_eq!(paths, vec!["C:\\file.txt".to_string()]);
    }
}
