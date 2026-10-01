# ExportJobs Lifecycle Design — Review Decision

**Status: NO-GO for the draft at renderer `8540f07fc7189a91b9235395681c742d378673ae`, with controlling GWZ root documents pinned to `0f2f1320be97b007b3e4d5276cbd7df9357fdcc7`. The design remains unaccepted. The two-remediation-round review cap is exhausted; routine patching of this object stops for an operator redesign-or-accept decision.**

**Date:** 2026-10-02  
**Review object:** [ExportJobs lifecycle and provenance design](export-jobs-lifecycle-design.md), companion render-engine and graph-schema designs, the Taut contract, generated bindings, fixtures, semantic response validator, and tests at the renderer commit above.  
**Final reports:** [Consistency round 3](export-jobs-lifecycle-design-ReviewConsistency-3.md) and [Safety round 3](export-jobs-lifecycle-design-ReviewSafety-3.md). Both independently reported NO-GO for the same new architectural P2 root cause.

## Verdict history

| Renderer / root reference | Consistency | Safety | Lane result |
| --- | --- | --- | --- |
| `c58006bb7edacb872b326cc2e2cc36df961a25a5` / `23b4a543fcb9b945505a66ea6f79637f2cf186e6` | NO-GO, two P2 | NO-GO, four P2 | [RemPlan](export-jobs-lifecycle-design-RemPlan.md) and one consolidated remediation |
| `7445b3fb6ee9fb97ac0301105267870addcabbf8` / `2868caeb9775fb79df43bbfebb5e08d7f67a1ae8` | NO-GO, one P2 | NO-GO, one new architectural P2 | [RemPlan-2](export-jobs-lifecycle-design-RemPlan-2.md) and second consolidated remediation |
| `8540f07fc7189a91b9235395681c742d378673ae` / pinned root documents `0f2f1320be97b007b3e4d5276cbd7df9357fdcc7` | NO-GO, one new architectural P2 | NO-GO, the same new architectural P2 | Stop the lane; redesign or explicitly accept the disclosure consequence |

A first final-round attempt used root `bfadedf8efe11de13b5470c215c281f7d81921b7`. The [Consistency report for that tuple](export-jobs-lifecycle-design-ReviewConsistency-3-prior-tuple.md) reported GO, but the [Safety report](export-jobs-lifecycle-design-ReviewSafety-3-interrupted.md) withheld its verdict because unrelated root UI work advanced HEAD during inspection. A retry also [withheld its Safety verdict](export-jobs-lifecycle-design-ReviewSafety-3-root-moved.md) after unrelated GWZ bookkeeping advanced root HEAD. Neither interrupted attempt counted as a completed final gate. The completed final Safety review pinned immutable root product documents at `0f2f132`; later root commits changed only GWZ bookkeeping and unrelated UI work, not those documents. Renderer HEAD remained at the reviewed SHA.

## Open architectural finding

The governed idempotency key is project-wide while job-read authority may be narrower than project export authority. A collaborator who can submit a new export but cannot read another collaborator's job can try that job's request ID. Existing IDs take the unauthorized branch; unused IDs can take the accepted or invalid branch. The difference reveals a private live job or retired tombstone. Both final reviewers independently found this same defect: [Consistency P2-1](export-jobs-lifecycle-design-ReviewConsistency-3.md#p2-1-new-architectural--submit-reveals-occupancy-of-an-inaccessible-request-id) and [Safety P2-1](export-jobs-lifecycle-design-ReviewSafety-3.md#p2-1-new-architectural--submit-reveals-an-inaccessible-accepted-request-id).

The operator must choose a new design object that defines a stable authority partition for request IDs, or explicitly accept that submitters in a project can detect collisions with other users' accepted IDs. **Recommended redesign:** key accepted IDs by a durable, non-reused requester/owner or capability namespace authorized before index lookup. Preserve exact replay, tombstone retention, concurrent duplicate suppression, and CLI import provenance within that namespace. Specify ownership transfer and authorized collaboration before changing the Taut contract. A project-wide visibility policy is another coherent choice if sharing all job existence with export-capable collaborators is intended. A response-field validator alone cannot settle this authority decision.

A closure gate for the new design should compare complete submit responses for another owner's live ID, retired ID, and an unused ID under the same valid and invalid payloads, including concurrent submit, reopen, and restart. No inaccessible collision may launch or replace work. The original owner's exact replay and retired retry must still behave as specified.

## Closed findings and evidence limits

The final reviewers confirmed the earlier offline-CLI provenance, accepted-ID retirement, nonterminal retention, event caller provenance, typed status unavailable, cancel disclosure, negative diagnostic fields, and accepted-job journal reserve findings as **closed at the contract level**. This decision does not accept the broader [render-engine design](render-engine-design-ReviewDecision.md) or assert runtime conformance.

The Python renderer contract suite passed **7 tests** on the reviewed renderer commit. It checks wire shapes and the reference negative-response validator. Host authorization, durable journal quota enforcement, crash recovery, destination reconciliation, and Rust/TypeScript runtime parity remain implementation gates. No additional remediation patch, tag, push, merge, or release is authorized by this NO-GO decision.
