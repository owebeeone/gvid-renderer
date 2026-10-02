# ExportJobs Owner-Scoped Request Namespace — SAFETY-AXIS RE-REVIEW

**Review object:** Round 1 remediation of `dev-docs/export-jobs-owner-scope-design.md` and companion changes at renderer `730b52cd81456f4d7863548fde6b7f3613f5bf12`.  
**Baseline:** Renderer parent `acf12b40a5aeaab715df188297ee002d24b01459`; root product documents pinned to `fa0db729054a9aeb0d3d855cbdea273bc509f419`. Renderer HEAD matched at start and end, with a clean working tree.  
**Date:** 2026-10-02  
**Axis:** Safety. Independent, adversarial, read-only; nothing here relies on the parallel reviewer’s current report. Filed verbatim by the lane owner.

**Verdict: NO-GO** — the two original P2 counterexamples are closed, but one P2 omission remains on job-ID operations. I pre-commit to GO when P2-3 is resolved as specified and the corrected revision introduces no new blocker.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| P2-1 | Serialize scope grants, revocation, retirement, accepted-ID classification and admission; gate index-derived response handoff. | `export-jobs-owner-scope-design.md:17,25,27,35,45` now requires a durable shared order and a second disclosure check. A preliminary approval followed by revocation cannot authorize the later index read or handoff. Retirement ordered before admission prevents commit, including after restart. | Closed for the original request-ID lookup and admission counterexamples. |
| P2-2 | Compare denial context and contract version with the originating request. | `validate_export_submit_denial(request, ack)` now checks both echoes at lines 61–69. Tests at `tests/test_taut_contract.py:473–503` reject another job’s context and a wrong version. | Closed. |

## Changed-range analysis

`acf12b4..730b52c` changes the owner-scope, lifecycle, engine and graph-schema documents, the reference denial validator and focused tests. It does not change Taut tags, generated bindings or the standalone namespace. The response comparison proof now correctly distinguishes byte-identical requests from requests with different mandatory echoes.

The new shared fence expressly governs **accepted-ID** classification, admission and index-derived disclosure. Job-ID status and cancel remain outside that stated order. P2-3 below is an omitted endpoint for the same revocation safety rule, rather than a new architecture choice.

## 0. Evidence base

Read the filed Safety report and RemPlan, `git diff acf12b4 730b52c`, the corrected owner-scope design, companion changed clauses, validator and tests. Inspected job-ID rules in `export-jobs-owner-scope-design.md:13,17,37,45` and `export-jobs-lifecycle-design.md:23–27`. The reference suite passed: `python -B -m unittest discover -s tests -p 'test_*.py'` ran 8 tests. No host or durable-store implementation exists in this gate, so concurrency closure is judged from the contract text.

## 1. Findings

### [P2-3] Job-ID status and cancel can cross scope revocation

**Location:** `dev-docs/export-jobs-owner-scope-design.md:17,37`; `dev-docs/export-jobs-lifecycle-design.md:23,27`; changed fence language in `dev-docs/render-engine-design.md:84`.

**Violated invariant:** Revocation stops further caller access. Unknown and inaccessible job IDs must remain indistinguishable, and a revoked caller must not cancel a job.

**Reproduction:** B knows job J’s ID from an earlier delegated grant. B starts `export.status` and passes a preliminary job/scope authority check. The grant is revoked. B then reads J’s updated record and hands off a `found` snapshot. A different unknown job ID yields `unavailable`; the post-revocation responses disclose occupancy and state. Similarly, B can pass a preliminary cancel check, be revoked, then commit cancellation or receive an `already_terminal` acknowledgement. The revised fence gates only an “index-derived fact”; these job-ID operations do not read the accepted request-ID index. The status and cancel clauses require a check “before” response or action but do not serialize that check with revocation, commit and response handoff. Events already have an explicit delivery barrier.

**Impact:** A revoked delegate can receive job details or cause cancellation after revocation. A later response can also distinguish a known private job from an unknown ID.

**Required correction:** Extend the same per-scope authorization order to job-ID status classification and response handoff, and to cancel classification, durable action and response handoff. A revocation ordered first must yield the ordinary inaccessible result and no cancellation effect. Preserve the existing event barrier.

**Closure test:** Pause status and cancel after preliminary grant approval; revoke B; then resume probes for J and an unknown ID. Require indistinguishable unavailable responses with no snapshot, terminal state or cancellation effect. Repeat when J becomes terminal between the preliminary check and final gate, and after restart.

## 2. Invariant analysis

The original request-ID attack now fails under the written rules: grant state and accepted-ID reads share a durable order, admission rechecks at commit, and index-derived frames are gated at handoff. The validator now rejects an unauthorized acknowledgement whose context or contract version differs from its request. The sequential owner-scope separation, tombstone retention, canonical destination contention and standalone import rules remain intact.

The job-ID path does not inherit the new fence merely because its positive payload contains `ExportJobContext`. Its authority check needs the same revocation ordering at the point of action and disclosure.

## 3. Risks and next action

Runtime authentication, transactions, crash recovery and cross-language parity remain implementation gates. The next action is a bounded amendment and focused test for P2-3, followed by Safety re-verdict on the corrected pinned tuple.
