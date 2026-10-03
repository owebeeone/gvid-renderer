# GVid Renderer Runtime Verification Design — CONSISTENCY-AXIS REVIEW, ROUND 2

**Review object:** `gvid-renderer/dev-docs/render-runtime-verification-design.md` at renderer `f4860c74fb7ee58ea34f5451a7d6a9bcc8ebe4e1`; corrected committed DRAFT  
**Baseline:** GWZ root `ebba69d3a3f6d9d682efd64979d518b9ad47502a`; renderer `f4860c74fb7ee58ea34f5451a7d6a9bcc8ebe4e1`  
**Date:** 2026-10-03  
**Axis:** Consistency; independent, focused, adversarial read-only re-review  
**Verdict: GO** — original P2-1 through P2-7 closed; no new P0–P3 finding

## Evidence

I read `render-runtime-verification-design-RemPlan.md`, the corrected committed design, and the diff from `bb2c78e2179a6ee06144e70d3f1c9060c117b1b6` to `f4860c74fb7ee58ea34f5451a7d6a9bcc8ebe4e1`. I retraced each original counterexample against the corrected pass condition and its controlling contract. This review did not run the planned harness or claim runtime implementation evidence.

Both Git SHAs matched the requested tuple at the start and end. `git status --short` was empty in both repositories. No files were changed.

## Prior-finding closure table

| ID | Disposition claimed | Verified original counterexample | Status |
| --- | --- | --- | --- |
| P2-1 | Separate pre-commit absence from post-commit durable admission | EX-05 now requires no ID, job, attempt or tombstone after a crash immediately before commit, followed by fresh admission. A crash after commit requires one recoverable ID/job and replay. The state assertion repeats this split (`design` lines 76, 85). | Closed |
| P2-2 | Add exact graph, binding and footprint oracles | The new `graph-mutations` fixture and RG-01 require an in-span split and shift, atomic registered-asset slot/binding creation, typed rejection, substitution-policy cases and dependent-footprint invalidation. The former append-only and empty-footprint mutants now fail (`design` lines 49, 53, 61). | Closed |
| P2-3 | Add presentation-order and audio synchronization evidence | `timing-audio` now includes B-frame reordering, fractional rates, mixed sample rates/layouts, independent frame and audio references, encoder-delay accounting and A/V offset. MD-01 compares these at declared tolerances; a reordered frame or shifted audio window fails (`design` lines 45, 53, 67). | Closed |
| P2-4 | Mutate the named profile after Prepare | The fixture, `after_profile_prepare` barrier and final-export rule now pin ID, version, digest and effective settings through Run, retry, verification and publication, or require typed failure without publication. A mutable-name reread fails (`design` lines 47, 79, 83). | Closed |
| P2-5 | Cross denied occupancy with valid and invalid payloads | EX-02 now compares complete denials across live, retired and unused IDs under inaccessible, unknown and retired scopes, with valid and intentionally invalid payloads. It requires null job/diagnostic IDs, `replayed=false`, zero index reads and zero launches. Payload validation before scope denial fails (`design` line 73). | Closed |
| P2-6 | Test separate imports of identical portable bytes | The new `import-namespaces` fixture and CL-01 require distinct persistent import IDs and accepted-ID histories, reject reconstruction from a supplied ID when the journal is missing, and retain standalone provenance with a Glade source bridge. A graph-hash-derived import ID fails (`design` lines 50, 70). | Closed |
| P2-7 | Add host-only death with a surviving child and PID-reuse control | The fault harness and EX-03 now run both whole-tree termination and host-only death, record PID plus creation token, terminate the owned orphan on restart and preserve an unrelated process with a reused PID. A supervisor lacking orphan reconciliation fails (`design` lines 34, 74, 85). | Closed |

## Changed-range analysis

The corrected admission oracle agrees with the lifecycle contract’s atomic commit boundary: tentative pre-commit ownership is reclaimed without reserving an ID; committed admissions retain recoverable ownership. The new profile barrier tests the admitted immutable definition rather than a later name lookup. The source-change, destination-alias, retired-ID and storage-restoration additions preserve the controlling distinction between fail-closed intermediate states and durably reconciled outcomes (`design` lines 69, 72, 75–76, 79, 85).

The new fixture and barrier requirements are implementation gates, not changes to the shared interface or architecture. I found no new architectural root cause and no new blocking defect in the changed ranges.

## Invariant analysis

The corrected plan still uses separate freshness identities for independent source and accepted sequence preview; keeps binding-only invalidation separate from graph revision; distinguishes offline CLI provenance from governed Glade authority; and assigns structured control to Taut and raw media ranges to authorized HTTP or local/package access. Its provider fallback, scoped ExportJobs denial, permanent accepted-ID retirement, journal reserve and destination ownership assertions remain consistent with the controlling designs.

## Risks and next action

This GO accepts the corrected **test-design shape** only. Numeric performance budgets and media tolerances still require a versioned baseline before release, and the harness and runtime tests remain to be implemented and executed. File this re-verdict against the corrected tuple and proceed with those implementation gates.
