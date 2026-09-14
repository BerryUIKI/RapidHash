//! Tauri 2 desktop adapter for RapidHash.

use fluent_bundle::{FluentBundle, FluentResource};
use rapidhash_core::algorithm::AlgorithmId;
use rapidhash_core::digest::Digest;
use rapidhash_core::job::VerificationStatus;
use rapidhash_core::path_policy::resolve_manifest_path;
use rapidhash_core::traversal::{traverse_paths, TraversalOptions};
use rapidhash_formats::{parse_gnu_manifest, parse_sfv_manifest, ManifestEntry};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
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
                    });
                } else {
                    let mut digests = BTreeMap::new();
                    for (algo, hasher) in hashers {
                        let digest = hasher.finalize();
                        digests.insert(algo.as_str().to_string(), digest.to_canonical_hex());
                    }
                    results.push(FileResultDto {
                        path: file.path.display().to_string(),
                        file_name,
                        size_bytes: Some(file.size_bytes),
                        digests,
                        status: None,
                        error: None,
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
                });
                continue;
            }
        };

        let file_name = resolved_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| entry.path_str.clone());

        if !resolved_path.exists() {
            summary.missing += 1;
            summary.items.push(FileResultDto {
                path: resolved_path.display().to_string(),
                file_name,
                size_bytes: None,
                digests: BTreeMap::new(),
                status: Some(VerificationStatus::Missing.stable_id().to_string()),
                error: None,
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
                });
            }
        }
    }

    Ok(summary)
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_supported_algorithms,
            get_locale_strings,
            calculate_hashes,
            verify_file_checksum,
            verify_manifest_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running rapidhash desktop application");
}
