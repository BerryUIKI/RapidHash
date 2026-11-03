//! Tauri 2 desktop adapter for RapidHash.

pub mod shell_integration;

use fluent_bundle::{FluentBundle, FluentResource};
use rapidhash_core::algorithm::AlgorithmId;
use rapidhash_core::digest::Digest;
use rapidhash_core::filename_crc::{
    extract_crc_from_filename, insert_crc_into_filename_with_options, FilenameCrcOptions,
};
use rapidhash_core::job::VerificationStatus;
use rapidhash_core::path_policy::resolve_manifest_path;
use rapidhash_core::traversal::{traverse_paths, TraversalOptions};
use rapidhash_formats::{
    format_gnu_manifest, format_sfv_manifest, parse_gnu_manifest, parse_sfv_manifest, ManifestEntry,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};
use unic_langid::LanguageIdentifier;

const EN_COMMON: &str = include_str!("../../../../locales/en/common.ftl");
const EN_CLI: &str = include_str!("../../../../locales/en/cli.ftl");
const EN_DESKTOP: &str = include_str!("../../../../locales/en/desktop.ftl");

const ZH_COMMON: &str = include_str!("../../../../locales/zh-CN/common.ftl");
const ZH_CLI: &str = include_str!("../../../../locales/zh-CN/cli.ftl");
const ZH_DESKTOP: &str = include_str!("../../../../locales/zh-CN/desktop.ftl");

/// Supported algorithm descriptor representation for frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub is_recommended: bool,
    pub digest_length_bytes: usize,
}

/// Item calculation or verification result item for frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResultDto {
    pub path: String,
    pub file_name: String,
    pub size_bytes: Option<u64>,
    pub digests: BTreeMap<String, String>,
    pub status: Option<String>,
    pub error: Option<String>,
    pub filename_crc: Option<String>,
}

/// Manifest verification summary for frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestVerificationSummaryDto {
    pub total: usize,
    pub matches: usize,
    pub mismatches: usize,
    pub missing: usize,
    pub unreadable: usize,
    pub malformed: usize,
    pub unsupported: usize,
    pub items: Vec<FileResultDto>,
}

/// Result of a rename/insert CRC into filename operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameResultDto {
    pub old_path: String,
    pub new_path: String,
    pub crc_hex: Option<String>,
    pub success: bool,
    pub error: Option<String>,
}

/// Options for CRC filename writing format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteCrcOptionsDto {
    pub pattern: Option<String>,
    pub uppercase: Option<bool>,
}

#[tauri::command]
fn get_supported_algorithms() -> Vec<AlgorithmInfo> {
    AlgorithmId::ALL
        .iter()
        .map(|id| {
            let desc = id.descriptor();
            AlgorithmInfo {
                id: id.as_str().to_string(),
                name: id.as_str().to_uppercase(),
                category: format!("{:?}", desc.category),
                is_recommended: desc.is_recommended,
                digest_length_bytes: desc.digest_length_bytes,
            }
        })
        .collect()
}

#[tauri::command]
fn get_locale_strings(locale: String) -> BTreeMap<String, String> {
    let target_lang: LanguageIdentifier = locale.parse().unwrap_or_else(|_| "en".parse().unwrap());
    let mut bundle = FluentBundle::new(vec![target_lang.clone()]);
    bundle.set_use_isolating(false);

    let (common_src, cli_src, desktop_src) = if target_lang.language.as_str() == "zh" {
        (ZH_COMMON, ZH_CLI, ZH_DESKTOP)
    } else {
        (EN_COMMON, EN_CLI, EN_DESKTOP)
    };

    let _ = bundle.add_resource(FluentResource::try_new(common_src.to_string()).unwrap());
    let _ = bundle.add_resource(FluentResource::try_new(cli_src.to_string()).unwrap());
    let _ = bundle.add_resource(FluentResource::try_new(desktop_src.to_string()).unwrap());

    // Extract defined keys
    let keys = [
        "app-name",
        "nav-calculate",
        "nav-verify",
        "nav-generate",
        "nav-settings",
        "calculate-drop-zone",
        "calculate-add-files",
        "calculate-add-folder",
        "calculate-clear-all",
        "calculate-start",
        "verify-target-input",
        "verify-digest-input",
        "verify-manifest-input",
        "verify-action-compare",
        "verify-action-verify-manifest",
        "table-column-name",
        "table-column-path",
        "table-column-size",
        "table-column-algorithm",
        "table-column-status",
        "table-column-digest",
        "table-column-actions",
        "settings-title",
        "settings-algorithms",
        "settings-appearance",
        "settings-theme-auto",
        "settings-theme-light",
        "settings-theme-dark",
        "settings-language",
        "settings-reset",
        "verification-status-match",
        "verification-status-mismatch",
        "verification-status-missing",
        "verification-status-unreadable",
        "verification-status-malformed",
        "verification-status-unsupported",
        "verification-status-cancelled",
        "error-file-not-found",
        "error-permission-denied",
        "action-crc-into-filename",
        "action-save-manifest",
        "action-fold-all",
        "action-expand-all",
        "action-hash-mode-auto",
        "auto-calculate-label",
        "settings-crc-format-title",
        "settings-crc-format-pattern",
        "settings-crc-format-casing",
        "settings-crc-format-preview",
        "settings-crc-uppercase",
        "settings-crc-lowercase",
        "settings-shell-integration",
        "settings-shell-integration-desc",
        "settings-shell-integration-win11-note",
        "settings-shell-integration-unsupported",
        "shell-menu-entry-title",
        "calculate-filter-placeholder",
        "calculate-filter-status",
        "calculate-filter-status-all",
        "calculate-filter-status-ok",
        "calculate-filter-status-mismatch",
        "calculate-filter-status-error",
        "calculate-show-more",
        "calculate-filter-empty",
    ];

    let mut result = BTreeMap::new();
    for key in keys {
        if let Some(msg) = bundle.get_message(key) {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let value = bundle.format_pattern(pattern, None, &mut errors);
                result.insert(key.to_string(), value.to_string());
            }
        }
    }
    result
}

#[tauri::command]
fn calculate_hashes(
    paths: Vec<String>,
    algorithm_ids: Vec<String>,
) -> Result<Vec<FileResultDto>, String> {
    if paths.is_empty() {
        return Ok(Vec::new());
    }

    let mut algorithms = Vec::new();
    for id_str in &algorithm_ids {
        let algo = AlgorithmId::from_str(id_str).map_err(|e| e.to_string())?;
        algorithms.push(algo);
    }
    if algorithms.is_empty() {
        algorithms.push(AlgorithmId::Sha256);
    }

    let input_paths: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
    let options = TraversalOptions::default();

    let discovered = traverse_paths(&input_paths, &options, None).map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    let mut buffer = vec![0u8; 64 * 1024];

    for file in discovered {
        let file_name = file
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| file.path.display().to_string());

        let filename_crc_digest = extract_crc_from_filename(&file_name);
        let filename_crc_str = filename_crc_digest.as_ref().map(|d| d.to_hex_uppercase());

        let mut hashers: Vec<(AlgorithmId, Box<dyn rapidhash_core::Hasher>)> =
            algorithms.iter().map(|a| (*a, a.hasher())).collect();

        match File::open(&file.path) {
            Ok(mut reader) => {
                let mut read_err = None;
                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => {
                            for (_, hasher) in &mut hashers {
                                hasher.update(&buffer[..n]);
                            }
                        }
                        Err(e) => {
                            read_err = Some(e.to_string());
                            break;
                        }
                    }
                }

                if let Some(err_msg) = read_err {
                    results.push(FileResultDto {
                        path: file.path.display().to_string(),
                        file_name,
                        size_bytes: Some(file.size_bytes),
                        digests: BTreeMap::new(),
                        status: Some("Unreadable".to_string()),
                        error: Some(err_msg),
                        filename_crc: filename_crc_str,
                    });
                } else {
                    let mut digests = BTreeMap::new();
                    for (algo, hasher) in hashers {
                        let digest = hasher.finalize();
                        digests.insert(algo.as_str().to_string(), digest.to_canonical_hex());
                    }

                    // Check if filename CRC matches calculated CRC32
                    let mut status = None;
                    if let (Some(ref fn_crc), Some(calc_crc)) =
                        (&filename_crc_digest, digests.get("crc32"))
                    {
                        if fn_crc.to_canonical_hex().eq_ignore_ascii_case(calc_crc) {
                            status = Some(VerificationStatus::Match.stable_id().to_string());
                        } else {
                            status = Some(VerificationStatus::Mismatch.stable_id().to_string());
                        }
                    }

                    results.push(FileResultDto {
                        path: file.path.display().to_string(),
                        file_name,
                        size_bytes: Some(file.size_bytes),
                        digests,
                        status,
                        error: None,
                        filename_crc: filename_crc_str,
                    });
                }
            }
            Err(e) => {
                results.push(FileResultDto {
                    path: file.path.display().to_string(),
                    file_name,
                    size_bytes: Some(file.size_bytes),
                    digests: BTreeMap::new(),
                    status: Some("Unreadable".to_string()),
                    error: Some(e.to_string()),
                    filename_crc: filename_crc_str,
                });
            }
        }
    }

    Ok(results)
}

#[tauri::command]
fn verify_file_checksum(
    path: String,
    algorithm: String,
    expected_digest: String,
) -> Result<FileResultDto, String> {
    let p = PathBuf::from(&path);
    let algo = AlgorithmId::from_str(&algorithm).map_err(|e| e.to_string())?;

    let file_name = p
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| p.display().to_string());

    let filename_crc_str = extract_crc_from_filename(&file_name).map(|d| d.to_hex_uppercase());

    let mut hasher = algo.hasher();
    let mut file = match File::open(&p) {
        Ok(f) => f,
        Err(e) => {
            return Ok(FileResultDto {
                path: p.display().to_string(),
                file_name,
                size_bytes: None,
                digests: BTreeMap::new(),
                status: Some(VerificationStatus::Unreadable.stable_id().to_string()),
                error: Some(e.to_string()),
                filename_crc: filename_crc_str,
            });
        }
    };

    let size = file.metadata().map(|m| m.len()).ok();
    let mut buffer = vec![0u8; 64 * 1024];

    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => hasher.update(&buffer[..n]),
            Err(e) => {
                return Ok(FileResultDto {
                    path: p.display().to_string(),
                    file_name,
                    size_bytes: size,
                    digests: BTreeMap::new(),
                    status: Some(VerificationStatus::Unreadable.stable_id().to_string()),
                    error: Some(e.to_string()),
                    filename_crc: filename_crc_str,
                });
            }
        }
    }

    let computed_digest = hasher.finalize();
    let computed_hex = computed_digest.to_canonical_hex();

    let expected_parsed = Digest::from_hex(algo, &expected_digest);
    let status = match expected_parsed {
        Ok(exp) => {
            if exp == computed_digest {
                VerificationStatus::Match
            } else {
                VerificationStatus::Mismatch
            }
        }
        Err(_) => VerificationStatus::Malformed,
    };

    let mut digests = BTreeMap::new();
    digests.insert(algo.as_str().to_string(), computed_hex);

    Ok(FileResultDto {
        path: p.display().to_string(),
        file_name,
        size_bytes: size,
        digests,
        status: Some(status.stable_id().to_string()),
        error: None,
        filename_crc: filename_crc_str,
    })
}

#[tauri::command]
fn verify_manifest_file(manifest_path: String) -> Result<ManifestVerificationSummaryDto, String> {
    let p = PathBuf::from(&manifest_path);
    let content = std::fs::read_to_string(&p).map_err(|e| e.to_string())?;
    let root_dir = p.parent().unwrap_or_else(|| Path::new("."));

    let ext = p
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let entries: Vec<ManifestEntry> = if ext == "sfv" {
        parse_sfv_manifest(&content).map_err(|e| e.to_string())?
    } else {
        match parse_gnu_manifest(&content, None) {
            Ok(e) => e,
            Err(_) => parse_sfv_manifest(&content).map_err(|e| e.to_string())?,
        }
    };

    let mut summary = ManifestVerificationSummaryDto {
        total: 0,
        matches: 0,
        mismatches: 0,
        missing: 0,
        unreadable: 0,
        malformed: 0,
        unsupported: 0,
        items: Vec::new(),
    };

    for entry in &entries {
        summary.total += 1;
        let algo = entry.digest.algorithm();
        let expected_digest = &entry.digest;

        let resolved_path = match resolve_manifest_path(root_dir, &entry.path_str) {
            Ok(path) => path,
            Err(_) => {
                summary.missing += 1;
                summary.items.push(FileResultDto {
                    path: entry.path_str.clone(),
                    file_name: entry.path_str.clone(),
                    size_bytes: None,
                    digests: BTreeMap::new(),
                    status: Some(VerificationStatus::Missing.stable_id().to_string()),
                    error: Some("Path outside approved root or invalid".to_string()),
                    filename_crc: None,
                });
                continue;
            }
        };

        let file_name = resolved_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| entry.path_str.clone());

        let filename_crc_str = extract_crc_from_filename(&file_name).map(|d| d.to_hex_uppercase());

        if !resolved_path.exists() {
            summary.missing += 1;
            summary.items.push(FileResultDto {
                path: resolved_path.display().to_string(),
                file_name,
                size_bytes: None,
                digests: BTreeMap::new(),
                status: Some(VerificationStatus::Missing.stable_id().to_string()),
                error: None,
                filename_crc: filename_crc_str,
            });
            continue;
        }

        let mut hasher = algo.hasher();
        let mut buffer = vec![0u8; 64 * 1024];
        match File::open(&resolved_path) {
            Ok(mut f) => {
                let size = f.metadata().map(|m| m.len()).ok();
                let mut read_failed = false;
                loop {
                    match f.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => hasher.update(&buffer[..n]),
                        Err(_) => {
                            read_failed = true;
                            break;
                        }
                    }
                }

                if read_failed {
                    summary.unreadable += 1;
                    summary.items.push(FileResultDto {
                        path: resolved_path.display().to_string(),
                        file_name,
                        size_bytes: size,
                        digests: BTreeMap::new(),
                        status: Some(VerificationStatus::Unreadable.stable_id().to_string()),
                        error: None,
                        filename_crc: filename_crc_str,
                    });
                } else {
                    let computed = hasher.finalize();
                    let status = if &computed == expected_digest {
                        summary.matches += 1;
                        VerificationStatus::Match
                    } else {
                        summary.mismatches += 1;
                        VerificationStatus::Mismatch
                    };

                    let mut computed_digests = BTreeMap::new();
                    computed_digests.insert(algo.as_str().to_string(), computed.to_canonical_hex());

                    summary.items.push(FileResultDto {
                        path: resolved_path.display().to_string(),
                        file_name,
                        size_bytes: size,
                        digests: computed_digests,
                        status: Some(status.stable_id().to_string()),
                        error: None,
                        filename_crc: filename_crc_str,
                    });
                }
            }
            Err(e) => {
                summary.unreadable += 1;
                summary.items.push(FileResultDto {
                    path: resolved_path.display().to_string(),
                    file_name,
                    size_bytes: None,
                    digests: BTreeMap::new(),
                    status: Some(VerificationStatus::Unreadable.stable_id().to_string()),
                    error: Some(e.to_string()),
                    filename_crc: filename_crc_str,
                });
            }
        }
    }

    Ok(summary)
}

#[tauri::command]
fn write_crc_to_filename(
    file_path: String,
    crc_hex: Option<String>,
    options: Option<WriteCrcOptionsDto>,
) -> RenameResultDto {
    let p = PathBuf::from(&file_path);
    if !p.exists() {
        return RenameResultDto {
            old_path: file_path,
            new_path: String::new(),
            crc_hex: None,
            success: false,
            error: Some("File does not exist".to_string()),
        };
    }

    let digest = match crc_hex.and_then(|h| {
        let trimmed = h.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }) {
        Some(h) => match Digest::from_hex(AlgorithmId::Crc32, &h) {
            Ok(d) => d,
            Err(e) => {
                return RenameResultDto {
                    old_path: file_path,
                    new_path: String::new(),
                    crc_hex: None,
                    success: false,
                    error: Some(format!("Invalid CRC32 hex: {e}")),
                };
            }
        },
        None => {
            // Compute CRC32 on the fly if not provided
            match File::open(&p) {
                Ok(mut file) => {
                    let mut hasher = AlgorithmId::Crc32.hasher();
                    let mut buffer = vec![0u8; 64 * 1024];
                    let mut read_err = None;
                    loop {
                        match file.read(&mut buffer) {
                            Ok(0) => break,
                            Ok(n) => hasher.update(&buffer[..n]),
                            Err(e) => {
                                read_err = Some(e.to_string());
                                break;
                            }
                        }
                    }
                    if let Some(err) = read_err {
                        return RenameResultDto {
                            old_path: file_path,
                            new_path: String::new(),
                            crc_hex: None,
                            success: false,
                            error: Some(format!("Failed to read file for CRC32: {err}")),
                        };
                    }
                    hasher.finalize()
                }
                Err(e) => {
                    return RenameResultDto {
                        old_path: file_path,
                        new_path: String::new(),
                        crc_hex: None,
                        success: false,
                        error: Some(format!("Cannot open file for CRC32: {e}")),
                    };
                }
            }
        }
    };

    let crc_options = FilenameCrcOptions {
        pattern: options
            .as_ref()
            .and_then(|o| o.pattern.clone())
            .unwrap_or_else(|| "{name} [{crc}]".to_string()),
        uppercase: options.as_ref().and_then(|o| o.uppercase).unwrap_or(true),
    };

    let new_path = match insert_crc_into_filename_with_options(&p, &digest, &crc_options) {
        Ok(np) => np,
        Err(e) => {
            return RenameResultDto {
                old_path: file_path,
                new_path: String::new(),
                crc_hex: None,
                success: false,
                error: Some(format!("Failed to generate new filename: {e}")),
            };
        }
    };

    let hex_to_return = if crc_options.uppercase {
        digest.to_hex_uppercase()
    } else {
        digest.to_hex_lowercase()
    };

    if new_path == p {
        return RenameResultDto {
            old_path: file_path,
            new_path: new_path.display().to_string(),
            crc_hex: Some(hex_to_return),
            success: true,
            error: None,
        };
    }

    if new_path.exists() {
        return RenameResultDto {
            old_path: file_path,
            new_path: new_path.display().to_string(),
            crc_hex: Some(hex_to_return),
            success: false,
            error: Some(format!(
                "Destination file already exists: {}",
                new_path.display()
            )),
        };
    }

    match std::fs::rename(&p, &new_path) {
        Ok(_) => RenameResultDto {
            old_path: file_path,
            new_path: new_path.display().to_string(),
            crc_hex: Some(hex_to_return),
            success: true,
            error: None,
        },
        Err(e) => RenameResultDto {
            old_path: file_path,
            new_path: new_path.display().to_string(),
            crc_hex: Some(hex_to_return),
            success: false,
            error: Some(e.to_string()),
        },
    }
}

#[tauri::command]
fn save_manifest_file(
    output_path: String,
    format: String,
    items: Vec<FileResultDto>,
) -> Result<String, String> {
    let out_p = PathBuf::from(&output_path);
    let root = out_p.parent().unwrap_or_else(|| Path::new("."));

    let mut entries = Vec::new();
    for (idx, item) in items.iter().enumerate() {
        let p = PathBuf::from(&item.path);
        let rel_path = match p.strip_prefix(root) {
            Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
            Err(_) => item.file_name.clone(),
        };

        if format == "sfv" {
            if let Some(crc_hex) = item.digests.get("crc32") {
                if let Ok(d) = Digest::from_hex(AlgorithmId::Crc32, crc_hex) {
                    entries.push(ManifestEntry::new(rel_path, d, idx + 1));
                }
            }
        } else {
            // default gnu sha256 or chosen
            let (algo_key, algo_id) = if item.digests.contains_key("sha256") {
                ("sha256", AlgorithmId::Sha256)
            } else if item.digests.contains_key("blake3") {
                ("blake3", AlgorithmId::Blake3)
            } else if item.digests.contains_key("crc32") {
                ("crc32", AlgorithmId::Crc32)
            } else {
                continue;
            };

            if let Some(hex) = item.digests.get(algo_key) {
                if let Ok(d) = Digest::from_hex(algo_id, hex) {
                    entries.push(ManifestEntry::new(rel_path, d, idx + 1));
                }
            }
        }
    }

    let manifest_text = if format == "sfv" {
        format_sfv_manifest(&entries, Some("Generated by RapidHash"))
    } else {
        format_gnu_manifest(&entries)
    };

    std::fs::write(&out_p, manifest_text).map_err(|e| e.to_string())?;
    Ok(out_p.display().to_string())
}

/// Escape a value for RFC 4180 comma-separated output.
fn csv_escape(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// Render result records as CSV with a stable column order.
///
/// Fixed columns come first, followed by one column per digest algorithm found
/// in the records, sorted by identifier.
pub fn render_results_csv(items: &[FileResultDto]) -> String {
    let mut algorithm_keys: BTreeSet<&str> = BTreeSet::new();
    for item in items {
        for key in item.digests.keys() {
            algorithm_keys.insert(key.as_str());
        }
    }

    let mut header = vec![
        "path",
        "file_name",
        "size_bytes",
        "status",
        "error",
        "filename_crc",
    ];
    header.extend(algorithm_keys.iter().copied());

    let mut output = String::new();
    output.push_str(
        &header
            .iter()
            .map(|column| csv_escape(column))
            .collect::<Vec<_>>()
            .join(","),
    );
    output.push_str("\r\n");

    for item in items {
        let mut row = Vec::with_capacity(header.len());
        row.push(csv_escape(&item.path));
        row.push(csv_escape(&item.file_name));
        row.push(
            item.size_bytes
                .map(|size| size.to_string())
                .unwrap_or_default(),
        );
        row.push(csv_escape(item.status.as_deref().unwrap_or("")));
        row.push(csv_escape(item.error.as_deref().unwrap_or("")));
        row.push(csv_escape(item.filename_crc.as_deref().unwrap_or("")));
        for key in &algorithm_keys {
            row.push(csv_escape(
                item.digests.get(*key).map(String::as_str).unwrap_or(""),
            ));
        }
        output.push_str(&row.join(","));
        output.push_str("\r\n");
    }

    output
}

#[tauri::command]
fn save_results_file(
    output_path: String,
    format: String,
    items: Vec<FileResultDto>,
) -> Result<String, String> {
    let out_p = PathBuf::from(&output_path);
    let content = match format.as_str() {
        "csv" => render_results_csv(&items),
        "json" => serde_json::to_string_pretty(&items).map_err(|e| e.to_string())?,
        other => return Err(format!("Unsupported export format: {other}")),
    };

    std::fs::write(&out_p, content).map_err(|e| e.to_string())?;
    Ok(out_p.display().to_string())
}

#[cfg(test)]
mod export_tests {
    use super::*;

    fn sample_item(path: &str, name: &str, digests: &[(&str, &str)]) -> FileResultDto {
        FileResultDto {
            path: path.to_string(),
            file_name: name.to_string(),
            size_bytes: Some(42),
            digests: digests
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
            status: Some("Match".to_string()),
            error: None,
            filename_crc: None,
        }
    }

    #[test]
    fn csv_escapes_commas_quotes_and_newlines() {
        let item = sample_item("C:\\data\\a,b.txt", "a,\"b\".txt", &[("sha256", "aa")]);
        let csv = render_results_csv(&[item]);
        assert!(csv.contains("\"C:\\data\\a,b.txt\""));
        assert!(csv.contains("\"a,\"\"b\"\".txt\""));
    }

    #[test]
    fn csv_uses_union_of_algorithm_columns() {
        let first = sample_item(
            "C:\\one.bin",
            "one.bin",
            &[("sha256", "aa"), ("crc32", "bb")],
        );
        let second = sample_item("C:\\two.bin", "two.bin", &[("blake3", "cc")]);
        let csv = render_results_csv(&[first, second]);
        let mut lines = csv.lines();

        let header = lines.next().expect("header row");
        assert_eq!(
            header,
            "path,file_name,size_bytes,status,error,filename_crc,blake3,crc32,sha256"
        );

        let first_row = lines.next().expect("first data row");
        assert_eq!(first_row, "C:\\one.bin,one.bin,42,Match,,,,bb,aa");
        let second_row = lines.next().expect("second data row");
        assert_eq!(second_row, "C:\\two.bin,two.bin,42,Match,,,cc,,");
    }

    #[test]
    fn json_export_preserves_records() {
        let item = sample_item("C:\\one.bin", "one.bin", &[("sha256", "aa")]);
        let json = serde_json::to_string_pretty(&vec![item]).expect("serialize results");
        assert!(json.contains("\"file_name\": \"one.bin\""));
        assert!(json.contains("\"sha256\": \"aa\""));
    }
}

/// Pending paths passed via CLI arguments on launch.
#[derive(Default)]
pub struct PendingPaths(pub Mutex<Vec<String>>);

#[tauri::command]
fn take_pending_paths(state: State<PendingPaths>) -> Vec<String> {
    let mut lock = state.0.lock().unwrap_or_else(|e| e.into_inner());
    std::mem::take(&mut *lock)
}

#[tauri::command]
fn get_shell_integration_status() -> shell_integration::ShellIntegrationStatusDto {
    shell_integration::get_shell_integration_status()
}

#[tauri::command]
fn set_shell_integration_enabled(
    enabled: bool,
) -> Result<shell_integration::ShellIntegrationStatusDto, String> {
    shell_integration::set_shell_integration_enabled(enabled)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            let paths = shell_integration::extract_paths_from_argv(&argv);
            if !paths.is_empty() {
                let _ = app.emit("incoming-paths", &paths);
            }
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let cold_argv: Vec<String> = std::env::args_os()
                .map(|s| s.to_string_lossy().to_string())
                .collect();
            let cold_paths = shell_integration::extract_paths_from_argv(&cold_argv);
            app.manage(PendingPaths(Mutex::new(cold_paths)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_supported_algorithms,
            get_locale_strings,
            calculate_hashes,
            verify_file_checksum,
            verify_manifest_file,
            write_crc_to_filename,
            save_manifest_file,
            save_results_file,
            take_pending_paths,
            get_shell_integration_status,
            set_shell_integration_enabled,
        ])
        .run(tauri::generate_context!())
        .expect("error while running rapidhash desktop application");
}
