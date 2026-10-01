# GVid Renderer Requirements

**Status:** Initial draft
**Scope:** Portable edit graph contract, renderer-facing edit commands, interactive preview rendering, and batch export in the gvid-renderer member
**Source:** [Product and system requirements](../../dev-docs/gvid-requirements.md)
**Related:** [High-level architecture](../../dev-docs/gvid-arch.md), [Taut protocol](https://github.com/owebeeone/taut)

## Purpose

This document translates the GVid product baseline into requirements for the renderer member. It does not select an output codec, operating system, color policy, or implementation layout while those product decisions remain open. Requirement IDs are stable references for design, implementation, and tests. MUST, SHOULD, and MAY have the meanings defined in the product and system requirements.

The renderer consumes validated project, asset, effect, audio, color, job, and capability contracts. The project service owns canonical editorial state and acceptance of UI edits. A portable edit graph expresses that state for sharing and repeatable rendering; a normalized render plan and native tool commands are derived execution artifacts. The renderer does not change source media.

All structured messages crossing GVid component boundaries use governed Taut contracts and wire. Encoded media bytes remain opaque payloads served through the authenticated HTTP range endpoint; Taut governs resource descriptors and control messages. Taut is a candidate for the portable graph file format, subject to an explicit schema and performance decision.

## Requirements

### Taut communication and graph-format evaluation

| ID | Requirement | Baseline |
| --- | --- | --- |
| GVR-RDR-056 | Every structured renderer-related message crossing the browser, host, renderer, media service, or supervised sidecar boundary MUST be defined by a versioned Taut message and service contract and encoded on the Taut wire. This includes edit commands and acknowledgements, graph snapshots and changes, asset bindings, preview requests and results, job progress, cancellation, warnings, errors, and export results. | GVR-CON-007; GVR-REND-016 |
| GVR-RDR-057 | Each Taut method MUST declare delivery semantics appropriate to its use. The protocol design MUST choose and document request/response and stream shapes, ordering, replay or latest-value behavior, backpressure, cancellation, and recovery; it MUST NOT create an untyped JSON or shell-command bypass. | GVR-CON-007-008; GVR-NFR-SEC-001-005 |
| GVR-RDR-058 | Taut envelopes MUST carry the applicable schema/capability version, request or command ID, project and sequence identity, accepted revision, asset-binding version, and result fidelity so receivers can reject stale or incompatible results. | GVR-CON-008; GVR-PREV-009; GVR-RDR-034,041 |
| GVR-RDR-059 | Taut decoding at every privileged boundary MUST enforce declared size and nesting limits and validate identifiers, exact time fields, node parameters, and resource access before scheduling work. Unsupported required semantics MUST fail explicitly under a documented compatibility policy. | GVR-CON-008; GVR-NFR-SEC-005; GVR-RDR-026 |
| GVR-RDR-060 | The Taut schema, compiler/runtime version, and golden wire corpus used by GVid MUST be pinned. Browser and native implementations MUST pass cross-language encode/decode, version evolution, malformed-input, and request-correlation tests for the GVid contracts. | GVR-CON-008; GVR-NFR-MNT-004 |
| GVR-RDR-061 | Before the portable graph format is frozen, the design MUST evaluate a Taut schema and deterministic wire as the saved graph representation. The evaluation MUST test typed node variants, exact time, graph depth and size, deterministic identity, incremental snapshot/change exchange, schema evolution, asset rebinding, and browser/native round trips. The chosen format and reasons MUST be recorded. | GVR-REND-015; GVR-RDR-023-032 |
| GVR-RDR-062 | If the saved graph uses Taut, it MUST carry a durable schema version and migration path. If another saved format is chosen, its graph snapshots and changes crossing GVid component boundaries MUST map to Taut messages without silent semantic loss. | GVR-REND-015-016; GVR-NFR-MNT-004 |
| GVR-RDR-063 | Taut messages MUST issue opaque media resource descriptors and convey related status and errors. Encoded video and audio bytes MAY remain raw byte ranges over the authenticated loopback HTTP endpoint; it MUST NOT expose an alternative application control API. | GVR-CON-009; GVR-NFR-SEC-004 |

### Portable edit graph and asset bindings

| ID | Requirement | Baseline |
| --- | --- | --- |
| GVR-RDR-023 | GVid MUST define a versioned, serializable edit graph that describes the intended video and audio result independently of a renderer executable, device, or machine-local path. | GVR-CON-004; GVR-NFR-MNT-002,004 |
| GVR-RDR-024 | The graph MUST identify sequences, ordered tracks, stable node and connection IDs, typed operations and parameters, asset references, output roots, and the effect and profile schema versions needed to interpret it. | GVR-PROJ-002; GVR-VFX-004,007 |
| GVR-RDR-025 | Graph time values MUST use exact rational or integer units. Source, timeline, and output domains, half-open ranges, retiming, and rounding rules MUST be explicit. | GVR-EDIT-005-007,012 |
| GVR-RDR-026 | Graph validation MUST reject cycles, dangling references, invalid ranges, unsupported node or effect versions, and incompatible media connections before work is scheduled. A future nested-sequence feature MUST define its own acyclic expansion semantics. | GVR-EDIT-011,013; GVR-VFX-007 |
| GVR-RDR-027 | Serialization and normalization MUST be deterministic: equivalent accepted graph state with the same schema version MUST produce the same normalized semantic identity regardless of UI operation order that leaves the same state. | GVR-EDIT-010; GVR-NFR-COR-001 |
| GVR-RDR-028 | Asset references MUST use stable logical IDs or slots with a separate binding manifest for locations, selected streams, fingerprints, and relevant timing, color, and audio properties. A shared graph MUST NOT require the original machine's absolute paths to be meaningful. | GVR-ASSET-003-005; GVR-NFR-PORT-001,003 |
| GVR-RDR-029 | Rebinding an asset slot to new media MUST validate the replacement and report material differences. The user MUST be able to choose an explicit substitution policy for duration, frame rate, aspect ratio, channel layout, and missing ranges; the renderer MUST NOT silently shift cuts or substitute unrelated media. | GVR-ASSET-005; GVR-NFR-COR-004 |
| GVR-RDR-030 | Importing and exporting a graph without edits MUST preserve its supported render meaning. Unknown required semantics MUST be reported and MUST NOT be silently dropped or rewritten. | GVR-PROJ-005-006; GVR-NFR-MNT-004 |
| GVR-RDR-031 | A user MUST be able to save, download, and share the edit graph with its schema and asset binding manifest. Inclusion of source media or derived artifacts MUST be an explicit packaging choice; credentials and private machine paths MUST be excluded from the portable graph. | GVR-PROJ-008; GVR-NFR-SEC-007; GVR-NFR-PORT-003 |
| GVR-RDR-032 | The normalized render IR, cache plan, hardware choice, and native command lines MUST remain derived from a specific graph revision, asset binding set, output profile, and toolchain; they MUST NOT become authoritative editable project state. | GVR-REND-004; GVR-NFR-MNT-001-002 |

The first schema design should define these records and their compatibility rules. This is a contract outline, not a choice of file format:

| Record | Required content |
| --- | --- |
| Graph header | Schema version, graph identity, revision identity, sequence roots, and semantic feature versions. |
| Sequence and node | Stable IDs, typed operation, input and output ports, ordered layer or track placement, exact time ranges, and versioned parameters. |
| Asset slot and binding | Logical asset ID, selected streams, expected source properties and fingerprint, plus a separately replaceable local binding. |
| Render request | Taut message carrying graph revision, binding set, sequence, requested time or range, quality/fidelity target, profile, priority, and request identity. |
| Render result | Taut message carrying request identity and revision, actual time range, fidelity, artifact identity, warnings, diagnostics, and completion state. |

### Browser and editor mutations

| ID | Requirement | Baseline |
| --- | --- | --- |
| GVR-RDR-033 | Browser and editor edits MUST be submitted as typed composition commands, not direct edits to a live render plan or native renderer arguments. | GVR-EDIT-009-011; GVR-NFR-SEC-001-002 |
| GVR-RDR-034 | Every edit command MUST carry a command ID, project and sequence IDs, expected project revision, target IDs, exact source or timeline ranges where relevant, and validated typed parameters. | GVR-EDIT-005-007,011; GVR-NFR-SEC-005 |
| GVR-RDR-035 | The project authority MUST accept or reject each command atomically and return the resulting revision or an actionable validation failure. A rejected command MUST NOT partially change the graph. | GVR-EDIT-010-011; GVR-DIAG-002 |
| GVR-RDR-036 | Repeated delivery of the same command ID MUST be idempotent, and commands against a stale revision MUST be rejected or explicitly reconciled under documented rules. | GVR-EDIT-010; GVR-NFR-REL-002 |
| GVR-RDR-037 | Accepted commands MUST enter normal undo/redo history and yield a revision-scoped graph change description sufficient to invalidate affected previews and derived artifacts. | GVR-EDIT-009-010; GVR-NFR-PERF-004 |
| GVR-RDR-038 | The UI MAY display tentative gesture feedback, but authoritative preview requests MUST identify an accepted revision; tentative results MUST be distinguishable and MUST NOT replace results for a newer accepted revision. | GVR-PREV-009-010; GVR-UI-009,013 |
| GVR-RDR-039 | A graph change MUST identify the affected timeline intervals, dependent nodes, and asset bindings so unchanged regions can retain valid cached results. If the affected set cannot be proven, the system MUST invalidate conservatively. | GVR-NFR-PERF-004; GVR-NFR-MNT-005 |
### On-demand interactive rendering

| ID | Requirement | Baseline |
| --- | --- | --- |
| GVR-RDR-040 | A seek, scrub, jog, or parked-playhead request MUST be able to request the intended frame or short range at an arbitrary sequence time without waiting for a whole-sequence render. | GVR-PREV-001,003 |
| GVR-RDR-041 | Each interactive request MUST carry a request ID, project and sequence revision, requested exact time or half-open range, quality target, and relevant asset versions. A result MUST be presented as current only while that context still matches. | GVR-PREV-009; GVR-UI-009,013 |
| GVR-RDR-042 | The scheduler MUST prioritize the requested frame or near-playhead range, then use bounded look-ahead or look-behind prefetch based on playback direction and available resources. It MUST NOT require rendering from sequence start to reach a later point. | GVR-JOB-004-005; GVR-NFR-PERF-001,005 |
| GVR-RDR-043 | A local range render MUST evaluate all visible layers, transitions, effect dependencies, audio context, and required source preroll or handles for that range. Its visible result MUST match the same range in a full reference render within the declared fidelity tolerance. | GVR-PREV-005,010; GVR-VFX-002,005 |
| GVR-RDR-044 | Superseded interactive requests MUST be cancelled or ignored. Late results from an older seek, edit, asset binding, project, or sequence context MUST NOT overwrite the current frame. | GVR-PREV-009; GVR-UI-009,013 |
| GVR-RDR-045 | The renderer MAY return a fast approximate result before an exact result, but MUST label the fidelity and replace or clearly distinguish it when the exact result arrives. Effects without a valid live implementation MUST trigger a range pre-render or a visible reduced-fidelity state. | GVR-PREV-004,010; GVR-VFX-005-006 |
| GVR-RDR-046 | Playback rendering MUST use a defined synchronization clock and bounded decode, memory, and GPU queues. Under pressure it MUST degrade predictably through quality reduction, pre-render, or a clear pause rather than silently changing composition meaning. | GVR-PREV-006-008; GVR-NFR-PERF-005 |
| GVR-RDR-047 | Interactive render artifacts MUST be reusable only when the requested region's subgraph semantics, asset versions, requested range, fidelity, effect versions, color policy, and toolchain inputs still match. A new graph revision MAY reuse a valid unaffected artifact, but every displayed result MUST still match the current request and revision context. | GVR-REND-014; GVR-NFR-MNT-005 |
| GVR-RDR-048 | Reference hardware MUST have measured budgets for cold seek, warm seek, exact parked frame, first usable frame after an edit, and sustained playback under a concurrent export. Budgets and tested layer/effect complexity MUST be set before release. | GVR-NFR-PERF-003; GVR-NFR-PERF-001 |
| GVR-RDR-049 | An interactive range render MUST remain a disposable preview artifact. It MUST NOT be presented as a completed final export without the final profile, verification, and publication steps. | GVR-REND-005,008; GVR-NFR-REL-003 |

### Efficient batch rendering

| ID | Requirement | Baseline |
| --- | --- | --- |
| GVR-RDR-050 | A batch MUST support multiple explicit render requests over the same edit graph, including different output profiles, destinations, sequence ranges, or validated asset binding sets. Each item MUST have its own immutable graph revision, bindings, profile, job ID, result, and export record. | GVR-REND-001,009; GVR-JOB-001 |
| GVR-RDR-051 | Batch planning SHOULD reuse valid source probes, decoded or intermediate artifacts, and common graph subresults where semantic and toolchain keys match, without coupling the success or failure state of separate outputs. | GVR-REND-014; GVR-NFR-PERF-004 |
| GVR-RDR-052 | Partitioned or parallel batch execution MUST preserve exact boundary, transition, audio, color, and output semantics. Every assembled output MUST pass the same final verification as a single export. | GVR-REND-005,008; GVR-NFR-COR-001-003 |
| GVR-RDR-053 | A failed or interrupted batch item SHOULD be resumable from verified reusable work when safe. Retry MUST retain the original revision and bindings unless the user explicitly creates a new request. | GVR-JOB-006-007; GVR-REND-009,014 |
| GVR-RDR-054 | Batch work MUST obey resource admission and fairness so a long export cannot indefinitely block active playback, seeks, or user-requested exact frames. | GVR-JOB-004-005; GVR-NFR-PERF-001 |
| GVR-RDR-055 | Re-rendering the same graph with newly bound assets MUST create a new immutable binding set and export record. The old graph revision and prior export evidence MUST remain identifiable. | GVR-ASSET-003-005; GVR-REND-009; GVR-NFR-COR-001 |
### Export request and render plan

| ID | Requirement | Baseline |
| --- | --- | --- |
| GVR-RDR-001 | An export request MUST identify an active sequence, an immutable accepted project revision, a named and versioned output profile, and a selected destination. Later edits MUST NOT change the in-flight export. | GVR-REND-001; GVR-NFR-COR-001 |
| GVR-RDR-002 | Planning MUST resolve source references against registered master asset versions. Proxy-based export MUST require an explicit user choice and be recorded in the export record. | GVR-REND-002; GVR-ASSET-003-004 |
| GVR-RDR-003 | Planning MUST reject or report missing or changed media, invalid ranges, insufficient transition handles, unsupported effects, incompatible output settings, and insufficient required disk space before execution where these can be determined. | GVR-REND-003; GVR-EDIT-011; GVR-NFR-COR-004 |
| GVR-RDR-004 | Planning MUST use exact rational or integer time, half-open ranges, and explicit conversions among source presentation time, timeline time, and output frame or sample time. Conversion and rounding rules MUST be documented. | GVR-EDIT-005-007; GVR-NFR-COR-003 |
| GVR-RDR-005 | The same valid revision, source identities, profile, and toolchain MUST produce the same normalized render semantics. The plan MUST be represented independently of a native renderer command line. | GVR-REND-004; GVR-NFR-COR-001; GVR-NFR-MNT-002 |
| GVR-RDR-006 | The plan MUST preserve supported cuts, layer order, transforms, crop, opacity, dissolves, timed text or captions, audio gain, mute, fades, mixing, and source-to-timeline timing. Unsupported operations MUST fail explicitly. | GVR-VFX-001-007; GVR-AUD-001-005; GVR-REND-010 |
| GVR-RDR-007 | The output plan MUST explicitly specify stream mapping, video frame rate and dimensions, audio sample rate and channel layout, color policy, codecs, and container settings; it MUST NOT inherit unspecified tool defaults as product semantics. | GVR-REND-005 |
| GVR-RDR-008 | The renderer MUST offer a validated software reference path for every operation in the supported baseline. | GVR-REND-010 |

### Execution and job lifecycle

| ID | Requirement | Baseline |
| --- | --- | --- |
| GVR-RDR-009 | Each render or export operation MUST have a stable job identity, explicit lifecycle state, meaningful progress when available, warnings, cancellation state, and a diagnostic reference. | GVR-REND-006; GVR-JOB-001-003; GVR-DIAG-002 |
| GVR-RDR-010 | Native render processes MUST be supervised, cancellable when safe, and cleaned up on normal shutdown or termination. An abandoned job and its incomplete artifacts MUST be detected on restart. | GVR-JOB-003; GVR-JOB-006-007; GVR-NFR-REL-004 |
| GVR-RDR-011 | Execution MUST respect admitted CPU, memory, disk, I/O, and accelerator resources and MUST allow interactive preview work to retain its required priority. | GVR-JOB-004-005; GVR-NFR-PERF-001,005 |
| GVR-RDR-012 | Native media tools MUST be invoked without a shell, with executable selection separate from arguments, and with untrusted project or media values validated at the privileged boundary. | GVR-NFR-SEC-003,005 |
| GVR-RDR-013 | The renderer MUST use a pinned or validated media toolchain and record the versions and build configuration used for an export. It MUST NOT silently select an arbitrary executable from the user's path. | GVR-CON-005; GVR-NFR-PORT-004; GVR-NFR-LIC-001 |
| GVR-RDR-014 | Hardware execution MAY be chosen only from a host-validated complete decode, filter, and encode path for the requested profile. If that path fails, fallback to the software reference path SHOULD occur when policy and user intent permit, with the change recorded. | GVR-REND-011-012; GVR-NFR-PORT-002 |
| GVR-RDR-015 | Packet-copy or smart-render execution MUST use conservative eligibility checks and output verification. Ineligible work MUST use the normal render path without changing the requested composition. | GVR-REND-013 |
| GVR-RDR-016 | When an intermediate render is reused, its cache identity and validation metadata MUST identify every semantic input and toolchain version needed for safe reuse, including asset-binding fingerprints. The renderer MUST compare that identity and metadata with the consuming request before reuse. Incomplete, mismatched, invalid, or corrupt intermediates MUST be treated as cache misses and regenerated, or fail clearly when regeneration is unavailable. | GVR-REND-014; GVR-NFR-REL-003; GVR-NFR-MNT-005 |

### Output integrity and evidence

| ID | Requirement | Baseline |
| --- | --- | --- |
| GVR-RDR-017 | A failed or cancelled export MUST NOT appear at the final destination as a successful file. Source media MUST remain unchanged. | GVR-REND-007; GVR-NFR-REL-005 |
| GVR-RDR-018 | A completed candidate output MUST be probed and checked against the requested profile, including required streams, timing, dimensions, and relevant metadata, before atomic publication. Verification failure MUST leave the prior destination intact. | GVR-REND-008 |
| GVR-RDR-019 | The export record MUST identify the project revision, source fingerprints, profile, engine and toolchain builds, selected execution path, warnings, and output probe summary. | GVR-REND-009 |
| GVR-RDR-020 | User-visible failures MUST give an actionable summary and stable diagnostic reference. Local logs MUST correlate the project revision, job, process, and render plan without exposing credentials or media-derived content by default. | GVR-DIAG-002-004; GVR-NFR-SEC-007 |
| GVR-RDR-021 | Exported audio and video MUST satisfy the timing and synchronization tolerances established by the quality test design, including source sample-rate conversion and encoder delay. | GVR-AUD-004-005; GVR-NFR-COR-003 |
| GVR-RDR-022 | Effects declared exact or equivalent between preview and final rendering MUST meet their declared parity tolerance on shared fixtures. Approximate or pre-render-only behavior MUST remain explicit to the caller. | GVR-VFX-005-006; GVR-PREV-010 |

## Initial acceptance scenarios

These are required checks for implementation; this draft does not claim they have run.

1. Export a fixed revision with a named profile, edit the project while the job runs, and confirm the output and record still identify the original revision.
2. Render CFR and VFR fixtures with non-zero source timestamps, B-frames, fractional rates, cuts, and a dissolve. Check frame boundaries and duration against the exact time contract.
3. Mix sources with different audio sample rates and layouts, apply gain and fades, and check output synchronization and declared layout.
4. Remove or change a registered master after planning begins. Confirm the export fails explicitly rather than substituting a proxy or stale derivative.
5. Cancel and crash an export before publication. Confirm the final destination remains absent or retains its prior valid file, and incomplete artifacts are recovered or removed on restart.
6. Produce an output with a deliberately wrong stream or profile property. Confirm probing rejects it before publication and retains a diagnostic record.
7. Compare a software reference render with an eligible hardware render against the profile's timing, quality, and metadata contract; inject a hardware failure and check the permitted fallback path.
8. Test smart-render eligibility with both safe and hostile cut boundaries, then verify every candidate output and normal-render fallback.
9. Attempt to pass hostile file names and metadata through planning and execution. Confirm they remain data and cannot alter executable selection or invoke a shell.

10. Save and reload a graph, then export both copies with the same bindings and profile. Compare normalized render semantics and resulting profile compliance.
11. Share a graph without source files, relink its asset slots on another machine, and inspect reported frame-rate, duration, aspect-ratio, and audio-layout differences before rendering.
12. Submit duplicate, stale-revision, and invalid UI edit commands. Check atomic acceptance, idempotent retries, undo/redo, and revision-scoped preview invalidation.
13. Seek directly to a late, effect-heavy timeline point. Confirm the first requested frame or local range appears without a full-sequence render and includes all active layers and transition context.
14. Scrub rapidly across several points, then edit and relink an asset. Confirm no superseded result can be displayed as current and valid unaffected cache entries remain reusable.
15. Compare an on-demand range render with the same interval in the full software reference export, including a transition crossing the range boundary.
16. Run a batch with multiple profiles and asset binding sets while playing the timeline. Check job isolation, fair resource scheduling, verified outputs, and safe retry of an interrupted item.

17. Exchange representative edit, seek, cancellation, progress, and export messages between the browser and native runtimes using the pinned Taut schema; compare canonical wire bytes and decoded meaning.
18. Send malformed, oversized, too-deep, stale-revision, and future-version Taut messages. Confirm bounded failure, no privileged action, and a correlated diagnostic.
19. Evaluate a large layered graph with asset rebinding and incremental changes as a Taut saved document. Record size, decode cost, deterministic identity, migration behavior, and any reason to use a different durable format.
20. Render a graph with asset slot S bound to source A and retain its intermediate. Rebind S to source B with the same duration, dimensions, streams, and timing but different content, then export the same graph and profile. Confirm A's intermediate cannot satisfy B's request, the output and export evidence identify B, and missing cache identity evidence causes regeneration or explicit failure. Repeat with a changed semantic effect parameter and toolchain version while the prior intermediate remains available.

## Open decisions and dependencies

- The initial operating systems, accepted source envelope, output profiles, working/output color policy, audio layouts, and reference performance budgets are product decisions in the [baseline](../../dev-docs/gvid-requirements.md).
- The exact render IR, partition and intermediate strategy, and output verification schema belong to the render model design identified in the [architecture](../../dev-docs/gvid-arch.md).
- Tool invocation, hardware capability evidence, effect semantics, color conversion, job scheduling, and artifact identity require their respective shared contracts and detailed designs.
- The product must decide whether smart rendering is visible to users or remains an internal optimization before a user-facing promise is added here.

- The graph schema design must settle node and port types, canonical serialization, version migration, nested sequence expansion, and the exact asset substitution policies. These decisions must preserve the renderer-neutral contract above.
- The preview and jobs designs must set prefetch policy, latency budgets, and fairness thresholds on named reference hardware. The render model design must settle safe partition boundaries and reuse rules for batch work.
- The Taut graph-format evaluation must decide whether the durable file itself uses Taut. The communication boundary remains Taut in either case.
