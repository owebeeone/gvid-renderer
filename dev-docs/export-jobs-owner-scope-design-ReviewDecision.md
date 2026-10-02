# ExportJobs Owner-Scoped Request Namespace — Review Decision

**Status: GO for the owner-scoped ExportJobs draft contract shape at renderer `92d2610af38bb23f0b4f16fba627541ba56775d6`, with root product documents pinned to `fa0db729054a9aeb0d3d855cbdea273bc509f419`. This accepts the owner-scope design object and companion contract changes only.**

**Date:** 2026-10-02  
**Controlling design:** [Owner-scoped request namespace](export-jobs-owner-scope-design.md)  
**Final peer-blind reports:** [Consistency round 2](export-jobs-owner-scope-design-ReviewConsistency-3.md) and [Safety round 2](export-jobs-owner-scope-design-ReviewSafety-3.md), both GO on the same renderer revision.

## Verdict history

| Renderer checkpoint | Consistency | Safety | Lane result |
| --- | --- | --- | --- |
| `acf12b40a5aeaab715df188297ee002d24b01459` | NO-GO: impossible complete-response comparison across different echoed IDs | NO-GO: grant/index race; denial could echo another job's context | [Round 1 plan](export-jobs-owner-scope-design-RemPlan.md) |
| `730b52cd81456f4d7863548fde6b7f3613f5bf12` | GO: comparison gate corrected | NO-GO: original findings closed, job-ID status/cancel omitted from revocation fence | [Round 2 plan](export-jobs-owner-scope-design-RemPlan-2.md) |
| `92d2610af38bb23f0b4f16fba627541ba56775d6` | GO: prior finding remains closed; no new inconsistency | GO: job-ID path closed; no new architectural root cause | **Accepted at draft contract level** |

## Accepted contract

Governed accepted request IDs are keyed by `(governed_host, project_id, owner_scope_id, request_id)`. A host-minted owner scope is non-reused and its submit grant includes read visibility to every accepted job and tombstone in that scope. An unauthorized caller is denied independently of request-ID occupancy; the denial echoes only its own submitted context and contract version. An authorized scope still supports exact replay, changed-payload conflict and permanent accepted-ID retirement. Standalone imports keep their independent durable namespace with null owner scope.

Current grants and scope retirement share a durable ordering fence with accepted-ID reads, admission, job-ID status classification, cancel action and positive response handoff. Revocation ordered before classification or handoff prevents later disclosure; revocation ordered before cancellation prevents the action. Scope retirement ordered before admission prevents acceptance. The response comparison gate uses byte-identical requests across occupancy states, and treats the mandatory echo separately when request IDs differ.

The Taut schema appends `owner_scope_id` at tags 14, 6 and 7 for job context, lookup query and lookup result. Pinned Python, Rust and TypeScript bindings were regenerated. The reference Python contract suite passed 8 tests and `git diff --check` was clean on the accepted checkpoint. Tests cover wire round trips, provenance shapes and denial echo validation; they do not execute a host authorization or durable store.

## Remaining gates

The broader [render-engine design decision](render-engine-design-ReviewDecision.md) and [lifecycle decision](export-jobs-lifecycle-design-ReviewDecision.md) are historical NO-GO records and are not silently replaced by this narrower GO. A later settled-tree acceptance gate must decide the integrated renderer design. Host implementation must still prove grant provisioning and revocation, durable serialization, privacy equivalence, concurrent admission, journal reserve, crash recovery, destination publication and Rust/TypeScript runtime parity. No production implementation, push, merge or release is claimed by this decision.
