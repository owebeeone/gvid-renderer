# GVid Integrated Renderer Design — CONSISTENCY-AXIS RE-REVIEW

**Review object:** Draft `dev-docs/render-engine-integrated-design.md` and its package at renderer `21ba5ef9a704ebeeb2857566a27700402944a1cc`.  
**Baseline:** Renderer HEAD matched that SHA at review start and end; `git status --short` was empty. Root product documents remain pinned to `4f75d5814f6033282e09f985e86dfca51d84e548`.  
**Date:** 2026-10-02  
**Axis:** Consistency. Independent, adversarial, read-only. Nothing here relies on the parallel reviewer’s current-round report.

**Verdict: GO** — prior P2-1 is closed; no new blocking finding.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| P2-1 | Give source and sequence previews distinct result identities and freshness checks. | `render-engine-integrated-design.md:23` now requires asset/version/stream and fingerprint for source results without fabricated graph fields, and graph revision plus binding context for sequence results. The changed test round-trips both shapes and asserts the source result lacks graph and binding fields. This matches `ir/gvid_render_graph.taut.py:317–358` and the pinned UI contract. | Closed |

## Changed-range analysis

`git diff 1800cd3..21ba5ef` adds the prior review reports and remediation plan, changes the integrated preview paragraph and proof-matrix row, and extends `tests/test_taut_contract.py`. No Taut tags or other product contracts changed. The new text directs each viewer to check its applicable live authority before presentation; the proof matrix leaves stale-result behavior as a runtime test obligation. No new contradiction arose in the changed product text or test. `git diff --check` passed for those two changed product files.

## 0. Evidence base

I read the filed remediation plan and my prior report, inspected the complete changed ranges, retraced source preview before insertion against the Taut request/result fields, `GVR-RDR-058/069`, the graph-schema design and pinned `ui-design-v2.md`. `python -B -m unittest discover -s tests -p 'test_*.py'` passed **8 tests**. The renderer commit and clean status were verified again at the end.

## 2. Invariant analysis

An uninserted source result can now be accepted without a graph revision or binding set and becomes stale when its catalog version or fingerprint differs. A sequence result retains accepted graph revision and binding-set identity. The test proves their wire shapes and matching fixture fingerprint; the design explicitly assigns presentation-time stale rejection to future UI/adapter conformance. The ExportJobs and other renderer rules were unchanged by this remediation.

## 3. Risks and next action

Runtime stale-result rejection, host behavior and cross-language parity remain implementation gates. The lane owner can merge this GO with the independent verdict on the same renderer commit.
