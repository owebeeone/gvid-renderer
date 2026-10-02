# GVid Integrated Renderer Design — Remediation Plan

**Status:** Round 1 remediation for the integrated draft at renderer `1800cd317f61afb0a1c74b9a0c4a67ea8c843710`, with root product documents pinned to `4f75d5814f6033282e09f985e86dfca51d84e548`.
**Inputs:** [Consistency review](render-engine-integrated-design-ReviewConsistency.md) reported NO-GO with one P2; [Safety review](render-engine-integrated-design-ReviewSafety.md) reported GO. No finding is disputed or deferred.

| Finding | Disposition and one correction | Closure test |
| --- | --- | --- |
| Consistency P2-1: universal preview identity conflicts with independent source results | Make preview context type-specific. Sequence results echo accepted graph revision and binding set; source results echo registered asset/version/stream, expected fingerprint and source request context, without fabricated graph fields. Both echo fidelity and are checked against their own live authority before presentation. | Extend the reference wire test to assert a source result has no graph/binding fields and a sequence result carries its accepted revision/binding. UI/adapter conformance must discard a source result after asset version/fingerprint change and a sequence result after revision/binding change. |

This is a bounded wording and reference-test correction, not a change to Taut tags, preview ownership, or the ExportJobs contract. The implementer does not self-close the P2; the original Consistency reviewer must re-trace the source-result counterexample on a committed tuple. Safety must confirm the changed range introduces no new blocker so both axes judge the same revision.
