# GVid Integrated Renderer Design — SAFETY-AXIS RE-REVIEW 2

**Review object:** Focused round-1 remediation of `dev-docs/render-engine-integrated-design.md` and its preview reference test at renderer `21ba5ef9a704ebeeb2857566a27700402944a1cc`.  
**Baseline:** Parent renderer `1800cd317f61afb0a1c74b9a0c4a67ea8c843710`; root product documents remain pinned to `4f75d5814f6033282e09f985e86dfca51d84e548`. Renderer HEAD matched the reviewed SHA at the start and end; `git status --short` was clean.  
**Date:** 2026-10-02  
**Axis:** Safety. Independent, adversarial, read-only. Nothing here relies on the Consistency reviewer’s current-round report.

**Verdict: GO** — the changed range introduces no P0–P3 Safety finding. This is a design-contract verdict; the originating Consistency reviewer must verify closure of its P2-1.

---

## Prior-finding closure table

| ID | Disposition claimed | Safety verification on corrected tree | Status |
| --- | --- | --- | --- |
| Consistency P2-1, universal preview identity | Separate source asset/version/stream/fingerprint freshness from sequence graph revision/binding freshness. | The integrated draft now states distinct identities and live-authority checks (`render-engine-integrated-design.md:23,40`). The unchanged Taut types carry those distinct fields (`ir/gvid_render_graph.taut.py:317–358`). The reference test asserts their respective shapes (`tests/test_taut_contract.py:249–293`). | Safety counterexample closed at contract level; formal finding closure belongs to its originating reviewer. |
| Prior Safety review | No findings. | The prior GO’s ExportJobs, publication, resolver and scheduler rules are untouched by this remediation. | Remains GO. |

## Changed-range analysis

`git diff 1800cd3..21ba5ef` changes the integrated draft’s preview identity sentence and proof-matrix row, extends one preview wire test, and adds filed reports and the remediation plan. It changes no Taut tags, resolver, scheduler, ExportJobs or publication contract. The filed reports have Markdown trailing spaces for line breaks; these do not affect the Safety verdict.

The corrected rule does not require a source preview to fabricate graph or binding context. It rejects a source result if the current catalog version or fingerprint differs, and rejects a sequence result if its accepted graph revision or binding set no longer matches the viewer. Both require matching request context and applicable live authority. I found no new safety root cause in the changed range.

## 0. Evidence base

I read the filed remediation plan and my prior Safety report, inspected the full product/test diff, and checked the corrected draft against the pinned Taut preview declarations and graph-schema preview contract. `python -B -m unittest discover -s tests -p 'test_*.py'` passed **8 tests**. The suite checks reference wire shapes; it does not run the UI adapter or media host. Inspection made no file or Git changes.

## 2. Invariant analysis

For a source request made against asset version V and fingerprint F, a later catalog change to V′ or F′ prevents the old result from being presented as current under the corrected rule. Its result carries asset/version/stream and content fingerprint, with no graph fields (`ir/gvid_render_graph.taut.py:336–346`). A changed source during resolution or consumption must fail or use an explicitly new binding under the unchanged resolver contract (`render-engine-design.md:70–72`).

For a sequence request made against graph revision N and binding set B, a later edit or rebind makes the old result fail the current-viewer comparison. Its request and result carry revision, binding-set ID and binding revision (`ir/gvid_render_graph.taut.py:325–332,347–358`). Request identity, fidelity and live-authority checks also apply. Neither branch can satisfy freshness by borrowing the other branch’s identity.

## 3. Risks and next action

The new assertions prove wire shape, while stale-result rejection after a real catalog update or graph/binding change remains an explicit UI/adapter implementation gate. The lane owner may merge this GO with the independent Consistency re-verdict for renderer `21ba5ef9a704ebeeb2857566a27700402944a1cc`.
