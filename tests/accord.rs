//! The contract's identity on the exchange layer.
//!
//! The expected digest comes from outside the code under test: FNV-1a over
//! the bytes of `ethos/signal.ethos`, computed independently from the
//! published algorithm rather than through `Handshake::of_source`.

use signal::{Handshake, HandshakeReceipt, HandshakeRejection};
use signal_mirror::{Contracted, Query};

/// FNV-1a over the bytes of `ethos/signal.ethos`, computed outside this crate.
const MIRROR_CONTRACT_DIGEST: i64 = -6996542818645850325;

#[test]
fn the_contract_identifies_itself_by_the_digest_of_its_authored_source() {
    assert_eq!(Query::contract_digest(), MIRROR_CONTRACT_DIGEST);
}

#[test]
fn a_peer_built_from_the_same_source_is_greeted() {
    assert_eq!(
        Query::receipt(&Query::greeting()),
        HandshakeReceipt::Greeted(MIRROR_CONTRACT_DIGEST)
    );
}

#[test]
fn a_peer_built_from_a_different_source_is_refused_with_this_sides_digest() {
    let foreign = Handshake::of_source("Signal\n[]\n[]\n[]\n[]\n");
    assert_eq!(
        Query::receipt(&foreign),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(
            MIRROR_CONTRACT_DIGEST
        ))
    );
}
