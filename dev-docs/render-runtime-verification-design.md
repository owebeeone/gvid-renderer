# GVid Renderer Runtime Verification Design

**Status:** Draft test design for review. This document specifies checks to implement and run; it records no runtime pass. The [integrated design decision](render-engine-integrated-design-ReviewDecision.md) accepts the design contract only.

**Applies to:** The [integrated renderer design](render-engine-integrated-design.md), [render requirements](render-requirements.md), [flat graph/Taut design](render-graph-schema-design.md), [engine design](render-engine-design.md), [ExportJobs lifecycle](export-jobs-lifecycle-design.md), and [owner-scope contract](export-jobs-owner-scope-design.md). The exact tested commits, schema/compiler version, provider builds, and profile definitions belong in each run record, not in this living plan.

## 1. Goal and acceptance boundary

Prove that the implemented headless core produces the specified media and evidence through an offline CLI and a Glade adapter, and that the interactive path remains correct and responsive while exports run. Test actual decoding, composition, audio, encoding, probing, transport, authorization, durable jobs, process supervision and publication. The existing Python contract tests are a prerequisite, not runtime evidence.

Separate four gates so that a passing narrow test cannot be mistaken for a release pass:

1. **Contract gate:** golden Taut corpus, semantic validators and graph/editor state transitions across Python, Rust and TypeScript.
2. **Core/media gate:** real media inputs, the common planner/provider interfaces, resolver and output probe with no network service.
3. **Client/host gate:** CLI and Glade adapter against the same core; Taut control traffic and authorized HTTP media bytes; durable host and local supervisors.
4. **Release gate:** fault/concurrency matrix, provider parity, measured latency and capacity on pinned reference machines.

Each scenario below records `pass`, `fail`, or `not_run`. `not_run` never counts as pass. A supported semantic operation needs an exact reference oracle or an explicitly declared tolerance and fidelity; unsupported operations must reject. A proxy preview can be acceptable when labeled, but cannot satisfy an exact-frame or export assertion.

## 2. Harness and observability

Build one deterministic fixture generator and run manifest. The generator writes local media and Taut graph/binding/profile fixtures from checked-in seeds, records content digests and exact rational times, and offers independent expected frame/sample observations. Do not derive the expected pixels, timing or audio envelope from the renderer under test. Store only synthetic media in the repository.

The harness has five drivers around the production boundaries:

| Driver | What it exercises | Required observations |
| --- | --- | --- |
| Core driver | Validated immutable request, planner, resolver, provider, scheduler and artifact store | Task dependency ranges, chosen provider/build, cache keys, attempt IDs, bounded resource use and verification result |
| CLI driver | Fresh offline import, local/package resolver, local journal, cancellation, restart and final file | Exit/status contract, durable lookup, final file/probe and export record; no host incarnation |
| Glade driver | Authenticated Taut gateway, catalog/editor/bindings/preview/ExportJobs, host resolver and journal | Exact request/response/event envelopes, authority decisions, descriptor leases and HTTP ranges |
| Fault controller | Deterministic named barriers and process, store, resolver and provider faults | Barrier order, durable records before/after, owned processes/artifacts and recovery action |
| Media oracle | Independent decoder/probe and synthetic signal checks | Decoded frame landmarks, audio sample windows, exact timing, streams, colors and output digest |

Fault barriers must pause at named state transitions, not at arbitrary sleeps. A process kill must kill the actual supervised host/CLI process tree at a recorded barrier. A second process then reopens the same journal and destination. Every driver records the exact request and final response class, with credentials and privileged locators redacted. Observe both state and external effects: a typed error alone is insufficient if a process still runs or a file was replaced.

The harness must be able to hold and release an event or HTTP response, advance an injected lease clock, revoke a scope, change a registered source fingerprint, fail a provider after Prepare, exhaust ordinary journal space, fail a write/fsync, and crash at publication boundaries. Time control may be injected at service seams; media timestamp or wall-clock calculations in production must not be replaced by test-only semantics.

## 3. Fixture set and independent oracles

Keep fixtures short enough for frequent local runs, with one longer sequence for late seek and fairness. Each fixture has a manifest of expected frame PTS, visible layers, sample windows, graph revision, binding identity, stream properties and profile definition.

| Fixture | Purpose and oracle |
| --- | --- |
| `layered-seek` | Two overlapping layers with distinct position/color/alpha marks, an implicit gap, a late-frame marker and a finite transition handle. A reference frame for each chosen time proves both layers and order; removing a layer or reversing order must fail the oracle. |
| `timing-audio` | CFR and VFR sources with nonzero PTS origin and decode preroll, plus channel-specific tones, silence, exact gain and fade windows. Probe frame PTS and sample-window RMS/phase at cut and fade boundaries. A shifted fade or nominal-fps VFR lookup must fail. |
| `asset-versions` | Same-byte new binding, changed-byte same duration, unrelated-slot rebind, missing asset and expiring Glade-only source. Expected identities and cache hit/miss decisions are explicit. |
| `profile-output` | Two pinned profiles with distinct resolution, color/audio settings and output destinations. The oracle verifies stream count, codec/profile, dimensions, duration, frame/sample timing and final probe summary. |
| `publication` | Existing destination and path alias that resolve to the same canonical target, plus a second successful generation. The manifest records original and replacement digests. |
| `invalid-wire` | Oversize/deep/invalid Taut messages, unsafe exact integers, unsupported semantic versions, wrong provenance branches and malformed negative result envelopes. No media task may be admitted. |

Fixtures for unsupported transitions/effects are rejection checks until their semantics and reference implementation are defined. Pin FFmpeg/FFprobe and every other candidate provider build in the run manifest. The software provider is the reference implementation; compare candidate decoded output at the operation's declared exact/proxy tolerance. Define pixel/color and audio numeric tolerances per operation/profile before enabling that candidate, and version them with the test manifest. Exact identity, state, authorization and publication assertions have no tolerance.

## 4. Scenario matrix and pass conditions

Every scenario runs at the lowest applicable gate, then repeats across CLI and Glade where the boundary is relevant. The scenario IDs are stable names for implementation and evidence.

| ID | Runtime action and observable pass condition | Requirements / design seam |
| --- | --- | --- |
| RG-01 | Save/import a flat graph, perform user-facing insert/undo/redo, privileged full-record remove/reorder batches and binding-only rebind. Normalized graph meaning round-trips; only accepted edits advance one contiguous revision; a rejected edit changes neither graph nor binding journal; a rebind preserves graph digest and creates a new immutable binding set. An in-flight render keeps its original snapshot. | GVR-RDR-023–039, 065–068, 071–072 |
| RG-02 | Send duplicate, gapped, expired and out-of-order graph/binding/catalog changes, including ack before change. The client promotes only contiguous verified state, marks mirrors and previews stale on an unverified gap, and recovers by snapshot; no ack alone advances accepted state. | GVR-RDR-037–039, 065–068 |
| PV-01 | Request independent source preview before insertion at an exact indexed VFR frame. The result identifies asset version, stream and fingerprint, with no graph/binding identity. Change the catalog version/fingerprint while the response is held: the old frame is never displayed as current. | GVR-RDR-067, 069–070 |
| PV-02 | Request a late sequence frame while a full export is running. The plan includes only contributing layers, handles and decoder preroll; the requested visible frame is scheduled before bounded adjacent prefetch. Compare decoded pixels against the same time in a full reference render. No whole-sequence prerequisite occurs unless the effect declares earlier state. | GVR-RDR-040–043, 054 |
| PV-03 | Hold old preview responses while seeking, editing, rebinding, closing/reopening the project, cancelling and changing viewer. Only a response matching all current sequence or source identity, feed freshness and live lease can become current. Expired/replaced descriptors fail HTTP access or release; an old result cannot displace a new one. | GVR-RDR-041, 044, 069–070 |
| PV-04 | Return proxy then exact preview and apply memory/I/O pressure. Fidelity is accurately labeled; exact result replaces or remains distinguishable from proxy; bounded queues and fair admission protect exact requests. If service cannot be bounded, export admission defers/rejects with capacity rather than silently starving the viewer. | GVR-RDR-045–048, 054 |
| MD-01 | Run the implemented fade, gain, mix, layer composition, color conversion, encode and mux semantics through software reference and each advertising provider; add transition boundaries only when a versioned transition is supported. Compare boundary frames and sample windows to independent oracles at declared tolerances; final file passes pinned profile/timing probe. A provider missing a semantic version rejects before execution. | GVR-RDR-043, 049–052; engine §3 |
| MD-02 | Fail hardware after Prepare. A permitted software fallback starts a new attempt with new dependent artifact keys, records provider/build, and cannot consume failed partial artifacts. With fallback forbidden, report typed failure and publish nothing. | Engine §§2–3, 6 |
| AS-01 | Resolve local, package and Glade-only sources. Offline CLI succeeds for local/package, and gives an actionable unavailable result for Glade-only media without a bridge. An authorized package materialization or bridge succeeds. Lease expiry, changed bytes/version and missing range capability fail or use explicitly permitted bounded staging; no silent substitution or credential/path leak. | GVR-RDR-028–031, 055, 068; engine §4 |
| CL-01 | Render the same validated graph, binding set, profile and provider semantics through offline CLI and Glade. Normalized plan and decoded output agree at declared tolerance; provenance and authority fields remain branch-specific. Glade uses Taut for structured control and opaque descriptors, authorized HTTP ranges for raw bytes; CLI uses local/package bytes and does not need Glade. | GVR-RDR-056–063; engine §§1–2 |
| CL-02 | Cross-language Python/Rust/TypeScript encode/decode and malformed-input runs hit live service boundaries. Correlation, exact integer/rational values, size/depth limits and negative response payload rules survive round-trip; invalid wire is rejected before scheduling. | GVR-RDR-056–060, 072 |
| EX-01 | Submit same request concurrently, lose acceptance ack, reopen host or restart CLI, then lookup and retry. Exactly one durable accepted ID/job/attempt is created; authorized identical retry replays, changed payload conflicts; invalid pre-acceptance request reserves no ID. Governed and standalone namespaces never impersonate each other. | ExportJobs lifecycle §§2, 4 |
| EX-02 | Compare identical denied requests against another owner's live, retired and unused accepted-ID states, then revoke/retire a scope at barriers before admission, lookup, status, cancel and event handoff. Denial envelopes and external effects are occupancy-independent; no post-revocation job fact, event or cancellation crosses the durable authorization fence. Authorized grants see their entire scope history. | Owner-scope contract; lifecycle §§2, 4 |
| EX-03 | Cancel one of two jobs while the other runs, then kill/restart at queued, preparing, running and verifying states. Cancellation ack means requested only; final state is durable. Reopen reconciles owned processes, leases and artifacts before reuse; no orphan continues and no unverified candidate becomes success. Other job proceeds independently. | Lifecycle §§2–4; engine §6 |
| EX-04 | Admit near quota, exhaust ordinary progress/journal space, force failure and restart. Reserved space records terminal/cleanup/publication reconciliation; new admissions return capacity. Inject write/fsync failure despite reserve: success and ownership release are withheld until durable reconciliation. Accepted IDs survive compaction and restart. | Lifecycle §3 |
| EX-05 | Crash before and after acceptance commit, tombstone conversion, publication intent, replacement, result commit and backup cleanup. Each recovery state has one accepted ID or tombstone, one canonical destination owner, and either old generation or verified new generation; a partial candidate never replaces final bytes. Old retry after a later generation neither reruns nor restores an old backup over it. | Lifecycle §§2–4; engine §6 |
| EX-06 | Run a batch with shared graph but different profile, range, destination and binding set. Each item has separate immutable job/result/verification. Verified subresults reuse only matching semantics/toolchain keys; cache poisoning and one-item failure do not change another result. Partition boundaries, audio timing and output probes match single-export references. | GVR-RDR-047, 049–055 |

For every final export, assert the candidate is fully rendered at the pinned final profile, independently probed, durably recorded and published under destination ownership before reporting success. A preview artifact or native-process exit code is never sufficient evidence.

## 5. Fault schedule and state assertions

The controller publishes named barriers: `before_admission_commit`, `after_admission_commit_before_ack`, `before_attempt_record`, `after_attempt_record_before_tool_launch`, `after_tool_launch`, `after_candidate_write`, `before_probe`, `after_probe_before_intent`, `after_publication_intent`, `after_replace_before_result_commit`, `after_result_commit_before_backup_cleanup`, `before_tombstone_conversion`, `after_tombstone_conversion`, and `before_positive_response_handoff`. Add scope-grant and event-delivery barriers before authorization-sensitive reads/actions and at handoff. These are conceptual seam names; implement them as one controlled test hook interface at production transition points, without changing the state machine.

At each applicable barrier, run normal continuation, process kill/restart and repeat/retry variants. Record the durable accepted-ID entry, journal reservation, job state, process creation token, owned spool paths/leases, destination identity/generation, backup and result digest both before and after recovery. The assertion is state conservation: every durable claim has a matching owned resource, every launched resource has a durable owner, and neither a success nor release is inferred from an incomplete write. Inject alias collisions and concurrent claimants with a barrier rather than racing by sleep. A failure report must include the minimal seed, barrier and resulting state trace for reproduction.

## 6. Performance and fairness gate

GVR-RDR-048 requires budgets before release; this draft deliberately sets no invented latency numbers. Pin at least one interactive reference machine and one constrained machine, OS/toolchain, fixture complexity, output profiles and resource limits. Establish numeric cold seek, warm seek, exact parked frame, first usable post-edit frame, sustained playback with concurrent export, export throughput and preemption/defer deadlines before declaring the release gate active.

Measure request-to-first-usable and request-to-exact latency from client submission through decoded presentation, not only planner time. Capture cold/warm cache separately, early/middle/late seeks, rapid scrub, edit/rebind invalidation, one export and batch pressure. Record median and tail latency, deadline misses, decode queue/spool maxima, interactive progress and export progress across repeated seeded runs. The gate passes only when every correctness invariant holds, configured resource caps are respected, the agreed latency/deadline budgets pass on the pinned machines, and exports make fair progress when capacity permits. The run record states sample count, warm-up, cache state, machine load and variance policy. Any budget change needs a new versioned baseline and rationale, not a silent threshold update.

## 7. Evidence and implementation order

Each run emits a machine-readable manifest and a short human report: renderer and GWZ root commits, Taut source/compiler/runtime versions, generated-binding versions, provider executable/build digests, profile definitions, OS/hardware/resource limits, fixture digests/seeds, scenario ID, driver/client branch, injected barrier, start/end state, oracle/tolerance version, metric distribution, result, and hashes of retained synthetic output/trace artifacts. Redact credentials, private locators and source media outside synthetic fixtures. Keep failed reproducers and crash journals in an access-controlled test artifact store; reports cite their IDs without embedding secrets.

Implement in four slices with a demonstrable exit for each:

1. **Media vertical slice:** deterministic fixtures, core driver, software provider, late seek and one verified offline CLI export. Exit: PV-02, MD-01 reference path, AS-01 local/package and CL-01 offline half pass with actual media.
2. **Glade and wire slice:** Taut drivers, cross-language round-trips, source/sequence preview, lease-aware HTTP, Glade-only resolver. Exit: RG-01/02, PV-01/03, AS-01 Glade half and CL-01/02 pass.
3. **Durability slice:** persistent supervisors and barrier controller. Exit: EX-01–05 pass for governed and standalone branches where applicable; no unresolved owned artifacts or destination intents.
4. **Parity and load slice:** candidate providers, batch/cache cases and measured scheduler gate. Exit: PV-04, MD-02, EX-06 and versioned GVR-RDR-048 budgets pass on pinned machines.

Before implementing each slice, convert its rows to executable test cases with exact fixture digests, expected status envelopes, numeric media tolerances and runner commands. Review this test design against the accepted integrated contract first; later implementation results are a separate evidence record and require their own acceptance decision.
