// GENERATED native Rust types + codec — do not edit.
#![allow(dead_code)]
use crate::cbor::{Cbor, DecodeError};

// The file's bounds, for a decode rooted at a type that is not a message:
// `cbor::try_decode_with(bytes, MAX_DEPTH, MAX_ENCODED_LEN)`.
pub const MAX_DEPTH: usize = 16;
pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MediaKind {
    #[default] Video,
    Audio,
    Text,
}
impl MediaKind {
    pub fn wire(self) -> i64 { match self {
        Self::Video => 1,
        Self::Audio => 2,
        Self::Text => 3,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Video,
        2 => Self::Audio,
        3 => Self::Text,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "MediaKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NodeKind {
    #[default] Source,
    Effect,
    Composite,
    Transition,
    Title,
    Output,
    Silence,
}
impl NodeKind {
    pub fn wire(self) -> i64 { match self {
        Self::Source => 1,
        Self::Effect => 2,
        Self::Composite => 3,
        Self::Transition => 4,
        Self::Title => 5,
        Self::Output => 6,
        Self::Silence => 7,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Source,
        2 => Self::Effect,
        3 => Self::Composite,
        4 => Self::Transition,
        5 => Self::Title,
        6 => Self::Output,
        7 => Self::Silence,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "NodeKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ParamKind {
    #[default] Integer,
    Rational,
    Boolean,
    Text,
}
impl ParamKind {
    pub fn wire(self) -> i64 { match self {
        Self::Integer => 1,
        Self::Rational => 2,
        Self::Boolean => 3,
        Self::Text => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Integer,
        2 => Self::Rational,
        3 => Self::Boolean,
        4 => Self::Text,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ParamKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum EditKind {
    #[default] PutNode,
    RemoveNode,
    PutEdge,
    RemoveEdge,
    PutTrack,
    RemoveTrack,
    PutSequence,
    RemoveSequence,
    PutSlot,
    RemoveSlot,
}
impl EditKind {
    pub fn wire(self) -> i64 { match self {
        Self::PutNode => 1,
        Self::RemoveNode => 2,
        Self::PutEdge => 3,
        Self::RemoveEdge => 4,
        Self::PutTrack => 5,
        Self::RemoveTrack => 6,
        Self::PutSequence => 7,
        Self::RemoveSequence => 8,
        Self::PutSlot => 9,
        Self::RemoveSlot => 10,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::PutNode,
        2 => Self::RemoveNode,
        3 => Self::PutEdge,
        4 => Self::RemoveEdge,
        5 => Self::PutTrack,
        6 => Self::RemoveTrack,
        7 => Self::PutSequence,
        8 => Self::RemoveSequence,
        9 => Self::PutSlot,
        10 => Self::RemoveSlot,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "EditKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AckStatus {
    #[default] Accepted,
    Stale,
    Invalid,
    Unauthorized,
}
impl AckStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Accepted => 1,
        Self::Stale => 2,
        Self::Invalid => 3,
        Self::Unauthorized => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Accepted,
        2 => Self::Stale,
        3 => Self::Invalid,
        4 => Self::Unauthorized,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AckStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum GraphEventKind {
    #[default] Ready,
    Change,
    Heartbeat,
    RecoveryRequired,
}
impl GraphEventKind {
    pub fn wire(self) -> i64 { match self {
        Self::Ready => 1,
        Self::Change => 2,
        Self::Heartbeat => 3,
        Self::RecoveryRequired => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Ready,
        2 => Self::Change,
        3 => Self::Heartbeat,
        4 => Self::RecoveryRequired,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "GraphEventKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PlacementMode {
    #[default] RippleTrack,
    OverwriteTrack,
}
impl PlacementMode {
    pub fn wire(self) -> i64 { match self {
        Self::RippleTrack => 1,
        Self::OverwriteTrack => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::RippleTrack,
        2 => Self::OverwriteTrack,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "PlacementMode", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum EditorStatus {
    #[default] Accepted,
    StaleGraph,
    StaleBinding,
    InvalidRange,
    LockedTrack,
    UnsupportedRippleScope,
    Unauthorized,
    InvalidCommand,
    UnsupportedSchema,
    RecoveryRequired,
}
impl EditorStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Accepted => 1,
        Self::StaleGraph => 2,
        Self::StaleBinding => 3,
        Self::InvalidRange => 4,
        Self::LockedTrack => 5,
        Self::UnsupportedRippleScope => 6,
        Self::Unauthorized => 7,
        Self::InvalidCommand => 8,
        Self::UnsupportedSchema => 9,
        Self::RecoveryRequired => 10,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Accepted,
        2 => Self::StaleGraph,
        3 => Self::StaleBinding,
        4 => Self::InvalidRange,
        5 => Self::LockedTrack,
        6 => Self::UnsupportedRippleScope,
        7 => Self::Unauthorized,
        8 => Self::InvalidCommand,
        9 => Self::UnsupportedSchema,
        10 => Self::RecoveryRequired,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "EditorStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BindingStatus {
    #[default] Accepted,
    StaleGraph,
    StaleBinding,
    IncompatibleMedia,
    InvalidPolicy,
    Unauthorized,
    InvalidCommand,
}
impl BindingStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Accepted => 1,
        Self::StaleGraph => 2,
        Self::StaleBinding => 3,
        Self::IncompatibleMedia => 4,
        Self::InvalidPolicy => 5,
        Self::Unauthorized => 6,
        Self::InvalidCommand => 7,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Accepted,
        2 => Self::StaleGraph,
        3 => Self::StaleBinding,
        4 => Self::IncompatibleMedia,
        5 => Self::InvalidPolicy,
        6 => Self::Unauthorized,
        7 => Self::InvalidCommand,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "BindingStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BindingEventKind {
    #[default] Ready,
    Change,
    Heartbeat,
    RecoveryRequired,
}
impl BindingEventKind {
    pub fn wire(self) -> i64 { match self {
        Self::Ready => 1,
        Self::Change => 2,
        Self::Heartbeat => 3,
        Self::RecoveryRequired => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Ready,
        2 => Self::Change,
        3 => Self::Heartbeat,
        4 => Self::RecoveryRequired,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "BindingEventKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum CatalogEventKind {
    #[default] Ready,
    Change,
    Heartbeat,
    RecoveryRequired,
}
impl CatalogEventKind {
    pub fn wire(self) -> i64 { match self {
        Self::Ready => 1,
        Self::Change => 2,
        Self::Heartbeat => 3,
        Self::RecoveryRequired => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Ready,
        2 => Self::Change,
        3 => Self::Heartbeat,
        4 => Self::RecoveryRequired,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "CatalogEventKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AssetAvailability {
    #[default] Online,
    Missing,
    Changed,
    Offline,
}
impl AssetAvailability {
    pub fn wire(self) -> i64 { match self {
        Self::Online => 1,
        Self::Missing => 2,
        Self::Changed => 3,
        Self::Offline => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Online,
        2 => Self::Missing,
        3 => Self::Changed,
        4 => Self::Offline,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AssetAvailability", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum IndexStatus {
    #[default] Pending,
    Ready,
    Failed,
}
impl IndexStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Pending => 1,
        Self::Ready => 2,
        Self::Failed => 3,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Pending,
        2 => Self::Ready,
        3 => Self::Failed,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "IndexStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FrameLookupStatus {
    #[default] Ready,
    Pending,
    OutOfRange,
    Unavailable,
    StaleVersion,
    Failed,
}
impl FrameLookupStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Ready => 1,
        Self::Pending => 2,
        Self::OutOfRange => 3,
        Self::Unavailable => 4,
        Self::StaleVersion => 5,
        Self::Failed => 6,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Ready,
        2 => Self::Pending,
        3 => Self::OutOfRange,
        4 => Self::Unavailable,
        5 => Self::StaleVersion,
        6 => Self::Failed,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "FrameLookupStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DurationPolicy {
    #[default] Exact,
    Trim,
    Pad,
}
impl DurationPolicy {
    pub fn wire(self) -> i64 { match self {
        Self::Exact => 1,
        Self::Trim => 2,
        Self::Pad => 3,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Exact,
        2 => Self::Trim,
        3 => Self::Pad,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "DurationPolicy", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FrameRatePolicy {
    #[default] SourcePts,
    Conform,
}
impl FrameRatePolicy {
    pub fn wire(self) -> i64 { match self {
        Self::SourcePts => 1,
        Self::Conform => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::SourcePts,
        2 => Self::Conform,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "FrameRatePolicy", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AspectPolicy {
    #[default] Reject,
    Fit,
    Crop,
}
impl AspectPolicy {
    pub fn wire(self) -> i64 { match self {
        Self::Reject => 1,
        Self::Fit => 2,
        Self::Crop => 3,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Reject,
        2 => Self::Fit,
        3 => Self::Crop,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AspectPolicy", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ChannelPolicy {
    #[default] Reject,
    Map,
}
impl ChannelPolicy {
    pub fn wire(self) -> i64 { match self {
        Self::Reject => 1,
        Self::Map => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Reject,
        2 => Self::Map,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ChannelPolicy", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MissingRangePolicy {
    #[default] Reject,
    Gap,
}
impl MissingRangePolicy {
    pub fn wire(self) -> i64 { match self {
        Self::Reject => 1,
        Self::Gap => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Reject,
        2 => Self::Gap,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "MissingRangePolicy", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PreviewFidelity {
    #[default] Exact,
    Proxy,
}
impl PreviewFidelity {
    pub fn wire(self) -> i64 { match self {
        Self::Exact => 1,
        Self::Proxy => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Exact,
        2 => Self::Proxy,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "PreviewFidelity", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PreviewStatus {
    #[default] Ready,
    Cancelled,
    StaleGraph,
    StaleBinding,
    Unsupported,
    Failed,
    StaleAsset,
}
impl PreviewStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Ready => 1,
        Self::Cancelled => 2,
        Self::StaleGraph => 3,
        Self::StaleBinding => 4,
        Self::Unsupported => 5,
        Self::Failed => 6,
        Self::StaleAsset => 7,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Ready,
        2 => Self::Cancelled,
        3 => Self::StaleGraph,
        4 => Self::StaleBinding,
        5 => Self::Unsupported,
        6 => Self::Failed,
        7 => Self::StaleAsset,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "PreviewStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AuthorityState {
    #[default] Ready,
    ReadOnlyFutureVersion,
    RecoveryRequired,
    Closed,
}
impl AuthorityState {
    pub fn wire(self) -> i64 { match self {
        Self::Ready => 1,
        Self::ReadOnlyFutureVersion => 2,
        Self::RecoveryRequired => 3,
        Self::Closed => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Ready,
        2 => Self::ReadOnlyFutureVersion,
        3 => Self::RecoveryRequired,
        4 => Self::Closed,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AuthorityState", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum LeaseStatus {
    #[default] Granted,
    HeldByOther,
    StaleIncarnation,
    ReadOnly,
    RecoveryRequired,
}
impl LeaseStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Granted => 1,
        Self::HeldByOther => 2,
        Self::StaleIncarnation => 3,
        Self::ReadOnly => 4,
        Self::RecoveryRequired => 5,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Granted,
        2 => Self::HeldByOther,
        3 => Self::StaleIncarnation,
        4 => Self::ReadOnly,
        5 => Self::RecoveryRequired,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "LeaseStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ResourceActionStatus {
    #[default] Accepted,
    Unknown,
    Expired,
    Unauthorized,
}
impl ResourceActionStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Accepted => 1,
        Self::Unknown => 2,
        Self::Expired => 3,
        Self::Unauthorized => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Accepted,
        2 => Self::Unknown,
        3 => Self::Expired,
        4 => Self::Unauthorized,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ResourceActionStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ExportProvenance {
    #[default] GovernedHost,
    StandaloneImport,
}
impl ExportProvenance {
    pub fn wire(self) -> i64 { match self {
        Self::GovernedHost => 1,
        Self::StandaloneImport => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::GovernedHost,
        2 => Self::StandaloneImport,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ExportProvenance", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ExportSubmitStatus {
    #[default] Accepted,
    StaleContext,
    Invalid,
    Unauthorized,
    Unsupported,
    Capacity,
    RecoveryRequired,
    DestinationBusy,
    IdempotencyConflict,
}
impl ExportSubmitStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Accepted => 1,
        Self::StaleContext => 2,
        Self::Invalid => 3,
        Self::Unauthorized => 4,
        Self::Unsupported => 5,
        Self::Capacity => 6,
        Self::RecoveryRequired => 7,
        Self::DestinationBusy => 8,
        Self::IdempotencyConflict => 9,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Accepted,
        2 => Self::StaleContext,
        3 => Self::Invalid,
        4 => Self::Unauthorized,
        5 => Self::Unsupported,
        6 => Self::Capacity,
        7 => Self::RecoveryRequired,
        8 => Self::DestinationBusy,
        9 => Self::IdempotencyConflict,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ExportSubmitStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ExportDestinationPolicy {
    #[default] FailIfExists,
    ReplaceExisting,
}
impl ExportDestinationPolicy {
    pub fn wire(self) -> i64 { match self {
        Self::FailIfExists => 1,
        Self::ReplaceExisting => 2,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::FailIfExists,
        2 => Self::ReplaceExisting,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ExportDestinationPolicy", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ExportLookupStatus {
    #[default] Found,
    Unavailable,
    StaleContext,
}
impl ExportLookupStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Found => 1,
        Self::Unavailable => 2,
        Self::StaleContext => 3,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Found,
        2 => Self::Unavailable,
        3 => Self::StaleContext,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ExportLookupStatus", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ExportJobState {
    #[default] Queued,
    Preparing,
    Running,
    Verifying,
    Succeeded,
    Failed,
    Cancelled,
    Interrupted,
}
impl ExportJobState {
    pub fn wire(self) -> i64 { match self {
        Self::Queued => 1,
        Self::Preparing => 2,
        Self::Running => 3,
        Self::Verifying => 4,
        Self::Succeeded => 5,
        Self::Failed => 6,
        Self::Cancelled => 7,
        Self::Interrupted => 8,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Queued,
        2 => Self::Preparing,
        3 => Self::Running,
        4 => Self::Verifying,
        5 => Self::Succeeded,
        6 => Self::Failed,
        7 => Self::Cancelled,
        8 => Self::Interrupted,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ExportJobState", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ExportEventKind {
    #[default] Ready,
    StateChange,
    Progress,
    Warning,
    Heartbeat,
    RecoveryRequired,
}
impl ExportEventKind {
    pub fn wire(self) -> i64 { match self {
        Self::Ready => 1,
        Self::StateChange => 2,
        Self::Progress => 3,
        Self::Warning => 4,
        Self::Heartbeat => 5,
        Self::RecoveryRequired => 6,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Ready,
        2 => Self::StateChange,
        3 => Self::Progress,
        4 => Self::Warning,
        5 => Self::Heartbeat,
        6 => Self::RecoveryRequired,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ExportEventKind", value: v }),
    }) }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ExportCancelStatus {
    #[default] Accepted,
    AlreadyTerminal,
    Unknown,
    Unauthorized,
    StaleContext,
}
impl ExportCancelStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Accepted => 1,
        Self::AlreadyTerminal => 2,
        Self::Unknown => 3,
        Self::Unauthorized => 4,
        Self::StaleContext => 5,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Accepted,
        2 => Self::AlreadyTerminal,
        3 => Self::Unknown,
        4 => Self::Unauthorized,
        5 => Self::StaleContext,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "ExportCancelStatus", value: v }),
    }) }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl Rational {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.numerator)),
            (2, Cbor::Int(self.denominator)),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            numerator: c.try_get(1)?.try_int()?,
            denominator: c.try_get(2)?.try_int()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TimeRange {
    pub start: Rational,
    pub end: Rational,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl TimeRange {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, self.start.to_cbor()),
            (2, self.end.to_cbor()),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            start: Rational::from_cbor(c.try_get(1)?)?,
            end: Rational::from_cbor(c.try_get(2)?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ParamValue {
    pub kind: ParamKind,
    pub integer: Option<i64>,
    pub rational: Option<Rational>,
    pub boolean: Option<bool>,
    pub text: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ParamValue {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.kind.wire())),
            (2, match &self.integer { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (3, match &self.rational { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, match &self.boolean { Some(v) => Cbor::Bool(*v), None => Cbor::Null }),
            (5, match &self.text { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            kind: ParamKind::from_wire(c.try_get(1)?.try_int()?)?,
            integer: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            rational: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(Rational::from_cbor(v)?) } },
            boolean: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_bool()?) } },
            text: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SourcePayload {
    pub slot_id: String,
    pub stream_id: String,
    pub source_range: TimeRange,
    pub speed: Rational,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl SourcePayload {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.slot_id.clone())),
            (2, Cbor::Text(self.stream_id.clone())),
            (3, self.source_range.to_cbor()),
            (4, self.speed.to_cbor()),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            slot_id: c.try_get(1)?.try_text()?,
            stream_id: c.try_get(2)?.try_text()?,
            source_range: TimeRange::from_cbor(c.try_get(3)?)?,
            speed: Rational::from_cbor(c.try_get(4)?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EffectPayload {
    pub effect_id: String,
    pub effect_version: i64,
    pub params: std::collections::BTreeMap<String, ParamValue>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl EffectPayload {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.effect_id.clone())),
            (2, Cbor::Int(self.effect_version)),
            (3, Cbor::Array(self.params.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            effect_id: c.try_get(1)?.try_text()?,
            effect_version: c.try_get(2)?.try_int()?,
            params: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(3)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, ParamValue::from_cbor(ev)?); } m },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CompositePayload {
    pub blend_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl CompositePayload {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.blend_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            blend_id: c.try_get(1)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TransitionPayload {
    pub transition_id: String,
    pub transition_version: i64,
    pub params: std::collections::BTreeMap<String, ParamValue>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl TransitionPayload {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.transition_id.clone())),
            (2, Cbor::Int(self.transition_version)),
            (3, Cbor::Array(self.params.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            transition_id: c.try_get(1)?.try_text()?,
            transition_version: c.try_get(2)?.try_int()?,
            params: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(3)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, ParamValue::from_cbor(ev)?); } m },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TitlePayload {
    pub text: String,
    pub style_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl TitlePayload {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.text.clone())),
            (2, Cbor::Text(self.style_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            text: c.try_get(1)?.try_text()?,
            style_id: c.try_get(2)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OutputPayload {
    pub output_media: MediaKind,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl OutputPayload {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.output_media.wire())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            output_media: MediaKind::from_wire(c.try_get(1)?.try_int()?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    pub media: MediaKind,
    pub sequence_id: String,
    pub track_id: Option<String>,
    pub timeline_range: Option<TimeRange>,
    pub source: Option<SourcePayload>,
    pub effect: Option<EffectPayload>,
    pub composite: Option<CompositePayload>,
    pub transition: Option<TransitionPayload>,
    pub title: Option<TitlePayload>,
    pub output: Option<OutputPayload>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl Node {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.id.clone())),
            (2, Cbor::Int(self.kind.wire())),
            (3, Cbor::Int(self.media.wire())),
            (4, Cbor::Text(self.sequence_id.clone())),
            (5, match &self.track_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (6, match &self.timeline_range { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (7, match &self.source { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (8, match &self.effect { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (9, match &self.composite { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (10, match &self.transition { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (11, match &self.title { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (12, match &self.output { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            id: c.try_get(1)?.try_text()?,
            kind: NodeKind::from_wire(c.try_get(2)?.try_int()?)?,
            media: MediaKind::from_wire(c.try_get(3)?.try_int()?)?,
            sequence_id: c.try_get(4)?.try_text()?,
            track_id: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            timeline_range: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(TimeRange::from_cbor(v)?) } },
            source: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(SourcePayload::from_cbor(v)?) } },
            effect: { let v = c.try_get(8)?; if v.is_null() { None } else { Some(EffectPayload::from_cbor(v)?) } },
            composite: { let v = c.try_get(9)?; if v.is_null() { None } else { Some(CompositePayload::from_cbor(v)?) } },
            transition: { let v = c.try_get(10)?; if v.is_null() { None } else { Some(TransitionPayload::from_cbor(v)?) } },
            title: { let v = c.try_get(11)?; if v.is_null() { None } else { Some(TitlePayload::from_cbor(v)?) } },
            output: { let v = c.try_get(12)?; if v.is_null() { None } else { Some(OutputPayload::from_cbor(v)?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Edge {
    pub id: String,
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
    pub order_key: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl Edge {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.id.clone())),
            (2, Cbor::Text(self.from_node.clone())),
            (3, Cbor::Text(self.from_port.clone())),
            (4, Cbor::Text(self.to_node.clone())),
            (5, Cbor::Text(self.to_port.clone())),
            (6, match &self.order_key { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            id: c.try_get(1)?.try_text()?,
            from_node: c.try_get(2)?.try_text()?,
            from_port: c.try_get(3)?.try_text()?,
            to_node: c.try_get(4)?.try_text()?,
            to_port: c.try_get(5)?.try_text()?,
            order_key: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Track {
    pub id: String,
    pub sequence_id: String,
    pub media: MediaKind,
    pub order_key: String,
    pub enabled: bool,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl Track {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.id.clone())),
            (2, Cbor::Text(self.sequence_id.clone())),
            (3, Cbor::Int(self.media.wire())),
            (4, Cbor::Text(self.order_key.clone())),
            (5, Cbor::Bool(self.enabled)),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            id: c.try_get(1)?.try_text()?,
            sequence_id: c.try_get(2)?.try_text()?,
            media: MediaKind::from_wire(c.try_get(3)?.try_int()?)?,
            order_key: c.try_get(4)?.try_text()?,
            enabled: c.try_get(5)?.try_bool()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Sequence {
    pub id: String,
    pub video_root: Option<String>,
    pub audio_root: Option<String>,
    pub range: TimeRange,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl Sequence {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.id.clone())),
            (2, match &self.video_root { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (3, match &self.audio_root { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (4, self.range.to_cbor()),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            id: c.try_get(1)?.try_text()?,
            video_root: { let v = c.try_get(2)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            audio_root: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            range: TimeRange::from_cbor(c.try_get(4)?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AssetSlot {
    pub id: String,
    pub expected_media: MediaKind,
    pub expected_duration: Option<Rational>,
    pub expected_fingerprint: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl AssetSlot {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.id.clone())),
            (2, Cbor::Int(self.expected_media.wire())),
            (3, match &self.expected_duration { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, match &self.expected_fingerprint { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            id: c.try_get(1)?.try_text()?,
            expected_media: MediaKind::from_wire(c.try_get(2)?.try_int()?)?,
            expected_duration: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(Rational::from_cbor(v)?) } },
            expected_fingerprint: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GraphSnapshot {
    pub schema_major: i64,
    pub schema_minor: i64,
    pub graph_id: String,
    pub project_id: String,
    pub revision: i64,
    pub semantic_version: i64,
    pub sequences: std::collections::BTreeMap<String, Sequence>,
    pub tracks: std::collections::BTreeMap<String, Track>,
    pub nodes: std::collections::BTreeMap<String, Node>,
    pub edges: std::collections::BTreeMap<String, Edge>,
    pub slots: std::collections::BTreeMap<String, AssetSlot>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl GraphSnapshot {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.schema_major)),
            (2, Cbor::Int(self.schema_minor)),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.project_id.clone())),
            (5, Cbor::Int(self.revision)),
            (6, Cbor::Int(self.semantic_version)),
            (7, Cbor::Array(self.sequences.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (8, Cbor::Array(self.tracks.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (9, Cbor::Array(self.nodes.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (10, Cbor::Array(self.edges.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (11, Cbor::Array(self.slots.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            schema_major: c.try_get(1)?.try_int()?,
            schema_minor: c.try_get(2)?.try_int()?,
            graph_id: c.try_get(3)?.try_text()?,
            project_id: c.try_get(4)?.try_text()?,
            revision: c.try_get(5)?.try_int()?,
            semantic_version: c.try_get(6)?.try_int()?,
            sequences: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(7)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, Sequence::from_cbor(ev)?); } m },
            tracks: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(8)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, Track::from_cbor(ev)?); } m },
            nodes: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(9)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, Node::from_cbor(ev)?); } m },
            edges: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(10)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, Edge::from_cbor(ev)?); } m },
            slots: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(11)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, AssetSlot::from_cbor(ev)?); } m },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GraphSemantics {
    pub schema_major: i64,
    pub schema_minor: i64,
    pub semantic_version: i64,
    pub sequences: std::collections::BTreeMap<String, Sequence>,
    pub tracks: std::collections::BTreeMap<String, Track>,
    pub nodes: std::collections::BTreeMap<String, Node>,
    pub edges: std::collections::BTreeMap<String, Edge>,
    pub slots: std::collections::BTreeMap<String, AssetSlot>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl GraphSemantics {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.schema_major)),
            (2, Cbor::Int(self.schema_minor)),
            (3, Cbor::Int(self.semantic_version)),
            (4, Cbor::Array(self.sequences.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (5, Cbor::Array(self.tracks.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (6, Cbor::Array(self.nodes.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (7, Cbor::Array(self.edges.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (8, Cbor::Array(self.slots.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            schema_major: c.try_get(1)?.try_int()?,
            schema_minor: c.try_get(2)?.try_int()?,
            semantic_version: c.try_get(3)?.try_int()?,
            sequences: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(4)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, Sequence::from_cbor(ev)?); } m },
            tracks: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(5)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, Track::from_cbor(ev)?); } m },
            nodes: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(6)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, Node::from_cbor(ev)?); } m },
            edges: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(7)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, Edge::from_cbor(ev)?); } m },
            slots: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(8)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, AssetSlot::from_cbor(ev)?); } m },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GraphSnapshotDelivery {
    pub snapshot: GraphSnapshot,
    pub authority_incarnation_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl GraphSnapshotDelivery {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, self.snapshot.to_cbor()),
            (2, Cbor::Text(self.authority_incarnation_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            snapshot: GraphSnapshot::from_cbor(c.try_get(1)?)?,
            authority_incarnation_id: c.try_get(2)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EditOperation {
    pub kind: EditKind,
    pub target_id: String,
    pub node: Option<Node>,
    pub edge: Option<Edge>,
    pub track: Option<Track>,
    pub sequence: Option<Sequence>,
    pub slot: Option<AssetSlot>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl EditOperation {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.kind.wire())),
            (2, Cbor::Text(self.target_id.clone())),
            (3, match &self.node { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (4, match &self.edge { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (5, match &self.track { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (6, match &self.sequence { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (7, match &self.slot { Some(v) => v.to_cbor(), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            kind: EditKind::from_wire(c.try_get(1)?.try_int()?)?,
            target_id: c.try_get(2)?.try_text()?,
            node: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(Node::from_cbor(v)?) } },
            edge: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(Edge::from_cbor(v)?) } },
            track: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(Track::from_cbor(v)?) } },
            sequence: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(Sequence::from_cbor(v)?) } },
            slot: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(AssetSlot::from_cbor(v)?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EditBatch {
    pub project_id: String,
    pub graph_id: String,
    pub command_id: String,
    pub expected_revision: i64,
    pub undo_group_id: String,
    pub operations: Vec<EditOperation>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl EditBatch {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.project_id.clone())),
            (2, Cbor::Text(self.graph_id.clone())),
            (3, Cbor::Text(self.command_id.clone())),
            (4, Cbor::Int(self.expected_revision)),
            (5, Cbor::Text(self.undo_group_id.clone())),
            (6, Cbor::Array(self.operations.iter().map(|x| x.to_cbor()).collect())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            project_id: c.try_get(1)?.try_text()?,
            graph_id: c.try_get(2)?.try_text()?,
            command_id: c.try_get(3)?.try_text()?,
            expected_revision: c.try_get(4)?.try_int()?,
            undo_group_id: c.try_get(5)?.try_text()?,
            operations: c.try_get(6)?.try_array()?.iter().map(|x| EditOperation::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ApplyGraphBatch {
    pub batch: EditBatch,
    pub authority_incarnation_id: String,
    pub writer_capability: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ApplyGraphBatch {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, self.batch.to_cbor()),
            (2, Cbor::Text(self.authority_incarnation_id.clone())),
            (3, Cbor::Text(self.writer_capability.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            batch: EditBatch::from_cbor(c.try_get(1)?)?,
            authority_incarnation_id: c.try_get(2)?.try_text()?,
            writer_capability: c.try_get(3)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AffectedInterval {
    pub sequence_id: String,
    pub range: TimeRange,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl AffectedInterval {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.sequence_id.clone())),
            (2, self.range.to_cbor()),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            sequence_id: c.try_get(1)?.try_text()?,
            range: TimeRange::from_cbor(c.try_get(2)?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ChangeFootprint {
    pub intervals: Vec<AffectedInterval>,
    pub node_ids: Vec<String>,
    pub slot_ids: Vec<String>,
    pub full_invalidation: bool,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ChangeFootprint {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Array(self.intervals.iter().map(|x| x.to_cbor()).collect())),
            (2, Cbor::Array(self.node_ids.iter().map(|x| Cbor::Text(x.clone())).collect())),
            (3, Cbor::Array(self.slot_ids.iter().map(|x| Cbor::Text(x.clone())).collect())),
            (4, Cbor::Bool(self.full_invalidation)),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            intervals: c.try_get(1)?.try_array()?.iter().map(|x| AffectedInterval::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            node_ids: c.try_get(2)?.try_array()?.iter().map(|x| Ok(x.try_text()?)).collect::<Result<Vec<_>, DecodeError>>()?,
            slot_ids: c.try_get(3)?.try_array()?.iter().map(|x| Ok(x.try_text()?)).collect::<Result<Vec<_>, DecodeError>>()?,
            full_invalidation: c.try_get(4)?.try_bool()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EditAck {
    pub command_id: String,
    pub status: AckStatus,
    pub revision: i64,
    pub semantic_digest: Option<String>,
    pub reason: Option<String>,
    pub footprint: Option<ChangeFootprint>,
    pub project_id: String,
    pub graph_id: String,
    pub authority_incarnation_id: String,
    pub replayed: bool,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl EditAck {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.command_id.clone())),
            (2, Cbor::Int(self.status.wire())),
            (3, Cbor::Int(self.revision)),
            (4, match &self.semantic_digest { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (5, match &self.reason { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (6, match &self.footprint { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (7, Cbor::Text(self.project_id.clone())),
            (8, Cbor::Text(self.graph_id.clone())),
            (9, Cbor::Text(self.authority_incarnation_id.clone())),
            (10, Cbor::Bool(self.replayed)),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            command_id: c.try_get(1)?.try_text()?,
            status: AckStatus::from_wire(c.try_get(2)?.try_int()?)?,
            revision: c.try_get(3)?.try_int()?,
            semantic_digest: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            reason: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            footprint: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(ChangeFootprint::from_cbor(v)?) } },
            project_id: c.try_get(7)?.try_text()?,
            graph_id: c.try_get(8)?.try_text()?,
            authority_incarnation_id: c.try_get(9)?.try_text()?,
            replayed: c.try_get(10)?.try_bool()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AcceptedChange {
    pub graph_id: String,
    pub command_id: String,
    pub from_revision: i64,
    pub to_revision: i64,
    pub operations: Vec<EditOperation>,
    pub footprint: ChangeFootprint,
    pub project_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl AcceptedChange {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.graph_id.clone())),
            (2, Cbor::Text(self.command_id.clone())),
            (3, Cbor::Int(self.from_revision)),
            (4, Cbor::Int(self.to_revision)),
            (5, Cbor::Array(self.operations.iter().map(|x| x.to_cbor()).collect())),
            (6, self.footprint.to_cbor()),
            (7, Cbor::Text(self.project_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            graph_id: c.try_get(1)?.try_text()?,
            command_id: c.try_get(2)?.try_text()?,
            from_revision: c.try_get(3)?.try_int()?,
            to_revision: c.try_get(4)?.try_int()?,
            operations: c.try_get(5)?.try_array()?.iter().map(|x| EditOperation::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            footprint: ChangeFootprint::from_cbor(c.try_get(6)?)?,
            project_id: c.try_get(7)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct GraphChangeEvent {
    pub kind: GraphEventKind,
    pub project_id: String,
    pub graph_id: String,
    pub authority_incarnation_id: String,
    pub revision: i64,
    pub first_available_revision: i64,
    pub change: Option<AcceptedChange>,
    pub reason: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl GraphChangeEvent {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.kind.wire())),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Int(self.revision)),
            (6, Cbor::Int(self.first_available_revision)),
            (7, match &self.change { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (8, match &self.reason { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            kind: GraphEventKind::from_wire(c.try_get(1)?.try_int()?)?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            revision: c.try_get(5)?.try_int()?,
            first_available_revision: c.try_get(6)?.try_int()?,
            change: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(AcceptedChange::from_cbor(v)?) } },
            reason: { let v = c.try_get(8)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct InsertSourceSpan {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub sequence_id: String,
    pub authority_incarnation_id: String,
    pub command_id: String,
    pub expected_revision: i64,
    pub target_track_id: String,
    pub timeline_at: Rational,
    pub placement: PlacementMode,
    pub slot_id: Option<String>,
    pub stream_id: String,
    pub source_range: TimeRange,
    pub speed: Rational,
    pub expected_binding_set_id: String,
    pub undo_group_id: String,
    pub writer_capability: String,
    pub registered_asset_id: Option<String>,
    pub asset_version_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl InsertSourceSpan {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.sequence_id.clone())),
            (5, Cbor::Text(self.authority_incarnation_id.clone())),
            (6, Cbor::Text(self.command_id.clone())),
            (7, Cbor::Int(self.expected_revision)),
            (8, Cbor::Text(self.target_track_id.clone())),
            (9, self.timeline_at.to_cbor()),
            (10, Cbor::Int(self.placement.wire())),
            (11, match &self.slot_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (12, Cbor::Text(self.stream_id.clone())),
            (13, self.source_range.to_cbor()),
            (14, self.speed.to_cbor()),
            (15, Cbor::Text(self.expected_binding_set_id.clone())),
            (16, Cbor::Text(self.undo_group_id.clone())),
            (17, Cbor::Text(self.writer_capability.clone())),
            (18, match &self.registered_asset_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (19, match &self.asset_version_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            sequence_id: c.try_get(4)?.try_text()?,
            authority_incarnation_id: c.try_get(5)?.try_text()?,
            command_id: c.try_get(6)?.try_text()?,
            expected_revision: c.try_get(7)?.try_int()?,
            target_track_id: c.try_get(8)?.try_text()?,
            timeline_at: Rational::from_cbor(c.try_get(9)?)?,
            placement: PlacementMode::from_wire(c.try_get(10)?.try_int()?)?,
            slot_id: { let v = c.try_get(11)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            stream_id: c.try_get(12)?.try_text()?,
            source_range: TimeRange::from_cbor(c.try_get(13)?)?,
            speed: Rational::from_cbor(c.try_get(14)?)?,
            expected_binding_set_id: c.try_get(15)?.try_text()?,
            undo_group_id: c.try_get(16)?.try_text()?,
            writer_capability: c.try_get(17)?.try_text()?,
            registered_asset_id: { let v = c.try_get(18)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            asset_version_id: { let v = c.try_get(19)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryIntent {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub sequence_id: String,
    pub authority_incarnation_id: String,
    pub command_id: String,
    pub expected_revision: i64,
    pub target_undo_group_id: String,
    pub expected_binding_set_id: String,
    pub writer_capability: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl HistoryIntent {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.sequence_id.clone())),
            (5, Cbor::Text(self.authority_incarnation_id.clone())),
            (6, Cbor::Text(self.command_id.clone())),
            (7, Cbor::Int(self.expected_revision)),
            (8, Cbor::Text(self.target_undo_group_id.clone())),
            (9, Cbor::Text(self.expected_binding_set_id.clone())),
            (10, Cbor::Text(self.writer_capability.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            sequence_id: c.try_get(4)?.try_text()?,
            authority_incarnation_id: c.try_get(5)?.try_text()?,
            command_id: c.try_get(6)?.try_text()?,
            expected_revision: c.try_get(7)?.try_int()?,
            target_undo_group_id: c.try_get(8)?.try_text()?,
            expected_binding_set_id: c.try_get(9)?.try_text()?,
            writer_capability: c.try_get(10)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct UndoGroupSummary {
    pub group_id: String,
    pub sequence_id: String,
    pub label: String,
    pub command_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl UndoGroupSummary {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.group_id.clone())),
            (2, Cbor::Text(self.sequence_id.clone())),
            (3, Cbor::Text(self.label.clone())),
            (4, Cbor::Text(self.command_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            group_id: c.try_get(1)?.try_text()?,
            sequence_id: c.try_get(2)?.try_text()?,
            label: c.try_get(3)?.try_text()?,
            command_id: c.try_get(4)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct HistoryState {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub sequence_id: String,
    pub authority_incarnation_id: String,
    pub revision: i64,
    pub undo_groups: Vec<UndoGroupSummary>,
    pub redo_groups: Vec<UndoGroupSummary>,
    pub can_undo: bool,
    pub can_redo: bool,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl HistoryState {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.sequence_id.clone())),
            (5, Cbor::Text(self.authority_incarnation_id.clone())),
            (6, Cbor::Int(self.revision)),
            (7, Cbor::Array(self.undo_groups.iter().map(|x| x.to_cbor()).collect())),
            (8, Cbor::Array(self.redo_groups.iter().map(|x| x.to_cbor()).collect())),
            (9, Cbor::Bool(self.can_undo)),
            (10, Cbor::Bool(self.can_redo)),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            sequence_id: c.try_get(4)?.try_text()?,
            authority_incarnation_id: c.try_get(5)?.try_text()?,
            revision: c.try_get(6)?.try_int()?,
            undo_groups: c.try_get(7)?.try_array()?.iter().map(|x| UndoGroupSummary::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            redo_groups: c.try_get(8)?.try_array()?.iter().map(|x| UndoGroupSummary::from_cbor(x)).collect::<Result<Vec<_>, DecodeError>>()?,
            can_undo: c.try_get(9)?.try_bool()?,
            can_redo: c.try_get(10)?.try_bool()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EditorAck {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub sequence_id: String,
    pub authority_incarnation_id: String,
    pub command_id: String,
    pub status: EditorStatus,
    pub replayed: bool,
    pub revision: i64,
    pub binding_set_id: String,
    pub footprint: Option<ChangeFootprint>,
    pub diagnostic_id: Option<String>,
    pub binding_revision: i64,
    pub created_slot_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl EditorAck {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.sequence_id.clone())),
            (5, Cbor::Text(self.authority_incarnation_id.clone())),
            (6, Cbor::Text(self.command_id.clone())),
            (7, Cbor::Int(self.status.wire())),
            (8, Cbor::Bool(self.replayed)),
            (9, Cbor::Int(self.revision)),
            (10, Cbor::Text(self.binding_set_id.clone())),
            (11, match &self.footprint { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (12, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (13, Cbor::Int(self.binding_revision)),
            (14, match &self.created_slot_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            sequence_id: c.try_get(4)?.try_text()?,
            authority_incarnation_id: c.try_get(5)?.try_text()?,
            command_id: c.try_get(6)?.try_text()?,
            status: EditorStatus::from_wire(c.try_get(7)?.try_int()?)?,
            replayed: c.try_get(8)?.try_bool()?,
            revision: c.try_get(9)?.try_int()?,
            binding_set_id: c.try_get(10)?.try_text()?,
            footprint: { let v = c.try_get(11)?; if v.is_null() { None } else { Some(ChangeFootprint::from_cbor(v)?) } },
            diagnostic_id: { let v = c.try_get(12)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            binding_revision: c.try_get(13)?.try_int()?,
            created_slot_id: { let v = c.try_get(14)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CatalogStream {
    pub id: String,
    pub media: MediaKind,
    pub duration: Rational,
    pub pts_origin: Rational,
    pub frame_count: Option<i64>,
    pub frame_index_id: Option<String>,
    pub index_status: IndexStatus,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl CatalogStream {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.id.clone())),
            (2, Cbor::Int(self.media.wire())),
            (3, self.duration.to_cbor()),
            (4, self.pts_origin.to_cbor()),
            (5, match &self.frame_count { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (6, match &self.frame_index_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (7, Cbor::Int(self.index_status.wire())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            id: c.try_get(1)?.try_text()?,
            media: MediaKind::from_wire(c.try_get(2)?.try_int()?)?,
            duration: Rational::from_cbor(c.try_get(3)?)?,
            pts_origin: Rational::from_cbor(c.try_get(4)?)?,
            frame_count: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            frame_index_id: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            index_status: IndexStatus::from_wire(c.try_get(7)?.try_int()?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RegisteredAsset {
    pub id: String,
    pub version_id: String,
    pub content_fingerprint: String,
    pub display_name: String,
    pub streams: std::collections::BTreeMap<String, CatalogStream>,
    pub availability: AssetAvailability,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl RegisteredAsset {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.id.clone())),
            (2, Cbor::Text(self.version_id.clone())),
            (3, Cbor::Text(self.content_fingerprint.clone())),
            (4, Cbor::Text(self.display_name.clone())),
            (5, Cbor::Array(self.streams.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (6, Cbor::Int(self.availability.wire())),
            (7, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            id: c.try_get(1)?.try_text()?,
            version_id: c.try_get(2)?.try_text()?,
            content_fingerprint: c.try_get(3)?.try_text()?,
            display_name: c.try_get(4)?.try_text()?,
            streams: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(5)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, CatalogStream::from_cbor(ev)?); } m },
            availability: AssetAvailability::from_wire(c.try_get(6)?.try_int()?)?,
            diagnostic_id: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CatalogSnapshot {
    pub contract_version: i64,
    pub project_id: String,
    pub authority_incarnation_id: String,
    pub catalog_revision: i64,
    pub assets: std::collections::BTreeMap<String, RegisteredAsset>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl CatalogSnapshot {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.authority_incarnation_id.clone())),
            (4, Cbor::Int(self.catalog_revision)),
            (5, Cbor::Array(self.assets.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            authority_incarnation_id: c.try_get(3)?.try_text()?,
            catalog_revision: c.try_get(4)?.try_int()?,
            assets: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(5)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, RegisteredAsset::from_cbor(ev)?); } m },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CatalogChange {
    pub project_id: String,
    pub from_catalog_revision: i64,
    pub to_catalog_revision: i64,
    pub upserts: std::collections::BTreeMap<String, RegisteredAsset>,
    pub removed_asset_ids: Vec<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl CatalogChange {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.project_id.clone())),
            (2, Cbor::Int(self.from_catalog_revision)),
            (3, Cbor::Int(self.to_catalog_revision)),
            (4, Cbor::Array(self.upserts.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
            (5, Cbor::Array(self.removed_asset_ids.iter().map(|x| Cbor::Text(x.clone())).collect())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            project_id: c.try_get(1)?.try_text()?,
            from_catalog_revision: c.try_get(2)?.try_int()?,
            to_catalog_revision: c.try_get(3)?.try_int()?,
            upserts: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(4)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, RegisteredAsset::from_cbor(ev)?); } m },
            removed_asset_ids: c.try_get(5)?.try_array()?.iter().map(|x| Ok(x.try_text()?)).collect::<Result<Vec<_>, DecodeError>>()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CatalogChangeEvent {
    pub kind: CatalogEventKind,
    pub project_id: String,
    pub authority_incarnation_id: String,
    pub catalog_revision: i64,
    pub first_available_revision: i64,
    pub change: Option<CatalogChange>,
    pub reason: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl CatalogChangeEvent {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.kind.wire())),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.authority_incarnation_id.clone())),
            (4, Cbor::Int(self.catalog_revision)),
            (5, Cbor::Int(self.first_available_revision)),
            (6, match &self.change { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (7, match &self.reason { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            kind: CatalogEventKind::from_wire(c.try_get(1)?.try_int()?)?,
            project_id: c.try_get(2)?.try_text()?,
            authority_incarnation_id: c.try_get(3)?.try_text()?,
            catalog_revision: c.try_get(4)?.try_int()?,
            first_available_revision: c.try_get(5)?.try_int()?,
            change: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(CatalogChange::from_cbor(v)?) } },
            reason: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct FrameIndexQuery {
    pub contract_version: i64,
    pub project_id: String,
    pub authority_incarnation_id: String,
    pub registered_asset_id: String,
    pub asset_version_id: String,
    pub stream_id: String,
    pub frame_index: i64,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl FrameIndexQuery {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.authority_incarnation_id.clone())),
            (4, Cbor::Text(self.registered_asset_id.clone())),
            (5, Cbor::Text(self.asset_version_id.clone())),
            (6, Cbor::Text(self.stream_id.clone())),
            (7, Cbor::Int(self.frame_index)),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            authority_incarnation_id: c.try_get(3)?.try_text()?,
            registered_asset_id: c.try_get(4)?.try_text()?,
            asset_version_id: c.try_get(5)?.try_text()?,
            stream_id: c.try_get(6)?.try_text()?,
            frame_index: c.try_get(7)?.try_int()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct FrameIndexResult {
    pub contract_version: i64,
    pub project_id: String,
    pub authority_incarnation_id: String,
    pub registered_asset_id: String,
    pub asset_version_id: String,
    pub stream_id: String,
    pub frame_index: i64,
    pub status: FrameLookupStatus,
    pub pts: Option<Rational>,
    pub frame_count: Option<i64>,
    pub content_fingerprint: Option<String>,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl FrameIndexResult {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.authority_incarnation_id.clone())),
            (4, Cbor::Text(self.registered_asset_id.clone())),
            (5, Cbor::Text(self.asset_version_id.clone())),
            (6, Cbor::Text(self.stream_id.clone())),
            (7, Cbor::Int(self.frame_index)),
            (8, Cbor::Int(self.status.wire())),
            (9, match &self.pts { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (10, match &self.frame_count { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (11, match &self.content_fingerprint { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (12, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            authority_incarnation_id: c.try_get(3)?.try_text()?,
            registered_asset_id: c.try_get(4)?.try_text()?,
            asset_version_id: c.try_get(5)?.try_text()?,
            stream_id: c.try_get(6)?.try_text()?,
            frame_index: c.try_get(7)?.try_int()?,
            status: FrameLookupStatus::from_wire(c.try_get(8)?.try_int()?)?,
            pts: { let v = c.try_get(9)?; if v.is_null() { None } else { Some(Rational::from_cbor(v)?) } },
            frame_count: { let v = c.try_get(10)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            content_fingerprint: { let v = c.try_get(11)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            diagnostic_id: { let v = c.try_get(12)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AssetBinding {
    pub slot_id: String,
    pub registered_asset_id: String,
    pub asset_version_id: String,
    pub stream_id: String,
    pub content_fingerprint: String,
    pub media: MediaKind,
    pub duration: Rational,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl AssetBinding {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.slot_id.clone())),
            (2, Cbor::Text(self.registered_asset_id.clone())),
            (3, Cbor::Text(self.asset_version_id.clone())),
            (4, Cbor::Text(self.stream_id.clone())),
            (5, Cbor::Text(self.content_fingerprint.clone())),
            (6, Cbor::Int(self.media.wire())),
            (7, self.duration.to_cbor()),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            slot_id: c.try_get(1)?.try_text()?,
            registered_asset_id: c.try_get(2)?.try_text()?,
            asset_version_id: c.try_get(3)?.try_text()?,
            stream_id: c.try_get(4)?.try_text()?,
            content_fingerprint: c.try_get(5)?.try_text()?,
            media: MediaKind::from_wire(c.try_get(6)?.try_int()?)?,
            duration: Rational::from_cbor(c.try_get(7)?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BindingSnapshot {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub authority_incarnation_id: String,
    pub binding_set_id: String,
    pub binding_revision: i64,
    pub bindings: std::collections::BTreeMap<String, AssetBinding>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl BindingSnapshot {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Text(self.binding_set_id.clone())),
            (6, Cbor::Int(self.binding_revision)),
            (7, Cbor::Array(self.bindings.iter().map(|(k, v)| Cbor::Map(vec![(1, Cbor::Text(k.clone())), (2, v.to_cbor())])).collect())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            binding_set_id: c.try_get(5)?.try_text()?,
            binding_revision: c.try_get(6)?.try_int()?,
            bindings: { let mut m = std::collections::BTreeMap::new(); for e in c.try_get(7)?.try_array()? { let ek = e.try_get(1)?; let ev = e.try_get(2)?; let k = ek.try_text()?; if m.contains_key(&k) { return Err(DecodeError::DuplicateMapKey(k.into())); } m.insert(k, AssetBinding::from_cbor(ev)?); } m },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BindingPolicy {
    pub duration: DurationPolicy,
    pub frame_rate: FrameRatePolicy,
    pub aspect: AspectPolicy,
    pub channels: ChannelPolicy,
    pub missing_range: MissingRangePolicy,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl BindingPolicy {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.duration.wire())),
            (2, Cbor::Int(self.frame_rate.wire())),
            (3, Cbor::Int(self.aspect.wire())),
            (4, Cbor::Int(self.channels.wire())),
            (5, Cbor::Int(self.missing_range.wire())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            duration: DurationPolicy::from_wire(c.try_get(1)?.try_int()?)?,
            frame_rate: FrameRatePolicy::from_wire(c.try_get(2)?.try_int()?)?,
            aspect: AspectPolicy::from_wire(c.try_get(3)?.try_int()?)?,
            channels: ChannelPolicy::from_wire(c.try_get(4)?.try_int()?)?,
            missing_range: MissingRangePolicy::from_wire(c.try_get(5)?.try_int()?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct RebindSlot {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub authority_incarnation_id: String,
    pub command_id: String,
    pub expected_graph_revision: i64,
    pub expected_binding_set_id: String,
    pub slot_id: String,
    pub registered_asset_id: String,
    pub asset_version_id: String,
    pub stream_id: String,
    pub policy: BindingPolicy,
    pub writer_capability: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl RebindSlot {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Text(self.command_id.clone())),
            (6, Cbor::Int(self.expected_graph_revision)),
            (7, Cbor::Text(self.expected_binding_set_id.clone())),
            (8, Cbor::Text(self.slot_id.clone())),
            (9, Cbor::Text(self.registered_asset_id.clone())),
            (10, Cbor::Text(self.asset_version_id.clone())),
            (11, Cbor::Text(self.stream_id.clone())),
            (12, self.policy.to_cbor()),
            (13, Cbor::Text(self.writer_capability.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            command_id: c.try_get(5)?.try_text()?,
            expected_graph_revision: c.try_get(6)?.try_int()?,
            expected_binding_set_id: c.try_get(7)?.try_text()?,
            slot_id: c.try_get(8)?.try_text()?,
            registered_asset_id: c.try_get(9)?.try_text()?,
            asset_version_id: c.try_get(10)?.try_text()?,
            stream_id: c.try_get(11)?.try_text()?,
            policy: BindingPolicy::from_cbor(c.try_get(12)?)?,
            writer_capability: c.try_get(13)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BindingAck {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub authority_incarnation_id: String,
    pub command_id: String,
    pub status: BindingStatus,
    pub replayed: bool,
    pub graph_revision: i64,
    pub binding_set_id: String,
    pub binding_revision: i64,
    pub footprint: Option<ChangeFootprint>,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl BindingAck {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Text(self.command_id.clone())),
            (6, Cbor::Int(self.status.wire())),
            (7, Cbor::Bool(self.replayed)),
            (8, Cbor::Int(self.graph_revision)),
            (9, Cbor::Text(self.binding_set_id.clone())),
            (10, Cbor::Int(self.binding_revision)),
            (11, match &self.footprint { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (12, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            command_id: c.try_get(5)?.try_text()?,
            status: BindingStatus::from_wire(c.try_get(6)?.try_int()?)?,
            replayed: c.try_get(7)?.try_bool()?,
            graph_revision: c.try_get(8)?.try_int()?,
            binding_set_id: c.try_get(9)?.try_text()?,
            binding_revision: c.try_get(10)?.try_int()?,
            footprint: { let v = c.try_get(11)?; if v.is_null() { None } else { Some(ChangeFootprint::from_cbor(v)?) } },
            diagnostic_id: { let v = c.try_get(12)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BindingChange {
    pub project_id: String,
    pub graph_id: String,
    pub from_binding_revision: i64,
    pub to_binding_revision: i64,
    pub from_binding_set_id: String,
    pub to_binding_set_id: String,
    pub changed_slot_ids: Vec<String>,
    pub footprint: ChangeFootprint,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl BindingChange {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.project_id.clone())),
            (2, Cbor::Text(self.graph_id.clone())),
            (3, Cbor::Int(self.from_binding_revision)),
            (4, Cbor::Int(self.to_binding_revision)),
            (5, Cbor::Text(self.from_binding_set_id.clone())),
            (6, Cbor::Text(self.to_binding_set_id.clone())),
            (7, Cbor::Array(self.changed_slot_ids.iter().map(|x| Cbor::Text(x.clone())).collect())),
            (8, self.footprint.to_cbor()),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            project_id: c.try_get(1)?.try_text()?,
            graph_id: c.try_get(2)?.try_text()?,
            from_binding_revision: c.try_get(3)?.try_int()?,
            to_binding_revision: c.try_get(4)?.try_int()?,
            from_binding_set_id: c.try_get(5)?.try_text()?,
            to_binding_set_id: c.try_get(6)?.try_text()?,
            changed_slot_ids: c.try_get(7)?.try_array()?.iter().map(|x| Ok(x.try_text()?)).collect::<Result<Vec<_>, DecodeError>>()?,
            footprint: ChangeFootprint::from_cbor(c.try_get(8)?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct BindingChangeEvent {
    pub kind: BindingEventKind,
    pub project_id: String,
    pub graph_id: String,
    pub authority_incarnation_id: String,
    pub binding_revision: i64,
    pub first_available_revision: i64,
    pub change: Option<BindingChange>,
    pub reason: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl BindingChangeEvent {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.kind.wire())),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Int(self.binding_revision)),
            (6, Cbor::Int(self.first_available_revision)),
            (7, match &self.change { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (8, match &self.reason { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            kind: BindingEventKind::from_wire(c.try_get(1)?.try_int()?)?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            binding_revision: c.try_get(5)?.try_int()?,
            first_available_revision: c.try_get(6)?.try_int()?,
            change: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(BindingChange::from_cbor(v)?) } },
            reason: { let v = c.try_get(8)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SourcePreviewRequest {
    pub contract_version: i64,
    pub project_id: String,
    pub authority_incarnation_id: String,
    pub registered_asset_id: String,
    pub asset_version_id: String,
    pub stream_id: String,
    pub request_id: String,
    pub viewer_id: String,
    pub cancel_group_id: String,
    pub at: Rational,
    pub fidelity: PreviewFidelity,
    pub max_edge_px: i64,
    pub expected_content_fingerprint: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl SourcePreviewRequest {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.authority_incarnation_id.clone())),
            (4, Cbor::Text(self.registered_asset_id.clone())),
            (5, Cbor::Text(self.asset_version_id.clone())),
            (6, Cbor::Text(self.stream_id.clone())),
            (7, Cbor::Text(self.request_id.clone())),
            (8, Cbor::Text(self.viewer_id.clone())),
            (9, Cbor::Text(self.cancel_group_id.clone())),
            (10, self.at.to_cbor()),
            (11, Cbor::Int(self.fidelity.wire())),
            (12, Cbor::Int(self.max_edge_px)),
            (13, Cbor::Text(self.expected_content_fingerprint.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            authority_incarnation_id: c.try_get(3)?.try_text()?,
            registered_asset_id: c.try_get(4)?.try_text()?,
            asset_version_id: c.try_get(5)?.try_text()?,
            stream_id: c.try_get(6)?.try_text()?,
            request_id: c.try_get(7)?.try_text()?,
            viewer_id: c.try_get(8)?.try_text()?,
            cancel_group_id: c.try_get(9)?.try_text()?,
            at: Rational::from_cbor(c.try_get(10)?)?,
            fidelity: PreviewFidelity::from_wire(c.try_get(11)?.try_int()?)?,
            max_edge_px: c.try_get(12)?.try_int()?,
            expected_content_fingerprint: c.try_get(13)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SequencePreviewRequest {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub sequence_id: String,
    pub authority_incarnation_id: String,
    pub accepted_revision: i64,
    pub binding_set_id: String,
    pub binding_revision: i64,
    pub request_id: String,
    pub viewer_id: String,
    pub cancel_group_id: String,
    pub at: Rational,
    pub fidelity: PreviewFidelity,
    pub max_edge_px: i64,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl SequencePreviewRequest {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.sequence_id.clone())),
            (5, Cbor::Text(self.authority_incarnation_id.clone())),
            (6, Cbor::Int(self.accepted_revision)),
            (7, Cbor::Text(self.binding_set_id.clone())),
            (8, Cbor::Int(self.binding_revision)),
            (9, Cbor::Text(self.request_id.clone())),
            (10, Cbor::Text(self.viewer_id.clone())),
            (11, Cbor::Text(self.cancel_group_id.clone())),
            (12, self.at.to_cbor()),
            (13, Cbor::Int(self.fidelity.wire())),
            (14, Cbor::Int(self.max_edge_px)),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            sequence_id: c.try_get(4)?.try_text()?,
            authority_incarnation_id: c.try_get(5)?.try_text()?,
            accepted_revision: c.try_get(6)?.try_int()?,
            binding_set_id: c.try_get(7)?.try_text()?,
            binding_revision: c.try_get(8)?.try_int()?,
            request_id: c.try_get(9)?.try_text()?,
            viewer_id: c.try_get(10)?.try_text()?,
            cancel_group_id: c.try_get(11)?.try_text()?,
            at: Rational::from_cbor(c.try_get(12)?)?,
            fidelity: PreviewFidelity::from_wire(c.try_get(13)?.try_int()?)?,
            max_edge_px: c.try_get(14)?.try_int()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ResourceDescriptor {
    pub resource_id: String,
    pub lease_id: String,
    pub mime_type: String,
    pub width: i64,
    pub height: i64,
    pub byte_length: i64,
    pub expires_at_unix_ms: i64,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ResourceDescriptor {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.resource_id.clone())),
            (2, Cbor::Text(self.lease_id.clone())),
            (3, Cbor::Text(self.mime_type.clone())),
            (4, Cbor::Int(self.width)),
            (5, Cbor::Int(self.height)),
            (6, Cbor::Int(self.byte_length)),
            (7, Cbor::Int(self.expires_at_unix_ms)),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            resource_id: c.try_get(1)?.try_text()?,
            lease_id: c.try_get(2)?.try_text()?,
            mime_type: c.try_get(3)?.try_text()?,
            width: c.try_get(4)?.try_int()?,
            height: c.try_get(5)?.try_int()?,
            byte_length: c.try_get(6)?.try_int()?,
            expires_at_unix_ms: c.try_get(7)?.try_int()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SourcePreviewResult {
    pub contract_version: i64,
    pub project_id: String,
    pub authority_incarnation_id: String,
    pub registered_asset_id: String,
    pub asset_version_id: String,
    pub stream_id: String,
    pub request_id: String,
    pub viewer_id: String,
    pub cancel_group_id: String,
    pub status: PreviewStatus,
    pub fidelity: PreviewFidelity,
    pub actual_time: Option<Rational>,
    pub resource: Option<ResourceDescriptor>,
    pub error_code: Option<String>,
    pub diagnostic_id: Option<String>,
    pub content_fingerprint: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl SourcePreviewResult {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.authority_incarnation_id.clone())),
            (4, Cbor::Text(self.registered_asset_id.clone())),
            (5, Cbor::Text(self.asset_version_id.clone())),
            (6, Cbor::Text(self.stream_id.clone())),
            (7, Cbor::Text(self.request_id.clone())),
            (8, Cbor::Text(self.viewer_id.clone())),
            (9, Cbor::Text(self.cancel_group_id.clone())),
            (10, Cbor::Int(self.status.wire())),
            (11, Cbor::Int(self.fidelity.wire())),
            (12, match &self.actual_time { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (13, match &self.resource { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (14, match &self.error_code { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (15, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (16, Cbor::Text(self.content_fingerprint.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            authority_incarnation_id: c.try_get(3)?.try_text()?,
            registered_asset_id: c.try_get(4)?.try_text()?,
            asset_version_id: c.try_get(5)?.try_text()?,
            stream_id: c.try_get(6)?.try_text()?,
            request_id: c.try_get(7)?.try_text()?,
            viewer_id: c.try_get(8)?.try_text()?,
            cancel_group_id: c.try_get(9)?.try_text()?,
            status: PreviewStatus::from_wire(c.try_get(10)?.try_int()?)?,
            fidelity: PreviewFidelity::from_wire(c.try_get(11)?.try_int()?)?,
            actual_time: { let v = c.try_get(12)?; if v.is_null() { None } else { Some(Rational::from_cbor(v)?) } },
            resource: { let v = c.try_get(13)?; if v.is_null() { None } else { Some(ResourceDescriptor::from_cbor(v)?) } },
            error_code: { let v = c.try_get(14)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            diagnostic_id: { let v = c.try_get(15)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            content_fingerprint: c.try_get(16)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SequencePreviewResult {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub sequence_id: String,
    pub authority_incarnation_id: String,
    pub accepted_revision: i64,
    pub binding_set_id: String,
    pub binding_revision: i64,
    pub request_id: String,
    pub viewer_id: String,
    pub cancel_group_id: String,
    pub status: PreviewStatus,
    pub fidelity: PreviewFidelity,
    pub actual_time: Option<Rational>,
    pub plan_id: Option<String>,
    pub resource: Option<ResourceDescriptor>,
    pub error_code: Option<String>,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl SequencePreviewResult {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.sequence_id.clone())),
            (5, Cbor::Text(self.authority_incarnation_id.clone())),
            (6, Cbor::Int(self.accepted_revision)),
            (7, Cbor::Text(self.binding_set_id.clone())),
            (8, Cbor::Int(self.binding_revision)),
            (9, Cbor::Text(self.request_id.clone())),
            (10, Cbor::Text(self.viewer_id.clone())),
            (11, Cbor::Text(self.cancel_group_id.clone())),
            (12, Cbor::Int(self.status.wire())),
            (13, Cbor::Int(self.fidelity.wire())),
            (14, match &self.actual_time { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (15, match &self.plan_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (16, match &self.resource { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (17, match &self.error_code { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (18, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            sequence_id: c.try_get(4)?.try_text()?,
            authority_incarnation_id: c.try_get(5)?.try_text()?,
            accepted_revision: c.try_get(6)?.try_int()?,
            binding_set_id: c.try_get(7)?.try_text()?,
            binding_revision: c.try_get(8)?.try_int()?,
            request_id: c.try_get(9)?.try_text()?,
            viewer_id: c.try_get(10)?.try_text()?,
            cancel_group_id: c.try_get(11)?.try_text()?,
            status: PreviewStatus::from_wire(c.try_get(12)?.try_int()?)?,
            fidelity: PreviewFidelity::from_wire(c.try_get(13)?.try_int()?)?,
            actual_time: { let v = c.try_get(14)?; if v.is_null() { None } else { Some(Rational::from_cbor(v)?) } },
            plan_id: { let v = c.try_get(15)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            resource: { let v = c.try_get(16)?; if v.is_null() { None } else { Some(ResourceDescriptor::from_cbor(v)?) } },
            error_code: { let v = c.try_get(17)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            diagnostic_id: { let v = c.try_get(18)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CancelPreview {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: Option<String>,
    pub authority_incarnation_id: String,
    pub request_id: String,
    pub viewer_id: String,
    pub cancel_group_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl CancelPreview {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, match &self.graph_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Text(self.request_id.clone())),
            (6, Cbor::Text(self.viewer_id.clone())),
            (7, Cbor::Text(self.cancel_group_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            request_id: c.try_get(5)?.try_text()?,
            viewer_id: c.try_get(6)?.try_text()?,
            cancel_group_id: c.try_get(7)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ReleaseResource {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: Option<String>,
    pub authority_incarnation_id: String,
    pub lease_id: String,
    pub viewer_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ReleaseResource {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, match &self.graph_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Text(self.lease_id.clone())),
            (6, Cbor::Text(self.viewer_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            lease_id: c.try_get(5)?.try_text()?,
            viewer_id: c.try_get(6)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ResourceActionAck {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: Option<String>,
    pub authority_incarnation_id: String,
    pub action_id: String,
    pub status: ResourceActionStatus,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ResourceActionAck {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, match &self.graph_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Text(self.action_id.clone())),
            (6, Cbor::Int(self.status.wire())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            action_id: c.try_get(5)?.try_text()?,
            status: ResourceActionStatus::from_wire(c.try_get(6)?.try_int()?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportJobContext {
    pub request_id: String,
    pub provenance: ExportProvenance,
    pub project_id: String,
    pub graph_id: String,
    pub sequence_id: String,
    pub authority_incarnation_id: Option<String>,
    pub local_import_id: Option<String>,
    pub accepted_revision: i64,
    pub binding_set_id: String,
    pub binding_revision: i64,
    pub profile_id: String,
    pub profile_version: i64,
    pub profile_digest: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportJobContext {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.request_id.clone())),
            (2, Cbor::Int(self.provenance.wire())),
            (3, Cbor::Text(self.project_id.clone())),
            (4, Cbor::Text(self.graph_id.clone())),
            (5, Cbor::Text(self.sequence_id.clone())),
            (6, match &self.authority_incarnation_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (7, match &self.local_import_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (8, Cbor::Int(self.accepted_revision)),
            (9, Cbor::Text(self.binding_set_id.clone())),
            (10, Cbor::Int(self.binding_revision)),
            (11, Cbor::Text(self.profile_id.clone())),
            (12, Cbor::Int(self.profile_version)),
            (13, Cbor::Text(self.profile_digest.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            request_id: c.try_get(1)?.try_text()?,
            provenance: ExportProvenance::from_wire(c.try_get(2)?.try_int()?)?,
            project_id: c.try_get(3)?.try_text()?,
            graph_id: c.try_get(4)?.try_text()?,
            sequence_id: c.try_get(5)?.try_text()?,
            authority_incarnation_id: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            local_import_id: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            accepted_revision: c.try_get(8)?.try_int()?,
            binding_set_id: c.try_get(9)?.try_text()?,
            binding_revision: c.try_get(10)?.try_int()?,
            profile_id: c.try_get(11)?.try_text()?,
            profile_version: c.try_get(12)?.try_int()?,
            profile_digest: c.try_get(13)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportJobRequest {
    pub contract_version: i64,
    pub context: ExportJobContext,
    pub range: TimeRange,
    pub destination_ref: String,
    pub allow_software_fallback: bool,
    pub destination_policy: ExportDestinationPolicy,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportJobRequest {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, self.context.to_cbor()),
            (3, self.range.to_cbor()),
            (4, Cbor::Text(self.destination_ref.clone())),
            (5, Cbor::Bool(self.allow_software_fallback)),
            (6, Cbor::Int(self.destination_policy.wire())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            context: ExportJobContext::from_cbor(c.try_get(2)?)?,
            range: TimeRange::from_cbor(c.try_get(3)?)?,
            destination_ref: c.try_get(4)?.try_text()?,
            allow_software_fallback: c.try_get(5)?.try_bool()?,
            destination_policy: ExportDestinationPolicy::from_wire(c.try_get(6)?.try_int()?)?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportJobAck {
    pub contract_version: i64,
    pub context: ExportJobContext,
    pub status: ExportSubmitStatus,
    pub job_id: Option<String>,
    pub replayed: bool,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportJobAck {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, self.context.to_cbor()),
            (3, Cbor::Int(self.status.wire())),
            (4, match &self.job_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (5, Cbor::Bool(self.replayed)),
            (6, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            context: ExportJobContext::from_cbor(c.try_get(2)?)?,
            status: ExportSubmitStatus::from_wire(c.try_get(3)?.try_int()?)?,
            job_id: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            replayed: c.try_get(5)?.try_bool()?,
            diagnostic_id: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportProbeSummary {
    pub video_stream_count: i64,
    pub audio_stream_count: i64,
    pub duration: Rational,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub audio_sample_rate: Option<i64>,
    pub audio_channel_layout: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportProbeSummary {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.video_stream_count)),
            (2, Cbor::Int(self.audio_stream_count)),
            (3, self.duration.to_cbor()),
            (4, match &self.width { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (5, match &self.height { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (6, match &self.audio_sample_rate { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
            (7, match &self.audio_channel_layout { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            video_stream_count: c.try_get(1)?.try_int()?,
            audio_stream_count: c.try_get(2)?.try_int()?,
            duration: Rational::from_cbor(c.try_get(3)?)?,
            width: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            height: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            audio_sample_rate: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            audio_channel_layout: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportResult {
    pub export_record_id: String,
    pub output_artifact_id: String,
    pub probe_summary: ExportProbeSummary,
    pub output_probe_digest: String,
    pub destination_generation_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportResult {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Text(self.export_record_id.clone())),
            (2, Cbor::Text(self.output_artifact_id.clone())),
            (3, self.probe_summary.to_cbor()),
            (4, Cbor::Text(self.output_probe_digest.clone())),
            (5, Cbor::Text(self.destination_generation_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            export_record_id: c.try_get(1)?.try_text()?,
            output_artifact_id: c.try_get(2)?.try_text()?,
            probe_summary: ExportProbeSummary::from_cbor(c.try_get(3)?)?,
            output_probe_digest: c.try_get(4)?.try_text()?,
            destination_generation_id: c.try_get(5)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportJobEvent {
    pub contract_version: i64,
    pub context: ExportJobContext,
    pub job_id: String,
    pub event_sequence: i64,
    pub first_available_sequence: i64,
    pub kind: ExportEventKind,
    pub state: Option<ExportJobState>,
    pub progress: Option<Rational>,
    pub result: Option<ExportResult>,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportJobEvent {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, self.context.to_cbor()),
            (3, Cbor::Text(self.job_id.clone())),
            (4, Cbor::Int(self.event_sequence)),
            (5, Cbor::Int(self.first_available_sequence)),
            (6, Cbor::Int(self.kind.wire())),
            (7, match &self.state { Some(v) => Cbor::Int(v.wire()), None => Cbor::Null }),
            (8, match &self.progress { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (9, match &self.result { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (10, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            context: ExportJobContext::from_cbor(c.try_get(2)?)?,
            job_id: c.try_get(3)?.try_text()?,
            event_sequence: c.try_get(4)?.try_int()?,
            first_available_sequence: c.try_get(5)?.try_int()?,
            kind: ExportEventKind::from_wire(c.try_get(6)?.try_int()?)?,
            state: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(ExportJobState::from_wire(v.try_int()?)?) } },
            progress: { let v = c.try_get(8)?; if v.is_null() { None } else { Some(Rational::from_cbor(v)?) } },
            result: { let v = c.try_get(9)?; if v.is_null() { None } else { Some(ExportResult::from_cbor(v)?) } },
            diagnostic_id: { let v = c.try_get(10)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportStatusQuery {
    pub contract_version: i64,
    pub project_id: String,
    pub job_id: String,
    pub caller_incarnation_id: Option<String>,
    pub local_import_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportStatusQuery {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.job_id.clone())),
            (4, match &self.caller_incarnation_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (5, match &self.local_import_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            job_id: c.try_get(3)?.try_text()?,
            caller_incarnation_id: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            local_import_id: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportStatusSnapshot {
    pub contract_version: i64,
    pub context: ExportJobContext,
    pub job_id: String,
    pub state: ExportJobState,
    pub last_event_sequence: i64,
    pub result: Option<ExportResult>,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportStatusSnapshot {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, self.context.to_cbor()),
            (3, Cbor::Text(self.job_id.clone())),
            (4, Cbor::Int(self.state.wire())),
            (5, Cbor::Int(self.last_event_sequence)),
            (6, match &self.result { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (7, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            context: ExportJobContext::from_cbor(c.try_get(2)?)?,
            job_id: c.try_get(3)?.try_text()?,
            state: ExportJobState::from_wire(c.try_get(4)?.try_int()?)?,
            last_event_sequence: c.try_get(5)?.try_int()?,
            result: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(ExportResult::from_cbor(v)?) } },
            diagnostic_id: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportLookupQuery {
    pub contract_version: i64,
    pub project_id: String,
    pub request_id: String,
    pub caller_incarnation_id: Option<String>,
    pub local_import_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportLookupQuery {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.request_id.clone())),
            (4, match &self.caller_incarnation_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (5, match &self.local_import_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            request_id: c.try_get(3)?.try_text()?,
            caller_incarnation_id: { let v = c.try_get(4)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            local_import_id: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportLookupResult {
    pub contract_version: i64,
    pub project_id: String,
    pub request_id: String,
    pub status: ExportLookupStatus,
    pub snapshot: Option<ExportStatusSnapshot>,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportLookupResult {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.request_id.clone())),
            (4, Cbor::Int(self.status.wire())),
            (5, match &self.snapshot { Some(v) => v.to_cbor(), None => Cbor::Null }),
            (6, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            request_id: c.try_get(3)?.try_text()?,
            status: ExportLookupStatus::from_wire(c.try_get(4)?.try_int()?)?,
            snapshot: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(ExportStatusSnapshot::from_cbor(v)?) } },
            diagnostic_id: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CancelExportJob {
    pub contract_version: i64,
    pub project_id: String,
    pub job_id: String,
    pub command_id: String,
    pub caller_incarnation_id: Option<String>,
    pub local_import_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl CancelExportJob {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.job_id.clone())),
            (4, Cbor::Text(self.command_id.clone())),
            (5, match &self.caller_incarnation_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (6, match &self.local_import_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            job_id: c.try_get(3)?.try_text()?,
            command_id: c.try_get(4)?.try_text()?,
            caller_incarnation_id: { let v = c.try_get(5)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            local_import_id: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ExportCancelAck {
    pub contract_version: i64,
    pub project_id: String,
    pub job_id: String,
    pub command_id: String,
    pub status: ExportCancelStatus,
    pub terminal_state: Option<ExportJobState>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ExportCancelAck {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.job_id.clone())),
            (4, Cbor::Text(self.command_id.clone())),
            (5, Cbor::Int(self.status.wire())),
            (6, match &self.terminal_state { Some(v) => Cbor::Int(v.wire()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            job_id: c.try_get(3)?.try_text()?,
            command_id: c.try_get(4)?.try_text()?,
            status: ExportCancelStatus::from_wire(c.try_get(5)?.try_int()?)?,
            terminal_state: { let v = c.try_get(6)?; if v.is_null() { None } else { Some(ExportJobState::from_wire(v.try_int()?)?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ProjectOpen {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: Option<String>,
    pub authority_incarnation_id: String,
    pub state: AuthorityState,
    pub graph_revision: i64,
    pub binding_set_id: Option<String>,
    pub diagnostic_id: Option<String>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl ProjectOpen {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, match &self.graph_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Int(self.state.wire())),
            (6, Cbor::Int(self.graph_revision)),
            (7, match &self.binding_set_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (8, match &self.diagnostic_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: { let v = c.try_get(3)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            state: AuthorityState::from_wire(c.try_get(5)?.try_int()?)?,
            graph_revision: c.try_get(6)?.try_int()?,
            binding_set_id: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            diagnostic_id: { let v = c.try_get(8)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AcquireWriter {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub authority_incarnation_id: String,
    pub request_id: String,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl AcquireWriter {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Text(self.request_id.clone())),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            request_id: c.try_get(5)?.try_text()?,
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct WriterLease {
    pub contract_version: i64,
    pub project_id: String,
    pub graph_id: String,
    pub authority_incarnation_id: String,
    pub request_id: String,
    pub status: LeaseStatus,
    pub lease_id: Option<String>,
    pub writer_capability: Option<String>,
    pub expires_at_unix_ms: Option<i64>,
    pub wire_residual: Vec<(i64, Cbor)>,
}
impl WriterLease {
    pub const MAX_DEPTH: usize = 16;
    pub const MAX_ENCODED_LEN: Option<usize> = Some(16777216);
    pub fn to_cbor(&self) -> Cbor {
        let mut m = vec![
            (1, Cbor::Int(self.contract_version)),
            (2, Cbor::Text(self.project_id.clone())),
            (3, Cbor::Text(self.graph_id.clone())),
            (4, Cbor::Text(self.authority_incarnation_id.clone())),
            (5, Cbor::Text(self.request_id.clone())),
            (6, Cbor::Int(self.status.wire())),
            (7, match &self.lease_id { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (8, match &self.writer_capability { Some(v) => Cbor::Text(v.clone()), None => Cbor::Null }),
            (9, match &self.expires_at_unix_ms { Some(v) => Cbor::Int(*v), None => Cbor::Null }),
        ];
        for (t, v) in &self.wire_residual { m.push((*t, v.clone())); }
        Cbor::Map(m)
    }
    pub fn from_cbor(c: &Cbor) -> Result<Self, DecodeError> {
        Ok(Self {
            contract_version: c.try_get(1)?.try_int()?,
            project_id: c.try_get(2)?.try_text()?,
            graph_id: c.try_get(3)?.try_text()?,
            authority_incarnation_id: c.try_get(4)?.try_text()?,
            request_id: c.try_get(5)?.try_text()?,
            status: LeaseStatus::from_wire(c.try_get(6)?.try_int()?)?,
            lease_id: { let v = c.try_get(7)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            writer_capability: { let v = c.try_get(8)?; if v.is_null() { None } else { Some(v.try_text()?) } },
            expires_at_unix_ms: { let v = c.try_get(9)?; if v.is_null() { None } else { Some(v.try_int()?) } },
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
