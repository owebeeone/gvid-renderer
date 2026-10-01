# Render Requirements — SAFETY-AXIS REVIEW

**Review object:** `D:/projects/gvid-wz/gvid-renderer/dev-docs/render-requirements.md`, commit `7549fdbc5691db0e036d6db19cbd620312636db7`, Initial draft; focused re-review after remediation round 1.  
**Baseline:** Renderer `7549fdbc5691db0e036d6db19cbd620312636db7`; root `d253a2de9ea1f80cbdfff38f560df4ddb3e55492`; Taut `3c6d07ecb4cdb2496dd1a7524e270f8cc11a7640`. Pinned commits and required blobs verified at start and end through read-only Git inspection.  
**Date:** 2026-10-01  
**Axis:** Safety, independent, adversarial, read-only. Other axis in parallel; nothing here relies on it. Filed verbatim by lane owner.  
**Verdict: GO** — Prior P2-1 closed at the requirements level. Zero open P0/P1/P2/P3 findings. No new architectural root cause identified.

---

## Prior-finding closure table

| Finding | Disposition | Verified correction and closure evidence |
| --- | --- | --- |
| P2-1 — Intermediate-cache safety weakened from a mandatory invariant to a recommendation | **Closed** | Revised `GVR-RDR-016`, line 116, mandates complete semantic/toolchain identity, explicitly includes asset-binding fingerprints, requires comparison against the consuming request before reuse, and rejects incomplete or mismatched intermediates through regeneration or clear failure. Required acceptance scenario 20, line 154, directly exercises the original A-to-B rebinding counterexample, missing identity evidence, and semantic/toolchain changes. |

Closure concerns the reviewed requirements defect. The acceptance scenario has been specified and inspected; it has not been executed against an implementation.

## Changed-range analysis

Compared the prior requirements at `8a1e5a8f6b79ed1ce35eb5da04890d6d42e7388d` with the revised immutable document. An ordered line comparison confirmed `only_declared_changes=True`:

- Line 116 replaces `GVR-RDR-016`.
- Line 154 adds acceptance scenario 20.
- All other requirements text is unchanged; the document grows from 164 to 165 lines.

The correction makes safety mandatory whenever reuse occurs while leaving reuse optional. It preserves implementation freedom to use cache identity and validation metadata together. It introduces no new component boundary, graph representation, mutation authority, or compatibility mechanism.

## 0. Evidence base

Inspected:

- The complete revised renderer requirements through `git show`.
- `dev-docs/render-requirements-RemPlan.md` at the revised renderer commit.
- The prior immutable renderer requirements for comparison.
- Root `GVR-NFR-MNT-005`, line 311: each artifact must identify every semantic and toolchain input required for safe reuse.
- Root architecture line 107: cache identity and validation metadata must match the request before trust.
- Root architecture lines 494–501: cache failure must affect performance rather than correctness, with source integrity and export isolation preserved.

The controlling root blobs are identical to those reviewed in round 1. Taut remains at the same pinned commit; the remediation makes no new Taut capability claim. The earlier inspection of Taut decoding, evolution, and delivery-shape documentation therefore remains applicable.

At both verification points, all three pinned commits were available and `git rev-parse COMMIT:path` returned:

| Object | Verified blob |
| --- | --- |
| Renderer `dev-docs/render-requirements.md` | `bedb7911ef7171e25e96ad0ed7e40ff90e3c4936` |
| Root `dev-docs/gvid-requirements.md` | `aefbf035948a8ed43980ec42785ca7c0bd3cdf3e` |
| Root `dev-docs/gvid-arch.md` | `ff51b7cce50ff7b8618e780d1aaad28c6679497c` |

No current-round Consistency report was read. No files were modified, and no builds or implementation tests were run.

## 2. Invariant analysis

**Original P2-1 counterexample, re-traced:**

1. An intermediate is produced for graph R, slot S bound to fingerprint A, and profile P.
2. S is rebound to B, whose structural media properties match A but whose content differs. The old intermediate remains available.
3. Even if an implementation’s lookup key omits the binding fingerprint, revised `GVR-RDR-016` requires the artifact’s identity and validation metadata to identify that input.
4. Before reuse, the renderer must compare this evidence with B’s consuming request.
5. Evidence identifying A mismatches B; evidence omitting the fingerprint is incomplete. Either case requires a cache miss and regeneration, or clear failure if regeneration is unavailable.
6. A structurally valid media file can therefore no longer satisfy B’s request merely because profile probing succeeds.

The previously permitted exception is closed before consumption. Final output probing is no longer implicitly responsible for discovering wrong-content cache reuse.

**Closure scenario:** Scenario 20 retains A’s intermediate, changes only the binding to structurally equivalent but content-different B, and requires rejection of cross-binding reuse plus correct output and export evidence. It also exercises missing evidence and changes to a semantic effect parameter and toolchain version. This addresses both incomplete identity and mismatched identity.

**Batch reuse and retry:** `GVR-RDR-051` can continue to recommend reuse because the revised mandatory guard applies whenever an intermediate is reused. Immutable per-item inputs and the original-binding retry rule in `GVR-RDR-050`, `053`, and `055` remain intact. Retry does not authorize accepting a mismatched cached intermediate.

**Interactive behavior and resource pressure:** The change agrees with the existing mandatory interactive cache rule in `GVR-RDR-047`. Regeneration remains subject to job admission, bounded queues, fairness, and declared degradation (`GVR-RDR-046`, `054`, `011`). It provides no exception allowing stale content under resource pressure.

**Failure and publication:** Explicit failure when regeneration is unavailable remains consistent with actionable diagnostics and the prohibition on publishing failed or cancelled exports as successful (`GVR-RDR-017`–`020`). Source integrity, candidate verification, and atomic publication are unchanged.

The unchanged requirements retain the earlier reviewed safeguards for command atomicity and idempotency, stale preview rejection, explicit approximation, immutable exports, authenticated media access, credential exclusion, and mixed-version semantic rejection. The two changed ranges do not weaken those safeguards.

## 3. Risks and next action

Implementation evidence remains outstanding, as the document explicitly states. Cache metadata construction, fingerprint strength, concurrent artifact publication, and actual request validation still need verification in their owning designs and implementation gates.

**Next action:** File this GO re-verdict and merge it with the independent Consistency verdict for the same revised requirements tuple.