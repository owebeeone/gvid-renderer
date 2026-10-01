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
    Duplicate,
    Stale,
    Invalid,
}
impl AckStatus {
    pub fn wire(self) -> i64 { match self {
        Self::Accepted => 1,
        Self::Duplicate => 2,
        Self::Stale => 3,
        Self::Invalid => 4,
    } }
    pub fn from_wire(v: i64) -> Result<Self, DecodeError> { Ok(match v {
        1 => Self::Accepted,
        2 => Self::Duplicate,
        3 => Self::Stale,
        4 => Self::Invalid,
        _ => return Err(DecodeError::UnknownEnum { enum_name: "AckStatus", value: v }),
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
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
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
            wire_residual: c.map_entries().iter().filter(|(t, _)| !matches!(*t, 1 | 2 | 3 | 4 | 5 | 6)).map(|(t, v)| (*t, v.clone())).collect(),
        })
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        Self::from_cbor(&crate::cbor::try_decode_with(bytes, Self::MAX_DEPTH, Self::MAX_ENCODED_LEN)?)
    }
}
