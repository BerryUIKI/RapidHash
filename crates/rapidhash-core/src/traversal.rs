//! Deterministic and resource-bounded file traversal engine.
//!
//! Implements FILE_TRAVERSAL.md and Section 4 of SECURITY_MODEL.md.

use crate::error::CoreError;
use std::collections::HashSet;
use std::fs::{self, Metadata};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

/// Options configuring file discovery and traversal behaviors.
#[derive(Debug, Clone)]
pub struct TraversalOptions {
    /// Whether to follow directory symbolic links or junctions (default: false).
    pub follow_symlinks: bool,
    /// Whether to include hidden files and directories (default: true).
    pub include_hidden: bool,
    /// Maximum recursive directory depth (default: Some(64)).
    pub max_depth: Option<usize>,
    /// Maximum number of discovered file entries before aborting (default: Some(500,000)).
    pub max_entries: Option<usize>,
}

impl Default for TraversalOptions {
    fn default() -> Self {
        Self {
            follow_symlinks: false,
            include_hidden: true,
            max_depth: Some(64),
            max_entries: Some(500_000),
        }
    }
}

/// A discovered regular file ready for hashing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredFile {
    /// Absolute or explicit input path to the file.
    pub path: PathBuf,
    /// Relative path with respect to the traversal root, if traversed from a directory.
    pub relative_path: Option<PathBuf>,
    /// Snapshot of the file size in bytes at discovery time.
    pub size_bytes: u64,
    /// Whether the item was reached via a symbolic link.
    pub is_symlink: bool,
}

/// Traverse a set of input paths (files or directories) deterministically.
///
/// Returns a list of regular files sorted by normalized path.
/// Respects bounds on depth, total entry count, and cooperative cancellation.
pub fn traverse_paths(
    inputs: &[PathBuf],
    options: &TraversalOptions,
    cancel: Option<&AtomicBool>,
) -> Result<Vec<DiscoveredFile>, CoreError> {
    let mut discovered = Vec::new();
    let mut visited_dirs = HashSet::new();

    for input in inputs {
        if let Some(c) = cancel {
            if c.load(Ordering::Relaxed) {
                return Err(CoreError::Cancelled);
            }
        }

        let metadata = match fs::symlink_metadata(input) {
            Ok(meta) => meta,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return Err(CoreError::NotFound {
                        path: input.display().to_string(),
                    });
                } else if err.kind() == std::io::ErrorKind::PermissionDenied {
                    return Err(CoreError::PermissionDenied {
                        path: input.display().to_string(),
                    });
                } else {
                    return Err(CoreError::Io {
                        path: input.display().to_string(),
                        message: err.to_string(),
                    });
                }
            }
        };

        if metadata.is_dir() {
            traverse_dir_recursive(
                input,
                input,
                0,
                options,
                cancel,
                &mut discovered,
                &mut visited_dirs,
            )?;
        } else if metadata.is_file() {
            check_entry_limit(discovered.len(), options)?;
            discovered.push(DiscoveredFile {
                path: input.clone(),
                relative_path: None,
                size_bytes: metadata.len(),
                is_symlink: metadata.file_type().is_symlink(),
            });
        }
        // Special files (sockets, pipes, devices) are skipped or ignored by default
    }

    Ok(discovered)
}

fn traverse_dir_recursive(
    root: &Path,
    current_dir: &Path,
    current_depth: usize,
    options: &TraversalOptions,
    cancel: Option<&AtomicBool>,
    discovered: &mut Vec<DiscoveredFile>,
    visited_dirs: &mut HashSet<PathBuf>,
) -> Result<(), CoreError> {
    if let Some(c) = cancel {
        if c.load(Ordering::Relaxed) {
            return Err(CoreError::Cancelled);
        }
    }

    if let Some(max_depth) = options.max_depth {
        if current_depth > max_depth {
            return Err(CoreError::Io {
                path: current_dir.display().to_string(),
                message: format!("exceeded maximum directory depth of {max_depth}"),
            });
        }
    }

    // Cycle detection for directory symlinks if follow_symlinks is enabled
    if options.follow_symlinks {
        if let Ok(canonical) = fs::canonicalize(current_dir) {
            if !visited_dirs.insert(canonical) {
                // Already visited this physical directory, avoid infinite cycle
                return Ok(());
            }
        }
    }

    let read_dir = match fs::read_dir(current_dir) {
        Ok(iter) => iter,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::PermissionDenied {
                return Err(CoreError::PermissionDenied {
                    path: current_dir.display().to_string(),
                });
            } else {
                return Err(CoreError::Io {
                    path: current_dir.display().to_string(),
                    message: err.to_string(),
                });
            }
        }
    };

    // Collect and sort entries by file name for determinism
    let mut entries = Vec::new();
    for entry_res in read_dir {
        let entry = entry_res.map_err(|err| CoreError::Io {
            path: current_dir.display().to_string(),
            message: err.to_string(),
        })?;
        entries.push(entry);
    }
    entries.sort_by_key(|a| a.file_name());

    for entry in entries {
        if let Some(c) = cancel {
            if c.load(Ordering::Relaxed) {
                return Err(CoreError::Cancelled);
            }
        }

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !options.include_hidden && is_hidden_name(&file_name_str) {
            continue;
        }

        let entry_path = entry.path();
        let symlink_meta = match entry.metadata() {
            Ok(m) => m,
            Err(err) => {
                return Err(CoreError::Io {
                    path: entry_path.display().to_string(),
                    message: err.to_string(),
                });
            }
        };

        let is_symlink = symlink_meta.file_type().is_symlink();

        if is_symlink && !options.follow_symlinks {
            // Symlinks to directories or files are ignored when follow_symlinks is false
            continue;
        }

        // Resolve target metadata if follow_symlinks is true, or use symlink_meta directly
        let effective_meta: Metadata = if is_symlink && options.follow_symlinks {
            match fs::metadata(&entry_path) {
                Ok(m) => m,
                Err(_) => continue, // Broken symlink, ignore
            }
        } else {
            symlink_meta
        };

        if effective_meta.is_dir() {
            traverse_dir_recursive(
                root,
                &entry_path,
                current_depth + 1,
                options,
                cancel,
                discovered,
                visited_dirs,
            )?;
        } else if effective_meta.is_file() {
            check_entry_limit(discovered.len(), options)?;
            let rel_path = entry_path.strip_prefix(root).ok().map(|p| p.to_path_buf());
            discovered.push(DiscoveredFile {
                path: entry_path,
                relative_path: rel_path,
                size_bytes: effective_meta.len(),
                is_symlink,
            });
        }
    }

    Ok(())
}

#[inline]
fn check_entry_limit(current_count: usize, options: &TraversalOptions) -> Result<(), CoreError> {
    if let Some(max_entries) = options.max_entries {
        if current_count >= max_entries {
            return Err(CoreError::Io {
                path: "traversal".to_string(),
                message: format!("exceeded maximum discovered entries limit of {max_entries}"),
            });
        }
    }
    Ok(())
}

#[inline]
fn is_hidden_name(name: &str) -> bool {
    name.starts_with('.')
}
