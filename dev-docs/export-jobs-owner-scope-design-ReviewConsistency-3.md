# ExportJobs Owner-Scoped Request Namespace — CONSISTENCY-AXIS REVIEW, ROUND 2

**Review object:** Second remediation at renderer `92d2610af38bb23f0b4f16fba627541ba56775d6`  
**Baseline:** Renderer `730b52cd81456f4d7863548fde6b7f3613f5bf12`; root product documents pinned to `fa0db729054a9aeb0d3d855cbdea273bc509f419`  
**Date:** 2026-10-02  
**Axis:** Consistency. Independent, adversarial, read-only.

**Verdict: GO** — the previously closed P2-1 remains closed. No new finding or new architectural root cause was identified.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| Consistency P2-1 | Complete denial responses are compared for byte-identical requests; distinct IDs require exact individual echoes and comparison of non-echo fields. | The owner-scope §4 and lifecycle closure gates retain this distinction. The second remediation does not change the validator or tests that closed the counterexample at `730b52c`. | Remains closed. |

## Changed-range analysis

`730b52c..92d2610` changes only contract documents and files the round 1 reports and second remediation plan. The contract now places job-ID status classification, cancel classification and cancellation commit under the durable scope-authority order. It checks authority again at response handoff. The owner-scope, lifecycle, engine and graph-schema wording agree on the ordering and on the result after revocation. A cancellation committed before revocation may still take effect while its pending positive acknowledgement is suppressed; that is consistent with the stated commit and handoff boundaries. The Taut fields, fixtures, validator, tests and standalone branch are unchanged.

## 0. Evidence base

I read `export-jobs-owner-scope-design-RemPlan-2.md` and inspected `git diff 730b52c..92d2610` for the owner-scope, lifecycle, engine and graph-schema documents. I traced the original P2-1 comparison language in the corrected owner-scope and lifecycle gates. Renderer `HEAD` was `92d2610af38bb23f0b4f16fba627541ba56775d6` at both start and end; `git status --short` was empty. No tests were run in this round because no executable or wire artifact changed.

## 1. Findings

None.

## 2. Invariant analysis

The denial comparison remains satisfiable: full equality applies to the same request bytes across occupancy states, while different request IDs receive their own exact echoes. The added job-ID rule uses the same current-grant fence for classification, durable cancel action and transport handoff. A revocation ordered before the action blocks cancellation; a revocation ordered before handoff suppresses a queued positive response. Negative status and cancel responses retain the existing inaccessible shape. Scope retirement still prevents new admissions without forbidding access to earlier jobs.

## 3. Risks and next action

The documents remain draft contracts. Runtime serialization, revocation races, crash recovery and cross-language wire execution remain their stated later gates. The next action is to merge this GO with the independent Safety verdict on `92d2610`; this is the second and final ordinary remediation round.
