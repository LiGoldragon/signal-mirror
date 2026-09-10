//! Current binary Signal contract for payload-blind mirror operations.
pub mod generated;
pub use generated::*;

pub const MIRROR_SIGNAL_SOURCE: &str = include_str!("../ethos/signal.ethos");
pub const MIRROR_SIGNAL_RUST: &str = include_str!("generated/signal.rs");
