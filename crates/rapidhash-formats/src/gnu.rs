//! GNU-style checksum manifest parsing and formatting.
//!
//! Implements Section 3 of CHECKSUM_FORMATS.md:
//! `<lowercase hexadecimal digest><space><space><relative path>`

use crate::entry::ManifestEntry;
use crate::error::FormatError;
use rapidhash_core::{AlgorithmId, Digest};

/// Parse a GNU-style manifest string into a list of entries.
///
/// Supports:
/// - Canonical two-space separator: `<digest>  <path>`
/// - Binary marker single-space asterisk separator: `<digest> *<path>`
/// - Comment lines starting with `#`
/// - Empty/whitespace-only lines (ignored)
///
/// If `default_algo` is None, attempts to deduce the algorithm from the hex digest length.
pub fn parse_gnu_manifest(
    content: &str,
    default_algo: Option<AlgorithmId>,
) -> Result<Vec<ManifestEntry>, FormatError> {
    let mut entries = Vec::new();

    for (idx, raw_line) in content.lines().enumerate() {
        let line_number = idx + 1;
        let line = raw_line.trim_end_matches(['\r', '\n']);
        let trimmed = line.trim();

        // Skip blank lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Must have at least two parts separated by whitespace
        // Look for standard delimiter "  " (text mode) or " *" (binary mode)
        let (hex_part, path_part) = if let Some((h, p)) = line.split_once("  ") {
            (h.trim(), p)
        } else if let Some((h, p)) = line.split_once(" *") {
            (h.trim(), p)
        } else {
            // Fallback: split on first whitespace if standard delimiters not found
            let mut parts = line.splitn(2, |c: char| c.is_ascii_whitespace());
            let h = parts.next().unwrap_or("").trim();
            let p = parts.next().unwrap_or("").trim_start();
            (h, p)
        };

        if hex_part.is_empty() || path_part.is_empty() {
            return Err(FormatError::InvalidSyntax {
                line: line_number,
                reason: "line must contain both a hexadecimal digest and a path".to_string(),
            });
        }

        let algo = match default_algo {
            Some(a) => a,
            None => deduce_algorithm_from_length(hex_part.len(), line_number)?,
        };

        let digest =
            Digest::from_hex(algo, hex_part).map_err(|err| FormatError::InvalidSyntax {
                line: line_number,
                reason: format!("malformed digest hex '{hex_part}': {err}"),
            })?;

        entries.push(ManifestEntry::new(path_part, digest, line_number));
    }

    Ok(entries)
}

/// Format a list of manifest entries into a canonical GNU-style manifest string.
///
/// Uses lowercase hex and two spaces according to specification Section 3.
pub fn format_gnu_manifest(entries: &[ManifestEntry]) -> String {
    let mut output = String::new();
    for entry in entries {
        output.push_str(&entry.digest.to_hex_lowercase());
        output.push_str("  ");
        output.push_str(&entry.path_str);
        output.push('\n');
    }
    output
}

fn deduce_algorithm_from_length(hex_len: usize, line: usize) -> Result<AlgorithmId, FormatError> {
    match hex_len {
        64 => {
            // Both SHA-256 and BLAKE3 produce 32 bytes (64 hex characters).
            // Default to SHA-256 for standard 64-char GNU manifests.
            Ok(AlgorithmId::Sha256)
        }
        8 => Ok(AlgorithmId::Crc32),
        other => Err(FormatError::AmbiguousAlgorithm {
            line,
            length: other,
        }),
    }
}
