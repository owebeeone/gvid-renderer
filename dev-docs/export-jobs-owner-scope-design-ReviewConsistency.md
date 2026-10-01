# ExportJobs Owner-Scoped Request Namespace — CONSISTENCY-AXIS REVIEW

**Review object:** Draft `dev-docs/export-jobs-owner-scope-design.md` and companion changes at renderer `acf12b40a5aeaab715df188297ee002d24b01459`  
**Baseline:** Renderer `HEAD^..acf12b4`; root product documents read with `git show fa0db729054a9aeb0d3d855cbdea273bc509f419:`  
**Date:** 2026-10-02  
**Axis:** Consistency. Independent, adversarial, read-only; nothing here relies on the parallel review.

**Verdict: NO-GO** — one P2 finding blocks. I pre-commit to GO on a revision that resolves P2-1 as specified, provided no new inconsistency is introduced.

---

## 0. Evidence base

I read the owner-scope design, the changed lifecycle, engine and graph-schema clauses, the Taut schema and generated Python/Rust/TypeScript fields, fixtures, semantic validators, and contract tests. I checked the lifecycle review decision and the pinned root architecture, UI design and Taut feedback documents. `git diff HEAD^ HEAD` identified 23 changed files. `python -B -m unittest discover -s tests -p 'test_*.py'` passed all eight tests. Renderer `HEAD` was `acf12b40a5aeaab715df188297ee002d24b01459` at both start and end; `git status --short` was empty at both checks.

## 1. Findings

### [P2-1] The complete-denial equivalence gate cannot be satisfied across different request IDs

**Location:** `dev-docs/export-jobs-owner-scope-design.md` §§2 and 4, lines 21 and 41; `ir/gvid_render_graph.taut.py` `ExportJobContext` and `ExportJobAck` declarations, lines 370–386.

**Violated invariant:** The design requires a denied submit acknowledgement to echo the caller-supplied context, including `request_id`, while also requiring the *same complete unauthorized envelope* for A’s live ID, retired ID and an unallocated ID.

**Reproduction:** Submit three denied requests that differ only in those three request IDs. Under the specified echo rule, `ExportJobAck.context.request_id` must differ in each response. Their complete envelopes therefore cannot be equal. The current test uses one denied fixture and checks only null job/diagnostic IDs and `replayed=false`; it does not exercise the claimed three-way comparison.

**Impact:** The central submit-disclosure closure gate has contradictory pass conditions. An implementation can follow the echo contract and fail the written gate, or a test can silently ignore fields while claiming complete-response equivalence. This prevents the gate from establishing the privacy claim precisely.

**Correction:** Compare complete responses for an *identical serialized request* against live, retired and unused occupancy in isolated stores or controlled state transitions. For comparisons across different request IDs, explicitly compare only the non-echo denial fields and verify that each response echoes its own request unchanged.

**Closure test:** Exercise all three occupancy states with byte-identical denied request inputs and assert complete response equality; separately exercise distinct IDs and assert exact context echo plus equality of the remaining denial fields.

## 2. Invariant analysis

The scoped key, grant-before-index rule, historical read visibility, scope non-reuse, standalone branch, and Taut tag additions agree across the amended documents and schema. Tags 14, 6 and 7 are append-only in the respective messages. The new fields appear in all three generated binding languages. The reference validators and passing tests establish limited Python wire round trips and selected semantic shapes; they do not claim runtime authorization, durability or cross-language execution proof. I found no additional contradiction with the pinned root documents.

## 3. Risks and next action

Runtime grant enforcement, complete denial indistinguishability, crash recovery and Rust/TypeScript wire parity remain stated implementation or pre-freeze gates. The next action is to correct the impossible equivalence wording and add the closure test for P2-1, then obtain a focused re-verdict on the corrected renderer revision.
