//! Command-line argument definitions using Clap.

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// RapidHash: Fast, cross-platform checksum verification and generation.
#[derive(Parser, Debug)]
#[command(
    name = "rapidhash",
    version,
    about = "Fast, cross-platform checksum verification and generation",
    long_about = None
)]
pub struct Cli {
    /// Emit results as structured JSON without localized values.
    #[arg(long, global = true)]
    pub json: bool,

    /// Suppress informational messages and progress.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Enable detailed diagnostic output.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Override interface language (e.g. "en", "zh-CN").
    #[arg(long, global = true)]
    pub locale: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Calculate checksums for files or directories.
    Hash(HashArgs),

    /// Verify files against an existing checksum manifest.
    Verify(VerifyArgs),

    /// Compare a single file against an expected digest.
    Compare(CompareArgs),

    /// List available checksum algorithms and their status.
    Algorithms(AlgorithmsArgs),
}

#[derive(Args, Debug)]
pub struct HashArgs {
    /// Target file or directory paths.
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    /// Checksum algorithm(s) to use (default: sha256).
    #[arg(short, long = "algorithm", default_value = "sha256")]
    pub algorithms: Vec<String>,

    /// Traverse directories recursively.
    #[arg(short, long)]
    pub recursive: bool,
}

#[derive(Args, Debug)]
pub struct VerifyArgs {
    /// Path to checksum manifest file (.sha256, .sfv, etc.).
    pub manifest: PathBuf,

    /// Approved root directory for manifest paths (defaults to manifest parent directory).
    #[arg(long)]
    pub root: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct CompareArgs {
    /// Target file path.
    pub path: PathBuf,

    /// Expected hexadecimal digest.
    pub digest: String,

    /// Explicit algorithm (deduced from length if omitted).
    #[arg(short, long)]
    pub algorithm: Option<String>,
}

#[derive(Args, Debug)]
pub struct AlgorithmsArgs {}
