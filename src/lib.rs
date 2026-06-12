//! Schema-derived Signal contract for the payload-blind sema
//! version-control mirror.
//!
//! The wire vocabulary is generated from `schema/lib.schema`; this file
//! re-exports the generated nouns and attaches small accessors.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl StoreName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl CommitSequence {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl CheckpointSequence {
    pub fn into_u64(self) -> u64 {
        self.into_payload()
    }
}

impl EntryDigest {
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.payload().payload()
    }
}

impl ArtifactDigest {
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.payload().payload()
    }
}

impl PayloadBytes {
    pub fn as_slice(&self) -> &[u8] {
        self.payload().payload()
    }
}

impl ArtifactBytes {
    pub fn as_slice(&self) -> &[u8] {
        self.payload().payload()
    }
}
