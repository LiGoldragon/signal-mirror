//! Round-trip witnesses for the mirror's ordinary wire contract: every
//! operation crosses the length-prefixed rkyv frame and the NOTA text
//! surface without loss. Payload bytes stay opaque bytes on both
//! surfaces — the mirror is payload-blind.

use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};
use signal_mirror::{
    AppendReceipt, AppendRejection, AppendRejectionReason, ArtifactBytes, ArtifactDigest, Bytes,
    CheckpointArtifact, CheckpointReceipt, CheckpointSequence, CommitSequence, EntryDigest,
    EntryEnvelope, EntrySuffix, FixedBytes, Frame, FrameBody, HeadListing, HeadMark, HeadQuery,
    Input, MirrorAddress, ObjectNotice, ObjectNoticeReceipt, ObjectNoticeRejection,
    ObjectNoticeRejectionReason, Output, PayloadBytes, PublishRejection, PublishRejectionReason,
    RestoreBundle, RestoreQuery, RestoreRejection, RestoreRejectionReason, StoreHead, StoreName,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn store(name: &str) -> StoreName {
    StoreName::new(name.to_owned())
}

fn digest(seed: u8) -> EntryDigest {
    EntryDigest::new(FixedBytes::new([seed; 32]))
}

fn head(sequence: u64, seed: u8) -> HeadMark {
    HeadMark {
        commit_sequence: CommitSequence::new(sequence),
        entry_digest: digest(seed),
    }
}

fn envelope(sequence: u64, previous: Option<u8>, seed: u8) -> EntryEnvelope {
    EntryEnvelope::new(
        CommitSequence::new(sequence),
        previous.map(digest),
        digest(seed),
        PayloadBytes::new(Bytes::new(vec![0xde, 0xad, seed])),
    )
}

fn artifact(sequence: u64, covered_end: u64) -> CheckpointArtifact {
    CheckpointArtifact {
        store_name: store("spirit"),
        checkpoint_sequence: CheckpointSequence::new(sequence),
        commit_sequence: CommitSequence::new(covered_end),
        artifact_digest: ArtifactDigest::new(FixedBytes::new([7; 32])),
        artifact_bytes: ArtifactBytes::new(Bytes::new(vec![1, 2, 3, 4])),
    }
}

fn object_notice() -> ObjectNotice {
    ObjectNotice::new(
        store("spirit"),
        head(4, 0x44),
        Some(MirrorAddress::new(
            "router.ouranos.goldragon.criome:7476".to_owned(),
        )),
    )
}

fn request_frame(request: Input) -> Frame {
    Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    })
}

fn reply_frame(reply: Output) -> Frame {
    Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    })
}

fn assert_request_round_trips(request: Input) {
    let frame = request_frame(request.clone());
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => assert_eq!(decoded_request.payloads().head(), &request),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn assert_reply_round_trips(reply: Output) {
    let frame = reply_frame(reply.clone());
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply {
            reply: decoded_reply,
            ..
        } => match decoded_reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => assert_eq!(payload, reply),
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn assert_nota_round_trips<Value>(value: &Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(&recovered, value);
}

#[test]
fn append_request_round_trips_through_length_prefixed_frame() {
    let request = Input::Append(EntrySuffix::from_entries(
        store("spirit"),
        Some(head(2, 0x22)),
        vec![envelope(3, Some(0x22), 0x33), envelope(4, Some(0x33), 0x44)],
    ));
    assert_request_round_trips(request.clone());
    assert_nota_round_trips(&request);
}

#[test]
fn first_append_request_carries_no_expected_head() {
    let request = Input::Append(EntrySuffix::from_entries(
        store("spirit"),
        None,
        vec![envelope(1, None, 0x11)],
    ));
    assert_request_round_trips(request.clone());
    assert_nota_round_trips(&request);
}

#[test]
fn publish_checkpoint_request_round_trips() {
    let request = Input::PublishCheckpoint(artifact(1, 4));
    assert_request_round_trips(request.clone());
    assert_nota_round_trips(&request);
}

#[test]
fn notify_object_request_round_trips() {
    let request = Input::NotifyObject(object_notice());
    assert_request_round_trips(request.clone());
    assert_nota_round_trips(&request);
}

#[test]
fn restore_request_round_trips() {
    let request = Input::Restore(RestoreQuery::new(store("spirit")));
    assert_request_round_trips(request.clone());
    assert_nota_round_trips(&request);
}

#[test]
fn observe_heads_request_round_trips_for_one_store_and_for_all() {
    for query in [
        Input::ObserveHeads(HeadQuery::new(Some(store("spirit")))),
        Input::ObserveHeads(HeadQuery::new(None)),
    ] {
        assert_request_round_trips(query.clone());
        assert_nota_round_trips(&query);
    }
}

#[test]
fn appended_reply_round_trips() {
    let reply = Output::Appended(AppendReceipt {
        store_name: store("spirit"),
        head_mark: head(4, 0x44),
    });
    assert_reply_round_trips(reply.clone());
    assert_nota_round_trips(&reply);
}

#[test]
fn append_rejected_reply_round_trips_with_every_typed_reason() {
    for reason in [
        AppendRejectionReason::UnknownStore,
        AppendRejectionReason::SequenceGap,
        AppendRejectionReason::HeadForked,
        AppendRejectionReason::DigestMismatch,
        AppendRejectionReason::EmptySuffix,
    ] {
        let reply = Output::AppendRejected(AppendRejection::new(
            store("spirit"),
            reason,
            Some(head(2, 0x22)),
        ));
        assert_reply_round_trips(reply.clone());
        assert_nota_round_trips(&reply);
    }
}

#[test]
fn checkpoint_published_reply_round_trips() {
    let reply = Output::CheckpointPublished(CheckpointReceipt {
        store_name: store("spirit"),
        checkpoint_sequence: CheckpointSequence::new(1),
        commit_sequence: CommitSequence::new(4),
    });
    assert_reply_round_trips(reply.clone());
    assert_nota_round_trips(&reply);
}

#[test]
fn publish_rejected_reply_round_trips() {
    for reason in [
        PublishRejectionReason::UnknownStore,
        PublishRejectionReason::CoverageRegressed,
    ] {
        let reply = Output::PublishRejected(PublishRejection {
            store_name: store("spirit"),
            publish_rejection_reason: reason,
        });
        assert_reply_round_trips(reply.clone());
        assert_nota_round_trips(&reply);
    }
}

#[test]
fn object_notice_replies_round_trip() {
    let accepted = Output::ObjectNoticeAccepted(ObjectNoticeReceipt {
        store_name: store("spirit"),
        head_mark: head(4, 0x44),
    });
    assert_reply_round_trips(accepted.clone());
    assert_nota_round_trips(&accepted);

    for reason in [
        ObjectNoticeRejectionReason::UnknownStore,
        ObjectNoticeRejectionReason::SourceUnavailable,
        ObjectNoticeRejectionReason::HeadBehind,
    ] {
        let rejected = Output::ObjectNoticeRejected(ObjectNoticeRejection::new(
            store("spirit"),
            reason,
            Some(head(2, 0x22)),
        ));
        assert_reply_round_trips(rejected.clone());
        assert_nota_round_trips(&rejected);
    }
}

#[test]
fn restored_reply_round_trips_with_checkpoint_and_suffix() {
    let reply = Output::Restored(RestoreBundle::from_suffix(
        store("spirit"),
        artifact(1, 4),
        vec![envelope(5, Some(0x44), 0x55)],
    ));
    assert_reply_round_trips(reply.clone());
    assert_nota_round_trips(&reply);
}

#[test]
fn restore_rejected_reply_round_trips() {
    for reason in [
        RestoreRejectionReason::UnknownStore,
        RestoreRejectionReason::NoCheckpoint,
    ] {
        let reply = Output::RestoreRejected(RestoreRejection {
            store_name: store("spirit"),
            restore_rejection_reason: reason,
        });
        assert_reply_round_trips(reply.clone());
        assert_nota_round_trips(&reply);
    }
}

#[test]
fn heads_observed_reply_round_trips() {
    let reply = Output::HeadsObserved(HeadListing::from_heads(vec![
        StoreHead::new(store("spirit"), Some(head(4, 0x44))),
        StoreHead::new(store("message"), None),
    ]));
    assert_reply_round_trips(reply.clone());
    assert_nota_round_trips(&reply);
}

#[test]
fn payload_bytes_stay_opaque_through_the_frame() {
    let opaque = vec![0x00, 0xff, 0x10, 0x80, 0x7f];
    let request = Input::Append(EntrySuffix::from_entries(
        store("spirit"),
        None,
        vec![EntryEnvelope::new(
            CommitSequence::new(1),
            None,
            digest(0x11),
            PayloadBytes::new(Bytes::new(opaque.clone())),
        )],
    ));
    let frame = request_frame(request);
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => match request.payloads().head() {
            Input::Append(suffix) => {
                assert_eq!(
                    suffix.entries()[0].payload_bytes.as_slice(),
                    opaque.as_slice()
                );
            }
            other => panic!("expected Append, got {other:?}"),
        },
        other => panic!("expected request frame, got {other:?}"),
    }
}

#[test]
fn mirror_faulted_reply_round_trips() {
    let reply = Output::MirrorFaulted(signal_mirror::FaultReport::new(
        signal_mirror::FaultDetail::new("ledger storage: io".to_owned()),
    ));
    assert_reply_round_trips(reply.clone());
    assert_nota_round_trips(&reply);
}
