//! Path normalization and security confinement policies for RapidHash.
//!
//! Implements Section 4 (Manifest Path Traversal) of SECURITY_MODEL.md
//! and Section 6 of CHECKSUM_FORMATS.md.

use crate::error::CoreError;
use std::path::{Component, Path, PathBuf};

/// Safely resolve and confine an untrusted relative path against an approved root.
///
/// Returns an absolute, lexically confined PathBuf pointing inside `approved_root`.
/// Rejects:
/// - Absolute paths (POSIX `/...` or Windows drive/UNC `C:\...`, `\\...`).
/// - Paths containing NUL bytes.
/// - Paths attempting to traverse outside `approved_root` using `..` components.
/// - Paths attempting Windows-style alternate data streams (`:stream`).
pub fn resolve_manifest_path(
    approved_root: &Path,
    untrusted_rel: &str,
) -> Result<PathBuf, CoreError> {
    if untrusted_rel.contains('\0') {
        return Err(CoreError::PathEscape {
            path: untrusted_rel.to_string(),
        });
    }

    // Windows alternate data stream protection
    if untrusted_rel.contains(':') {
        return Err(CoreError::PathEscape {
            path: untrusted_rel.to_string(),
        });
    }

    let input_path = Path::new(untrusted_rel);

    // Reject absolute paths
    if input_path.is_absolute() {
        return Err(CoreError::PathEscape {
            path: untrusted_rel.to_string(),
        });
    }

    // Reject prefix components (e.g. Windows drive letters or UNC prefixes)
    for comp in input_path.components() {
        if matches!(comp, Component::Prefix(_)) {
            return Err(CoreError::PathEscape {
                path: untrusted_rel.to_string(),
            });
        }
    }

    // Lexically normalize relative components
    let mut normalized_parts: Vec<&str> = Vec::new();
    for comp in input_path.components() {
        match comp {
            Component::Normal(part) => {
                let part_str = part.to_str().ok_or_else(|| CoreError::PathEscape {
                    path: untrusted_rel.to_string(),
                })?;
                normalized_parts.push(part_str);
            }
            Component::CurDir => {
                // Ignore '.'
            }
            Component::ParentDir => {
                // Popping a component. If empty, the path attempts to escape the root!
                if normalized_parts.pop().is_none() {
                    return Err(CoreError::PathEscape {
                        path: untrusted_rel.to_string(),
                    });
                }
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(CoreError::PathEscape {
                    path: untrusted_rel.to_string(),
                });
            }
        }
    }

    if normalized_parts.is_empty() {
        return Err(CoreError::PathEscape {
            path: untrusted_rel.to_string(),
        });
    }

    let mut resolved = approved_root.to_path_buf();
    for part in normalized_parts {
        resolved.push(part);
    }

    Ok(resolved)
}

/// Convert a relative or absolute path to a portable manifest path using forward slashes.
///
/// Strip any leading `./` or redundant separators.
pub fn to_portable_manifest_path(path: &Path) -> String {
    let mut components = Vec::new();
    for comp in path.components() {
        match comp {
            Component::Normal(part) => {
                components.push(part.to_string_lossy());
            }
            Component::ParentDir => {
                components.push("..".into());
            }
            _ => {}
        }
    }
    components.join("/")
}
