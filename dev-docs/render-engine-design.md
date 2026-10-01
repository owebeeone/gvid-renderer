# GVid Render Engine Design

**Status:** Draft design for the Taut v0.2 requirements. This revision is proceeding without a new review loop at the product owner's request.
**Scope:** Headless render library, pluggable media tools, asset resolution, and separate Glade and CLI clients.
**Controls:** [renderer requirements](render-requirements.md), [flat graph and Taut design](render-graph-schema-design.md), [system architecture](../../dev-docs/gvid-arch.md), and [UI design v2](../../dev-docs/ui-design-v2.md).

## 1. Boundaries and ownership

The project service remains the only authority for accepted edits, graph revisions, binding changes, undo, and persistence. The renderer receives an immutable accepted graph snapshot and immutable binding set. A browser intent goes through the Editor service; neither the core library nor a render tool accepts a browser-authored native command or mutates editorial state. The flat graph stays small and editable in the project/UI model. A render workflow is derived from it and can be discarded.

| Part | Responsibility | Excluded responsibility |
| --- | --- | --- |
| Render core library | Validate render inputs; normalize a sequence or source request; create typed render IR and a workflow; select capable tools; calculate cache identities; execute admitted tasks; return artifacts and evidence. | Taut transport, HTTP endpoints, Glade authentication, CLI parsing, accepted edit authority. |
| Glade integration client | Adapt Taut requests/events to the core, obtain accepted snapshots and bindings, supply Glade asset access, map jobs to host supervision, and publish resource descriptors. | Reinterpreting composition semantics or embedding encoded media in Taut. |
| CLI client | Load a portable graph/package and bindings, choose a resolver and profile, run the same core with a local scheduler, verify and publish an export. | Requiring a Glade process for ordinary local/package rendering. |
| Tool providers | Implement typed media operations using pinned executable or library builds, report support and evidence. | Reading or editing the project graph, resolving user credentials, choosing product semantics. |
| Asset resolver providers | Resolve logical registered assets or bound slots to version-checked source access and leases. | Changing graph meaning or substituting a different asset silently. |

The library may be linked into a host process or run in a supervised sidecar without changing these contracts. A sidecar boundary uses Taut for structured requests, results, cancellation, and progress. The Glade host owns the authenticated loopback HTTP byte endpoint; the core only returns artifact handles and metadata. The command/event gateway owns transport, authorization, and correlation, while the preview service owns the request contract and fidelity policy. The render planner and effect registry own semantic lowering; the job supervisor owns global admission and process lifecycle.

## 2. Core API and execution flow

A core request identifies a graph ID, accepted graph revision and authority incarnation, sequence or registered source identity, immutable binding set where applicable, exact half-open time range, profile, fidelity, and request/job ID. A source preview can name a registered asset version and stream without a graph or binding set. The core accepts validated records corresponding to the governed Taut schema; it does not require a Taut network runtime for in-process calls.

The principal API has two stages:

1. **Prepare:** Validate schema and graph semantics, resolve and pin source identities, expand the requested output range through effects and source handles, build typed render IR, negotiate providers and intermediate formats, calculate cache keys and resource estimates, and return a workflow with actionable diagnostics. No media job starts on an invalid workflow.
2. **Run:** Submit workflow tasks to an injected scheduler, consume version-checked asset handles, reuse verified artifacts where keys match, run providers under cancellation and resource grants, verify outputs, and return an evidence-bearing result. Publication of a final destination happens only after final verification.

The workflow is a dependency DAG of typed tasks. Each task records its exact input and output time domains, format, semantic operation version, temporal handles, dependencies, resources, provider choice, and expected artifact identity. It is not serialized as the editable project or treated as a native command line. The same planner handles a single frame, a short preview range, and a complete export; the task selection and admission policy differ.

For a seek near the end of a sequence, planning walks backward from the requested output frame or range through only contributing layers and effects. It includes decoder preroll, transition handles, audio filter state, and source timing dependencies, then schedules the requested visible result before bounded adjacent prefetch. It never makes sequence-start rendering a prerequisite unless a declared effect truly requires earlier state. Batch planning can partition a full range, share valid subresults, and assemble only after boundary semantics and verification are satisfied.

The core returns a result containing request context, actual range/time, exact or proxy fidelity, artifact handles, selected provider/build manifest, source fingerprints, warnings, diagnostics, and verification evidence. A Glade client maps this to Taut results and resource descriptors; a CLI writes a file and export record. Cancellation is best effort for a running native process, but stale results never become current UI resources or published exports.

## 3. Pluggable render tools

Start with a pinned FFmpeg/FFprobe provider and a software reference path. The provider interface must allow later libav, GStreamer, MLT, hardware, or specialist implementations without changing graph semantics. Registration can initially be static in the application build; any future dynamically loaded provider must be host-approved and version-pinned. An arbitrary executable path from a project file is never a provider registration mechanism.

Use one small common provider contract plus typed operation capabilities. A provider advertises a stable ID, build/configuration, supported operation and semantic versions, input/output formats, exact or proxy fidelity, temporal handle requirements, deterministic behavior claims, resource needs, and available hardware paths. It assesses a typed task before selection, then executes only tasks it accepted. Narrow capability implementations may cover probing, decode, timed cuts/retiming, video composition and transitions, color conversion, audio gain/envelopes/mixing, encoding, muxing, and final verification. A provider need not implement every operation.

The language-neutral shape is:

    RenderTool.descriptor() -> ToolDescriptor
    RenderTool.assess(TypedTask, HostCapabilities) -> Supported | Unsupported(reason)
    RenderTool.execute(TypedTask, InputHandles, ArtifactSink, TaskContext) -> TaskEvidence | ToolError

    AssetResolver.resolve(ResolveRequest) -> ResolvedAsset | ResolveError
    SourceAccess.open_range(exact_source_range) -> byte stream or seekable handle
    SourceLease.refresh() / release()

TypedTask carries a semantic operation ID and version, validated parameters, exact time conversion rules, requested fidelity, and accepted intermediate media formats. TaskContext carries cancellation, progress reporting, a resource grant, and a diagnostic correlation ID. ToolError has stable categories such as unsupported semantics, missing input, changed input, expired access, capacity, tool failure, and invalid output. The adapter may attach native stderr to a protected diagnostic record, not to the portable graph.

The effect registry defines what an operation means and how it lowers to tasks. Tool providers implement that meaning; they cannot redefine it through hidden defaults. For example, an audio fade task identifies the source/timeline domain, half-open fade interval, exact gain endpoints, curve and curve version, channel behavior, sample rounding, and treatment at cut boundaries. A video fade or dissolve similarly fixes layer order, alpha/color space, interval, and boundary samples. A provider must advertise support for that semantic version or reject it. Unsupported effects cannot become an approximate export by accident; an allowed proxy must be declared in the result.

Provider selection validates an entire decode/filter/encode path and compatible intermediate formats. If conversion is needed, it becomes an explicit typed task with color, channel, sample-rate, and timing rules. Hardware fallback to the software reference path follows the request policy and is recorded. FFmpeg is invoked directly without a shell, using arguments or filter scripts built from validated typed tasks; provider executables and builds are pinned or probed by the capability service. The core records the actual provider choices in every export and cache identity.

Provider conformance fixtures must compare the same fade, mix, transition, and boundary range through the reference provider and each candidate provider at declared exact/proxy tolerances. Verification checks output streams and timing after muxing, not merely process exit status.

## 4. Asset resolution and Glade-only sources

The graph contains logical slots and expected media properties, never paths, URLs, tokens, or credentials. A separate immutable binding set names registered asset versions, selected streams, fingerprints, and substitution policy. Resolution is a distinct runtime step. A ResolveRequest names either a bound slot or an independently registered asset version for source preview, plus purpose (preview, export, probe, packaging), required source range and access capabilities.

ResolvedAsset contains verified identity/fingerprint, selected stream metadata, a SourceAccess handle, access characteristics, and a lease. Access characteristics state whether random seeking and byte ranges are supported, whether a stable local file can be supplied, whether bounded materialization is permitted, and when access expires. The resolver may be local-file, portable-package, or Glade-backed. The tool provider sees only SourceAccess or a validated staged file; it never receives a Glade credential or learns how the asset was registered.

Glade may be the only place that knows where an asset came from or how to authenticate to it. The Glade resolver therefore maps the privileged asset registry entry to an opaque, authorized source handle, maintains or renews its lease, and reports source availability and stream metadata through the existing MediaCatalog/AssetBindings contracts. It must detect a changed version or fingerprint before execution and while consuming an export. The request pins the immutable version; a change fails the job or requires an explicit new binding set. It cannot silently use a proxy, another URL, or another registered asset.

Some tools require a seekable filesystem path. A staging adapter may materialize the required source range or whole source only when the resolver allows it, after checking identity, free space, and bounded quota. It validates the staged bytes, cleans temporary material on cancellation/expiry, and keeps credentials out of logs and manifests. Interactive late seeks prefer range-capable access and local range artifacts; they must not silently trigger a full Glade-source download before yielding a frame. If a source cannot support the requested range within policy, the result is an explicit unavailable/slow-path status that the UI can show.

A portable graph can be shared without source bytes. A portable render package may additionally contain explicitly included, content-addressed source media and a binding manifest; export must respect source access and redistribution policy. The standalone CLI first resolves local paths or package content against the manifest fingerprints. A Glade-only source has two supported paths: an explicitly materialized authorized package, or an optional Glade resolver bridge configured by the user and authenticated at runtime. If neither is available, the CLI reports the missing logical asset/version and a way to supply or package it. A shared graph remains inspectable even when its assets cannot yet be rendered. No credential or machine-local locator is exported with the graph.

## 5. Separate clients and delivery

**Glade integration.** The Glade client obtains accepted snapshots, binding sets, catalog records, and resource grants from their owning services. It translates governed Taut preview/export/cancel/release messages to core calls and publishes Taut progress and result messages with the same request, revision, binding, and authority context. It rejects stale requests before execution where possible and stale results before presentation. The host maps a ready frame or finite preview segment artifact to a leased opaque resource descriptor. The authenticated loopback HTTP endpoint serves only its encoded bytes, including byte-range requests over a complete seekable artifact; it does not expose command APIs or machine paths. The Glade job supervisor schedules preview fairly against exports and releases resources when viewers replace, close, or expire.

**CLI.** The CLI loads and validates a graph or package, chooses an explicit sequence/range/profile/binding manifest and asset resolver, invokes the same prepare/run API, and writes a verified output plus evidence record. It has its own local scheduler and cancellation handling. Human-facing terminal progress is a presentation concern; any structured process-to-process CLI mode uses the governed Taut messages and wire. The baseline CLI runs offline with local or packaged assets and does not import Glade service dependencies into the core. Optional Glade access is an asset-resolver plugin, not a second render implementation.

Both clients must produce the same normalized semantic plan for the same graph, binding set, profile, range, fidelity and capability set. Different provider choices or machines may produce different encoded bytes within declared tolerances; provenance and verification make those differences visible. A preview artifact is disposable and cannot be published as a final export without the requested final profile and verification.

## 6. Cache, failure, and evidence rules

Artifact identities include the contributing subgraph semantic digest, graph/effect schema versions, bound source fingerprints and selected streams, exact range and handles, output profile, fidelity, color/audio policy, chosen provider/build and intermediate format versions. A binding-only rebind changes affected artifact keys while leaving the graph digest untouched. An artifact is reusable only after its metadata and bytes validate against the consuming request. Failed, partial, or mismatched artifacts are misses, never successful outputs.

The scheduler accepts resource estimates from the workflow and owns CPU, memory, I/O, disk, and accelerator admission. Preview work has bounded priority over long batch work; exports retain fair progress. Every task and result carries job/request correlation. On cancellation or failure, the core releases source leases and staging, the host reclaims resource descriptors, and the final destination remains intact. Final outputs are probed, checked against profile and timing, then atomically published with an export record. Logs and records identify source fingerprints and provider builds without exposing source credentials or raw media content.

## 7. Initial implementation slices and proof points

1. Define core request/result, typed task/workflow, SourceAccess/AssetResolver, RenderTool, and scheduler/artifact-store interfaces. Build a graph fixture that plans a late frame and a full export without any network runtime.
2. Implement local and package resolvers, pinned FFmpeg/FFprobe provider, software reference operations and final verifier. Expose an offline CLI using the shared core.
3. Implement the Glade integration and resolver, Taut request/result mapping, lease-aware HTTP artifact publication, and host job supervision. Keep Glade credentials entirely inside the integration/resolver boundary.
4. Add conformance cases: CLI and Glade plan parity; late seek without whole-sequence rendering; audio fade and video dissolve at range boundaries; cache invalidation after binding change; missing Glade-only asset offline with an actionable error; authorized package or bridge resolution; cancellation and lease cleanup; changed source during export; final output verification.

The first implementation should pin precise audio fade curves, transition/color semantics, supported source-access modes, and profile set before claiming provider parity. The exact dynamic plugin loading policy and remote-source bridge protocol can be decided after the in-process interfaces and offline CLI prove the boundary.
