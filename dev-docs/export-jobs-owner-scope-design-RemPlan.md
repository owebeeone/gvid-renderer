# ExportJobs Owner-Scoped Namespace — Remediation Plan

**Status:** Round 1 remediation for the draft at renderer `acf12b40a5aeaab715df188297ee002d24b01459`; controlling root product documents pinned to `fa0db729054a9aeb0d3d855cbdea273bc509f419`.
**Inputs:** [Consistency review](export-jobs-owner-scope-design-ReviewConsistency.md) and [Safety review](export-jobs-owner-scope-design-ReviewSafety.md). Both reported NO-GO. No findings are disputed or deferred.

| Finding | Disposition and one correction | Closure test |
| --- | --- | --- |
| Consistency P2-1: impossible complete-response comparison across different echoed IDs | Amend the proof rule to compare complete responses only for byte-identical requests against different occupancy states. For different request IDs, verify exact context echo and compare only non-echo denial fields. | Contract test constructs both comparison forms; host conformance repeats them for live, retired and unused occupancy. |
| Safety P2-1: grant check races with index access/admission | Define one per-scope durable serialization/fence for grant revocation, scope retirement, index reads and admission commit. Preliminary checks may occur outside it; every index-derived result and durable acceptance requires current authority under the fence. A final delivery gate suppresses queued disclosures after revocation. | Pause after preliminary authorization, revoke or retire, then resume lookup, replay/conflict/tombstone probes and new admission; require denial/no accepted work, including after restart. |
| Safety P2-2: denial validator can echo another job's context | Make the validator accept the originating request, require exact decoded context and contract-version echo, and use the same check on send and receive. | Mutate the acknowledgement to include another job's context or wrong version; reference validator rejects it. Host conformance compares whole responses for identical denied requests. |

The patch updates the owner-scope draft, lifecycle and companion design wording, the reference validator and focused tests. It does not change the Taut field layout or the offline CLI namespace. The implementer may not self-close these findings; the two original reviewers must verify their counterexamples on the corrected committed tuple.
