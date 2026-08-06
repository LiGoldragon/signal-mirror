//! Ordinary payload-blind Mirror Interface.
//!
//! `ethos/interface.ethos` is the canonical projection of one
//! authority-verified, role-free Interface. Content and endpoint identities
//! resolve through `signal-standard`; Rust carries encoded coordinates only.

pub mod bootstrap_manifest;
pub mod schema;

pub const MIRROR_INTERFACE_SOURCE: &str = include_str!("../ethos/interface.ethos");
pub const MIRROR_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");

pub use schema::lib::*;
