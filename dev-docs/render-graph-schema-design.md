# GVid flat render graph and edit protocol (draft v0.1)

**Status:** Interim design intended to be durable after measurement and review. The executable Taut draft is [`../ir/gvid_render_graph.taut.py`](../ir/gvid_render_graph.taut.py). This is a project/editor contract, not the backend render IR.

## Purpose, ownership, and traceability

The project service is the single writer of the accepted graph. The browser keeps a small mutable JSON-shaped projection for interaction. The renderer reads immutable accepted revisions, creates a derived plan for a requested interval or export, and never writes editorial state. Encoded video/audio bytes are opaque and travel over the authenticated HTTP range endpoint; graph, edits, descriptors, and results use Taut. This design addresses GVR-RDR-023–039 and 056–062, GVR-REND-015–016, GVR-EDIT-005–011, and the architecture's composition/render-plan boundary. Preview scheduling and render execution remain with their respective designs.

## Representation decision for the draft

Use Taut `GraphSnapshot`, `EditBatch`, `EditAck`, and `AcceptedChange` on every component boundary. Persist an accepted snapshot as deterministic Taut CBOR plus an append-only accepted-command journal. A share package may include a readable `graph.json` projection and a separate asset-binding manifest. Importing JSON always projects through the versioned Taut schema and full graph validator before acceptance. Taut CBOR, rather than JSON text bytes, defines the snapshot digest. The document is still a candidate for a final saved format under GVR-RDR-061: measure size, decode cost, migration, and browser/native parity before freezing it.

The in-memory object uses maps keyed by stable IDs (`nodes`, `edges`, `tracks`, `sequences`, `slots`). Editing a node is a map assignment; removing it and its incident edges is an atomic batch. The editor may mutate this object in place for tentative display. Accepted revisions are isolated by cloning or copy-on-write at the project boundary, so an in-flight render never observes further mutation. A full snapshot is small for ordinary projects because it contains metadata and parameters, not media or cached frames. Large projects can exchange accepted changes and periodically compact the journal into a snapshot.

Taut currently offers scalar types, enums, messages, lists, maps, field presence, services, and delivery shapes. It has no `oneof` and no canonical JSON profile. `Node.kind` plus optional typed payload fields are an application-level discriminated union: exactly the matching payload is non-null, except `silence`, which has none. `ParamValue.kind` similarly selects exactly one typed value. The validator enforces those facts; the Taut codec alone does not. Unknown required node kinds, effects, parameters, or schema major versions fail closed. We use no opaque JSON or bytes value as an escape hatch.

## Graph invariants

| Element | Rule |
| --- | --- |
| Header | `graph_id`, `project_id`, nonnegative monotonic `revision`, schema major/minor, and semantic-registry version identify the snapshot. `revision` is history identity; semantic digest is content identity. |
| IDs | Stable opaque collision-resistant strings, unique across entity maps; the map key equals the contained `id`. IDs are never deliberately recycled in a project history. Display names live outside semantic IDs. |
| Sequences | Each has an exact half-open timeline range and at least one root. A video root accepts video; an audio root accepts audio. Multiple sequences remain flat in the same maps. No nested-sequence node exists in v0.1. |
| Tracks | Each belongs to one sequence and has a media kind, `enabled`, and sortable `order_key`. Keys are unique within a sequence/media pair. Reordering changes keys, not IDs. Track order determines visual stack or audio mix order. |
| Nodes | A node belongs to one sequence. Source and title nodes have a `track_id` and timeline range. Processing nodes use explicit input/output edges; their active range is computed from inputs unless a validated timeline range limits them. A node has one `kind` and exactly its matching payload. |
| Edges | Each edge has a stable ID, source node/port, destination node/port, and optional order key. Ports have declared media types and cardinalities in the node/effect registry. `composite` inputs need unique order keys; single-input ports reject duplicates. No cross-sequence edges or cycles. |
| Asset slots | Source nodes refer to logical `slot_id` plus selected `stream_id`. The graph contains expected media/duration/fingerprint hints, never machine paths or credentials. A binding set outside the graph maps slots to registered asset versions and authorized local resources. |
| Time | `Rational` is signed numerator/positive denominator, reduced with positive denominator. Ranges are `[start,end)` and require `start < end`. Timeline and source ranges are distinct fields. Speed is positive rational in v0.1. No floating-point seconds enter canonical state. |

In the draft, numerator and denominator must fit both signed 64-bit and JavaScript's exact integer range (absolute value at most `2^53-1`); arithmetic uses checked integer or BigInt intermediates before reducing. Denominator has a further product limit of `10^9`. Rates, source PTS origin, rounding to output frames/samples, reverse speed, and VFR mapping must be settled with `gvid-time-model-design.md` before implementation. `Rational` is in seconds; source times are stream presentation times relative to an explicitly registered stream origin, not decode order or UI timecode.

`source` nodes select a bound stream, source range, speed, and timeline placement. Source duration under speed must equal the declared timeline duration unless a named policy permits pad/trim; v0.1 rejects mismatch. `effect` and `transition` payloads name a versioned registry entry and carry typed parameter values. The registry defines units, ranges, port signatures, handles, preview fidelity, and migration. `composite` is an ordered blend for video or ordered mix for audio; `blend_id` is registry-governed. `title` is text plus a versioned style ID; its style contract must avoid machine font drift. `output` is a sequence root. `silence` has no payload and is only valid for audio. Every node reachable from a root is semantically relevant; unreachable nodes are rejected on save rather than silently ignored.

## Example of the compact editor projection

This JSON is illustrative editor state. It omits null optional fields and empty maps; the Taut JSON adapter may emit a different complete projection. IDs and `order_key` values are strings, so insertion does not shift an array of nodes. The one edge connects the source to the sequence root.

```json
{
  "schema_major": 0, "schema_minor": 1, "graph_id": "g1", "project_id": "p1",
  "revision": 12, "semantic_version": 1,
  "sequences": {"s1": {"id": "s1", "video_root": "out-v", "range": {"start": {"numerator": 0, "denominator": 1}, "end": {"numerator": 10, "denominator": 1}}}},
  "tracks": {"v1": {"id": "v1", "sequence_id": "s1", "media": "video", "order_key": "m", "enabled": true}},
  "slots": {"camera-a": {"id": "camera-a", "expected_media": "video"}},
  "nodes": {
    "clip-1": {"id": "clip-1", "kind": "source", "media": "video", "sequence_id": "s1", "track_id": "v1", "timeline_range": {"start": {"numerator": 0, "denominator": 1}, "end": {"numerator": 10, "denominator": 1}}, "source": {"slot_id": "camera-a", "stream_id": "video-0", "source_range": {"start": {"numerator": 0, "denominator": 1}, "end": {"numerator": 10, "denominator": 1}}, "speed": {"numerator": 1, "denominator": 1}}},
    "out-v": {"id": "out-v", "kind": "output", "media": "video", "sequence_id": "s1", "output": {"output_media": "video"}}
  },
  "edges": {"edge-1": {"id": "edge-1", "from_node": "clip-1", "from_port": "video", "to_node": "out-v", "to_port": "input"}}
}
```

The sample contains one video sequence; absent audio root is intentional. An import fills all optional Taut fields with null and empty maps with `{}` before encoding. Canonicalization reduces rationals and validates unique order keys and ID equality; Taut sorts map entries on its deterministic wire. The digest is the hash of a `GraphSemantics` Taut message projected from the accepted snapshot. It includes graph schema and semantic versions, all nodes and edges, tracks, sequences, and asset-slot expectations; it excludes graph/project/revision identity, local asset bindings, UI selection, caches, and command history. A render cache key adds binding fingerprints, toolchain/effect versions, profile, fidelity, and requested range. History can have two revisions with one semantic digest (e.g. an undo).

## Typed edits, revision application, and undo

`EditBatch` is one user-intent transaction: `command_id`, `expected_revision`, `undo_group_id`, and ordered `EditOperation`s. Operations are `put`/`remove` for a keyed record. A `put` creates or replaces the full typed record with the same target ID; a `remove` carries no payload. Exactly one payload is present for a `put`, matching its kind; all are null for `remove`. Full-record replacement avoids an untyped field-path patch language and makes changed-field validation explicit. A gesture can coalesce many tentative states into one committed batch. Insertion uses new node IDs and edges; removal must include incident edge removals in the same batch or validation rejects it. Reordering uses `put_track` or ordered-edge replacements with new `order_key`s.

The project authority checks size and nesting limits before decoding, then command identity, expected revision, authorization, typed payloads, registry versions, and the entire candidate graph. It applies all operations to a candidate copy, validates the result, durably records the accepted batch and resulting revision, then publishes `AcceptedChange` and `EditAck`. Failure leaves graph and journal unchanged. A duplicate `command_id` with identical bytes returns the original acknowledgement; reuse with different bytes is invalid. A stale expected revision is rejected with current revision and a refresh path. No automatic merge/CRDT behavior is claimed. The authority generates collision-resistant 128-bit entity IDs and checks against live IDs and the retained journal before acceptance. An imported standalone snapshot without its journal starts a new graph identity/history; it cannot prove whether any previously deleted ID was used. The allocator must not intentionally reuse an ID in that new history either.

Undo and redo are new validated batches at the current revision, with fresh command IDs and inverse `put`/`remove` records captured from the prior accepted state. `undo_group_id` groups gesture-level history. The journal and snapshots let another client reproduce state from the same starting revision. The `changes` Taut method selects a project and graph; accepted changes arrive on its `log` in revision order. A cursor gap or expired history triggers a snapshot refresh. The service schema declares the delivery shape, while the host API design must specify cursor carriage, retention, recovery, authentication, and backpressure.

For example, inserting a gain effect between an existing clip and output is one batch containing `put_node(fx-1)`, `remove_edge(edge-1)`, `put_edge(clip-1→fx-1)`, and `put_edge(fx-1→out-v)`. The graph is never exposed with the temporary disconnect. Its command envelope is:

```json
{"project_id":"p1","graph_id":"g1","command_id":"cmd-49","expected_revision":12,"undo_group_id":"gesture-9","operations":[{"kind":"put_node","target_id":"fx-1","node":{"id":"fx-1","kind":"effect","media":"video","sequence_id":"s1","effect":{"effect_id":"gain","effect_version":1,"params":{"amount":{"kind":"rational","rational":{"numerator":11,"denominator":10}}}}}},{"kind":"remove_edge","target_id":"edge-1"},{"kind":"put_edge","target_id":"edge-2","edge":{"id":"edge-2","from_node":"clip-1","from_port":"video","to_node":"fx-1","to_port":"input"}},{"kind":"put_edge","target_id":"edge-3","edge":{"id":"edge-3","from_node":"fx-1","from_port":"output","to_node":"out-v","to_port":"input"}}]}
```

This command example is compact application JSON, with null optional fields omitted. The actual Taut message encoder supplies them. An effect named `gain` here only illustrates a registry ID; its port signature and parameter are not adopted by this design.

## Change footprints and on-demand rendering

The authority computes `ChangeFootprint` from the old and new graph, not from a client claim. Each affected half-open timeline interval has its own `sequence_id`; the footprint also lists changed nodes, changed slot IDs, and a full-invalidation flag. Dependency traversal follows downstream edges to roots; effects may expand intervals by declared temporal handles. A source retime or asset rebind maps affected source ranges to timeline ranges. Unknown effects, topology ambiguity, registry version change, or unbounded temporal dependency cause full invalidation. The footprint is a hint to cache planning; cache reuse still requires a full semantic key match.

At a seek, the renderer takes an immutable accepted revision and binding-set version and traverses backward from the output root for the requested frame or short interval. It expands dependencies for transitions, temporal effects, decode preroll, and audio context. It schedules that window first, then bounded adjacent prefetch. It need not render sequence time zero through the seek point. Results carry request/revision/binding/fidelity identity and are discarded if stale. Batch export uses the same graph and bindings but a declared output profile, whole-range plan, verification, and publication; its plan, FFmpeg graph, hardware selection, and caches are derived artifacts.

## Failure, security, performance, and evolution

All IDs, strings, counts, graph size, depth, Unicode text, rational arithmetic, edge degree, and parameter ranges are bounded before privileged work. The Taut draft sets depth 16 and encoded length 16 MiB as upper wire guards, not target project size. The project service should set lower product limits after measurement. URLs, source paths, credentials, and executable arguments are absent from this graph. The binding manifest records asset versions, stream selection, fingerprints, and substitution policy separately; replacing a slot requires validation and a new immutable binding-set identity. It never silently changes cuts.

Taut field tags are stable and retired tags/names must be reserved. Newly added compatible fields need `MISSING_OK` if new readers must accept old bytes; `optional=True` alone still requires a key on decode. Because the current draft is v0.1, adding fields after use requires explicit migration and compatibility tests. Major version changes require a migration step that preserves meaning or rejects the file. Minor versions may add optional nonsemantic metadata but must not reinterpret existing fields. Unknown required effects and node kinds block rendering. Byte-identical deterministic CBOR is tested across browser and native targets; JSON import/export tests compare semantic digest after round trip rather than JSON byte identity.

Recommended acceptance corpus:

1. Insert, remove, and replace a node in one atomic batch; reject dangling edges and verify deterministic replay.
2. Retry a command; return its prior acknowledgement. Reuse its ID with different content and submit at a stale revision; reject both without mutation.
3. Save CBOR, export JSON, import it, and confirm the same normalized graph digest and render meaning.
4. Rebind a slot to a new asset version; preserve graph digest and cuts, change binding identity, and report incompatible media before rendering.
5. Seek to a late transition and compare local exact range render with that interval of full reference export, including handles and audio context.
6. Measure typical and large graphs for snapshot bytes, browser parse/projection time, Taut encode/decode time, map edit latency, and change-feed catch-up. Set release budgets from named hardware.
7. Validate malformed/oversized/deep messages, unknown required semantics, cross-language CBOR parity, migration, and interrupted snapshot/journal recovery.

## Alternatives and open decisions

Flat keyed maps make insertion and targeted edits cheap, while a nested per-track tree would require path rewriting and duplicate dependency structure. A generic JSON patch would shrink some commands but weaken typing and validation. CRDT/multi-writer editing adds conflict semantics GVid does not currently need. A wholly Taut-CBOR editor object would make simple browser gestures cumbersome; a JSON-shaped projection is easier to inspect and edit.

The following remain open: exact common time and PTS-origin model; port/effect registry and automation curves; stable order-key alphabet and rebalance strategy; title/style portability; support for reverse/freeze speed and nested sequences; whether disabled/unreachable editorial nodes are retained; asset substitution policies and binding-manifest schema; snapshot cadence and maximum graph size; and whether the final downloadable file is CBOR only or a CBOR-plus-JSON package. These decisions require the corresponding project, time, effect, asset, and storage designs. The current graph schema should be revised against them before a persisted v1 is frozen.
