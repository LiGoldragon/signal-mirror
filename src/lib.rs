//! Current binary Signal contract for payload-blind mirror operations.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against a fresh
//! generation. `examples/canonical.datom` is the authored wire-text witness,
//! actualized line by line by `tests/canonical.rs`.
//!
//! The portable rkyv frame and its three kinds come from `signal` and are
//! re-exported here, so a mirror frame is the same type as every other
//! contract's frame.
pub mod generated;
pub use generated::*;

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// The authored Ethos source of this contract.
pub const MIRROR_SIGNAL_SOURCE: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`MIRROR_SIGNAL_SOURCE`].
pub const MIRROR_SIGNAL_RUST: &str = include_str!("generated/signal.rs");
