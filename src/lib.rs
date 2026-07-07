//! Schema-derived Signal contract for the payload-blind sema
//! version-control mirror.
//!
//! The wire vocabulary is generated from `schema/lib.schema`; this file
//! re-exports the generated nouns and attaches small accessors.

#[allow(dead_code, private_interfaces)]
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

impl EntryEnvelope {
    pub fn new(
        commit_sequence: CommitSequence,
        previous_digest: Option<EntryDigest>,
        entry_digest: EntryDigest,
        payload_bytes: PayloadBytes,
    ) -> Self {
        Self {
            commit_sequence,
            previous_digest: PreviousDigest::new(previous_digest),
            entry_digest,
            payload_bytes,
        }
    }

    pub fn previous_digest(&self) -> Option<&EntryDigest> {
        self.previous_digest.payload().as_ref()
    }

    pub fn into_previous_digest(self) -> Option<EntryDigest> {
        self.previous_digest.into_payload()
    }
}

impl EntrySuffix {
    pub fn from_entries(
        store_name: StoreName,
        expected_head: Option<HeadMark>,
        entries: Vec<EntryEnvelope>,
    ) -> Self {
        Self {
            store_name,
            expected_head: ExpectedHead::new(expected_head),
            entries: Entries::new(entries),
        }
    }

    pub fn expected_head(&self) -> Option<&HeadMark> {
        self.expected_head.payload().as_ref()
    }

    pub fn entries(&self) -> &[EntryEnvelope] {
        self.entries.payload()
    }

    pub fn into_entries(self) -> Vec<EntryEnvelope> {
        self.entries.into_payload()
    }
}

impl AppendRejection {
    pub fn new(
        store_name: StoreName,
        append_rejection_reason: AppendRejectionReason,
        head: Option<HeadMark>,
    ) -> Self {
        Self {
            store_name,
            append_rejection_reason,
            append_rejection_head: AppendRejectionHead::new(head),
        }
    }

    pub fn head(&self) -> Option<&HeadMark> {
        self.append_rejection_head.payload().as_ref()
    }
}

impl ObjectNotice {
    pub fn new(store_name: StoreName, head_mark: HeadMark, source: Option<MirrorAddress>) -> Self {
        Self {
            store_name,
            head_mark,
            source: Source::new(source),
        }
    }

    pub fn source(&self) -> Option<&MirrorAddress> {
        self.source.payload().as_ref()
    }
}

impl ObjectNoticeRejection {
    pub fn new(
        store_name: StoreName,
        object_notice_rejection_reason: ObjectNoticeRejectionReason,
        head: Option<HeadMark>,
    ) -> Self {
        Self {
            store_name,
            object_notice_rejection_reason,
            object_notice_rejection_head: ObjectNoticeRejectionHead::new(head),
        }
    }

    pub fn head(&self) -> Option<&HeadMark> {
        self.object_notice_rejection_head.payload().as_ref()
    }
}

impl RestoreBundle {
    pub fn from_suffix(
        store_name: StoreName,
        checkpoint_artifact: CheckpointArtifact,
        suffix: Vec<EntryEnvelope>,
    ) -> Self {
        Self {
            store_name,
            checkpoint_artifact,
            suffix: Suffix::new(suffix),
        }
    }

    pub fn suffix(&self) -> &[EntryEnvelope] {
        self.suffix.payload()
    }

    pub fn into_suffix(self) -> Vec<EntryEnvelope> {
        self.suffix.into_payload()
    }
}

impl StoreHead {
    pub fn new(store_name: StoreName, head: Option<HeadMark>) -> Self {
        Self {
            store_name,
            store_head_mark: StoreHeadMark::new(head),
        }
    }

    pub fn head(&self) -> Option<&HeadMark> {
        self.store_head_mark.payload().as_ref()
    }
}

impl HeadListing {
    pub fn from_heads(heads: Vec<StoreHead>) -> Self {
        Self::new(Heads::new(heads))
    }

    pub fn heads(&self) -> &[StoreHead] {
        self.payload().payload()
    }

    pub fn into_heads(self) -> Vec<StoreHead> {
        self.into_payload().into_payload()
    }
}
