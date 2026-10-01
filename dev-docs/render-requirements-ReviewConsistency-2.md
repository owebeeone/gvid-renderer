# Render Requirements — CONSISTENCY-AXIS REVIEW

**Review object:** `D:/projects/gvid-wz/gvid-renderer/dev-docs/render-requirements.md`, commit `7549fdbc5691db0e036d6db19cbd620312636db7`, initial draft; focused re-review after remediation round 1.

**Baseline:** Renderer `7549fdbc5691db0e036d6db19cbd620312636db7`; root `d253a2de9ea1f80cbdfff38f560df4ddb3e55492`; Taut `3c6d07ecb4cdb2496dd1a7524e270f8cc11a7640`. Prior renderer `8a1e5a8f6b79ed1ce35eb5da04890d6d42e7388d`. Pinned objects and specified blobs verified at start and end; content read through `git show COMMIT:path`.

**Date:** 2026-10-01

**Axis:** Consistency, independent, adversarial, read-only. Other axis in parallel; no current-round Safety report was read or relied upon. Filed verbatim by lane owner.

**Verdict: GO** — 0 P0, 0 P1, 0 P2, 0 P3 findings. No new architectural root cause identified. This verdict covers the requirements document, not implementation acceptance or the downstream graph schema.

---

## Prior-finding closure table

| Prior Consistency findings | Prior verdict | Current disposition |
| --- | --- | --- |
| None | GO | GO retained after focused review of the remediation and surrounding requirements. |

The merged remediation plan identifies Safety P2-1 as the reason for this patch. Its formal closure belongs to the Safety reviewer; this report independently assesses the correction’s consistency.

## Changed-range analysis

| Revised location | Change | Consistency assessment |
| --- | --- | --- |
| Line 116, `GVR-RDR-016` | Makes complete intermediate identity and validation metadata mandatory, explicitly includes asset-binding fingerprints, requires comparison against the consuming request, and defines the response to incomplete or mismatched evidence. | Agrees with root `GVR-NFR-MNT-005`, `GVR-NFR-REL-003`, and architecture §4’s cache trust boundary. It also preserves optional intermediate caching under `GVR-REND-014`. |
| Line 154, acceptance scenario 20 | Retains an intermediate from source A, rebinds to structurally equivalent but content-different source B, and checks rejection of stale reuse and correct output provenance. Includes missing identity evidence and changed effect/toolchain inputs. | Exercises semantic identity beyond media shape and profile compliance. The checks support the revised requirement without changing graph, binding, or export ownership. |

Comparison of the pinned document versions found exactly one replaced requirement line and one added scenario. After accounting for those two changes, the remaining ordered lines were identical. The document increased from 164 to 165 lines.

## 0. Evidence base

At both start and end, all three current pinned commits passed `git cat-file -e` with exit code 0. The prior renderer object was also available. Both verification passes returned:

| Document | Verified blob |
| --- | --- |
| Revised renderer requirements | `bedb7911ef7171e25e96ad0ed7e40ff90e3c4936` |
| Prior renderer requirements | `05c002d8cc7dcfca4c86b1c9cf48b58a2cf99091` |
| Root requirements | `aefbf035948a8ed43980ec42785ca7c0bd3cdf3e` |
| Root architecture | `ff51b7cce50ff7b8618e780d1aaad28c6679497c` |

Inspected this round:

- The complete revised renderer requirements and committed `render-requirements-RemPlan.md`.
- Both pinned renderer requirements versions for change isolation.
- Root requirements lines 117–119, 200, 255, 258, 267, 278, and 311.
- Root architecture AD-08, line 107’s cache trust boundary, §8.3 artifact contract, and §15 system failure invariants.

The controlling root document blobs and Taut commit are unchanged from the first consistency review. Its broader analysis remains applicable outside the verified changed ranges.

No files were changed. No tests, builds, network requests, Git mutations, or current-round peer-report reads were performed.

## 2. Invariant analysis

### Complete identity is required before reuse

The revised `GVR-RDR-016` matches root `GVR-NFR-MNT-005` at line 311: all semantic and toolchain inputs required for safe reuse must be identified. It separately requires comparing the stored evidence to the consuming request, matching architecture line 107.

I tested the requirement against an intermediate whose container, duration, dimensions, streams, and timing remain suitable after an asset rebind. Those structural properties cannot establish eligibility under the revised text: the changed asset-binding fingerprint must participate in validation. Missing evidence also cannot authorize reuse.

### The correction preserves valid reuse and batch isolation

The revision remains compatible with `GVR-RDR-047`, which permits reuse of unaffected interactive artifacts across revisions when their semantic context matches. Complete validation does not require invalidating an artifact solely because an unrelated graph revision changed.

It also agrees with `GVR-RDR-050–055`: each batch item retains its own revision, bindings, profile, and evidence; retries retain their original context; rebinding creates a new immutable binding set and export record. Shared cache storage therefore does not permit one item’s intermediate to satisfy a semantically different request.

### Failure behavior agrees with the root contract

Incomplete, mismatched, invalid, or corrupt intermediates become cache misses requiring regeneration, or clear failure when regeneration is unavailable. That is consistent with root `GVR-NFR-REL-003` and architecture §15’s requirement that cache failure cannot undermine correctness.

The correction does not weaken the separate output verification and publication obligations in `GVR-RDR-017–019`. A successfully probed stale intermediate still fails the earlier semantic identity check.

### Scenario 20 provides a meaningful counterexample

The A-to-B fixture deliberately holds media shape constant while changing content, preventing profile probing alone from satisfying the test. Checking both output and export evidence addresses actual composition and provenance. The missing-evidence branch verifies failure when eligibility cannot be established.

The effect-parameter and toolchain variants extend the same check beyond asset identity. During implementation, those variants should be exercised independently so each invalidation dimension has observable evidence. The document already requires every relevant input to be covered; no additional requirements change is needed for that test construction.

The scenario is required future acceptance work. Neither the revised document nor this review claims it has been executed.

## 3. Risks and next action

Residual risks concern implementation of fingerprints, complete semantic dependency tracking, cache validation, and acceptance fixtures. The revised requirement constrains those implementations without selecting a cache layout or changing the architecture. The unchanged root contracts continue to govern durability, compatibility, stale-frame rejection, and reproducible export.

**Next action:** Merge this GO with the independent Safety re-verdict on the same revised tuple, with Safety responsible for closing its prior P2-1.