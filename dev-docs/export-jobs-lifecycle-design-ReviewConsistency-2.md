# ExportJobs Lifecycle Design — CONSISTENCY-AXIS RE-REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/export-jobs-lifecycle-design.md` and its companion design, schema, generated-binding, fixture, and test remediation at renderer `7445b3fb6ee9fb97ac0301105267870addcabbf8` and GWZ root `2868caeb9775fb79df43bbfebb5e08d7f67a1ae8`. The draft remains unaccepted.  
**Baseline:** Prior reviewed renderer `c58006bb7edacb872b326cc2e2cc36df961a25a5`; root `23b4a543fcb9b945505a66ea6f79637f2cf186e6`. Both current HEADs matched the specified tuple at review start and end. Both working trees were clean.  
**Date:** 2026-10-02  
**Axis:** Independent adversarial read-only consistency review; no current-round Safety report consulted.  
**Verdict: NO-GO** — one P2 finding; no P0, P1, or P3 findings.

---

## Prior-finding closure table

| Prior finding | Re-review result |
| --- | --- |
| Safety P2-1 — nonterminal retirement | **Closed as a design rule.** The amendment requires durable terminal state and completed process, lease, artifact, resource, and destination reconciliation before pruning (`export-jobs-lifecycle-design.md:31–35`). Both companion designs agree (`render-engine-design.md:84`; `render-graph-schema-design.md:88`). The queued/running/verifying crash sequence is now a stated conformance obligation (`export-jobs-lifecycle-design.md:41`). Runtime proof remains due. |
| Consistency P2-1 and Safety P2-2 — event caller provenance | **Closed at the contract level.** `ExportEventsQuery` carries exactly one caller incarnation or local import ID (`ir/gvid_render_graph.taut.py:403–407`); the service uses it at `:65–67`. The amendment requires authorization before replay and at each delivery point, queued-event clearing, and stream closure on revocation (`export-jobs-lifecycle-design.md:25`). The companion designs and generated Python, Rust, and TypeScript message types agree. Revocation behavior remains an implementation proof obligation. |
| Consistency P2-2 and Safety P2-3 — typed status unavailable | **Closed at the contract level.** `export.status` returns `ExportStatusResult` with `found`, `unavailable`, and `stale_context`; only `found` has a snapshot (`ir/gvid_render_graph.taut.py:68–70,119,422–426`; `export-jobs-lifecycle-design.md:23,39`). Found, unavailable, and stale fixtures round-trip. |
| Safety P2-4 — cancel existence oracle | **Closed for cancel.** Unknown and inaccessible targets return `unavailable` without a terminal state; `already_terminal` requires job read authority (`export-jobs-lifecycle-design.md:27`). Former numeric codes 3 and 4 are reserved, with code 6 assigned to `unavailable` in the schema and generated bindings (`ir/gvid_render_graph.taut.py:125–127`). The new finding below concerns other negative responses. |

## Changed-range analysis

The substantive renderer diff revises the amendment and both companion designs, changes the ExportJobs Taut service and messages, regenerates Python/Rust/TypeScript types, and adds eight fixtures plus contract-test cases. The root diff contains GWZ workspace bookkeeping, not a changed product contract. No new architectural root cause was identified. The new finding is an incomplete negative-response rule in the changed wire contract.

## 0. Evidence base

I compared the changed range with renderer requirements, the render-engine and graph-schema designs, the first-round reviews and remediation plan, the prior render-engine ReviewDecision, the Taut schema, generated bindings, fixtures, and tests. I checked the named root requirements, architecture, UI design, and Taut feedback. Root `AGENTS.md` and `AGENTS_GWZ.md` were read. The named `CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, and `GwzProcessOptimization.md` are absent from root `dev-docs`.

The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` passed six tests. These establish Python schema encoding, round trips, and golden digests. They do not execute host authorization, revocation, retirement, or cross-language interoperability.

## 1. Findings

### [P2-1] Optional diagnostics reopen an existence distinction on unavailable status

**Root cause and exact location:** The new `ExportStatusResult` and `ExportEventDelivery` messages each permit a `diagnostic_id` on an `unavailable` response (`ir/gvid_render_graph.taut.py:408–412,422–426`). The amendment groups pruned, unknown, and inaccessible status targets under `unavailable` and requires no snapshot, but does not constrain that diagnostic field (`export-jobs-lifecycle-design.md:23,39`). It similarly requires the same unavailable event delivery for unknown and inaccessible jobs without defining diagnostic equality (`:25`). The companion status rules repeat the snapshot restriction without a diagnostic restriction (`render-graph-schema-design.md:90,92`). The negative fixtures set the field to null, but the tests only round-trip those fixtures (`tests/test_taut_contract.py:354–376`).

**Violated invariant:** A caller without job read authority must not learn whether a candidate job exists through a negative ExportJobs response. The same `unavailable` enum is insufficient if another response field can distinguish the cases.

**Credible reproduction:** With a current project context, query `export.status` for a real but inaccessible job and for an unknown job. The declared wire permits the former to return `status=unavailable, snapshot=null, diagnostic_id="job-access-denied-42"` and the latter to return the same status and snapshot with `diagnostic_id=null`. Both satisfy the explicitly stated status/snapshot rule, yet reveal which ID exists. The same distinction is representable in an unavailable event delivery. No runtime implementation is needed to establish that these are legal shapes under the current written validation rule.

**Impact:** The new status or event path can disclose accepted-job existence even though lookup and cancel use unavailable outcomes to avoid that disclosure. Implementations can make incompatible choices about negative diagnostics while claiming conformance.

**Required correction:** Specify and validate a single non-disclosing diagnostic policy for unavailable results across lookup, status, and event delivery. For example, require `diagnostic_id=null` for unknown, inaccessible, and pruned targets, with detailed diagnostics retained only behind an authorized `found` or `event` branch. Define the same rule in both companion designs.

**Closure test:** For the same caller and candidate identifier, compare the complete encoded unavailable status and event responses with the identifier absent, present but inaccessible, and pruned. Aside from fields that merely echo the identical query, the responses must be indistinguishable and contain no job-specific diagnostic. Verify the host semantic validator rejects a negative response that carries one.

## 2. Invariant analysis

The revised provenance query, typed status result, cancel enum, and terminal-pruning preconditions are coherent across the amendment, companion documents, Taut declarations, and generated message types. The Rust service and client files remain registration sketches; their message types reflect the new wire, while executable Rust service interoperability is still a future conformance gate. The current fixtures cover both valid event-query branches and positive/unavailable delivery, but do not prove stale or wrong-branch rejection, revocation barriers, or negative-response indistinguishability. Those runtime obligations are stated in `export-jobs-lifecycle-design.md:41`.

The new status and event envelopes can represent their intended success and unavailable branches. Their optional diagnostic fields also represent the prohibited disclosure sequence above because the negative-branch semantics do not restrict them.

## 3. Risks and next action

**NO-GO.** Define the non-disclosing diagnostic rule, add its semantic validation and closure tests, then re-review the corrected tuple. No files or git state were modified.
