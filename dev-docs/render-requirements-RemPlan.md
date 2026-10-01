# Render Requirements — Remediation Plan

**Review object:** `gvid-renderer/dev-docs/render-requirements.md` at renderer commit `8a1e5a8f6b79ed1ce35eb5da04890d6d42e7388d`, with root controls at `ac674e831d18234e23a2a54689256dc6d20c9016`.
**Date:** 2026-10-01
**Round:** 1 of at most 2 remediation rounds.
**Verdict merge:** Consistency GO with no findings; Safety NO-GO with one P2 finding. The two reports assessed the same pinned tuple. The earlier tuple-mismatch attempt produced no document verdict and does not count as a review round.

| Finding | Disposition | Correction | Closure test and reviewer |
| --- | --- | --- | --- |
| Safety P2-1 — intermediate-cache safety weakened to SHOULD | Accept. Root `GVR-NFR-MNT-005` and architecture require complete cache identity and request validation. | Revise `GVR-RDR-016` so any reused intermediate MUST identify all semantic and toolchain inputs and MUST be validated against the consuming request. Incomplete or mismatched evidence causes a miss/regeneration or explicit failure. Reuse remains optional. | Add a required scenario that changes an asset binding to structurally identical but content-different media while the old intermediate remains available; assert no cross-binding reuse and correct output/evidence. Repeat for a semantic parameter or toolchain change. Original Safety reviewer re-traces this exact counterexample on the revised tuple. |

The consistency report has no finding to dispose. The patch is limited to the cache requirement and its acceptance scenario; graph schema design and other repository work are outside this remediation. Focused GO re-verdicts from both original reviewers on the same revised committed document close this loop. Safety must re-trace its counterexample; Consistency must check the changed requirement and scenario against the root controls.