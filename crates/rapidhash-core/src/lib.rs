//! Core domain models, hashing engine, and execution models for RapidHash.

pub mod algorithm;
pub mod digest;
pub mod error;
pub mod job;

pub use algorithm::{
    AlgorithmCategory, AlgorithmDescriptor, AlgorithmId, Blake3Hasher, Crc32Hasher, Hasher,
    MultiHasher, Sha256Hasher,
};
pub use digest::Digest;
pub use error::CoreError;
pub use job::{ItemResult, VerificationStatus};
