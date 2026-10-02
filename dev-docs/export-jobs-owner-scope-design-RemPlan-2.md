# ExportJobs Owner-Scoped Namespace — Remediation Plan 2

**Status:** Round 2 remediation for renderer `730b52cd81456f4d7863548fde6b7f3613f5bf12`. The [Consistency re-review](export-jobs-owner-scope-design-ReviewConsistency-2.md) reported GO; the [Safety re-review](export-jobs-owner-scope-design-ReviewSafety-2.md) closed its original P2-1/P2-2 but reported one new P2-3. This is the second and final ordinary remediation round for this object.

| Finding | Disposition and one correction | Closure test |
| --- | --- | --- |
| Safety P2-3: job-ID status and cancel can cross scope revocation | Extend the same durable per-scope authority order to job-ID status classification and response handoff, and cancel classification, durable action and response handoff. A revoked caller receives the ordinary inaccessible result; a cancellation ordered after revocation has no effect. Event delivery keeps its existing barrier. | Pause status/cancel after preliminary approval, revoke, then resume for a known and unknown job ID; responses are equally unavailable and cancel has no effect. Repeat with a terminal transition between checks and after restart. |

The patch updates owner-scope, lifecycle, engine and graph-schema contract text and conformance obligations. It does not change Taut tags, fixtures or the standalone branch. The original Safety reviewer must verify its counterexample on the corrected tuple; Consistency must confirm the new wording remains coherent on that same tuple. A new architectural root cause in another round reaches the review cap and stops this object.
