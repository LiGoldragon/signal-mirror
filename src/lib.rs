//! Current binary Signal contract for payload-blind mirror operations.
pub mod generated;
pub use generated::*;

pub const MIRROR_SIGNAL_SOURCE: &str = include_str!("../ethos/signal.ethos");
pub const MIRROR_SIGNAL_RUST: &str = include_str!("generated/signal.rs");
use std::marker::PhantomData;

pub struct Signal<T> {
    bytes: Vec<u8>,
    target: PhantomData<fn() -> T>,
}
pub trait Signalizable: Sized {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error>;
}
pub trait ByteViewable {
    fn bytes(&self) -> &[u8];
}
pub trait Restorable<T> {
    fn restore(&self) -> Result<T, rkyv::rancor::Error>;
}
impl Signalizable for Query {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}
impl Signalizable for Response {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}
impl<T> From<Vec<u8>> for Signal<T> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            target: PhantomData,
        }
    }
}
impl<T> ByteViewable for Signal<T> {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
impl Restorable<Query> for Signal<Query> {
    fn restore(&self) -> Result<Query, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}
impl Restorable<Response> for Signal<Response> {
    fn restore(&self) -> Result<Response, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}
