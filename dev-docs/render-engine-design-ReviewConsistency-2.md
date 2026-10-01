# Render Engine Design — CONSISTENCY-AXIS RE-REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/render-engine-design.md`, its remediation patch, and the revised root architecture and composite-preview ADR, all at the exact tuple below. The design remains unaccepted.  
**Baseline:** Renderer `3e6e4883de5f43391e3df842dbd5a450dac635db`; gvid-wz root `567da536bf08a0d84b169c5408ff8dc019b0e87e`. Read-only `git rev-parse HEAD` matched both at start and end. Relevant working files had no diff from HEAD.  
**Date:** 2026-10-02  
**Axis:** Consistency; independent, adversarial, read-only.  
**Verdict: NO-GO** — two P2 findings and one P3 finding. The two P2 corrections are bounded; I pre-commit to GO if their stated closure tests pass on a settled tuple and no further blocker appears.

---

## Prior-finding closure table

| Prior ID | Disposition claimed | Original counterexample re-traced on corrected tuple | Status |
| --- | --- | --- | --- |
| Consistency P2-1 | ADR and architecture assign sequence composition to Preview/renderer. | `gvid-arch.md:40,64-66,117,209-210,339-347` now describes one composed resource, but the controlling UI design’s `GVR-UI-014` trace still says the Preview adapter consumes a layered plan (`ui-design-v2.md:240`). | **Partial; P2-1 below.** |
| Consistency P2-2 | Governed-host and standalone-import provenance are separate. | `render-engine-design.md:25,27,84` gives offline import a local ID without inventing a host incarnation; plan parity excludes provenance. | Contract closed; implementation proof remains due. |
| Consistency P2-3 | Bound consuming identities carry binding-set ID/revision; cross-set subresults require validation. | `render-engine-design.md:90` distinguishes consuming identity from checked content-addressed reuse, including same-byte and unrelated-slot rebinds. | Contract closed; implementation proof remains due. |
| Consistency P2-4 | Hardware fallback creates a new attempt and recalculates dependent keys. | `render-engine-design.md:34,60,90` prohibits relabeling failed-attempt artifacts and records actual provider/build. | Contract closed; implementation proof remains due. |
| Consistency P2-5 | CLI supervisor journals and reconciles jobs, artifacts, orphans and publication. | `render-engine-design.md:84,94-96` assigns startup reconciliation and prior-destination recovery to the local supervisor. | Contract closed; implementation proof remains due. |
| Safety P2-1 | ExportJobs Taut service, schema, bindings and fixtures added. | Submit, events, status and cancel now exist at `ir/gvid_render_graph.taut.py:59-69,360-408`. The original absence is corrected. The new job-ID-only recovery gap is P2-2 below. | Original counterexample closed; new blocker. |
| Safety P2-2 | Profile ID/version/digest and effective settings pinned. | `render-engine-design.md:27,80,90,96` carries the pinned definition through planning, cache, verification and publication. | Contract closed; implementation proof remains due. |
| Safety P2-3 | Durable ownership and restart reconciliation added. | `render-engine-design.md:72,94-96` covers staged sources, native processes, intermediate artifacts and publication. | Original counterexample closed; implementation proof remains due. |
| Safety P2-4 | Measured native and spool limits added. | `render-engine-design.md:92` requires enforceable grants, bounded queues and a capacity outcome. | Contract closed; implementation proof remains due. |

## Changed-range analysis

The renderer revision changes the design at `render-engine-design.md:3,21,25-27,31-34,60,72,80-84,90-96,103-104`; adds the ExportJobs contract at `render-graph-schema-design.md:80-104` and `ir/gvid_render_graph.taut.py:59-69,108-117,360-408`; adjusts the requirements status; and adds fixtures, generated Python/Rust/TypeScript bindings and contract tests. The root revision adds `adr-sequence-preview-composite.md:1-24` and revises architecture ownership, diagrams, preview flow and planned presentation design in `gvid-arch.md`. Root `gwz.conf` lock/marker changes are outside this review object.

The remediation did not update `ui-design-v2.md:240`, leaving one old layered-plan route in a controlling document. **NEW ARCHITECTURAL ROOT CAUSE:** the added ExportJobs boundary supplies job recovery only by a server-assigned job ID, with no defined recovery path when acceptance is durable but its acknowledgement is lost.

## 0. Evidence base

I read the prior Consistency and Safety reports and merged remediation plan, renderer requirements, graph schema design, Taut schema, fixtures, generated bindings, tests, root architecture, product requirements, UI design v2 and new ADR. An unignored workspace search found no `CurrentProgramCheckpoint.md`, `AgentProcessRules.md` or `GwzProcessOptimization.md`; `AGENTS.md` and `AGENTS_GWZ.md` are present. The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` command passed six tests. Those tests establish Python schema round trips and golden bytes, not host lifecycle behavior or cross-language execution.

## 1. Findings

### [P2-1] The UI requirement trace still directs the adapter to consume a layered plan

- **Root cause and location:** The ADR supersedes architecture AD-07’s UI composition path (`adr-sequence-preview-composite.md:5,10-16`), and the architecture now describes a completed resource (`gvid-arch.md:64-66,339-347`). The controlling UI design’s `GVR-UI-014` trace still assigns the Preview adapter a “layered plan” (`ui-design-v2.md:240`), despite that document’s resource-only description at lines 158-162.
- **Violated invariant:** A ready production sequence preview is one renderer-composited resource; no browser adapter receives an editorial layer plan to compose.
- **Reproduction:** Implement gate C from the `GVR-UI-014` trace. An adapter that consumes a layered plan cannot obtain that plan from `Preview.sequence_frame`, which returns one resource descriptor. An adapter that consumes only the descriptor does not satisfy the trace as written.
- **Impact:** The accepted UI gate and revised renderer contract can yield opposite conformance decisions for the same two-track preview.
- **Required correction:** Amend the `GVR-UI-014` trace to say the production Preview adapter resolves and presents the single composed resource. Align its gate language with the ADR.
- **Closure test:** Run the two-track UI contract gate using only a leased composite descriptor and verify the adapter performs no graph-layer assembly.

### [P2-2] A durably accepted export cannot reliably be recovered after its acknowledgement is lost

- **Root cause and location:** The host assigns `job_id` only after durable acceptance (`render-engine-design.md:80`). `export.status`, `export.events` and `export.cancel` require that ID (`ir/gvid_render_graph.taut.py:62-69,392-404`); none accepts the client’s `request_id`. Retry is keyed by project/request ID, but “canonical logical payload” does not say whether it excludes the changing host incarnation or an expired destination reference (`render-graph-schema-design.md:84`; compare the explicit Editor retry exclusion at line 54).
- **Violated invariant:** Once export acceptance is durable, an authorized client must be able to discover its job and terminal status after a lost response or project reopen, without starting a duplicate export.
- **Reproduction:** The host records an export, assigns a job ID, then crashes or loses the acknowledgement before the client receives it. On reopen the client knows only its request ID. It cannot call status or events. Resubmission changes the incarnation; the original destination reference may also have expired. The contract neither guarantees a replay before reference revalidation nor defines whether those changed fields conflict with the recorded logical payload.
- **Impact:** An accepted export can continue or finish while the client cannot correlate, cancel or report it. A retry may be rejected or start unintended work under an implementation-dependent rule.
- **Required correction:** Provide an authorized `(project_id, request_id)` lookup, or define cross-incarnation replay that returns the original job ID before revalidating an expired reference, with precise canonical-payload and authorization rules.
- **Closure test:** Fault after durable admission but before acknowledgement, reopen with a new incarnation and expired original destination reference, then recover exactly the original job and its status by the retained request ID. Confirm no second job or publication occurs.

### [P3-1] The composite proof point lacks an output oracle

- **Root cause and location:** `render-engine-design.md:103` and `adr-sequence-preview-composite.md:24` require a two-layer preview to yield one leased “composite” resource, but do not require comparing its pixels with the declared layer semantics.
- **Violated invariant:** A one-resource result must contain every active visible layer (`gvid-requirements.md:164`; `adr-sequence-preview-composite.md:14,18`).
- **Reproduction:** A provider returns one valid descriptor whose bytes show only the top track. It meets the stated count, lease and presentation checks while omitting the lower track.
- **Impact:** The proposed proof can pass a false composition.
- **Required correction:** Use a fixture with independently recognizable overlapping layers and compare the decoded ready frame against a reference composite at a specified time and fidelity tolerance.
- **Closure test:** The proof fails when either layer is omitted, duplicated or ordered incorrectly, while the one-resource and lease checks still pass.

## 2. Invariant analysis

The revised contracts now give the offline CLI legitimate provenance, distinguish binding context from validated subresult reuse, replan hardware fallback, pin output profiles and assign durable restart cleanup to both supervisors. The architecture’s old UI GPU compositor components and normal-playback flow have been revised. The remaining UI trace is the specific unreconciled path.

The ExportJobs schema can encode the declared request, event, status, cancel, profile and provenance fields. Its optional provenance fields and event/result combinations require the stated host semantic validation; generated bindings alone do not enforce them (`render-graph-schema-design.md:88,104`). The current round-trip tests do not exercise a lost acceptance acknowledgement. The revised reuse and fallback proof points name the relevant identity transitions, but production conformance still depends on the specified implementation tests.

## 3. Risks and next action

Correct the UI trace and the request-ID recovery contract as one settled amendment, then re-run the focused counterexamples at the new tuple. Strengthen the two-layer fixture with a pixel oracle before treating the preview proof as evidence of correct composition.
