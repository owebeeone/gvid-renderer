// GENERATED native TypeScript types — do not edit.

export type MediaKind = "video" | "audio" | "text";
export type NodeKind = "source" | "effect" | "composite" | "transition" | "title" | "output" | "silence";
export type ParamKind = "integer" | "rational" | "boolean" | "text";
export type EditKind = "put_node" | "remove_node" | "put_edge" | "remove_edge" | "put_track" | "remove_track" | "put_sequence" | "remove_sequence" | "put_slot" | "remove_slot";
export type AckStatus = "accepted" | "duplicate" | "stale" | "invalid";

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
}

export interface AcceptedChange {
  graph_id: string;
  command_id: string;
  from_revision: bigint;
  to_revision: bigint;
  operations: EditOperation[];
  footprint: ChangeFootprint;
}
