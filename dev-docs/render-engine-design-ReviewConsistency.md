# Render Engine Design — CONSISTENCY-AXIS REVIEW

**Review object:** `D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md` at `e168ecd815df679202cfd4bc60b9330df64e5305`; Draft design, committed 2026-10-02  
**Baseline:** Renderer `e168ecd815df679202cfd4bc60b9330df64e5305`; workspace root `a8f88fdbda3108005166a75545cc6c6b7ef5e2d4`. Both HEADs matched at review start and end. Sources were read from the unchanged working files at those HEADs using read-only commands.  
**Date:** 2026-10-02  
**Axis:** Consistency. Independent, adversarial, read-only; no reliance on the other axis.  
**Verdict: NO-GO** — five P2 findings. I pre-commit to GO on a revision that resolves P2-1 through P2-5 as specified.

---

## 0. Evidence base

I compared the design with `render-requirements.md`, `render-graph-schema-design.md`, `ir/gvid_render_graph.taut.py` at the renderer SHA, and `gvid-arch.md`, `gvid-requirements.md`, `ui-design-v2.md`, and `AGENTS_GWZ.md` at the root SHA. Neither repository had a tracked working-tree diff. A search including ignored files found no `CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, or `GwzProcessOptimization.md` under the root `dev-docs`.

## 1. Findings

### [P2-1] Preview composition ownership conflicts with the cited architecture

- **Location:** `render-engine-design.md:19,74`; `gvid-arch.md:64-66,209-210,339-343`; `render-graph-schema-design.md:14,75`.
- **Invariant:** A ready sequence preview is one already composited resource, with the Preview service and renderer owning composition.
- **Reproduction:** Implement the architecture’s normal-playback path: Preview sends active sources and transforms, and the UI GPU compositor assembles layers. That path cannot satisfy the design’s ready-result contract, which publishes one composed artifact. Implement the new path instead, and the architecture’s named UI compositor ownership and preview flow are false.
- **Impact:** Two cited controlling designs direct incompatible component interfaces and conformance targets. A two-layer preview could pass one design while failing the other.
- **Remedy:** Explicitly reconcile the superseded architecture clauses and flow with the v0.2 composite-resource contract, including the remaining role, if any, of the UI compositor.
- **Closure test:** A two-track sequence preview produces one leased composite resource; the production UI presents it without assembling graph layers, and the architecture’s component/flow description matches that behavior.

### [P2-2] The offline CLI has no defined way to satisfy the core’s mandatory authority incarnation

- **Location:** `render-engine-design.md:15,23,70,76,78`; `gvid_render_graph.taut.py:133-145`; `render-graph-schema-design.md:18,22`.
- **Invariant:** An ordinary portable graph/package must render offline through the same core without claiming a Glade project-open authority it does not possess.
- **Reproduction:** Download a graph snapshot and binding manifest, then invoke the offline CLI. `GraphSnapshot` carries project, graph, and revision but no authority incarnation; that value belongs to `GraphSnapshotDelivery`. The core request nevertheless requires an authority incarnation. The CLI must either fail or invent one under an unstated rule.
- **Impact:** The promised standalone path and Glade/CLI plan-parity test lack a valid common input contract. A fabricated incarnation could also be misreported as accepted-host provenance.
- **Remedy:** Define distinct governed-host and standalone-import request contexts, or define an explicit local incarnation with clear provenance and exclusion from semantic plan identity.
- **Closure test:** Render a downloaded graph/package with no Glade process or delivery envelope, then compare its normalized plan with the same graph and bindings supplied by Glade.

### [P2-3] The cache identity omits the binding-set identity required by the graph contract

- **Location:** `render-engine-design.md:82`; `render-graph-schema-design.md:67-69`; `render-requirements.md:41,95`; `ui-design-v2.md:256`.
- **Invariant:** Every bound sequence preview, cache, and export identity includes the immutable binding-set ID and content fingerprints. A rebind changes set identity without changing graph revision or digest.
- **Reproduction:** Rebind a slot to a new registered version with identical bytes, stream, and media properties. The design’s enumerated artifact-key inputs can remain identical, although the governing binding contract requires a new set ID. An unrelated-slot rebind likewise exposes the unresolved question of how unaffected subresults are reused while the consuming request has a new binding identity.
- **Impact:** Cache and evidence records cannot reliably distinguish the binding context required by the contract; implementations may disagree on safe cross-set reuse.
- **Remedy:** Specify binding-set ID in bound request, result, and artifact identity. If content-addressed subresults remain reusable across sets, define a separate verified reuse mapping that preserves the consuming set’s identity.
- **Closure test:** Exercise same-byte and different-byte rebinds and an unrelated-slot rebind. Verify new consuming identities, correct misses, and explicitly validated reuse of unaffected subresults.

### [P2-4] Runtime hardware fallback has no coherent artifact-key transition

- **Location:** `render-engine-design.md:27,30,56,82,91`; `render-requirements.md:131,133,156`.
- **Invariant:** An artifact’s identity and evidence must name the provider/build that actually produced it, including after permitted hardware fallback.
- **Reproduction:** Prepare a workflow selecting hardware. Its tasks already carry provider choice and expected artifact identity. During Run, the hardware path fails and policy permits software fallback. The precomputed identity names hardware, while the output was produced by software; using it miskeys the cache, and discarding it leaves dependent task identities undefined.
- **Impact:** Fallback can produce incorrect cache provenance or prevent safe downstream reuse and verification.
- **Remedy:** Define fallback as a new planned attempt, or specify how the failed task and every dependent artifact identity are recalculated before execution and publication.
- **Closure test:** Inject a hardware failure after Prepare, complete through the software reference path, and verify task keys, dependent keys, export record, cache hits, and output probe all identify the actual path.

### [P2-5] Restart reconciliation is absent from the standalone job lifecycle

- **Location:** `render-engine-design.md:15,76,82,84,88-91`; `render-requirements.md:106,126-127,154`.
- **Invariant:** Abandoned native jobs and incomplete artifacts must be detected on restart; a failed export must leave the previous final destination intact.
- **Reproduction:** Start a CLI export, terminate the CLI after a native process creates an intermediate or candidate output, then start a new CLI process. The design specifies in-process cancellation/failure cleanup and cache misses, but no durable job marker, startup scan, orphan-process reconciliation, or incomplete-artifact cleanup owner for the local scheduler.
- **Impact:** Abandoned processes or partial artifacts can survive indefinitely, and the required interrupted-job recovery cannot be demonstrated by the proposed proof points.
- **Remedy:** Assign restart reconciliation to the CLI’s local supervisor and define its durable markers, orphan detection, artifact cleanup or verified resume rule, and destination-publication recovery.
- **Closure test:** Kill an export at multiple pre-publication points, restart the CLI, and verify abandoned work is detected, partials are removed or safely resumed, and the previous destination and export record remain valid.

## 2. Invariant analysis

The design preserves the central editorial boundary: project authority owns accepted edits; the renderer consumes immutable graph and binding inputs. It also carries the separate source-preview path, exact-time intent, Taut control versus HTTP media delivery, typed tool semantics, and explicit final verification. The five findings concern places where those promises cannot yet be implemented consistently across the cited contracts or across Prepare, Run, fallback, and restart states.

## 3. Risks and next action

Revise the design and affected architecture clauses as one settled document change. Add the specified cases to the implementation proof points, especially offline import, binding-set identity, hardware fallback, two-layer preview ownership, and crash/restart recovery. Re-review those counterexamples at the new tuple.
