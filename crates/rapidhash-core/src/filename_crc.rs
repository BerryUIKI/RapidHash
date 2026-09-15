//! Filename CRC extraction, verification, and insertion logic.
//!
//! Inspired by RapidCRC conventions (e.g. `[A1B2C3D4]` or `(A1B2C3D4)` in filename).

use crate::algorithm::AlgorithmId;
use crate::digest::Digest;
use crate::error::CoreError;
use std::path::{Path, PathBuf};

/// Formatting options for embedding CRC into filenames.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilenameCrcOptions {
    /// Pattern template containing `{name}` and `{crc}`.
    /// E.g. `"{name} [{crc}]"` -> `"Movie [4E2A10FB].mkv"`.
    /// E.g. `"{name} ({crc})"` -> `"Movie (4E2A10FB).mkv"`.
    /// E.g. `"{name}_[{crc}]"` -> `"Movie_[4E2A10FB].mkv"`.
    pub pattern: String,
    /// Whether the CRC hex digits should be uppercase. Defaults to `true`.
    pub uppercase: bool,
}

impl Default for FilenameCrcOptions {
    fn default() -> Self {
        Self {
            pattern: "{name} [{crc}]".to_string(),
            uppercase: true,
        }
    }
}

/// Extract an 8-character hexadecimal CRC-32 from a filename, if present in `[...]` or `(...)`.
///
/// Matches typical release and checksum conventions:
/// e.g. "Release_v1.0_[12AB34CD].zip" -> Some(Digest for CRC32 12ab34cd).
pub fn extract_crc_from_filename(filename: &str) -> Option<Digest> {
    let chars: Vec<char> = filename.chars().collect();
    if chars.len() < 10 {
        return None;
    }

    // Search from rear to front
    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        let close_delim = chars[i];
        if close_delim == ']' || close_delim == ')' {
            let open_delim = if close_delim == ']' { '[' } else { '(' };
            if i >= 9 && chars[i - 9] == open_delim {
                let candidate: String = chars[i - 8..i].iter().collect();
                if candidate.chars().all(|c| c.is_ascii_hexdigit()) {
                    if let Ok(digest) = Digest::from_hex(AlgorithmId::Crc32, &candidate) {
                        return Some(digest);
                    }
                }
            }
        }
    }
    None
}

/// Strips an existing trailing `[CRC32]` or `(CRC32)` (and preceding separator) from a stem.
fn strip_existing_crc_from_stem(stem: &str) -> &str {
    let chars: Vec<char> = stem.chars().collect();
    if chars.len() < 10 {
        return stem;
    }

    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        let close_delim = chars[i];
        if close_delim == ']' || close_delim == ')' {
            let open_delim = if close_delim == ']' { '[' } else { '(' };
            if i >= 9 && chars[i - 9] == open_delim {
                let candidate: String = chars[i - 8..i].iter().collect();
                if candidate.chars().all(|c| c.is_ascii_hexdigit()) {
                    // Check if there is a preceding separator like ' ', '_', '-', or '.'
                    let mut start_idx = i - 9;
                    if start_idx > 0 {
                        let sep = chars[start_idx - 1];
                        if sep == ' ' || sep == '_' || sep == '-' || sep == '.' {
                            start_idx -= 1;
                        }
                    }
                    let byte_offset: usize = chars[..start_idx].iter().map(|c| c.len_utf8()).sum();
                    return &stem[..byte_offset];
                }
            }
        }
    }
    stem
}

/// Generate a new file path with the CRC32 embedded in the filename using default options.
pub fn insert_crc_into_filename(path: &Path, crc: &Digest) -> Result<PathBuf, CoreError> {
    insert_crc_into_filename_with_options(path, crc, &FilenameCrcOptions::default())
}

/// Generate a new file path with the CRC32 embedded in the filename using configurable options.
pub fn insert_crc_into_filename_with_options(
    path: &Path,
    crc: &Digest,
    options: &FilenameCrcOptions,
) -> Result<PathBuf, CoreError> {
    if crc.algorithm() != AlgorithmId::Crc32 {
        return Err(CoreError::UnsupportedAlgorithm {
            algorithm: crc.algorithm().as_str().to_string(),
        });
    }

    let crc_str = if options.uppercase {
        crc.to_hex_uppercase()
    } else {
        crc.to_hex_lowercase()
    };

    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let original_name =
        path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| CoreError::PathEscape {
                path: path.display().to_string(),
            })?;

    // Split extension
    let (raw_stem, ext) = match original_name.rfind('.') {
        Some(idx) => (&original_name[..idx], &original_name[idx..]),
        None => (original_name, ""),
    };

    let clean_stem = strip_existing_crc_from_stem(raw_stem);

    let pattern_buf;
    let pattern = if options.pattern.contains("{name}") {
        options.pattern.as_str()
    } else if options.pattern.contains("{crc}") {
        pattern_buf = format!("{{name}} {}", options.pattern);
        pattern_buf.as_str()
    } else {
        "{name} [{crc}]"
    };

    let new_stem = pattern
        .replace("{name}", clean_stem)
        .replace("{crc}", &crc_str);

    let new_filename = format!("{new_stem}{ext}");

    Ok(parent.join(new_filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_crc() {
        assert!(extract_crc_from_filename("file.txt").is_none());
        assert!(extract_crc_from_filename("file_[1234567].txt").is_none()); // too short
        assert!(extract_crc_from_filename("file_[1234567G].txt").is_none()); // not hex

        let d1 = extract_crc_from_filename("Movie_[4E2A10FB].mkv").unwrap();
        assert_eq!(d1.to_hex_uppercase(), "4E2A10FB");

        let d2 = extract_crc_from_filename("Song_(a1b2c3d4).mp3").unwrap();
        assert_eq!(d2.to_hex_uppercase(), "A1B2C3D4");
    }

    #[test]
    fn test_insert_crc() {
        let digest = Digest::from_hex(AlgorithmId::Crc32, "4e2a10fb").unwrap();

        let p1 = Path::new("C:/Videos/Movie.mkv");
        let new_p1 = insert_crc_into_filename(p1, &digest).unwrap();
        assert_eq!(new_p1, PathBuf::from("C:/Videos/Movie [4E2A10FB].mkv"));

        let p2 = Path::new("C:/Videos/Movie_[00000000].mkv");
        let new_p2 = insert_crc_into_filename(p2, &digest).unwrap();
        assert_eq!(new_p2, PathBuf::from("C:/Videos/Movie [4E2A10FB].mkv"));

        let p3 = Path::new("C:/Videos/Movie_(00000000).mkv");
        let new_p3 = insert_crc_into_filename(p3, &digest).unwrap();
        assert_eq!(new_p3, PathBuf::from("C:/Videos/Movie [4E2A10FB].mkv"));
    }

    #[test]
    fn test_insert_crc_with_options() {
        let digest = Digest::from_hex(AlgorithmId::Crc32, "4e2a10fb").unwrap();

        let p1 = Path::new("C:/Videos/Movie.mkv");
        let opts_parens = FilenameCrcOptions {
            pattern: "{name} ({crc})".to_string(),
            uppercase: true,
        };
        assert_eq!(
            insert_crc_into_filename_with_options(p1, &digest, &opts_parens).unwrap(),
            PathBuf::from("C:/Videos/Movie (4E2A10FB).mkv")
        );

        let opts_underscore = FilenameCrcOptions {
            pattern: "{name}_[{crc}]".to_string(),
            uppercase: true,
        };
        assert_eq!(
            insert_crc_into_filename_with_options(p1, &digest, &opts_underscore).unwrap(),
            PathBuf::from("C:/Videos/Movie_[4E2A10FB].mkv")
        );

        let opts_lowercase = FilenameCrcOptions {
            pattern: "{name} [{crc}]".to_string(),
            uppercase: false,
        };
        assert_eq!(
            insert_crc_into_filename_with_options(p1, &digest, &opts_lowercase).unwrap(),
            PathBuf::from("C:/Videos/Movie [4e2a10fb].mkv")
        );

        // Replacing existing CRC in another format
        let p2 = Path::new("C:/Videos/Movie [00000000].mkv");
        assert_eq!(
            insert_crc_into_filename_with_options(p2, &digest, &opts_parens).unwrap(),
            PathBuf::from("C:/Videos/Movie (4E2A10FB).mkv")
        );
    }
}
