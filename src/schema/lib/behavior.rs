// Handwritten operational behavior for the authority-verified ordinary Mirror Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, readable Dotos roles, and the allocated
// Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};
use signal_standard::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 {
        return Err(WireShapeError);
    }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self {
                Self(payload)
            }
            pub fn payload(&self) -> &$inner {
                &self.0
            }
            pub fn into_payload(self) -> $inner {
                self.0
            }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                self.0.to_wire()
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                Ok(Self(<$inner as WireShape>::from_wire(value)?))
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_enum!(z2Vcs2 { unit { 0 => z2VWLf : "UnknownStore", 1 => z2VLE1 : "CoverageRegressed" } unary {  } });
wire_struct!(z2VaxY { field_0: z2Ve8p, field_1: z2VcqM });
wire_enum!(z2VcyE { unit { 0 => z2VcFn : "UnknownStore", 1 => z2VPd1 : "DigestMismatch", 2 => z2VZT1 : "HeadForked", 3 => z2VRBT : "EmptySuffix", 4 => z2VQma : "SequenceGap" } unary {  } });
wire_newtype!(z2VbvA, z2Ve8p);
wire_struct!(z2VbBN { field_0: z2Ve8p, field_1: z2VPgu });
wire_struct!(z2VYSu { field_0: z2Ve8p, field_1: z2VTXE, field_2: Vec< z2VPuU> });
wire_enum!(z2VVny { unit {  } unary { 0 => z2VVjQ(z2VTq5) : "Append", 1 => z2VaYk(z2VZWt) : "NotifyObject", 2 => z2VNu6(z2VTXE) : "PublishCheckpoint", 3 => z2VZ8E(z2Vdqa) : "ObserveHeads", 4 => z2VdHF(z2VbvA) : "Restore" } });
wire_struct!(z2VZWt { field_0: z2Ve8p, field_1: z2VcqM, field_2: Option< signal_standard::schema::lib::z2VduW> });
wire_struct!(z2VWFj { field_0: z2Ve8p, field_1: z2VcqM });
wire_struct!(z2VLxP { field_0: z2Ve8p, field_1: z2VUKn, field_2: z2VSAK });
wire_enum!(z2VPgu { unit { 0 => z2VXM2 : "NoCheckpoint", 1 => z2Vf1c : "UnknownStore" } unary {  } });
wire_struct!(z2Vbm6 { field_0: z2Ve8p, field_1: Option< z2VcqM> });
wire_external_newtype!(z2VUxk, Vec< u64>);
wire_external_newtype!(z2VMCw, std::string::String);
wire_struct!(z2VTq5 { field_0: z2Ve8p, field_1: Option< z2VcqM>, field_2: Vec< z2VPuU> });
wire_external_newtype!(z2VUwg, Vec< u64>);
wire_external_newtype!(z2VUKn, u64);
wire_struct!(z2VcqM { field_0: z2VSAK, field_1: signal_standard::schema::lib::z2VSyM });
wire_struct!(z2VPuU { field_0: z2VSAK, field_1: Option< signal_standard::schema::lib::z2VSyM>, field_2: signal_standard::schema::lib::z2VSyM, field_3: z2VUwg });
wire_external_newtype!(z2Vdqa, Option< z2Ve8p>);
wire_struct!(z2VUTH { field_0: z2Ve8p, field_1: z2VcyE, field_2: Option< z2VcqM> });
wire_struct!(z2VbP4 { field_0: z2Ve8p, field_1: z2Vcs2 });
wire_enum!(z2VTqL { unit {  } unary { 0 => z2VLCz(z2VbBN) : "RestoreRejected", 1 => z2VMR1(z2VY7x) : "HeadsObserved", 2 => z2VWHb(z2VbP4) : "PublishRejected", 3 => z2VR8x(z2VWFj) : "ObjectNoticeAccepted", 4 => z2VPpj(z2Vc3D) : "MirrorFaulted", 5 => z2VSxB(z2VQTe) : "ObjectNoticeRejected", 6 => z2VXSq(z2VaxY) : "Appended", 7 => z2VaSa(z2VLxP) : "CheckpointPublished", 8 => z2VVve(z2VYSu) : "Restored", 9 => z2VX2Y(z2VUTH) : "AppendRejected" } });
wire_external_newtype!(z2Ve8p, std::string::String);
wire_struct!(z2VQTe { field_0: z2Ve8p, field_1: z2VdLR, field_2: Option< z2VcqM> });
wire_struct!(z2VTXE { field_0: z2Ve8p, field_1: z2VUKn, field_2: z2VSAK, field_3: signal_standard::schema::lib::z2VSyM, field_4: z2VUxk });
wire_enum!(z2VdLR { unit { 0 => z2VZJ4 : "UnknownStore", 1 => z2VXhC : "SourceUnavailable", 2 => z2VLZN : "HeadBehind" } unary {  } });
wire_external_newtype!(z2VSAK, u64);
wire_struct!(z2VY7x { field_0: Vec< z2Vbm6> });
wire_newtype!(z2Vc3D, z2VMCw);

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("value {value} at offset {offset} is outside the octet range")]
pub struct OctetRangeError {
    pub offset: usize,
    pub value: u64,
}

macro_rules! octet_vector {
    ($name:ident) => {
        impl $name {
            pub fn from_octets(octets: &[u8]) -> Self {
                Self::new(octets.iter().map(|value| u64::from(*value)).collect())
            }

            pub fn octets(&self) -> Result<Vec<u8>, OctetRangeError> {
                self.payload()
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(offset, value)| {
                        u8::try_from(value).map_err(|_| OctetRangeError { offset, value })
                    })
                    .collect()
            }
        }
    };
}

octet_vector!(z2VUwg);
octet_vector!(z2VUxk);

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer>
            for signal_standard::schema::lib::ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            signal_standard::schema::lib::ArchivedWireValue:
                RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <signal_standard::schema::lib::ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2Vcs2);
archive_root!(z2VaxY);
archive_root!(z2VcyE);
archive_root!(z2VbvA);
archive_root!(z2VbBN);
archive_root!(z2VYSu);
archive_root!(z2VVny);
archive_root!(z2VZWt);
archive_root!(z2VWFj);
archive_root!(z2VLxP);
archive_root!(z2VPgu);
archive_root!(z2Vbm6);
archive_root!(z2VUxk);
archive_root!(z2VMCw);
archive_root!(z2VTq5);
archive_root!(z2VUwg);
archive_root!(z2VUKn);
archive_root!(z2VcqM);
archive_root!(z2VPuU);
archive_root!(z2Vdqa);
archive_root!(z2VUTH);
archive_root!(z2VbP4);
archive_root!(z2VTqL);
archive_root!(z2Ve8p);
archive_root!(z2VQTe);
archive_root!(z2VTXE);
archive_root!(z2VdLR);
archive_root!(z2VSAK);
archive_root!(z2VY7x);
archive_root!(z2Vc3D);


pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(9) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason {
    Rejected,
    Unavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal {
    pub reason: EngineRefusalReason,
    pub detail: std::string::String,
}

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Rejected, detail }
    }

    pub fn unavailable(detail: std::string::String) -> Self {
        Self { reason: EngineRefusalReason::Unavailable, detail }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")]
    FrameEncode,
    #[error("failed to decode bound signal frame")]
    ArchiveDecode,
    #[error("unexpected signal frame body")]
    UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")]
    OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute {
    Append,
    NotifyObject,
    PublishCheckpoint,
    ObserveHeads,
    Restore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute {
    RestoreRejected,
    HeadsObserved,
    PublishRejected,
    ObjectNoticeAccepted,
    MirrorFaulted,
    ObjectNoticeRejected,
    Appended,
    CheckpointPublished,
    Restored,
    AppendRejected,
}

impl z2VVny {
    pub fn route(&self) -> InputRoute {
        match self {
            Self::z2VVjQ(_) => InputRoute::Append,
            Self::z2VaYk(_) => InputRoute::NotifyObject,
            Self::z2VNu6(_) => InputRoute::PublishCheckpoint,
            Self::z2VZ8E(_) => InputRoute::ObserveHeads,
            Self::z2VdHF(_) => InputRoute::Restore,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(0),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(
            route,
            FrameBody::Request {
                exchange,
                request: signal_frame::Request::from_payload(self),
            },
        )
    }

    pub fn encode_request_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl z2VTqL {
    pub fn route(&self) -> OutputRoute {
        match self {
            Self::z2VLCz(_) => OutputRoute::RestoreRejected,
            Self::z2VMR1(_) => OutputRoute::HeadsObserved,
            Self::z2VWHb(_) => OutputRoute::PublishRejected,
            Self::z2VR8x(_) => OutputRoute::ObjectNoticeAccepted,
            Self::z2VPpj(_) => OutputRoute::MirrorFaulted,
            Self::z2VSxB(_) => OutputRoute::ObjectNoticeRejected,
            Self::z2VXSq(_) => OutputRoute::Appended,
            Self::z2VaSa(_) => OutputRoute::CheckpointPublished,
            Self::z2VVve(_) => OutputRoute::Restored,
            Self::z2VX2Y(_) => OutputRoute::AppendRejected,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(1),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(
            signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)),
        );
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }

    pub fn encode_reply_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for z2VVny {}

impl signal_frame::SignalOperationHeads for z2VVny {
    const HEADS: &'static [&'static str] = &["Append", "NotifyObject", "PublishCheckpoint", "ObserveHeads", "Restore"];
}

impl signal_frame::LogVariant for z2VVny {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VVny, z2VTqL>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VVny, z2VTqL>;
pub type Request = signal_frame::Request<z2VVny>;
pub type ReplyEnvelope = signal_frame::Reply<z2VTqL>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VVny>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }

    pub fn decode_single_request(
        bytes: &[u8],
    ) -> Result<(signal_frame::ExchangeIdentifier, z2VVny), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 {
                    return Err(SignalFrameError::OperationCount { found });
                }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}
