//! Explicit producer-owned bootstrap authority state for the ordinary Mirror Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    165, 249, 199, 180, 79, 4, 110, 125, 94, 142, 185, 96, 250, 66, 151, 226, 76, 176, 79, 249, 9,
    63, 240, 109, 141, 20, 5, 9, 21, 185, 101, 56,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 37614;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 12405;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 21799, 0xb8bc968db34d16b9);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 22967, 0x3e10076de232f387);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 59936, 0xb39db178c3041f20);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 21571, 0x5b3e00bd2dce562f);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 2057, 0x8e653877c62994d7);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 45832, 0x69708d5aa14b7b6d);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 42146, 0x3813b5028d401ffb);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 36629, 0x5486ee301fa303b0);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 3947, 0xb9ff744662b234dc);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 4652, 0xbf11f8fd7bd68966);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 41402, 0xdb22bcfd88432137);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 40706, 0x445fb9b36d3de4a3);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 11045, 0x9a35c576aa420565);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 24070, 0x05f77975de336359);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 48985, 0x1b97d5885a119cce);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 61810, 0xba110574760598b4);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    24805, 50405, 14955, 44111, 23268, 16077, 37581, 8313, 89, 33725,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "StoreName", 61801, 0xc3d4a2a468afdef9),
    DeclarationSeat::new(None, "CommitSequence", 21520, 0xebc468f35e246e6b),
    DeclarationSeat::new(None, "CheckpointSequence", 28797, 0x8ddba57c8309fa70),
    DeclarationSeat::new(None, "PayloadBytes", 30879, 0x86ad02a599a32e6e),
    DeclarationSeat::new(None, "ArtifactBytes", 30941, 0x630edb2cc7f1d0cd),
    DeclarationSeat::new(None, "HeadMark", 57424, 0x92566ca5c0fb0d5b),
    DeclarationSeat::new(None, "EntryEnvelope", 13931, 0x988b5ffa7627944c),
    DeclarationSeat::new(None, "EntrySuffix", 27132, 0x7b1804c0fca8bd5e),
    DeclarationSeat::new(None, "AppendReceipt", 51113, 0x1966b9f2d2ef4d49),
    DeclarationSeat::new(None, "AppendRejectionReason", 57881, 0x24bcfbdf7ae62b5c),
    DeclarationSeat::new(Some(57881), "UnknownStore", 55477, 0x49745de195fde2dd),
    DeclarationSeat::new(Some(57881), "SequenceGap", 16837, 0xec5bd3f175c4effd),
    DeclarationSeat::new(Some(57881), "HeadForked", 46036, 0x777acaa3dea14264),
    DeclarationSeat::new(Some(57881), "DigestMismatch", 12976, 0x6ca8172d2ee15714),
    DeclarationSeat::new(Some(57881), "EmptySuffix", 18222, 0x796a4540aa8ed8dd),
    DeclarationSeat::new(None, "AppendRejection", 29232, 0xad42296552b1b278),
    DeclarationSeat::new(None, "CheckpointArtifact", 26097, 0xe51ceb29ef477627),
    DeclarationSeat::new(None, "CheckpointReceipt", 4008, 0x54bd7f27cdaad164),
    DeclarationSeat::new(None, "PublishRejectionReason", 57521, 0x116fafccf1c4d079),
    DeclarationSeat::new(Some(57521), "UnknownStore", 35576, 0x4f342f05b2047819),
    DeclarationSeat::new(Some(57521), "CoverageRegressed", 1550, 0x4facbb23ae302cb2),
    DeclarationSeat::new(None, "PublishRejection", 52535, 0xaf896056c2f90790),
    DeclarationSeat::new(None, "ObjectNotice", 46261, 0x4e76c917a0e9ede8),
    DeclarationSeat::new(None, "ObjectNoticeReceipt", 35290, 0x505f4e3ef2cb04ca),
    DeclarationSeat::new(
        None,
        "ObjectNoticeRejectionReason",
        59110,
        0xe6ba324d5fcae3b5,
    ),
    DeclarationSeat::new(Some(59110), "UnknownStore", 45517, 0x3a26fa0a85be103c),
    DeclarationSeat::new(Some(59110), "SourceUnavailable", 40131, 0x81d8828033dbed69),
    DeclarationSeat::new(Some(59110), "HeadBehind", 2673, 0xbf991b9dd3077fc1),
    DeclarationSeat::new(None, "ObjectNoticeRejection", 15797, 0xd654825cda9d757a),
    DeclarationSeat::new(None, "RestoreQuery", 54339, 0x2cd21481bfc39c8c),
    DeclarationSeat::new(None, "RestoreBundle", 42666, 0x42853a23f80cd06c),
    DeclarationSeat::new(None, "RestoreRejectionReason", 13202, 0x5cfc65e6da88a03f),
    DeclarationSeat::new(Some(13202), "UnknownStore", 64747, 0xd0c3f446404c347b),
    DeclarationSeat::new(Some(13202), "NoCheckpoint", 38961, 0x8bcda743e85a5a08),
    DeclarationSeat::new(None, "RestoreRejection", 51857, 0x348ee5381c5dd84f),
    DeclarationSeat::new(None, "FaultDetail", 4852, 0x773ee9bef0803653),
    DeclarationSeat::new(None, "FaultReport", 54748, 0xf26f72b582b03062),
    DeclarationSeat::new(None, "HeadQuery", 60801, 0xa948858812274d75),
    DeclarationSeat::new(None, "StoreHead", 53813, 0x629f3d37a10bf8df),
    DeclarationSeat::new(None, "HeadListing", 41567, 0xf0ffae63bc2482d9),
    DeclarationSeat::new(None, "MirrorRequest", 33738, 0x43e19a0086c2719b),
    DeclarationSeat::new(Some(33738), "Append", 33531, 0x46e98062e9826a68),
    DeclarationSeat::new(Some(33738), "PublishCheckpoint", 10545, 0x94f6032c15ce70ba),
    DeclarationSeat::new(Some(33738), "NotifyObject", 49733, 0x92c120fb99adb15d),
    DeclarationSeat::new(Some(33738), "Restore", 58926, 0xc28133b99aa9ec7c),
    DeclarationSeat::new(Some(33738), "ObserveHeads", 44947, 0x9f20559812912432),
    DeclarationSeat::new(None, "MirrorReply", 27147, 0xc080e9a634d7f177),
    DeclarationSeat::new(Some(27147), "Appended", 39298, 0xb0f0d0443d6d9d9d),
    DeclarationSeat::new(Some(27147), "AppendRejected", 37889, 0xcaa434ddec437cef),
    DeclarationSeat::new(
        Some(27147),
        "CheckpointPublished",
        49375,
        0xb8081850c8c1eca5,
    ),
    DeclarationSeat::new(Some(27147), "PublishRejected", 35398, 0x0f34f41196cf8b5a),
    DeclarationSeat::new(
        Some(27147),
        "ObjectNoticeAccepted",
        18077,
        0x38e4c11a72948ed2,
    ),
    DeclarationSeat::new(
        Some(27147),
        "ObjectNoticeRejected",
        24180,
        0xa301850878cfd7d1,
    ),
    DeclarationSeat::new(Some(27147), "Restored", 34183, 0xc7f2e6903e9d2755),
    DeclarationSeat::new(Some(27147), "RestoreRejected", 1491, 0x0610aea1af6e8710),
    DeclarationSeat::new(Some(27147), "HeadsObserved", 5552, 0x08834e2479262d95),
    DeclarationSeat::new(Some(27147), "MirrorFaulted", 13656, 0x465149e04e903964),
];
