//! Implementation of the `algorithms` subcommand.

use crate::args::AlgorithmsArgs;
use crate::exit_codes::ExitCode;
use crate::i18n::I18n;
use rapidhash_core::AlgorithmId;
use serde::Serialize;

#[derive(Serialize)]
struct AlgorithmJsonRecord {
    id: &'static str,
    digest_length_bytes: usize,
    category: &'static str,
    is_recommended: bool,
}

pub fn run(_args: &AlgorithmsArgs, json: bool, _i18n: &I18n) -> ExitCode {
    if json {
        let records: Vec<AlgorithmJsonRecord> = AlgorithmId::ALL
            .iter()
            .map(|&algo| {
                let desc = algo.descriptor();
                AlgorithmJsonRecord {
                    id: desc.id_str,
                    digest_length_bytes: desc.digest_length_bytes,
                    category: match desc.category {
                        rapidhash_core::AlgorithmCategory::ModernCryptographic => {
                            "modern_cryptographic"
                        }
                        rapidhash_core::AlgorithmCategory::LegacyCryptographic => {
                            "legacy_cryptographic"
                        }
                        rapidhash_core::AlgorithmCategory::NonCryptographic => "non_cryptographic",
                    },
                    is_recommended: desc.is_recommended,
                }
            })
            .collect();

        if let Ok(serialized) = serde_json::to_string_pretty(&records) {
            println!("{serialized}");
        }
        return ExitCode::Success;
    }

    println!(
        "{:<12} {:<8} {:<24} Recommended",
        "Algorithm", "Bytes", "Category"
    );
    println!("{:-<12} {:-<8} {:-<24} {:-<11}", "", "", "", "");
    for &algo in AlgorithmId::ALL {
        let desc = algo.descriptor();
        let cat_str = match desc.category {
            rapidhash_core::AlgorithmCategory::ModernCryptographic => "Modern Cryptographic",
            rapidhash_core::AlgorithmCategory::LegacyCryptographic => "Legacy Cryptographic",
            rapidhash_core::AlgorithmCategory::NonCryptographic => "Non-Cryptographic",
        };
        let rec_str = if desc.is_recommended { "Yes" } else { "No" };
        println!(
            "{:<12} {:<8} {:<24} {}",
            desc.id_str, desc.digest_length_bytes, cat_str, rec_str
        );
    }

    ExitCode::Success
}
