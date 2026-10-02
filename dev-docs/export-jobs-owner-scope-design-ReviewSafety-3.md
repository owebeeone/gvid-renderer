# ExportJobs Owner-Scoped Request Namespace — SAFETY-AXIS RE-REVIEW 3

**Review object:** Round 2 remediation at renderer `92d2610af38bb23f0b4f16fba627541ba56775d6`, parent `730b52cd81456f4d7863548fde6b7f3613f5bf12`.  
**Baseline:** Root product documents pinned to `fa0db729054a9aeb0d3d855cbdea273bc509f419`. Renderer HEAD matched at start and end; its working tree was clean.  
**Date:** 2026-10-02  
**Axis:** Safety. Independent, adversarial, read-only; nothing here relies on the parallel reviewer’s current report. Filed verbatim by the lane owner.

**Verdict: GO** — P2-3 is closed at the contract level; P2-1 and P2-2 remain closed. No new finding or new architectural root cause.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| P2-1 | Serialize grant revocation and scope retirement with accepted-ID classification, admission and disclosure. | `export-jobs-owner-scope-design.md:17,25,27,35` still requires the shared durable order, commit recheck and transport-handoff gate. Preliminary approval cannot authorize a post-revocation index read or post-retirement admission. | Closed |
| P2-2 | Bind denial context and contract version to the originating request. | `ir/validate_export_owner_scope.py:61–70` still rejects a substituted private context or wrong version; focused tests remain at `tests/test_taut_contract.py:473–503`. | Closed |
| P2-3 | Extend the scope fence to job-ID status and cancel. | `export-jobs-owner-scope-design.md:17,37,45` now orders job-ID classification, status snapshot read, cancel action and positive response handoff with revocation. Revocation first produces the same unavailable envelope for known and unknown IDs, without snapshot, terminal state or cancel effect. The rule persists across restart. | Closed |

## Changed-range analysis

`git diff 730b52c 92d2610` changes the owner-scope, lifecycle, render-engine and graph-schema contract wording and adds RemPlan-2 and filed prior reports. It does not change Taut tags, fixtures, validators, generated bindings or the standalone branch.

The corrected wording includes job records in the per-scope durable order. It distinguishes cancellation committed before revocation, which may take effect, from cancellation ordered afterward, which cannot. A positive acknowledgement queued before revocation is suppressed if revocation precedes transport handoff. I found no new architectural root cause in the changed range.

## 0. Evidence base

Read RemPlan-2 and the complete `730b52c..92d2610` product diff; retraced P2-3 against `export-jobs-owner-scope-design.md:17,37,45`, the matching lifecycle, render-engine and graph-schema clauses, and the unchanged validator. `python -B -m unittest discover -s tests -p 'test_*.py'` passed 8 tests. The suite checks reference wire and validator behavior; host concurrency and crash tests remain implementation gates.

## 2. Invariant analysis

For a known job ID, pause B’s status or cancel request after preliminary approval. If revocation orders next, the shared fence rejects later classification or cancellation. If classification happened first, the transport-handoff gate suppresses a later `found` or `already_terminal` response. An unknown ID yields the same unavailable envelope. A terminal transition between preliminary approval and final gate cannot expose terminal state after revocation. The durable fence and revoked state survive restart, so reopening does not restore the stale preliminary approval.

A cancel action durably committed before revocation may complete; this is an authorized earlier action under the stated order. Events retain their separate per-delivery revocation barrier. Request-ID replay, conflict, tombstone and lookup paths retain the earlier fence, and denial acknowledgements retain exact request echo validation.

## 3. Risks and next action

This GO accepts the draft contract shape only. Host authorization, durable transactions, response delivery, crash recovery and cross-language runtime parity still require the stated implementation conformance gates. The lane owner may merge the peer verdicts for this exact tuple.
