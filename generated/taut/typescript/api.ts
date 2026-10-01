// GENERATED native TypeScript types — do not edit.

export type MediaKind = "video" | "audio" | "text";
export type NodeKind = "source" | "effect" | "composite" | "transition" | "title" | "output" | "silence";
export type ParamKind = "integer" | "rational" | "boolean" | "text";
export type EditKind = "put_node" | "remove_node" | "put_edge" | "remove_edge" | "put_track" | "remove_track" | "put_sequence" | "remove_sequence" | "put_slot" | "remove_slot";
export type AckStatus = "accepted" | "stale" | "invalid" | "unauthorized";
export type GraphEventKind = "ready" | "change" | "heartbeat" | "recovery_required";
export type PlacementMode = "ripple_track" | "overwrite_track";
export type EditorStatus = "accepted" | "stale_graph" | "stale_binding" | "invalid_range" | "locked_track" | "unsupported_ripple_scope" | "unauthorized" | "invalid_command" | "unsupported_schema" | "recovery_required";
export type BindingStatus = "accepted" | "stale_graph" | "stale_binding" | "incompatible_media" | "invalid_policy" | "unauthorized" | "invalid_command";
export type BindingEventKind = "ready" | "change" | "heartbeat" | "recovery_required";
export type CatalogEventKind = "ready" | "change" | "heartbeat" | "recovery_required";
export type AssetAvailability = "online" | "missing" | "changed" | "offline";
export type IndexStatus = "pending" | "ready" | "failed";
export type FrameLookupStatus = "ready" | "pending" | "out_of_range" | "unavailable" | "stale_version" | "failed";
export type DurationPolicy = "exact" | "trim" | "pad";
export type FrameRatePolicy = "source_pts" | "conform";
export type AspectPolicy = "reject" | "fit" | "crop";
export type ChannelPolicy = "reject" | "map";
export type MissingRangePolicy = "reject" | "gap";
export type PreviewFidelity = "exact" | "proxy";
export type PreviewStatus = "ready" | "cancelled" | "stale_graph" | "stale_binding" | "unsupported" | "failed" | "stale_asset";
export type AuthorityState = "ready" | "read_only_future_version" | "recovery_required" | "closed";
export type LeaseStatus = "granted" | "held_by_other" | "stale_incarnation" | "read_only" | "recovery_required";
export type ResourceActionStatus = "accepted" | "unknown" | "expired" | "unauthorized";
export type ExportProvenance = "governed_host" | "standalone_import";
export type ExportSubmitStatus = "accepted" | "stale_context" | "invalid" | "unauthorized" | "unsupported" | "capacity" | "recovery_required" | "destination_busy" | "idempotency_conflict" | "retired_request";
export type ExportDestinationPolicy = "fail_if_exists" | "replace_existing";
export type ExportLookupStatus = "found" | "unavailable" | "stale_context" | "retired_request";
export type ExportStatusResultStatus = "found" | "unavailable" | "stale_context";
export type ExportEventDeliveryStatus = "event" | "unavailable" | "stale_context";
export type ExportJobState = "queued" | "preparing" | "running" | "verifying" | "succeeded" | "failed" | "cancelled" | "interrupted";
export type ExportEventKind = "ready" | "state_change" | "progress" | "warning" | "heartbeat" | "recovery_required";
export type ExportCancelStatus = "accepted" | "already_terminal" | "stale_context" | "unavailable";

export interface Rational {
  numerator: bigint;
  denominator: bigint;
}

export interface TimeRange {
  start: Rational;
  end: Rational;
}

export interface ParamValue {
  kind: ParamKind;
  integer: bigint | null;
  rational: Rational | null;
  boolean: boolean | null;
  text: string | null;
}

export interface SourcePayload {
  slot_id: string;
  stream_id: string;
  source_range: TimeRange;
  speed: Rational;
}

export interface EffectPayload {
  effect_id: string;
  effect_version: bigint;
  params: Map<string, ParamValue>;
}

export interface CompositePayload {
  blend_id: string;
}

export interface TransitionPayload {
  transition_id: string;
  transition_version: bigint;
  params: Map<string, ParamValue>;
}

export interface TitlePayload {
  text: string;
  style_id: string;
}

export interface OutputPayload {
  output_media: MediaKind;
}

export interface Node {
  id: string;
  kind: NodeKind;
  media: MediaKind;
  sequence_id: string;
  track_id: string | null;
  timeline_range: TimeRange | null;
  source: SourcePayload | null;
  effect: EffectPayload | null;
  composite: CompositePayload | null;
  transition: TransitionPayload | null;
  title: TitlePayload | null;
  output: OutputPayload | null;
}

export interface Edge {
  id: string;
  from_node: string;
  from_port: string;
  to_node: string;
  to_port: string;
  order_key: string | null;
}

export interface Track {
  id: string;
  sequence_id: string;
  media: MediaKind;
  order_key: string;
  enabled: boolean;
}

export interface Sequence {
  id: string;
  video_root: string | null;
  audio_root: string | null;
  range: TimeRange;
}

export interface AssetSlot {
  id: string;
  expected_media: MediaKind;
  expected_duration: Rational | null;
  expected_fingerprint: string | null;
}

export interface GraphSnapshot {
  schema_major: bigint;
  schema_minor: bigint;
  graph_id: string;
  project_id: string;
  revision: bigint;
  semantic_version: bigint;
  sequences: Map<string, Sequence>;
  tracks: Map<string, Track>;
  nodes: Map<string, Node>;
  edges: Map<string, Edge>;
  slots: Map<string, AssetSlot>;
}

export interface GraphSemantics {
  schema_major: bigint;
  schema_minor: bigint;
  semantic_version: bigint;
  sequences: Map<string, Sequence>;
  tracks: Map<string, Track>;
  nodes: Map<string, Node>;
  edges: Map<string, Edge>;
  slots: Map<string, AssetSlot>;
}

export interface GraphSnapshotDelivery {
  snapshot: GraphSnapshot;
  authority_incarnation_id: string;
}

export interface EditOperation {
  kind: EditKind;
  target_id: string;
  node: Node | null;
  edge: Edge | null;
  track: Track | null;
  sequence: Sequence | null;
  slot: AssetSlot | null;
}

export interface EditBatch {
  project_id: string;
  graph_id: string;
  command_id: string;
  expected_revision: bigint;
  undo_group_id: string;
  operations: EditOperation[];
}

export interface ApplyGraphBatch {
  batch: EditBatch;
  authority_incarnation_id: string;
  writer_capability: string;
}

export interface AffectedInterval {
  sequence_id: string;
  range: TimeRange;
}

export interface ChangeFootprint {
  intervals: AffectedInterval[];
  node_ids: string[];
  slot_ids: string[];
  full_invalidation: boolean;
}

export interface EditAck {
  command_id: string;
  status: AckStatus;
  revision: bigint;
  semantic_digest: string | null;
  reason: string | null;
  footprint: ChangeFootprint | null;
  project_id: string;
  graph_id: string;
  authority_incarnation_id: string;
  replayed: boolean;
}

export interface AcceptedChange {
  graph_id: string;
  command_id: string;
  from_revision: bigint;
  to_revision: bigint;
  operations: EditOperation[];
  footprint: ChangeFootprint;
  project_id: string;
}

export interface GraphChangeEvent {
  kind: GraphEventKind;
  project_id: string;
  graph_id: string;
  authority_incarnation_id: string;
  revision: bigint;
  first_available_revision: bigint;
  change: AcceptedChange | null;
  reason: string | null;
}

export interface InsertSourceSpan {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  sequence_id: string;
  authority_incarnation_id: string;
  command_id: string;
  expected_revision: bigint;
  target_track_id: string;
  timeline_at: Rational;
  placement: PlacementMode;
  slot_id: string | null;
  stream_id: string;
  source_range: TimeRange;
  speed: Rational;
  expected_binding_set_id: string;
  undo_group_id: string;
  writer_capability: string;
  registered_asset_id: string | null;
  asset_version_id: string | null;
}

export interface HistoryIntent {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  sequence_id: string;
  authority_incarnation_id: string;
  command_id: string;
  expected_revision: bigint;
  target_undo_group_id: string;
  expected_binding_set_id: string;
  writer_capability: string;
}

export interface UndoGroupSummary {
  group_id: string;
  sequence_id: string;
  label: string;
  command_id: string;
}

export interface HistoryState {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  sequence_id: string;
  authority_incarnation_id: string;
  revision: bigint;
  undo_groups: UndoGroupSummary[];
  redo_groups: UndoGroupSummary[];
  can_undo: boolean;
  can_redo: boolean;
}

export interface EditorAck {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  sequence_id: string;
  authority_incarnation_id: string;
  command_id: string;
  status: EditorStatus;
  replayed: boolean;
  revision: bigint;
  binding_set_id: string;
  footprint: ChangeFootprint | null;
  diagnostic_id: string | null;
  binding_revision: bigint;
  created_slot_id: string | null;
}

export interface CatalogStream {
  id: string;
  media: MediaKind;
  duration: Rational;
  pts_origin: Rational;
  frame_count: bigint | null;
  frame_index_id: string | null;
  index_status: IndexStatus;
}

export interface RegisteredAsset {
  id: string;
  version_id: string;
  content_fingerprint: string;
  display_name: string;
  streams: Map<string, CatalogStream>;
  availability: AssetAvailability;
  diagnostic_id: string | null;
}

export interface CatalogSnapshot {
  contract_version: bigint;
  project_id: string;
  authority_incarnation_id: string;
  catalog_revision: bigint;
  assets: Map<string, RegisteredAsset>;
}

export interface CatalogChange {
  project_id: string;
  from_catalog_revision: bigint;
  to_catalog_revision: bigint;
  upserts: Map<string, RegisteredAsset>;
  removed_asset_ids: string[];
}

export interface CatalogChangeEvent {
  kind: CatalogEventKind;
  project_id: string;
  authority_incarnation_id: string;
  catalog_revision: bigint;
  first_available_revision: bigint;
  change: CatalogChange | null;
  reason: string | null;
}

export interface FrameIndexQuery {
  contract_version: bigint;
  project_id: string;
  authority_incarnation_id: string;
  registered_asset_id: string;
  asset_version_id: string;
  stream_id: string;
  frame_index: bigint;
}

export interface FrameIndexResult {
  contract_version: bigint;
  project_id: string;
  authority_incarnation_id: string;
  registered_asset_id: string;
  asset_version_id: string;
  stream_id: string;
  frame_index: bigint;
  status: FrameLookupStatus;
  pts: Rational | null;
  frame_count: bigint | null;
  content_fingerprint: string | null;
  diagnostic_id: string | null;
}

export interface AssetBinding {
  slot_id: string;
  registered_asset_id: string;
  asset_version_id: string;
  stream_id: string;
  content_fingerprint: string;
  media: MediaKind;
  duration: Rational;
}

export interface BindingSnapshot {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  authority_incarnation_id: string;
  binding_set_id: string;
  binding_revision: bigint;
  bindings: Map<string, AssetBinding>;
}

export interface BindingPolicy {
  duration: DurationPolicy;
  frame_rate: FrameRatePolicy;
  aspect: AspectPolicy;
  channels: ChannelPolicy;
  missing_range: MissingRangePolicy;
}

export interface RebindSlot {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  authority_incarnation_id: string;
  command_id: string;
  expected_graph_revision: bigint;
  expected_binding_set_id: string;
  slot_id: string;
  registered_asset_id: string;
  asset_version_id: string;
  stream_id: string;
  policy: BindingPolicy;
  writer_capability: string;
}

export interface BindingAck {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  authority_incarnation_id: string;
  command_id: string;
  status: BindingStatus;
  replayed: boolean;
  graph_revision: bigint;
  binding_set_id: string;
  binding_revision: bigint;
  footprint: ChangeFootprint | null;
  diagnostic_id: string | null;
}

export interface BindingChange {
  project_id: string;
  graph_id: string;
  from_binding_revision: bigint;
  to_binding_revision: bigint;
  from_binding_set_id: string;
  to_binding_set_id: string;
  changed_slot_ids: string[];
  footprint: ChangeFootprint;
}

export interface BindingChangeEvent {
  kind: BindingEventKind;
  project_id: string;
  graph_id: string;
  authority_incarnation_id: string;
  binding_revision: bigint;
  first_available_revision: bigint;
  change: BindingChange | null;
  reason: string | null;
}

export interface SourcePreviewRequest {
  contract_version: bigint;
  project_id: string;
  authority_incarnation_id: string;
  registered_asset_id: string;
  asset_version_id: string;
  stream_id: string;
  request_id: string;
  viewer_id: string;
  cancel_group_id: string;
  at: Rational;
  fidelity: PreviewFidelity;
  max_edge_px: bigint;
  expected_content_fingerprint: string;
}

export interface SequencePreviewRequest {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  sequence_id: string;
  authority_incarnation_id: string;
  accepted_revision: bigint;
  binding_set_id: string;
  binding_revision: bigint;
  request_id: string;
  viewer_id: string;
  cancel_group_id: string;
  at: Rational;
  fidelity: PreviewFidelity;
  max_edge_px: bigint;
}

export interface ResourceDescriptor {
  resource_id: string;
  lease_id: string;
  mime_type: string;
  width: bigint;
  height: bigint;
  byte_length: bigint;
  expires_at_unix_ms: bigint;
}

export interface SourcePreviewResult {
  contract_version: bigint;
  project_id: string;
  authority_incarnation_id: string;
  registered_asset_id: string;
  asset_version_id: string;
  stream_id: string;
  request_id: string;
  viewer_id: string;
  cancel_group_id: string;
  status: PreviewStatus;
  fidelity: PreviewFidelity;
  actual_time: Rational | null;
  resource: ResourceDescriptor | null;
  error_code: string | null;
  diagnostic_id: string | null;
  content_fingerprint: string;
}

export interface SequencePreviewResult {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  sequence_id: string;
  authority_incarnation_id: string;
  accepted_revision: bigint;
  binding_set_id: string;
  binding_revision: bigint;
  request_id: string;
  viewer_id: string;
  cancel_group_id: string;
  status: PreviewStatus;
  fidelity: PreviewFidelity;
  actual_time: Rational | null;
  plan_id: string | null;
  resource: ResourceDescriptor | null;
  error_code: string | null;
  diagnostic_id: string | null;
}

export interface CancelPreview {
  contract_version: bigint;
  project_id: string;
  graph_id: string | null;
  authority_incarnation_id: string;
  request_id: string;
  viewer_id: string;
  cancel_group_id: string;
}

export interface ReleaseResource {
  contract_version: bigint;
  project_id: string;
  graph_id: string | null;
  authority_incarnation_id: string;
  lease_id: string;
  viewer_id: string;
}

export interface ResourceActionAck {
  contract_version: bigint;
  project_id: string;
  graph_id: string | null;
  authority_incarnation_id: string;
  action_id: string;
  status: ResourceActionStatus;
}

export interface ExportJobContext {
  request_id: string;
  provenance: ExportProvenance;
  project_id: string;
  graph_id: string;
  sequence_id: string;
  authority_incarnation_id: string | null;
  local_import_id: string | null;
  accepted_revision: bigint;
  binding_set_id: string;
  binding_revision: bigint;
  profile_id: string;
  profile_version: bigint;
  profile_digest: string;
}

export interface ExportJobRequest {
  contract_version: bigint;
  context: ExportJobContext;
  range: TimeRange;
  destination_ref: string;
  allow_software_fallback: boolean;
  destination_policy: ExportDestinationPolicy;
}

export interface ExportJobAck {
  contract_version: bigint;
  context: ExportJobContext;
  status: ExportSubmitStatus;
  job_id: string | null;
  replayed: boolean;
  diagnostic_id: string | null;
}

export interface ExportProbeSummary {
  video_stream_count: bigint;
  audio_stream_count: bigint;
  duration: Rational;
  width: bigint | null;
  height: bigint | null;
  audio_sample_rate: bigint | null;
  audio_channel_layout: string | null;
}

export interface ExportResult {
  export_record_id: string;
  output_artifact_id: string;
  probe_summary: ExportProbeSummary;
  output_probe_digest: string;
  destination_generation_id: string;
}

export interface ExportJobEvent {
  contract_version: bigint;
  context: ExportJobContext;
  job_id: string;
  event_sequence: bigint;
  first_available_sequence: bigint;
  kind: ExportEventKind;
  state: ExportJobState | null;
  progress: Rational | null;
  result: ExportResult | null;
  diagnostic_id: string | null;
}

export interface ExportEventsQuery {
  contract_version: bigint;
  project_id: string;
  job_id: string;
  after_event_sequence: bigint;
  caller_incarnation_id: string | null;
  local_import_id: string | null;
}

export interface ExportEventDelivery {
  contract_version: bigint;
  project_id: string;
  job_id: string;
  status: ExportEventDeliveryStatus;
  event: ExportJobEvent | null;
  diagnostic_id: string | null;
}

export interface ExportStatusQuery {
  contract_version: bigint;
  project_id: string;
  job_id: string;
  caller_incarnation_id: string | null;
  local_import_id: string | null;
}

export interface ExportStatusSnapshot {
  contract_version: bigint;
  context: ExportJobContext;
  job_id: string;
  state: ExportJobState;
  last_event_sequence: bigint;
  result: ExportResult | null;
  diagnostic_id: string | null;
}

export interface ExportStatusResult {
  contract_version: bigint;
  project_id: string;
  job_id: string;
  status: ExportStatusResultStatus;
  snapshot: ExportStatusSnapshot | null;
  diagnostic_id: string | null;
}

export interface ExportLookupQuery {
  contract_version: bigint;
  project_id: string;
  request_id: string;
  caller_incarnation_id: string | null;
  local_import_id: string | null;
}

export interface ExportLookupResult {
  contract_version: bigint;
  project_id: string;
  request_id: string;
  status: ExportLookupStatus;
  snapshot: ExportStatusSnapshot | null;
  diagnostic_id: string | null;
}

export interface CancelExportJob {
  contract_version: bigint;
  project_id: string;
  job_id: string;
  command_id: string;
  caller_incarnation_id: string | null;
  local_import_id: string | null;
}

export interface ExportCancelAck {
  contract_version: bigint;
  project_id: string;
  job_id: string;
  command_id: string;
  status: ExportCancelStatus;
  terminal_state: ExportJobState | null;
}

export interface ProjectOpen {
  contract_version: bigint;
  project_id: string;
  graph_id: string | null;
  authority_incarnation_id: string;
  state: AuthorityState;
  graph_revision: bigint;
  binding_set_id: string | null;
  diagnostic_id: string | null;
}

export interface AcquireWriter {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  authority_incarnation_id: string;
  request_id: string;
}

export interface WriterLease {
  contract_version: bigint;
  project_id: string;
  graph_id: string;
  authority_incarnation_id: string;
  request_id: string;
  status: LeaseStatus;
  lease_id: string | null;
  writer_capability: string | null;
  expires_at_unix_ms: bigint | null;
}
