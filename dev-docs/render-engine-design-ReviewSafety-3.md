# Render Engine Design — SAFETY-AXIS FINAL RE-REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/render-engine-design.md` and the final remediation diff at renderer `26a45d176b4cce541dc58edd1c0f5ccc4a5a435c`, root `c24cc8a0640ed2fc84e454338521a947785e7c42`. The design remains unaccepted.  
**Baseline:** Both HEADs matched those SHAs at review start and end. I read the committed diff and verified that the reviewed working files had no diff from HEAD.  
**Date:** 2026-10-02  
**Axis:** Safety, independent adversarial read-only  
**Verdict: NO-GO** — one P2 finding; no P0, P1 or P3 findings.

---

## Prior-finding closure table

| Round-2 finding | Original counterexample re-traced | Status |
| --- | --- | --- |
| Safety P1-1, same-destination rollback | The revised design assigns one canonical destination a cross-process owner from admission through reconciliation, blocks a second job while an intent is unresolved, and forbids rollback over another committed generation (`render-engine-design.md:98-100`). The A/B crash sequence can no longer erase A’s committed generation under the stated contract. | Closed |
| Safety P2-1, lost accepted acknowledgement | Request-ID lookup returns the original job and status after reopen; replay checks the durable index before destination-reference expiry and excludes delivery incarnation from payload identity (`render-engine-design.md:80-82`; `render-graph-schema-design.md:84-88`). The original lost-acknowledgement sequence is closed **while the export record and index remain retained**. A distinct retention-boundary defect is filed below. | Closed for the original sequence; new P2-1 |
| Safety P2-2, one export without a hard limit | Each interactive resource now requires an enforceable limit or measured, bounded preemption or termination. If neither can protect a late seek, export admission is rejected or deferred (`render-engine-design.md:94`). Reduced concurrency alone no longer qualifies. | Closed |
| Consistency P2-1, stale UI trace | `ui-design-v2.md:240` now directs the adapter to present one leased composite resource, without browser graph-layer assembly. | Closed |
| Consistency P2-2, lost acknowledgement | The new lookup wire shape and durable index address the same original reopen sequence (`gvid_render_graph.taut.py:408-416`). The new retention-boundary defect is filed below. | Closed for the original sequence; new P2-1 |
| Consistency P3-1, pixel oracle | The design and ADR now require recognizable overlapping layers, a decoded-frame reference comparison, and negative controls for omission and wrong order (`render-engine-design.md:107`; `adr-sequence-preview-composite.md:24`). | Closed |

## Changed-range analysis

The renderer diff adds lookup, destination policy and generation fields to the Taut schema, generated Python/Rust/TypeScript bindings and fixtures. It revises idempotency, publication ownership, degraded-resource admission and proof points in the controlling design and schema design. The root diff corrects the UI trace and composite-preview oracle. Root `gwz.conf` changes are workspace bookkeeping outside the substantive review object.

**NEW ARCHITECTURAL root cause — P2-1:** Idempotency protection ends with the export record’s lifetime, but the request-ID namespace and submit wire have no retirement or expiry boundary. The second and final remediation round is exhausted. Under the review-loop cap, this NO-GO stops the lane for a redesign-or-accept decision; it does not call for another routine patch round.

## 0. Evidence base

I read the controlling renderer requirements, schema design and Taut schema; root architecture, requirements, UI design and composite-preview ADR; both round-2 reports and RemPlan-2; the final diff, fixtures and generated bindings. I did not read the parallel current-round report. `CurrentProgramCheckpoint.md`, `AgentProcessRules.md` and `GwzProcessOptimization.md` were absent from root `dev-docs`; root `AGENTS.md` and `AGENTS_GWZ.md` were read.

The permitted test command, `python -B -m unittest discover -s tests -p 'test_*.py'`, passed six tests. The ExportJobs tests check wire round trips and golden bytes. They do not execute host authorization, durable lookup retention, cross-process publication or resource enforcement.

## 1. Findings

### [P2-1] Retiring an export record can turn an accepted request ID into new work

**Root cause and location:** The accepted `(project ID, request ID)` index is durable only “for the project’s export-record lifetime” (`render-engine-design.md:82`; `render-graph-schema-design.md:84`). Submit treats an ID absent from that index as a new request after current-context and destination validation. Neither the design nor `ExportSubmitStatus` defines an expired-ID outcome or a retained tombstone (`gvid_render_graph.taut.py:112-117`).

**Violated invariant:** A replay of a once-accepted request must never silently start a second job or replace a later destination generation.

**Credible sequence:** CLI request X with `replace_existing` and a durable local destination reference is accepted as job J and publishes generation G1. A separate, intentional export later replaces that destination with G2. After J’s export record reaches its permitted retention end, its lookup index is pruned while the project and destination remain usable. A delayed exact retry of X now misses the index, passes new-request validation, creates J2 and may replace G2 using X’s old policy. Lookup of X likewise cannot recover J. The design prevents duplication during log compaction but does not close this later retirement boundary.

**Impact:** A retry can perform an unintended export and overwrite a newer committed output. The caller receives a new job rather than the accepted job’s historical outcome.

**Required correction:** Define request-ID retirement as part of the durable protocol. Retain a compact accepted-ID tombstone for as long as that project namespace can receive retries, or establish an explicit epoch/expiry rule whose wire status rejects an old ID and requires a fresh ID for new work. Record pruning must not silently reclassify an accepted ID as unused.

**Closure test:** Accept X, lose its acknowledgement, commit a later generation at the same destination, then prune J’s event and export records according to the chosen retention policy. Replay and lookup X from a current caller with a still-valid destination reference. Verify that no second job or publication occurs and that the response explicitly identifies the retained or expired request state.

## 2. Invariant analysis

For concurrent Glade and CLI exports, the revised owner and durable intent block B while A is active or unreconciled. Recovery compares the expected generation and cannot restore A’s backup over a separately committed B generation. Unprovable destination aliases fail admission. The remaining issue arises after the job’s index is retired, not during the protected publication transaction.

Lookup requires current project and job read authority, gives unknown and unauthorized callers the same unavailable status, and returns the originating context without making an old-incarnation result current. A stale incarnation cannot start new work. The schema provides the lookup and generation shapes, while host semantic validation remains an implementation obligation.

The resource rule now covers one underestimated export on a host without a hard limit: bounded preemption must protect interactive work, otherwise admission must defer or reject. The design also assigns startup process and artifact reconciliation, terminal diagnostics, source-lease cleanup, and resource-descriptor reclamation (`render-engine-design.md:94-100`). Runtime conformance remains to be demonstrated by the specified proof points.

## 3. Risks and next action

The original round-2 counterexamples are closed by the revised contract, but P2-1 leaves durable idempotency unsafe across record retirement. The lane should stop at this final re-review and present the retention/namespace issue for redesign-or-accept. No files or git state were modified.
