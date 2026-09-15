//! Filename CRC extraction, verification, and insertion logic.
//!
//! Inspired by RapidCRC conventions (e.g. `[A1B2C3D4]` or `(A1B2C3D4)` in filename).

use crate::algorithm::AlgorithmId;
use crate::digest::Digest;
use crate::error::CoreError;
use std::path::{Path, PathBuf};

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

/// Generate a new file path with the CRC32 embedded in the filename.
///
/// If a CRC already exists in brackets/parentheses, it is replaced.
/// Otherwise, ` [CRC32]` is inserted right before the file extension.
pub fn insert_crc_into_filename(path: &Path, crc: &Digest) -> Result<PathBuf, CoreError> {
    if crc.algorithm() != AlgorithmId::Crc32 {
        return Err(CoreError::UnsupportedAlgorithm {
            algorithm: crc.algorithm().as_str().to_string(),
        });
    }

    let crc_str = crc.to_hex_uppercase();
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let original_name =
        path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| CoreError::PathEscape {
                path: path.display().to_string(),
            })?;

    // Check if filename already has `[OLD_CRC]` or `(OLD_CRC)`
    let chars: Vec<char> = original_name.chars().collect();
    let mut replaced = None;

    if chars.len() >= 10 {
        let mut i = chars.len();
        while i > 0 {
            i -= 1;
            let close_delim = chars[i];
            if close_delim == ']' || close_delim == ')' {
                let open_delim = if close_delim == ']' { '[' } else { '(' };
                if i >= 9 && chars[i - 9] == open_delim {
                    let candidate: String = chars[i - 8..i].iter().collect();
                    if candidate.chars().all(|c| c.is_ascii_hexdigit()) {
                        let mut new_name = String::new();
                        new_name.extend(&chars[..i - 8]);
                        new_name.push_str(&crc_str);
                        new_name.extend(&chars[i..]);
                        replaced = Some(new_name);
                        break;
                    }
                }
            }
        }
    }

    let new_filename = if let Some(r) = replaced {
        r
    } else {
        // Insert right before extension
        if let Some(ext_idx) = original_name.rfind('.') {
            let (stem, ext) = original_name.split_at(ext_idx);
            format!("{stem} [{crc_str}]{ext}")
        } else {
            format!("{original_name} [{crc_str}]")
        }
    };

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
        assert_eq!(new_p2, PathBuf::from("C:/Videos/Movie_[4E2A10FB].mkv"));

        let p3 = Path::new("C:/Videos/Movie_(00000000).mkv");
        let new_p3 = insert_crc_into_filename(p3, &digest).unwrap();
        assert_eq!(new_p3, PathBuf::from("C:/Videos/Movie_(4E2A10FB).mkv"));
    }
}
