#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PublishRejectionReason {
    UnknownStore,
    CoverageRegressed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AppendReceipt {
    pub store_name: StoreName,
    pub head_mark: HeadMark,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AppendRejectionReason {
    UnknownStore,
    DigestMismatch,
    HeadForked,
    EmptySuffix,
    SequenceGap,
}
#[rustfmt::skip]
pub type RestoreQuery = StoreName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RestoreRejection {
    pub store_name: StoreName,
    pub restore_rejection_reason: RestoreRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RestoreBundle {
    pub store_name: StoreName,
    pub checkpoint_artifact: CheckpointArtifact,
    pub entry_envelope_vector: std::vec::Vec<EntryEnvelope>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ObjectNotice {
    pub store_name: StoreName,
    pub head_mark: HeadMark,
    pub standard_socket_option: Option<signal::StandardSocket>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ObjectNoticeReceipt {
    pub store_name: StoreName,
    pub head_mark: HeadMark,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CheckpointReceipt {
    pub store_name: StoreName,
    pub checkpoint_sequence: CheckpointSequence,
    pub commit_sequence: CommitSequence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RestoreRejectionReason {
    NoCheckpoint,
    UnknownStore,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StoreHead {
    pub store_name: StoreName,
    pub head_mark_option: Option<HeadMark>,
}
#[rustfmt::skip]
pub type ArtifactBytes = std::vec::Vec<i64>;
#[rustfmt::skip]
pub type FaultDetail = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EntrySuffix {
    pub store_name: StoreName,
    pub head_mark_option: Option<HeadMark>,
    pub entry_envelope_vector: std::vec::Vec<EntryEnvelope>,
}
#[rustfmt::skip]
pub type PayloadBytes = std::vec::Vec<i64>;
#[rustfmt::skip]
pub type CheckpointSequence = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HeadMark {
    pub commit_sequence: CommitSequence,
    pub object_digest: signal::ObjectDigest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EntryEnvelope {
    pub commit_sequence: CommitSequence,
    pub object_digest_option: Option<signal::ObjectDigest>,
    pub object_digest: signal::ObjectDigest,
    pub payload_bytes: PayloadBytes,
}
#[rustfmt::skip]
pub type HeadQuery = std::option::Option<StoreName>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AppendRejection {
    pub store_name: StoreName,
    pub append_rejection_reason: AppendRejectionReason,
    pub head_mark_option: Option<HeadMark>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PublishRejection {
    pub store_name: StoreName,
    pub publish_rejection_reason: PublishRejectionReason,
}
#[rustfmt::skip]
pub type StoreName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ObjectNoticeRejection {
    pub store_name: StoreName,
    pub object_notice_rejection_reason: ObjectNoticeRejectionReason,
    pub head_mark_option: Option<HeadMark>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CheckpointArtifact {
    pub store_name: StoreName,
    pub checkpoint_sequence: CheckpointSequence,
    pub commit_sequence: CommitSequence,
    pub object_digest: signal::ObjectDigest,
    pub artifact_bytes: ArtifactBytes,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ObjectNoticeRejectionReason {
    UnknownStore,
    SourceUnavailable,
    HeadBehind,
}
#[rustfmt::skip]
pub type CommitSequence = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HeadListing {
    pub store_head_vector: std::vec::Vec<StoreHead>,
}
#[rustfmt::skip]
pub type FaultReport = FaultDetail;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Append(EntrySuffix),
    NotifyObject(ObjectNotice),
    PublishCheckpoint(CheckpointArtifact),
    ObserveHeads(HeadQuery),
    Restore(RestoreQuery),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    RestoreRejected(RestoreRejection),
    HeadsObserved(HeadListing),
    PublishRejected(PublishRejection),
    ObjectNoticeAccepted(ObjectNoticeReceipt),
    MirrorFaulted(FaultReport),
    ObjectNoticeRejected(ObjectNoticeRejection),
    Appended(AppendReceipt),
    CheckpointPublished(CheckpointReceipt),
    Restored(RestoreBundle),
    AppendRejected(AppendRejection),
}
