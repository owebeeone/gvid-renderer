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
    stale = 2
    invalid = 3
    unauthorized = 4

class GraphEventKind(Enum):
    ready = 1
    change = 2
    heartbeat = 3
    recovery_required = 4

class PlacementMode(Enum):
    ripple_track = 1
    overwrite_track = 2

class EditorStatus(Enum):
    accepted = 1
    stale_graph = 2
    stale_binding = 3
    invalid_range = 4
    locked_track = 5
    unsupported_ripple_scope = 6
    unauthorized = 7
    invalid_command = 8
    unsupported_schema = 9
    recovery_required = 10

class BindingStatus(Enum):
    accepted = 1
    stale_graph = 2
    stale_binding = 3
    incompatible_media = 4
    invalid_policy = 5
    unauthorized = 6
    invalid_command = 7

class BindingEventKind(Enum):
    ready = 1
    change = 2
    heartbeat = 3
    recovery_required = 4

class CatalogEventKind(Enum):
    ready = 1
    change = 2
    heartbeat = 3
    recovery_required = 4

class AssetAvailability(Enum):
    online = 1
    missing = 2
    changed = 3
    offline = 4

class IndexStatus(Enum):
    pending = 1
    ready = 2
    failed = 3

class FrameLookupStatus(Enum):
    ready = 1
    pending = 2
    out_of_range = 3
    unavailable = 4
    stale_version = 5
    failed = 6

class DurationPolicy(Enum):
    exact = 1
    trim = 2
    pad = 3

class FrameRatePolicy(Enum):
    source_pts = 1
    conform = 2

class AspectPolicy(Enum):
    reject = 1
    fit = 2
    crop = 3

class ChannelPolicy(Enum):
    reject = 1
    map = 2

class MissingRangePolicy(Enum):
    reject = 1
    gap = 2

class PreviewFidelity(Enum):
    exact = 1
    proxy = 2

class PreviewStatus(Enum):
    ready = 1
    cancelled = 2
    stale_graph = 3
    stale_binding = 4
    unsupported = 5
    failed = 6
    stale_asset = 7

class AuthorityState(Enum):
    ready = 1
    read_only_future_version = 2
    recovery_required = 3
    closed = 4

class LeaseStatus(Enum):
    granted = 1
    held_by_other = 2
    stale_incarnation = 3
    read_only = 4
    recovery_required = 5

class ResourceActionStatus(Enum):
    accepted = 1
    unknown = 2
    expired = 3
    unauthorized = 4

class ExportProvenance(Enum):
    governed_host = 1
    standalone_import = 2

class ExportSubmitStatus(Enum):
    accepted = 1
    stale_context = 2
    invalid = 3
    unauthorized = 4
    unsupported = 5
    capacity = 6
    recovery_required = 7

class ExportJobState(Enum):
    queued = 1
    preparing = 2
    running = 3
    verifying = 4
    succeeded = 5
    failed = 6
    cancelled = 7
    interrupted = 8

class ExportEventKind(Enum):
    ready = 1
    state_change = 2
    progress = 3
    warning = 4
    heartbeat = 5
    recovery_required = 6

class ExportCancelStatus(Enum):
    accepted = 1
    already_terminal = 2
    unknown = 3
    unauthorized = 4
    stale_context = 5

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
class GraphSnapshotDelivery:
    snapshot: GraphSnapshot
    authority_incarnation_id: str

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
class ApplyGraphBatch:
    batch: EditBatch
    authority_incarnation_id: str
    writer_capability: str

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
    project_id: str
    graph_id: str
    authority_incarnation_id: str
    replayed: bool

@dataclass(slots=True)
class AcceptedChange:
    graph_id: str
    command_id: str
    from_revision: int
    to_revision: int
    operations: list[EditOperation]
    footprint: ChangeFootprint
    project_id: str

@dataclass(slots=True)
class GraphChangeEvent:
    kind: GraphEventKind
    project_id: str
    graph_id: str
    authority_incarnation_id: str
    revision: int
    first_available_revision: int
    change: AcceptedChange | None
    reason: str | None

@dataclass(slots=True)
class InsertSourceSpan:
    contract_version: int
    project_id: str
    graph_id: str
    sequence_id: str
    authority_incarnation_id: str
    command_id: str
    expected_revision: int
    target_track_id: str
    timeline_at: Rational
    placement: PlacementMode
    slot_id: str | None
    stream_id: str
    source_range: TimeRange
    speed: Rational
    expected_binding_set_id: str
    undo_group_id: str
    writer_capability: str
    registered_asset_id: str | None
    asset_version_id: str | None

@dataclass(slots=True)
class HistoryIntent:
    contract_version: int
    project_id: str
    graph_id: str
    sequence_id: str
    authority_incarnation_id: str
    command_id: str
    expected_revision: int
    target_undo_group_id: str
    expected_binding_set_id: str
    writer_capability: str

@dataclass(slots=True)
class UndoGroupSummary:
    group_id: str
    sequence_id: str
    label: str
    command_id: str

@dataclass(slots=True)
class HistoryState:
    contract_version: int
    project_id: str
    graph_id: str
    sequence_id: str
    authority_incarnation_id: str
    revision: int
    undo_groups: list[UndoGroupSummary]
    redo_groups: list[UndoGroupSummary]
    can_undo: bool
    can_redo: bool

@dataclass(slots=True)
class EditorAck:
    contract_version: int
    project_id: str
    graph_id: str
    sequence_id: str
    authority_incarnation_id: str
    command_id: str
    status: EditorStatus
    replayed: bool
    revision: int
    binding_set_id: str
    footprint: ChangeFootprint | None
    diagnostic_id: str | None
    binding_revision: int
    created_slot_id: str | None

@dataclass(slots=True)
class CatalogStream:
    id: str
    media: MediaKind
    duration: Rational
    pts_origin: Rational
    frame_count: int | None
    frame_index_id: str | None
    index_status: IndexStatus

@dataclass(slots=True)
class RegisteredAsset:
    id: str
    version_id: str
    content_fingerprint: str
    display_name: str
    streams: dict[str, CatalogStream]
    availability: AssetAvailability
    diagnostic_id: str | None

@dataclass(slots=True)
class CatalogSnapshot:
    contract_version: int
    project_id: str
    authority_incarnation_id: str
    catalog_revision: int
    assets: dict[str, RegisteredAsset]

@dataclass(slots=True)
class CatalogChange:
    project_id: str
    from_catalog_revision: int
    to_catalog_revision: int
    upserts: dict[str, RegisteredAsset]
    removed_asset_ids: list[str]

@dataclass(slots=True)
class CatalogChangeEvent:
    kind: CatalogEventKind
    project_id: str
    authority_incarnation_id: str
    catalog_revision: int
    first_available_revision: int
    change: CatalogChange | None
    reason: str | None

@dataclass(slots=True)
class FrameIndexQuery:
    contract_version: int
    project_id: str
    authority_incarnation_id: str
    registered_asset_id: str
    asset_version_id: str
    stream_id: str
    frame_index: int

@dataclass(slots=True)
class FrameIndexResult:
    contract_version: int
    project_id: str
    authority_incarnation_id: str
    registered_asset_id: str
    asset_version_id: str
    stream_id: str
    frame_index: int
    status: FrameLookupStatus
    pts: Rational | None
    frame_count: int | None
    content_fingerprint: str | None
    diagnostic_id: str | None

@dataclass(slots=True)
class AssetBinding:
    slot_id: str
    registered_asset_id: str
    asset_version_id: str
    stream_id: str
    content_fingerprint: str
    media: MediaKind
    duration: Rational

@dataclass(slots=True)
class BindingSnapshot:
    contract_version: int
    project_id: str
    graph_id: str
    authority_incarnation_id: str
    binding_set_id: str
    binding_revision: int
    bindings: dict[str, AssetBinding]

@dataclass(slots=True)
class BindingPolicy:
    duration: DurationPolicy
    frame_rate: FrameRatePolicy
    aspect: AspectPolicy
    channels: ChannelPolicy
    missing_range: MissingRangePolicy

@dataclass(slots=True)
class RebindSlot:
    contract_version: int
    project_id: str
    graph_id: str
    authority_incarnation_id: str
    command_id: str
    expected_graph_revision: int
    expected_binding_set_id: str
    slot_id: str
    registered_asset_id: str
    asset_version_id: str
    stream_id: str
    policy: BindingPolicy
    writer_capability: str

@dataclass(slots=True)
class BindingAck:
    contract_version: int
    project_id: str
    graph_id: str
    authority_incarnation_id: str
    command_id: str
    status: BindingStatus
    replayed: bool
    graph_revision: int
    binding_set_id: str
    binding_revision: int
    footprint: ChangeFootprint | None
    diagnostic_id: str | None

@dataclass(slots=True)
class BindingChange:
    project_id: str
    graph_id: str
    from_binding_revision: int
    to_binding_revision: int
    from_binding_set_id: str
    to_binding_set_id: str
    changed_slot_ids: list[str]
    footprint: ChangeFootprint

@dataclass(slots=True)
class BindingChangeEvent:
    kind: BindingEventKind
    project_id: str
    graph_id: str
    authority_incarnation_id: str
    binding_revision: int
    first_available_revision: int
    change: BindingChange | None
    reason: str | None

@dataclass(slots=True)
class SourcePreviewRequest:
    contract_version: int
    project_id: str
    authority_incarnation_id: str
    registered_asset_id: str
    asset_version_id: str
    stream_id: str
    request_id: str
    viewer_id: str
    cancel_group_id: str
    at: Rational
    fidelity: PreviewFidelity
    max_edge_px: int
    expected_content_fingerprint: str

@dataclass(slots=True)
class SequencePreviewRequest:
    contract_version: int
    project_id: str
    graph_id: str
    sequence_id: str
    authority_incarnation_id: str
    accepted_revision: int
    binding_set_id: str
    binding_revision: int
    request_id: str
    viewer_id: str
    cancel_group_id: str
    at: Rational
    fidelity: PreviewFidelity
    max_edge_px: int

@dataclass(slots=True)
class ResourceDescriptor:
    resource_id: str
    lease_id: str
    mime_type: str
    width: int
    height: int
    byte_length: int
    expires_at_unix_ms: int

@dataclass(slots=True)
class SourcePreviewResult:
    contract_version: int
    project_id: str
    authority_incarnation_id: str
    registered_asset_id: str
    asset_version_id: str
    stream_id: str
    request_id: str
    viewer_id: str
    cancel_group_id: str
    status: PreviewStatus
    fidelity: PreviewFidelity
    actual_time: Rational | None
    resource: ResourceDescriptor | None
    error_code: str | None
    diagnostic_id: str | None
    content_fingerprint: str

@dataclass(slots=True)
class SequencePreviewResult:
    contract_version: int
    project_id: str
    graph_id: str
    sequence_id: str
    authority_incarnation_id: str
    accepted_revision: int
    binding_set_id: str
    binding_revision: int
    request_id: str
    viewer_id: str
    cancel_group_id: str
    status: PreviewStatus
    fidelity: PreviewFidelity
    actual_time: Rational | None
    plan_id: str | None
    resource: ResourceDescriptor | None
    error_code: str | None
    diagnostic_id: str | None

@dataclass(slots=True)
class CancelPreview:
    contract_version: int
    project_id: str
    graph_id: str | None
    authority_incarnation_id: str
    request_id: str
    viewer_id: str
    cancel_group_id: str

@dataclass(slots=True)
class ReleaseResource:
    contract_version: int
    project_id: str
    graph_id: str | None
    authority_incarnation_id: str
    lease_id: str
    viewer_id: str

@dataclass(slots=True)
class ResourceActionAck:
    contract_version: int
    project_id: str
    graph_id: str | None
    authority_incarnation_id: str
    action_id: str
    status: ResourceActionStatus

@dataclass(slots=True)
class ExportJobContext:
    request_id: str
    provenance: ExportProvenance
    project_id: str
    graph_id: str
    sequence_id: str
    authority_incarnation_id: str | None
    local_import_id: str | None
    accepted_revision: int
    binding_set_id: str
    binding_revision: int
    profile_id: str
    profile_version: int
    profile_digest: str

@dataclass(slots=True)
class ExportJobRequest:
    contract_version: int
    context: ExportJobContext
    range: TimeRange
    destination_ref: str
    allow_software_fallback: bool

@dataclass(slots=True)
class ExportJobAck:
    contract_version: int
    context: ExportJobContext
    status: ExportSubmitStatus
    job_id: str | None
    replayed: bool
    diagnostic_id: str | None

@dataclass(slots=True)
class ExportProbeSummary:
    video_stream_count: int
    audio_stream_count: int
    duration: Rational
    width: int | None
    height: int | None
    audio_sample_rate: int | None
    audio_channel_layout: str | None

@dataclass(slots=True)
class ExportResult:
    export_record_id: str
    output_artifact_id: str
    probe_summary: ExportProbeSummary
    output_probe_digest: str

@dataclass(slots=True)
class ExportJobEvent:
    contract_version: int
    context: ExportJobContext
    job_id: str
    event_sequence: int
    first_available_sequence: int
    kind: ExportEventKind
    state: ExportJobState | None
    progress: Rational | None
    result: ExportResult | None
    diagnostic_id: str | None

@dataclass(slots=True)
class ExportStatusQuery:
    contract_version: int
    project_id: str
    job_id: str
    caller_incarnation_id: str | None
    local_import_id: str | None

@dataclass(slots=True)
class ExportStatusSnapshot:
    contract_version: int
    context: ExportJobContext
    job_id: str
    state: ExportJobState
    last_event_sequence: int
    result: ExportResult | None
    diagnostic_id: str | None

@dataclass(slots=True)
class CancelExportJob:
    contract_version: int
    project_id: str
    job_id: str
    command_id: str
    caller_incarnation_id: str | None
    local_import_id: str | None

@dataclass(slots=True)
class ExportCancelAck:
    contract_version: int
    project_id: str
    job_id: str
    command_id: str
    status: ExportCancelStatus
    terminal_state: ExportJobState | None

@dataclass(slots=True)
class ProjectOpen:
    contract_version: int
    project_id: str
    graph_id: str | None
    authority_incarnation_id: str
    state: AuthorityState
    graph_revision: int
    binding_set_id: str | None
    diagnostic_id: str | None

@dataclass(slots=True)
class AcquireWriter:
    contract_version: int
    project_id: str
    graph_id: str
    authority_incarnation_id: str
    request_id: str

@dataclass(slots=True)
class WriterLease:
    contract_version: int
    project_id: str
    graph_id: str
    authority_incarnation_id: str
    request_id: str
    status: LeaseStatus
    lease_id: str | None
    writer_capability: str | None
    expires_at_unix_ms: int | None
