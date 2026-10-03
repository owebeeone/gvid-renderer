# GVid Renderer Runtime Verification Design — CONSISTENCY-AXIS REVIEW, ROUND 3

**Review object:** `gvid-renderer/dev-docs/render-runtime-verification-design.md` at renderer `c0242c235a7aa0ac3b6eda5edc19726427baca09`; corrected committed DRAFT  
**Baseline:** GWZ root `8cf745b199aa5f9b2a140cbacd3ae33949a1cfdf`; renderer `c0242c235a7aa0ac3b6eda5edc19726427baca09`  
**Date:** 2026-10-03  
**Axis:** Consistency; focused adversarial read-only re-verdict  
**Verdict: GO** — original P2-1 through P2-7 remain closed; no new finding or architectural root cause

## Evidence

I read `render-runtime-verification-design-RemPlan-2.md`, `render-runtime-verification-design-ReviewSafety-2.md`, the design at the new renderer SHA, and its diff from `f4860c74fb7ee58ea34f5451a7d6a9bcc8ebe4e1`. I independently checked the changed clauses against the controlling engine and ExportJobs lifecycle designs. The Safety report supplied the two counterexamples for this round; its verdict was not used as my verdict.

Both Git SHAs matched the requested tuple at the start and end. `git status --short` was empty in the root and renderer. No files were changed, and no runtime tests or builds were run.

## Prior-finding closure summary

| Original Consistency finding | Counterexample at the new tuple | Status |
| --- | --- | --- |
| P2-1 — pre-commit ID oracle | EX-05 still distinguishes no accepted ID before commit from one recoverable ID after commit (`design` line 76). | Closed |
| P2-2 — graph/editor mutation semantics | Exact split, shift, atomic binding and footprint oracles remain in the `graph-mutations` fixture and RG-01 (`design` lines 49, 61). | Closed |
| P2-3 — media timing and synchronization | B-frame, fractional-rate, resampling, encoder-delay and A/V alignment oracles remain (`design` lines 45, 53, 67). | Closed |
| P2-4 — mutable named profile | The post-Prepare mutation barrier and pinned-definition rule remain (`design` lines 47, 79, 83). | Closed |
| P2-5 — denied invalid payload | EX-02 still compares complete denial envelopes for valid and invalid payloads without index reads or work (`design` line 73). | Closed |
| P2-6 — CLI import namespace reuse | Distinct imports of identical bytes, missing-journal rejection and standalone bridge provenance remain (`design` lines 50, 70). | Closed |
| P2-7 — orphan process recovery | Host-only death, surviving child and reused-PID token checks remain (`design` lines 34, 74). | Closed |

## Changed-range analysis

**Source mutation.** The new fixture and AS-01 require two separate faults after a pinned range: one changes backing bytes and registered metadata; the other changes backing bytes while registry metadata stays unchanged through the next consumed read or lease renewal (`design` lines 36, 46, 69). Both leased access and bounded staging are exercised. The oracle validates consumed content against the pinned identity and forbids mixed-source verification, publication or cache reuse. A resolver that trusts unchanged registry metadata while consuming modified bytes now fails. This preserves the engine contract’s pinned-version and validated-source requirements (`render-engine-design.md` lines 68–72).

**Destination alias admission.** EX-05 now rejects an unprovable canonical identity before the admission commit barrier. After retry or restart it requires no accepted ID, job, attempt, tombstone, journal reserve or durable destination owner; the corrected destination may use the same request ID for fresh admission (`design` line 76). That order agrees with the lifecycle rule that canonical ownership precedes atomic acceptance (`export-jobs-lifecycle-design.md` line 21) and the engine rule that unprovable aliases make admission fail (`render-engine-design.md` line 102).

These changes refine test inputs and pass conditions without changing a shared interface or architectural ownership boundary. **No new architectural root cause was found.**

## Findings

None.

## Invariant analysis

The new backing-only mutation closes the stale-registry path while retaining the paired metadata-change path. The alias correction aligns the test’s durable-state oracle with the pre-acceptance boundary already used by EX-05. Neither change weakens the existing source/sequence identity, binding invalidation, offline provenance, Taut/HTTP transport, provider fallback, ExportJobs ownership or journal-reserve assertions.

## Risks and next action

This GO accepts the corrected **runtime verification design** at the stated tuple. Numeric performance budgets and media tolerances still require a versioned baseline, and the harness and runtime gates remain future work. File this report with the same-tuple final review decision; do not treat the design verdict as runtime evidence.
