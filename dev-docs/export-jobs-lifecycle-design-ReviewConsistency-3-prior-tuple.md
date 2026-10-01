# ExportJobs Lifecycle Design — CONSISTENCY-AXIS FINAL RE-REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/export-jobs-lifecycle-design.md` and companion design, validator, and test changes at renderer `8540f07fc7189a91b9235395681c742d378673ae` and GWZ root `bfadedf8efe11de13b5470c215c281f7d81921b7`. The draft remains unaccepted pending the review decision.  
**Baseline:** Prior reviewed renderer `7445b3fb6ee9fb97ac0301105267870addcabbf8` and root `2868caeb9775fb79df43bbfebb5e08d7f67a1ae8`. Both HEADs matched the assigned tuple at review start and end. The renderer tree was clean. Root `dev-docs/ui-wireframe.svg` was untracked, unrelated, and excluded.  
**Date:** 2026-10-02  
**Axis:** Independent adversarial read-only Consistency review; the parallel current-round Safety report was not consulted.  
**Verdict: GO** — no open P0, P1, or P2 finding.

---

## Prior-finding closure table

| Prior finding | Result |
| --- | --- |
| Safety P2-1 — nonterminal retirement | **Closed at the contract level.** Full-record pruning requires a durable terminal state and completed process, lease, artifact, resource, and destination reconciliation (`export-jobs-lifecycle-design.md:31–33`). Restart and compaction remain named conformance cases (`:45`). |
| Consistency P2-1 / Safety P2-2 — event caller provenance | **Closed.** `ExportEventsQuery` carries the current governed incarnation or local import ID; the design requires branch authorization before replay and at every delivery point, with queue clearing and stream closure on revocation (`ir/gvid_render_graph.taut.py:403–407`; `export-jobs-lifecycle-design.md:25`). |
| Consistency P2-2 / Safety P2-3 — typed status unavailable | **Closed.** `ExportStatusResult` represents `found`, `unavailable`, and `stale_context`; only `found` carries a snapshot. Status and retirement are serialized (`ir/gvid_render_graph.taut.py:422–426`; `export-jobs-lifecycle-design.md:23`). |
| Safety P2-4 — cancel existence oracle | **Closed.** Unknown and inaccessible targets share `unavailable` without a terminal state; `already_terminal` requires job read authority (`export-jobs-lifecycle-design.md:27`; `ir/gvid_render_graph.taut.py:125–127,440–443`). |
| Round-2 Consistency P2-1 — diagnostic disclosure | **Closed.** The amendment and both companion designs require null diagnostics and null job payloads on unavailable or stale lookup, status, and event responses. The shared validator rejects a diagnostic or payload on every declared negative branch (`export-jobs-lifecycle-design.md:21–25,43`; `ir/validate_export_responses.py:12–36`). |
| Round-2 Safety P2-1 — accepted-job journal capacity | **Closed as a design rule.** Admission reserves a permanent accepted-ID slot and physically backed, restart-persistent capacity for bounded failure, terminal, cleanup, and publication reconciliation. Attempt and ownership growth require a reserve increase before expansion; ordinary event history cannot consume the reserve. Physical write failure retains ownership for recovery (`export-jobs-lifecycle-design.md:35–37`). |

## Changed-range analysis

The renderer change adds the two round-2 reports and RemPlan-2, revises the lifecycle amendment and both companion designs, adds `ir/validate_export_responses.py`, and extends its contract test. The root change is GWZ lock and marker bookkeeping; it changes no product contract. No Taut message layout or generated binding changed in this round.

The revised capacity rule agrees with the renderer supervisor’s ownership and publication order (`render-engine-design.md:96–102`) and the graph-schema ExportJobs boundary (`render-graph-schema-design.md:87–94`). The permanent accepted-ID slot survives tombstone conversion; temporary reserve release follows terminal reconciliation and tombstone persistence. The separate bounded event budget can expire a prefix with an explicit replay gap while the retained full record preserves current state and terminal outcome. I found no new architectural root cause in this changed range.

## 0. Evidence base

I inspected the committed changed range, the controlling amendment, both companion designs, renderer requirements, Taut declarations, generated message types, fixtures and tests, the earlier ExportJobs reports and remediation plans, and the render-engine ReviewDecision. I checked root `AGENTS.md`, `AGENTS_GWZ.md`, `gvid-requirements.md`, `gvid-arch.md`, `ui-design-v2.md`, and `GvidTautProto-Feedback.md`. The named `CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, and `GwzProcessOptimization.md` are absent from root `dev-docs`; they supplied no additional controlling clause.

The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` passed **7 tests**. Inspection and testing changed neither reviewed tree.

## 2. Invariant analysis

For lookup, status, and event delivery, `validate_export_response` recognizes each declared negative status and rejects a non-null snapshot or event and a non-null `diagnostic_id`. Thus a job-specific diagnostic in those declared fields cannot pass the reference validator on an unavailable or stale response. The test exercises representative negative and positive envelopes. Complete response indistinguishability, query-field echo, authorization, and client-side validation remain host and client conformance obligations; the Python test does not claim to prove them.

The journal rule now covers the round-2 quota counterexample: ordinary progress or event growth cannot consume an accepted job’s reserved transition space; a new attempt or owned resource must fit the admitted bound or first obtain a durable reserve increase. Publication intent and rollback or completion records have reserved space before replacement. A physical write failure still fails closed, retaining ownership until durable reconciliation is possible. The near-quota crash/restart case is explicitly required for implementation conformance.

## 3. Risks and next action

**GO on this draft contract tuple.** The seven Python tests establish the reference validator and wire fixtures, not runtime authorization, quota enforcement, crash recovery, or Rust/TypeScript execution parity. Those proofs remain required at the implementation gate. No files or git state were modified.
