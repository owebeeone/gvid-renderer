"""GENERATED native Python types — do not edit."""
from __future__ import annotations
from dataclasses import dataclass
from enum import Enum

class MediaKind(Enum):
    video = 1
    audio = 2
    text = 3

class NodeKind(Enum):
    source = 1
    effect = 2
    composite = 3
    transition = 4
    title = 5
    output = 6
    silence = 7

class ParamKind(Enum):
    integer = 1
    rational = 2
    boolean = 3
    text = 4

class EditKind(Enum):
    put_node = 1
    remove_node = 2
    put_edge = 3
    remove_edge = 4
    put_track = 5
    remove_track = 6
    put_sequence = 7
    remove_sequence = 8
    put_slot = 9
    remove_slot = 10

class AckStatus(Enum):
    accepted = 1
    duplicate = 2
    stale = 3
    invalid = 4

@dataclass(slots=True)
class Rational:
    numerator: int
    denominator: int

@dataclass(slots=True)
class TimeRange:
    start: Rational
    end: Rational

@dataclass(slots=True)
class ParamValue:
    kind: ParamKind
    integer: int | None
    rational: Rational | None
    boolean: bool | None
    text: str | None

@dataclass(slots=True)
class SourcePayload:
    slot_id: str
    stream_id: str
    source_range: TimeRange
    speed: Rational

@dataclass(slots=True)
class EffectPayload:
    effect_id: str
    effect_version: int
    params: dict[str, ParamValue]

@dataclass(slots=True)
class CompositePayload:
    blend_id: str

@dataclass(slots=True)
class TransitionPayload:
    transition_id: str
    transition_version: int
    params: dict[str, ParamValue]

@dataclass(slots=True)
class TitlePayload:
    text: str
    style_id: str

@dataclass(slots=True)
class OutputPayload:
    output_media: MediaKind

@dataclass(slots=True)
class Node:
    id: str
    kind: NodeKind
    media: MediaKind
    sequence_id: str
    track_id: str | None
    timeline_range: TimeRange | None
    source: SourcePayload | None
    effect: EffectPayload | None
    composite: CompositePayload | None
    transition: TransitionPayload | None
    title: TitlePayload | None
    output: OutputPayload | None

@dataclass(slots=True)
class Edge:
    id: str
    from_node: str
    from_port: str
    to_node: str
    to_port: str
    order_key: str | None

@dataclass(slots=True)
class Track:
    id: str
    sequence_id: str
    media: MediaKind
    order_key: str
    enabled: bool

@dataclass(slots=True)
class Sequence:
    id: str
    video_root: str | None
    audio_root: str | None
    range: TimeRange

@dataclass(slots=True)
class AssetSlot:
    id: str
    expected_media: MediaKind
    expected_duration: Rational | None
    expected_fingerprint: str | None

@dataclass(slots=True)
class GraphSnapshot:
    schema_major: int
    schema_minor: int
    graph_id: str
    project_id: str
    revision: int
    semantic_version: int
    sequences: dict[str, Sequence]
    tracks: dict[str, Track]
    nodes: dict[str, Node]
    edges: dict[str, Edge]
    slots: dict[str, AssetSlot]

@dataclass(slots=True)
class GraphSemantics:
    schema_major: int
    schema_minor: int
    semantic_version: int
    sequences: dict[str, Sequence]
    tracks: dict[str, Track]
    nodes: dict[str, Node]
    edges: dict[str, Edge]
    slots: dict[str, AssetSlot]

@dataclass(slots=True)
class EditOperation:
    kind: EditKind
    target_id: str
    node: Node | None
    edge: Edge | None
    track: Track | None
    sequence: Sequence | None
    slot: AssetSlot | None

@dataclass(slots=True)
class EditBatch:
    project_id: str
    graph_id: str
    command_id: str
    expected_revision: int
    undo_group_id: str
    operations: list[EditOperation]

@dataclass(slots=True)
class AffectedInterval:
    sequence_id: str
    range: TimeRange

@dataclass(slots=True)
class ChangeFootprint:
    intervals: list[AffectedInterval]
    node_ids: list[str]
    slot_ids: list[str]
    full_invalidation: bool

@dataclass(slots=True)
class EditAck:
    command_id: str
    status: AckStatus
    revision: int
    semantic_digest: str | None
    reason: str | None
    footprint: ChangeFootprint | None

@dataclass(slots=True)
class AcceptedChange:
    graph_id: str
    command_id: str
    from_revision: int
    to_revision: int
    operations: list[EditOperation]
    footprint: ChangeFootprint
