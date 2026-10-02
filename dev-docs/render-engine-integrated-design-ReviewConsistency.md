# GVid Integrated Renderer Design — CONSISTENCY-AXIS REVIEW

**Review object:** `dev-docs/render-engine-integrated-design.md` and its stated package at renderer `1800cd317f61afb0a1c74b9a0c4a67ea8c843710`, draft, 2026-10-02.  
**Baseline:** Renderer HEAD was `1800cd317f61afb0a1c74b9a0c4a67ea8c843710` at start and end; `git status --short` was empty. Root product documents were read with `git show` at `4f75d5814f6033282e09f985e86dfca51d84e548`.  
**Date:** 2026-10-02  
**Axis:** Consistency across the integrated document, controlling contracts, Taut schema and acceptance claims. Independent, adversarial, read-only; nothing here relies on the parallel axis. Filed verbatim by the lane owner.

**Verdict: NO-GO** — one P2 finding blocks. I pre-commit to GO on a revision that resolves P2-1 as specified, provided the correction introduces no new inconsistency.

---

## 0. Evidence base

I read the integrated document and `git diff 8bd582b..1800cd3`; the diff adds only that 45-line document. I checked the renderer requirements, engine, graph-schema, ExportJobs lifecycle and owner-scope designs; their review decisions; `ir/gvid_render_graph.taut.py`, generated binding fields, fixtures and semantic validators. I read the four pinned root product documents, including the UI source/sequence result distinction and composite-preview ADR. `python -B -m unittest discover -s tests -p 'test_*.py'` passed **8 tests**. These are wire and selected semantic checks, not host or runtime proof.

## 1. Findings

### [P2-1] The integrated preview identity rule requires fields an independent source result cannot carry

**Location and violated invariant:** `dev-docs/render-engine-integrated-design.md:23` says *every* preview result carries “request, revision, binding and fidelity context,” immediately after allowing source preview without a graph. `render-requirements.md:24,42` (`GVR-RDR-058`, `069`), `render-graph-schema-design.md:74–76`, and the pinned `ui-design-v2.md` instead distinguish source identity from sequence identity. `SourcePreviewResult` in `ir/gvid_render_graph.taut.py:336–346` has registered asset/version/stream and fingerprint, but no graph revision or binding set; `SequencePreviewResult` at lines 347–358 has those fields.

**Reproduction and impact:** Preview a registered library asset before insertion. The existing `tests/test_taut_contract.py:265–283` round-trips that source request and result without a graph. An adapter implementing the integrated document’s universal rule must reject the valid result or fabricate a revision and binding, defeating the specified independent source viewer and its stale-result check.

**Required correction:** Qualify the sentence by result type: sequence results echo accepted graph and binding context; source results echo registered asset/version/stream, expected fingerprint and their request context. Both echo fidelity and are checked against their applicable live authority.

**Closure test:** Round-trip and present an uninserted registered-source result with no graph or binding fields; invalidate it on asset-version or fingerprint change. Separately require graph revision and binding-set matching for a sequence result.

## 2. Invariant analysis

The principal ExportJobs counterexamples did not reproduce at the contract level. The two provenance branches and accepted-ID keys agree across the integrated document, lifecycle design, owner-scope design and Taut fields. The owner-scope grant supplies historical visibility; inaccessible scopes are denied before ID lookup; retained live records and permanent tombstones prevent a pruned ID from starting a later overwrite. The documents also place revocation, job-ID classification, cancellation and positive handoff under the durable fence. Runtime enforcement remains an implementation gate.

Flat graph edits remain project-authority operations, while render workflows are derived. The pinned root ADR and UI design assign sequence composition to Preview and display to the UI. Provider semantic versions, fallback attempts, Glade/CLI provenance, authenticated HTTP media delivery, and separate batch output records align across the package. The acceptance table identifies future runtime evidence and does not claim those gates ran.

## 3. Risks and next action

Correct P2-1 in the integrated document and rerun its source-versus-sequence identity comparison against the Taut schema and pinned UI contract. The passing Python suite does not establish host authorization, durable recovery, cross-language runtime parity, media parity or publication behavior; the integrated document correctly leaves those as implementation gates.
