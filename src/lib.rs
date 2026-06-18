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
        sequence: CommitSequence,
        previous_digest: Option<EntryDigest>,
        digest: EntryDigest,
        payload: PayloadBytes,
    ) -> Self {
        Self {
            sequence,
            previous_digest: PreviousDigest::new(previous_digest),
            digest,
            payload,
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
        store: StoreName,
        expected_head: Option<HeadMark>,
        entries: Vec<EntryEnvelope>,
    ) -> Self {
        Self {
            store,
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
    pub fn new(store: StoreName, reason: AppendRejectionReason, head: Option<HeadMark>) -> Self {
        Self {
            store,
            reason,
            append_rejection_head: AppendRejectionHead::new(head),
        }
    }

    pub fn head(&self) -> Option<&HeadMark> {
        self.append_rejection_head.payload().as_ref()
    }
}

impl ObjectNotice {
    pub fn new(store: StoreName, head: HeadMark, source: Option<MirrorAddress>) -> Self {
        Self {
            store,
            head,
            source: Source::new(source),
        }
    }

    pub fn source(&self) -> Option<&MirrorAddress> {
        self.source.payload().as_ref()
    }
}

impl ObjectNoticeRejection {
    pub fn new(
        store: StoreName,
        reason: ObjectNoticeRejectionReason,
        head: Option<HeadMark>,
    ) -> Self {
        Self {
            store,
            reason,
            object_notice_rejection_head: ObjectNoticeRejectionHead::new(head),
        }
    }

    pub fn head(&self) -> Option<&HeadMark> {
        self.object_notice_rejection_head.payload().as_ref()
    }
}

impl RestoreBundle {
    pub fn from_suffix(
        store: StoreName,
        checkpoint: CheckpointArtifact,
        suffix: Vec<EntryEnvelope>,
    ) -> Self {
        Self {
            store,
            checkpoint,
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
    pub fn new(store: StoreName, head: Option<HeadMark>) -> Self {
        Self {
            store,
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
