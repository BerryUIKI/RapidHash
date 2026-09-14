//! Implementation of the `verify` subcommand.

use crate::args::VerifyArgs;
use crate::exit_codes::ExitCode;
use crate::i18n::I18n;
use rapidhash_core::{resolve_manifest_path, Hasher};
use rapidhash_formats::{parse_gnu_manifest, parse_sfv_manifest, ManifestEntry};
use serde::Serialize;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct VerifyJsonFileRecord {
    path: String,
    status: &'static str,
    expected: String,
    calculated: Option<String>,
}

#[derive(Serialize)]
struct VerifyJsonSummary {
    total: usize,
    matched: usize,
    mismatched: usize,
    failed: usize,
    files: Vec<VerifyJsonFileRecord>,
}

pub fn run(args: &VerifyArgs, json: bool, i18n: &I18n) -> ExitCode {
    let manifest_path = &args.manifest;

    let content = match fs::read_to_string(manifest_path) {
        Ok(c) => c,
        Err(err) => {
            let path_str = manifest_path.display().to_string();
            let err_str = err.to_string();
            let err_msg = i18n.format(
                "cli-error-io",
                &[("path", path_str.as_str()), ("error", err_str.as_str())],
            );
            eprintln!("{err_msg}");
            return ExitCode::InputUnreadable;
        }
    };

    let approved_root: PathBuf = match &args.root {
        Some(r) => r.clone(),
        None => manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf(),
    };

    // Determine format from file extension or content heuristic
    let ext = manifest_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let entries: Vec<ManifestEntry> = if ext == "sfv" {
        match parse_sfv_manifest(&content) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("{err}");
                return ExitCode::ManifestFailure;
            }
        }
    } else {
        match parse_gnu_manifest(&content, None) {
            Ok(e) => e,
            Err(err) => {
                // Try fallback to SFV if GNU fails
                if let Ok(sfv_entries) = parse_sfv_manifest(&content) {
                    sfv_entries
                } else {
                    eprintln!("{err}");
                    return ExitCode::ManifestFailure;
                }
            }
        }
    };

    let mut total = 0usize;
    let mut matched = 0usize;
    let mut mismatched = 0usize;
    let mut failed = 0usize;

    let mut json_records = Vec::new();

    for entry in &entries {
        total += 1;

        let target_path = match resolve_manifest_path(&approved_root, &entry.path_str) {
            Ok(p) => p,
            Err(_) => {
                failed += 1;
                let err_msg = i18n.format(
                    "cli-error-path-traversal",
                    &[("path", entry.path_str.as_str())],
                );
                eprintln!("{err_msg}");
                if json {
                    json_records.push(VerifyJsonFileRecord {
                        path: entry.path_str.clone(),
                        status: "path_escape",
                        expected: entry.digest.to_hex_lowercase(),
                        calculated: None,
                    });
                }
                continue;
            }
        };

        let mut file = match File::open(&target_path) {
            Ok(f) => f,
            Err(err) => {
                failed += 1;
                let err_str = err.to_string();
                let err_msg = if err.kind() == std::io::ErrorKind::NotFound {
                    i18n.format(
                        "cli-error-file-not-found",
                        &[("path", entry.path_str.as_str())],
                    )
                } else {
                    i18n.format(
                        "cli-error-io",
                        &[
                            ("path", entry.path_str.as_str()),
                            ("error", err_str.as_str()),
                        ],
                    )
                };
                eprintln!("{err_msg}");
                if json {
                    json_records.push(VerifyJsonFileRecord {
                        path: entry.path_str.clone(),
                        status: "unreadable",
                        expected: entry.digest.to_hex_lowercase(),
                        calculated: None,
                    });
                }
                continue;
            }
        };

        let mut hasher = entry.digest.algorithm().hasher();
        if let Err(err) = std::io::copy(&mut file, &mut HasherWriteAdapter(&mut *hasher)) {
            failed += 1;
            let err_str = err.to_string();
            let err_msg = i18n.format(
                "cli-error-io",
                &[
                    ("path", entry.path_str.as_str()),
                    ("error", err_str.as_str()),
                ],
            );
            eprintln!("{err_msg}");
            if json {
                json_records.push(VerifyJsonFileRecord {
                    path: entry.path_str.clone(),
                    status: "unreadable",
                    expected: entry.digest.to_hex_lowercase(),
                    calculated: None,
                });
            }
            continue;
        }

        let calculated_digest = hasher.finalize();
        let is_match = calculated_digest.constant_time_eq(&entry.digest);

        if is_match {
            matched += 1;
            if !json {
                println!("{}: OK", entry.path_str);
            }
            if json {
                json_records.push(VerifyJsonFileRecord {
                    path: entry.path_str.clone(),
                    status: "matched",
                    expected: entry.digest.to_hex_lowercase(),
                    calculated: Some(calculated_digest.to_hex_lowercase()),
                });
            }
        } else {
            mismatched += 1;
            if !json {
                eprintln!("{}: FAILED", entry.path_str);
            }
            if json {
                json_records.push(VerifyJsonFileRecord {
                    path: entry.path_str.clone(),
                    status: "mismatched",
                    expected: entry.digest.to_hex_lowercase(),
                    calculated: Some(calculated_digest.to_hex_lowercase()),
                });
            }
        }
    }

    if json {
        let summary = VerifyJsonSummary {
            total,
            matched,
            mismatched,
            failed,
            files: json_records,
        };
        if let Ok(serialized) = serde_json::to_string_pretty(&summary) {
            println!("{serialized}");
        }
    } else {
        let total_str = total.to_string();
        let matched_str = matched.to_string();
        let mismatched_str = mismatched.to_string();
        let failed_str = failed.to_string();

        let summary_msg = i18n.format(
            "cli-summary-verified",
            &[
                ("total", total_str.as_str()),
                ("matched", matched_str.as_str()),
                ("mismatched", mismatched_str.as_str()),
                ("failed", failed_str.as_str()),
            ],
        );
        println!("{summary_msg}");
    }

    if mismatched > 0 {
        ExitCode::VerificationMismatch
    } else if failed > 0 {
        ExitCode::InputUnreadable
    } else {
        ExitCode::Success
    }
}

struct HasherWriteAdapter<'a>(&'a mut dyn Hasher);

impl<'a> std::io::Write for HasherWriteAdapter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
