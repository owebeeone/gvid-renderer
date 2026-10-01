# Render Engine Design — Review Decision

**Status: NO-GO at renderer `26a45d176b4cce541dc58edd1c0f5ccc4a5a435c` and gvid-wz root `c24cc8a0640ed2fc84e454338521a947785e7c42`. The design remains unaccepted. The review-loop limit of two remediation rounds is exhausted; the next step is an operator redesign-or-accept decision.**

**Date:** 2026-10-02  
**Review object:** `dev-docs/render-engine-design.md` and its companion graph schema, ExportJobs Taut contract, generated bindings, fixtures, tests, root architecture/UI alignment, and composite-preview ADR at the settled tuple above.  
**Final reports:** [Consistency round 3](render-engine-design-ReviewConsistency-3.md) and [Safety round 3](render-engine-design-ReviewSafety-3.md). Both reviewed the same committed tuple independently and reported NO-GO.

## Verdict history

| Reviewed renderer / root tuple | Consistency | Safety | Lane result |
| --- | --- | --- | --- |
| `e168ecd815df679202cfd4bc60b9330df64e5305` / `a8f88fdbda3108005166a75545cc6c6b7ef5e2d4` | NO-GO, five P2 findings | NO-GO, four P2 findings | One consolidated remediation plan |
| `3e6e4883de5f43391e3df842dbd5a450dac635db` / `567da536bf08a0d84b169c5408ff8dc019b0e87e` | NO-GO, two P2 and one P3 | NO-GO, one P1 and two P2 | Second and final consolidated remediation plan |
| `26a45d176b4cce541dc58edd1c0f5ccc4a5a435c` / `c24cc8a0640ed2fc84e454338521a947785e7c42` | NO-GO, one new P2 architectural finding | NO-GO, one new P2 architectural finding | Stop the lane; redesign-or-accept |

The first two remediation plans are [RemPlan](render-engine-design-RemPlan.md) and [RemPlan-2](render-engine-design-RemPlan-2.md). The final reviewers re-traced and closed the earlier UI composition, lost-acknowledgement, pixel-oracle, destination-publication, and resource-starvation counterexamples as design rules. Runtime proof remains future implementation work.

## Open findings and required operator decision

| Finding | Why the design cannot be accepted as written | Redesign decision to make |
| --- | --- | --- |
| Consistency round 3 P2-1 | New ExportJobs admission unconditionally requires a current authority incarnation, while a valid standalone CLI import deliberately has no Glade incarnation. | Define admission, lookup, and replay separately for governed-host and standalone-import provenance; preserve shared request-ID semantics without fabricating host authority. |
| Safety round 3 P2-1 | Removing an old export record also removes its accepted request-ID index. A delayed retry can then start a second job and replace a newer output. | Define durable request-ID retirement: retain a compact tombstone for the namespace lifetime, or introduce an explicit epoch/expiry protocol that rejects retired IDs on the wire. |

These are distinct new architectural root causes in the final re-review. No further patch is authorized by this review loop. The operator must choose a redesign of the ExportJobs lifecycle and provenance rules for a new review object, or explicitly accept the two open defects and their consequences. This decision document records NO-GO; it does not imply acceptance.

## Evidence and limits

The final reviewers verified the settled commit pair at review start and end and found no uncommitted changes in the reviewed files. `python -B -m unittest discover -s tests -p 'test_*.py'` passed six renderer schema and golden-wire tests. Those tests do not establish runtime admission, durable retention, authorization, concurrent publication, or resource enforcement. The review judged the written design and contract, not an implemented renderer service.

No `CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, or `GwzProcessOptimization.md` was present in root `dev-docs` to update as a separate program ledger. This decision and the filed reports are the lane record.
