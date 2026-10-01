# Render Engine Design — CONSISTENCY-AXIS FINAL RE-REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/render-engine-design.md` and the final remediation diff at renderer `26a45d176b4cce541dc58edd1c0f5ccc4a5a435c` and gvid-wz root `c24cc8a0640ed2fc84e454338521a947785e7c42`. The design remains unaccepted.  
**Baseline:** Both HEADs matched those SHAs at review start and end, verified with read-only `git rev-parse HEAD`. Prior tuple: renderer `3e6e4883de5f43391e3df842dbd5a450dac635db`; root `567da536bf08a0d84b169c5408ff8dc019b0e87e`.  
**Date:** 2026-10-02  
**Axis:** Consistency, independent adversarial read-only  
**Verdict: NO-GO** — one new P2 architectural finding.

---

## Prior-finding closure table

| Round-2 finding | Original counterexample re-traced at the revised tuple | Status |
| --- | --- | --- |
| Consistency P2-1 — UI layered-plan trace | `ui-design-v2.md:240` now directs the Preview adapter to resolve and present one leased resource without browser graph-layer assembly. This agrees with its section 5, the ADR and root architecture. | Closed |
| Consistency P2-2 — lost accepted acknowledgement | `export.lookup` accepts project/request ID and current caller context, then returns the original job and snapshot. The durable index precedes destination-reference expiry checks; canonical submit identity excludes delivery incarnation (`render-engine-design.md:82`; `render-graph-schema-design.md:84,88`; Taut lines 408–416). | Closed for the governed-host counterexample |
| Consistency P3-1 — missing pixel oracle | The design and ADR now require recognizable overlapping layers, a decoded reference comparison at a specified time and tolerance, and omission/order negative controls (`render-engine-design.md:103`; ADR line 24). | Closed |
| Safety P1-1 — same-destination rollback | A shared canonical-destination owner spans admission through reconciliation. Durable intent blocks a new owner after a crash; rollback is conditioned on the owned generation, and later replacement is explicit (`render-engine-design.md:94–96`; schema design line 86). | Closed as a design rule; implementation proof remains due |
| Safety P2-1 — lost accepted acknowledgement | The same request-ID lookup and original-context snapshot close the reopen sequence, including an expired original destination reference. The fixture demonstrates the new wire shape. | Closed for the original sequence |
| Safety P2-2 — no-hard-limit starvation | The design now requires an enforceable limit or measured bounded preemption for each resource that could block preview; otherwise export is rejected or deferred while interactive service is promised (`render-engine-design.md:92`). | Closed as a design rule; implementation proof remains due |

## Changed-range analysis

The renderer diff changes the ExportJobs admission and recovery rules, destination ownership and generation rules, resource fallback and proof points; adds `export.lookup`, destination policy and generation fields to Taut and generated Python, Rust and TypeScript bindings; and updates fixtures and contract tests. The root diff corrects the UI trace and ADR pixel oracle. Root workspace bookkeeping changes are outside the review object.

**NEW ARCHITECTURAL root cause:** The newly added *new-request admission rule* unconditionally requires a current incarnation in the shared ExportJobs contract, although its standalone-import branch expressly has no host incarnation. This is P2-1 below. It is a new cross-client boundary defect in the second and final remediation round. Under the review-loop cap, the lane stops for redesign-or-accept; this report does not prescribe another routine patch round.

## 0. Evidence base

I compared the exact diffs with the revised design, graph schema design, Taut definitions, generated bindings, fixtures, tests, renderer requirements, root architecture, product requirements, UI design and composite-preview ADR. I read the committed round-2 reports and RemPlan-2, without reading the parallel current-round Safety report. Root `dev-docs/CurrentProgramCheckpoint.md`, `AgentProcessRules.md` and `GwzProcessOptimization.md` were absent at their named paths; the repository search found references to them, not those documents. Root `AGENTS.md` and `AGENTS_GWZ.md` supplied workspace instructions.

The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` passed six tests. These establish schema round trips and golden bytes, not admission behavior.

## 1. Findings

### [P2-1] New standalone export requests cannot satisfy the shared admission rule — **NEW ARCHITECTURAL**

- **Root cause and exact location:** The final remediation adds “A new request ID must pass current-incarnation and destination validation” at `render-engine-design.md:82` and “must have a current incarnation” at `render-graph-schema-design.md:86`. The same contracts require standalone imports to carry a local import ID **and no host incarnation** (`render-engine-design.md:25,80`; `render-graph-schema-design.md:82`). Taut makes `authority_incarnation_id` optional for this branch (`ir/gvid_render_graph.taut.py:366–373`).
- **Violated invariant:** A validated offline CLI import must be able to start an export through the shared core and optional Taut machine protocol without fabricating Glade authority.
- **Credible sequence:** The CLI validates a portable graph and bindings, creates a fresh `standalone_import` request with `local_import_id=import-1` and `authority_incarnation_id=null`, as exercised by `tests/test_taut_contract.py:276–289`. Enforcing the new admission sentence rejects it for lacking a current incarnation. Accepting it requires an unstated provenance-specific exception to both controlling paragraphs.
- **Impact:** The final contract gives opposite conformance decisions for the same valid offline request, breaking the required standalone path and Glade/CLI parity.
- **Required correction:** State admission by provenance: require a current project authority incarnation for a *new governed-host* request; validate the local import identity and local destination authority for a *new standalone-import* request. Apply corresponding provenance-specific authorization to lookup and replay without granting standalone imports host authority.
- **Closure test:** Submit and recover a fresh offline CLI job with a local import ID and no host incarnation; reject a fresh governed-host request with a stale incarnation; confirm both variants preserve request-ID idempotency and return their original job after acknowledgement loss.

## 2. Invariant analysis

The revised UI, ADR and renderer requirements agree that a ready sequence preview is one leased, renderer-composited resource, with a pixel oracle capable of detecting omitted or misordered layers. The lost-acknowledgement path is expressible in the new lookup query/result and generated bindings: the result’s status snapshot contains the original job ID and context. The revised destination contract defines one active canonical owner, typed `destination_busy`, explicit replacement policy and a committed generation. The no-hard-limit rule now has a reject/defer outcome.

The six passing tests do not resolve P2-1: they round-trip a standalone request that the new prose would reject. Publication crash behavior, host authorization, cross-language execution and resource preemption remain implementation proof points rather than evidence of a running service.

## 3. Risks and next action

**NO-GO.** Resolve the provenance-specific admission contradiction through the review-loop’s redesign-or-accept decision. The final remediation round is exhausted; no routine third architectural patch round follows from this verdict.
