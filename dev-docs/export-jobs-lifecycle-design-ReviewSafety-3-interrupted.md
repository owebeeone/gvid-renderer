# ExportJobs Lifecycle Design — SAFETY-AXIS FINAL RE-REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/export-jobs-lifecycle-design.md` and companion design, validator, and test diff at renderer `8540f07fc7189a91b9235395681c742d378673ae` and GWZ root `bfadedf8efe11de13b5470c215c281f7d81921b7`. The draft remains unaccepted.  
**Baseline:** Prior reviewed renderer `7445b3fb6ee9fb97ac0301105267870addcabbf8`; root `2868caeb9775fb79df43bbfebb5e08d7f67a1ae8`. At review start, both HEADs matched the requested tuple. At review end, renderer still matched, but root HEAD had moved to `0f2f1320be97b007b3e4d5276cbd7df9357fdcc7`. The root change committed the previously untracked, out-of-scope `dev-docs/ui-wireframe.svg` and workspace bookkeeping.  
**Date:** 2026-10-02  
**Axis:** Independent adversarial read-only Safety review; parallel current-round Consistency report not consulted.  
**Verdict:** **Withheld.** The root HEAD moved during review, so the requested exact-tuple gate cannot receive a GO or NO-GO verdict from this report.

---

## Prior-finding closure table

| Prior finding | Observation at the requested renderer commit |
| --- | --- |
| Safety P2-1, nonterminal retirement | The amendment requires durable terminal state and reconciliation of attempt, process, lease, artifact, resource, and publication ownership before full-record pruning (`export-jobs-lifecycle-design.md:31`). |
| Consistency P2-1 / Safety P2-2, event caller provenance | `ExportEventsQuery` carries the current governed incarnation or local import ID. The amendment requires authorization before replay, rechecking at delivery, queued-event clearing, and stream closure after revocation (`:25`). |
| Consistency P2-2 / Safety P2-3, typed status unavailable | `ExportStatusResult` has found, unavailable, and stale-context branches. The amendment permits a snapshot only for found and serializes status lookup with retirement (`:23`). |
| Safety P2-4, cancel existence oracle | Unknown and inaccessible jobs share `unavailable`, with no terminal state; `already_terminal` requires job read authority (`:27`). |
| Round-2 Safety P2-1, accepted-job journal capacity | Admission now reserves a permanent accepted-ID slot and physically backed, restart-persistent space for bounded failure, terminal, cleanup, and publication-reconciliation records. Growth requires a reserve increase before new ownership; ordinary event history cannot consume the reserve. Physical write failure retains ownership for recovery (`:35–39`). |
| Round-2 Consistency P2-1, negative diagnostic disclosure | Lookup, status, and event negative branches require null payload and diagnostic ID. The new reference validator rejects those fields, and tests cover that rejection (`:21–27,43`; `ir/validate_export_responses.py:11–36`). |

These are contract observations, not runtime closure proofs. The HEAD discrepancy prevents a final gate determination.

## Changed-range analysis

The renderer diff since the prior tuple adds the round-2 reports and plan, changes the lifecycle amendment and two companion designs, and adds `ir/validate_export_responses.py` plus contract tests. The Taut field layout is unchanged. The validator checks the status-dependent payload and diagnostic fields of lookup, status, and event responses; it does not establish caller authorization or validate submit acknowledgements.

The requested root diff contained workspace lock and marker changes. At review start, `dev-docs/ui-wireframe.svg` was untracked and explicitly out of scope. It became part of a different root commit during review. I did not treat that file as part of the ExportJobs object.

## 0. Evidence base

I read root `AGENTS.md` and `AGENTS_GWZ.md`; the lifecycle amendment, both rounds of filed reports and remediation plans, renderer requirements, render-engine and graph-schema designs, prior render-engine ReviewDecision, Taut ExportJobs declarations, validator, fixtures, tests, and the named root requirements, architecture, UI design, and Taut feedback. Root `dev-docs/CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, and `GwzProcessOptimization.md` are absent.

The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` passed **7 tests**. These exercise schema and reference-validator behavior, not host authorization, crash recovery, journal reservation, or cross-language execution. Inspection made no filesystem or git changes.

## 1. Finding requiring settled-tuple adjudication

### [P2-1] NEW ARCHITECTURAL — submit distinguishes an inaccessible accepted ID from an unused ID

**Root cause and location:** Request IDs are unique across the governed project namespace, while an existing live record requires project/**job** read authority and a new ID requires project/graph read and destination write authority (`export-jobs-lifecycle-design.md:9,17–19`). For a retired ID, a caller lacking the original job’s read authority receives an unauthorized response before original profile or destination validation (`:33`). An unused ID proceeds to new-request validation. `ExportSubmitStatus` has distinct `unauthorized` and `invalid` values (`ir/gvid_render_graph.taut.py:112–115`). The negative-response rule and reference validator cover lookup, status, and events, but not this submit distinction (`export-jobs-lifecycle-design.md:43`; `ir/validate_export_responses.py:11–16`).

**Violated invariant:** A caller without an accepted job’s read authority must not learn whether a candidate request ID belongs to that job or its tombstone.

**Credible sequence:** User A accepts request ID `R` for a job that user B cannot read. B has a current project incarnation, graph read authority, and destination write authority, so B may submit new work. B submits `R` with an intentionally invalid profile. The index hit and job-read check yield `unauthorized`. B submits an unused ID `R2` with the same invalid profile; it reaches new-request validation and yields `invalid`, without creating a job or tombstone. The distinction confirms that `R` was accepted. It persists after `R` becomes a tombstone. Nulling `ExportJobAck.job_id` would not remove the status oracle.

**Impact:** Submit can disclose another user’s accepted-job or tombstone existence despite the amendment’s claim that an unauthorized caller receives no evidence of a tombstone. This is an authority-namespace problem, not a missing optional-field check.

**Required correction:** Define an authorization partition for request-ID ownership that makes an inaccessible accepted ID indistinguishable from an unused ID to a caller allowed to submit within that partition. One possible design is to scope accepted IDs by a non-reused owner or capability namespace and authorize that namespace before index access; another is to require namespace-wide job read authority for every caller permitted to submit there. Preserve durable duplicate suppression and cross-restart replay within the chosen partition.

**Closure test:** Give B new-export rights but no read authority for A’s job. Compare B’s complete submit responses for A’s live ID, its retired ID, and an unused ID using the same intentionally invalid payload. B must learn no accepted-ID distinction and must launch no duplicate work. Verify that A’s authorized exact replay and retired retry still preserve their original semantics.

## 2. Invariant analysis

The amended reserve rule addresses the prior near-quota sequence at the design level: accepted IDs remain counted, growth in attempts and owned resources requires capacity before acquisition, ordinary progress history is bounded separately, and ownership is retained after a physical write failure. The declared near-quota crash test is still future host/CLI evidence. The status and event negative branches now have a checkable payload rule, but the reference validator is a wire-semantic check; it cannot prove the caller’s job authority.

The submit finding survives even if every optional negative field is null. The response status changes because the design places inaccessible accepted IDs and unused IDs on different authorization and validation paths within one request-ID namespace.

## 3. Risks and next action

No verdict can be issued against the requested tuple because root HEAD moved during inspection. Re-establish a settled renderer/root pair and rerun the gate. The submit oracle above is a **new architectural P2 candidate** for that review. Because this object has exhausted two remediation rounds, confirmation on a settled tuple would trigger the review loop’s redesign-or-accept stop rather than another routine correction.
