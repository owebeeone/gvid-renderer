# ExportJobs Owner-Scoped Request Namespace — CONSISTENCY-AXIS RE-REVIEW

**Review object:** Round 1 remediation at renderer `730b52cd81456f4d7863548fde6b7f3613f5bf12`  
**Baseline:** Renderer `acf12b40a5aeaab715df188297ee002d24b01459`; root product documents pinned to `fa0db729054a9aeb0d3d855cbdea273bc509f419`  
**Date:** 2026-10-02  
**Axis:** Consistency. Independent, adversarial, read-only.

**Verdict: GO** — prior P2-1 is closed; no new P0–P2 finding.

---

## Prior-finding closure table

| ID | Disposition claimed | Verified on corrected tree | Status |
| --- | --- | --- | --- |
| P2-1 | Compare complete responses only for byte-identical denied requests; verify individual echoes and compare non-echo fields when IDs differ. | `export-jobs-owner-scope-design.md` §§2, 4 and the lifecycle/graph-schema clauses now make that distinction. The validator binds a denial to its originating request. The test checks encoded equality for identical denials, and exact echo plus non-echo equality for a different ID. | Closed at the draft contract and reference-test level. |

## Changed-range analysis

`acf12b4..730b52c` changes the owner-scope, lifecycle, engine and graph-schema wording; the denial validator; and focused tests. It also files the prior reviews and remediation plan. The new durable authorization fence orders grant revocation, scope retirement, index classification and admission commit, with a disclosure check before transport handoff. These clauses agree across the changed documents. The Taut layout, generated bindings and standalone namespace are unchanged. I found no new contradiction in the changed ranges.

## 0. Evidence base

I read the filed Consistency review and remediation plan, inspected the corrected documents, validator and tests, and reviewed `git diff acf12b4..730b52c`. I reran `python -B -m unittest discover -s tests -p 'test_*.py'`: eight tests passed. Renderer `HEAD` matched `730b52cd81456f4d7863548fde6b7f3613f5bf12` at start and end, and `git status --short` remained empty.

## 1. Findings

None.

## 2. Invariant analysis

The original counterexample no longer applies: distinct request IDs necessarily produce distinct echoed contexts, and the corrected gate compares their non-echo fields. Complete-response comparison now holds the serialized request fixed while varying occupancy in isolated stores or controlled transitions. The test’s three identical acknowledgements prove only the reference wire comparison shape; the documents explicitly reserve actual live/retired/unused occupancy checks for host conformance. The denial validator rejects another job’s context or a mismatched contract version.

The new fence also preserves the stated grant-before-index ordering at classification, durable acceptance and disclosure. Its implementation remains a later runtime gate.

## 3. Risks and next action

The passing Python tests do not prove host transactions, occupancy indistinguishability, crash recovery or Rust/TypeScript runtime parity. Those limits are stated accurately. The next action is to merge this GO verdict with the independent Safety re-verdict for the same renderer revision.
