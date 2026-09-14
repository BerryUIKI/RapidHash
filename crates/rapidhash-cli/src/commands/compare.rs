//! Implementation of the `compare` subcommand.

use crate::args::CompareArgs;
use crate::exit_codes::ExitCode;
use crate::i18n::I18n;
use rapidhash_core::{AlgorithmId, Digest, Hasher};
use serde::Serialize;
use std::fs::File;
use std::str::FromStr;

#[derive(Serialize)]
struct CompareJsonRecord {
    path: String,
    expected: String,
    calculated: Option<String>,
    matched: bool,
}

pub fn run(args: &CompareArgs, json: bool, i18n: &I18n) -> ExitCode {
    let algo = if let Some(ref a_str) = args.algorithm {
        match AlgorithmId::from_str(a_str) {
            Ok(a) => a,
            Err(_) => {
                let err_msg = i18n.format(
                    "cli-error-unsupported-algorithm",
                    &[("algorithm", a_str.as_str())],
                );
                eprintln!("{err_msg}");
                return ExitCode::UnsupportedFeature;
            }
        }
    } else {
        // Deduce from length of expected digest
        match args.digest.trim().len() {
            64 => AlgorithmId::Sha256,
            8 => AlgorithmId::Crc32,
            _ => {
                let err_msg = i18n.format(
                    "cli-error-invalid-digest",
                    &[("digest", args.digest.as_str())],
                );
                eprintln!("{err_msg}");
                return ExitCode::InvalidUsage;
            }
        }
    };

    let expected_digest = match Digest::from_hex(algo, &args.digest) {
        Ok(d) => d,
        Err(_) => {
            let err_msg = i18n.format(
                "cli-error-invalid-digest",
                &[("digest", args.digest.as_str())],
            );
            eprintln!("{err_msg}");
            return ExitCode::InvalidUsage;
        }
    };

    let mut file = match File::open(&args.path) {
        Ok(f) => f,
        Err(err) => {
            let path_str = args.path.display().to_string();
            let err_str = err.to_string();
            let err_msg = if err.kind() == std::io::ErrorKind::NotFound {
                i18n.format("cli-error-file-not-found", &[("path", path_str.as_str())])
            } else if err.kind() == std::io::ErrorKind::PermissionDenied {
                i18n.format(
                    "cli-error-permission-denied",
                    &[("path", path_str.as_str())],
                )
            } else {
                i18n.format(
                    "cli-error-io",
                    &[("path", path_str.as_str()), ("error", err_str.as_str())],
                )
            };
            eprintln!("{err_msg}");
            return ExitCode::InputUnreadable;
        }
    };

    let mut hasher = algo.hasher();
    if let Err(err) = std::io::copy(&mut file, &mut HasherWriteAdapter(&mut *hasher)) {
        let path_str = args.path.display().to_string();
        let err_str = err.to_string();
        let err_msg = i18n.format(
            "cli-error-io",
            &[("path", path_str.as_str()), ("error", err_str.as_str())],
        );
        eprintln!("{err_msg}");
        return ExitCode::InputUnreadable;
    }

    let calculated_digest = hasher.finalize();
    let is_match = calculated_digest.constant_time_eq(&expected_digest);

    if json {
        let record = CompareJsonRecord {
            path: args.path.display().to_string(),
            expected: expected_digest.to_hex_lowercase(),
            calculated: Some(calculated_digest.to_hex_lowercase()),
            matched: is_match,
        };
        if let Ok(serialized) = serde_json::to_string_pretty(&record) {
            println!("{serialized}");
        }
    } else {
        let path_display = args.path.display().to_string();
        if is_match {
            let msg = i18n.format("cli-compare-matched", &[("path", path_display.as_str())]);
            println!("{msg}");
        } else {
            let msg = i18n.format("cli-compare-mismatched", &[("path", path_display.as_str())]);
            eprintln!("{msg}");
        }
    }

    if is_match {
        ExitCode::Success
    } else {
        ExitCode::VerificationMismatch
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
