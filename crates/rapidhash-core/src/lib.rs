//! Core domain models, hashing engine, and execution models for RapidHash.

pub mod algorithm;
pub mod digest;
pub mod error;
pub mod filename_crc;
pub mod job;
pub mod path_policy;
pub mod traversal;

pub use algorithm::{
    AlgorithmCategory, AlgorithmDescriptor, AlgorithmId, Blake3Hasher, Crc32Hasher, Hasher,
    MultiHasher, Sha256Hasher,
};
pub use digest::Digest;
pub use error::CoreError;
pub use filename_crc::{extract_crc_from_filename, insert_crc_into_filename};
pub use job::{ItemResult, VerificationStatus};
pub use path_policy::{resolve_manifest_path, to_portable_manifest_path};
pub use traversal::{traverse_paths, DiscoveredFile, TraversalOptions};
