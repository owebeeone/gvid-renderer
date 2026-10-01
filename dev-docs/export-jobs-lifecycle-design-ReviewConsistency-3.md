# ExportJobs Lifecycle Design — CONSISTENCY-AXIS FINAL RE-REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/export-jobs-lifecycle-design.md` and companion designs, Taut contract, generated bindings, fixtures, validator, and tests at renderer `8540f07fc7189a91b9235395681c742d378673ae` and GWZ root `0f2f1320be97b007b3e4d5276cbd7df9357fdcc7`.  
**Baseline:** Final remediation diff from renderer `7445b3fb6ee9fb97ac0301105267870addcabbf8`; prior reviewed root `2868caeb9775fb79df43bbfebb5e08d7f67a1ae8`. Both HEADs matched the assigned tuple at review start and end; both working trees were clean.  
**Date:** 2026-10-02  
**Axis:** Independent adversarial, peer-blind, read-only consistency review. No current-round Safety report was consulted.  
**Verdict: NO-GO** — one P2 finding, classified **NEW ARCHITECTURAL**. No P0, P1, or P3 findings.

## Prior-finding closure table

| Prior finding | Result on this tuple |
| --- | --- |
| Prior render-engine Consistency P2-1 — offline CLI cannot supply a host incarnation | **Closed at contract level.** The standalone import has a durable, non-reused local namespace and no fabricated Glade incarnation (`export-jobs-lifecycle-design.md:9–13,19`). |
| Prior render-engine Safety P2-1 — accepted ID can be reused after full-record pruning | **Closed at contract level.** Atomic live-record-to-tombstone conversion preserves the accepted ID for the namespace lifetime (`export-jobs-lifecycle-design.md:31–33`). |
| Round-1 Safety P2-1 — nonterminal retirement | **Closed at contract level.** Durable terminal state and reconciliation of process, lease, artifact, resource, and publication obligations precede pruning (`export-jobs-lifecycle-design.md:31`). |
| Round-1 Consistency P2-1 / Safety P2-2 — event caller provenance | **Closed at contract level.** `ExportEventsQuery` carries the current governed incarnation or local import ID; authorization precedes replay and is rechecked at delivery (`ir/gvid_render_graph.taut.py:403–407`; `export-jobs-lifecycle-design.md:25`). |
| Round-1 Consistency P2-2 / Safety P2-3 — unavailable status lacks a Taut shape | **Closed at contract level.** `ExportStatusResult` has found, unavailable, and stale-context branches; only found carries a snapshot (`ir/gvid_render_graph.taut.py:422–426`). |
| Round-1 Safety P2-4 — cancel existence oracle | **Closed at contract level.** Unknown and inaccessible targets share `unavailable`; `already_terminal` follows job read authorization (`export-jobs-lifecycle-design.md:27`). |
| Round-2 Consistency P2-1 — negative-response diagnostic disclosure | **Closed at contract level.** Lookup, status, and event negative branches require null diagnostic and payload. The new reference validator and test reject a diagnostic or payload on those branches (`export-jobs-lifecycle-design.md:21–25,43`; `ir/validate_export_responses.py:22–36`; `tests/test_taut_contract.py:421–446`). Host enforcement remains a future implementation proof. |
| Round-2 Safety P2-1 — no durable capacity for an accepted job’s failure and cleanup path | **Closed as a design rule.** Admission reserves a permanent accepted-ID slot and physically backed, restart-persistent journal space; attempts and ownership growth are bounded or require reserve enlargement. Ordinary history cannot consume the reserve (`export-jobs-lifecycle-design.md:35–37`; `render-engine-design.md:98`; `render-graph-schema-design.md:94`). Runtime quota and crash proof remains due. |

## Changed-range analysis

The renderer diff adds the filed round-2 reports and remediation plan, changes the lifecycle amendment and both companion designs, adds `ir/validate_export_responses.py`, and extends the Python contract test. The Taut schema, generated bindings, and golden fixtures did **not** change in this final remediation. The root diff adds unrelated `dev-docs/ui-wireframe.svg` and GWZ bookkeeping; neither is judged as part of this renderer contract.

The negative-response and reserve changes align across the three amended designs. The finding below arises from the unchanged project-wide request-ID namespace and its interaction with the stated per-job read authority and new-ID admission branch. It was not resolved by either final-round correction.

## 0. Evidence base

I read root `AGENTS.md` and `AGENTS_GWZ.md`; the controlling root requirements, architecture, UI design, and Taut feedback; renderer requirements, render-engine and graph-schema designs, the prior render-engine ReviewDecision, all filed ExportJobs reports and remediation plans through round 2, the Taut declarations, generated bindings, fixtures, validator, and tests. The named root `dev-docs/CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, and `GwzProcessOptimization.md` are absent.

The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` passed **7 tests**. These test Python wire round trips and the reference negative-response validator. They do not execute host authorization, journal recovery, or Rust/TypeScript interoperability. Inspection modified no files.

## 1. Findings

### [P2-1] NEW ARCHITECTURAL — submit reveals occupancy of an inaccessible request ID

**Root cause and exact location:** The governed idempotency key is shared by the whole project, `(governed_host, project_id, request_id)`, while an existing request requires the caller to have that particular job’s read authority (`export-jobs-lifecycle-design.md:9,13,17`). A fresh request instead requires current project and destination authority and can be accepted (`:19`). The retirement rule promises that a caller without the original job’s read authority receives no evidence of its tombstone (`:33,45`), but the submit branches cannot make an inaccessible occupied ID behave like a fresh ID. `ExportJobAck` exposes distinct `accepted` and `unauthorized` statuses (`ir/gvid_render_graph.taut.py:112–114,382–385`). The companion designs retain the same project-wide key and branch order (`render-engine-design.md:80–82`; `render-graph-schema-design.md:84–86`).

**Violated invariant:** A caller lacking an existing job’s read authority must not learn that a candidate request ID was accepted, live or retired, merely by submitting under that ID.

**Credible sequence:** Caller A accepts request ID `R` in project P. Caller B has a current P incarnation, read authority for the selected graph and bindings, and write authority for a different destination, but lacks read authority for A’s job. B submits the same otherwise valid request under `R`: the accepted-ID lookup finds A’s record or tombstone, and B receives an unauthorized/unavailable result. B submits the same request under a fresh `R2`: the new-ID branch accepts it, assuming ordinary capacity and no destination collision. Comparing these responses reveals that `R` is occupied. The probe works after pruning because the permanent tombstone preserves the occupied key. Returning `retired_request` would reveal more, but replacing it with `unauthorized` does not meet the stated non-disclosure rule.

**Impact:** Project members permitted to create exports but excluded from a particular job can probe candidate request IDs for accepted-job existence. The lookup, status, events, and cancel contracts deliberately merge unknown and inaccessible targets, while submit reopens that distinction. Implementers cannot satisfy both project-wide ID uniqueness and the absolute unauthorized non-disclosure claim under the stated authority split without an additional identity or response rule.

**Required correction:** Settle the submit authorization and namespace model explicitly. For example, scope idempotency keys to an authorized requester/owner while preserving retry and retirement guarantees, or define a project-wide job-read policy if that is the intended trust boundary. State what an authorized submitter can learn from collisions with another owner’s live and retired IDs, and align the amendment, companion designs, and conformance obligations. This requires an architectural decision, not another negative-response field check.

**Closure test:** Give A and B the authority split above. Test B’s same-payload and changed-payload submissions against A’s live `R`, against its retired tombstone, and against a fresh ID, including after reopen. Verify the chosen, documented disclosure policy while preserving A’s exact replay, permanent accepted-ID protection, and prevention of duplicate execution.

## 2. Invariant analysis

The final amendment coherently distinguishes governed Glade requests from offline standalone imports. Live replay skips expired original destination-reference revalidation, and retirement preserves accepted IDs without inventing historical status after pruning. The status and event envelopes can express unavailable branches, and the new validator rejects a negative lookup, status, or event response carrying a job payload or diagnostic ID.

The reserve rule addresses the prior quota deadlock in the written contract: an accepted job’s bounded failure and reconciliation writes are reserved before acknowledgement, while ordinary event history has a separate budget and cursor-gap recovery. Physical-write failure retains ownership for later recovery rather than inferring success. These are coherent obligations, with runtime proof explicitly deferred.

The submit non-disclosure obligation remains unsatisfiable for a project-wide request namespace when job read authority can be narrower than new export admission authority. The new validator cannot close this because it does not govern submit acknowledgements or the occupancy-dependent choice between existing-ID and new-ID branches.

## 3. Risks and next action

**NO-GO.** This is a further **NEW ARCHITECTURAL** root cause in the final remediation review. Under the filed review-loop cap and RemPlan-2, the lane owner should stop routine remediation and put the namespace/authorization choice to the operator as redesign or explicit acceptance of the disclosure consequence. No files or git state were modified.
