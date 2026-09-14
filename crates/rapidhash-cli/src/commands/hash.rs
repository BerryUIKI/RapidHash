//! Implementation of the `hash` subcommand.

use crate::args::HashArgs;
use crate::exit_codes::ExitCode;
use crate::i18n::I18n;
use rapidhash_core::{traverse_paths, AlgorithmId, MultiHasher, TraversalOptions};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::str::FromStr;

#[derive(Serialize)]
struct HashJsonFileRecord {
    path: String,
    size_bytes: u64,
    digests: BTreeMap<String, String>,
}

pub fn run(args: &HashArgs, json: bool, i18n: &I18n) -> ExitCode {
    let mut algos = Vec::new();
    for a_str in &args.algorithms {
        match AlgorithmId::from_str(a_str) {
            Ok(a) => {
                if !algos.contains(&a) {
                    algos.push(a);
                }
            }
            Err(_) => {
                let err_msg = i18n.format(
                    "cli-error-unsupported-algorithm",
                    &[("algorithm", a_str.as_str())],
                );
                eprintln!("{err_msg}");
                return ExitCode::UnsupportedFeature;
            }
        }
    }

    if algos.is_empty() {
        algos.push(AlgorithmId::Sha256);
    }

    let traversal_opts = TraversalOptions {
        max_depth: if args.recursive { Some(64) } else { Some(0) },
        ..Default::default()
    };

    let files = match traverse_paths(&args.paths, &traversal_opts, None) {
        Ok(f) => f,
        Err(err) => {
            let err_str = err.to_string();
            let err_msg = i18n.format(
                err.stable_id(),
                &[("path", "traversal"), ("error", err_str.as_str())],
            );
            eprintln!("{err_msg}");
            return ExitCode::InputUnreadable;
        }
    };

    let mut json_records = Vec::new();
    let mut has_io_error = false;

    for file_entry in files {
        let mut file = match File::open(&file_entry.path) {
            Ok(f) => f,
            Err(err) => {
                has_io_error = true;
                let path_str = file_entry.path.display().to_string();
                let err_str = err.to_string();
                let err_msg = i18n.format(
                    "cli-error-io",
                    &[("path", path_str.as_str()), ("error", err_str.as_str())],
                );
                eprintln!("{err_msg}");
                continue;
            }
        };

        let mut multi = MultiHasher::new(&algos);
        if let Err(err) = multi.update_from_reader(&mut file, 64 * 1024) {
            has_io_error = true;
            let path_str = file_entry.path.display().to_string();
            let err_str = err.to_string();
            let err_msg = i18n.format(
                "cli-error-io",
                &[("path", path_str.as_str()), ("error", err_str.as_str())],
            );
            eprintln!("{err_msg}");
            continue;
        }

        let digests = multi.finalize();

        if json {
            let mut map = BTreeMap::new();
            for (algo, digest) in &digests {
                map.insert(algo.as_str().to_string(), digest.to_hex_lowercase());
            }
            json_records.push(HashJsonFileRecord {
                path: file_entry.path.display().to_string(),
                size_bytes: file_entry.size_bytes,
                digests: map,
            });
        } else {
            let path_str = file_entry.path.display().to_string();
            for (&algo, digest) in &digests {
                if algos.len() == 1 {
                    println!("{}  {}", digest.to_canonical_hex(), path_str);
                } else {
                    println!(
                        "{:<8} {}  {}",
                        algo.as_str(),
                        digest.to_canonical_hex(),
                        path_str
                    );
                }
            }
        }
    }

    if json {
        if let Ok(serialized) = serde_json::to_string_pretty(&json_records) {
            println!("{serialized}");
        }
    }

    if has_io_error {
        ExitCode::InputUnreadable
    } else {
        ExitCode::Success
    }
}
