# GVid Renderer Runtime Verification Design — SAFETY-AXIS RE-REVIEW

**Review object:** `dev-docs/render-runtime-verification-design.md` at renderer `f4860c74fb7ee58ea34f5451a7d6a9bcc8ebe4e1`; corrected Draft  
**Baseline:** GWZ root `ebba69d3a3f6d9d682efd64979d518b9ad47502a`; renderer `f4860c74fb7ee58ea34f5451a7d6a9bcc8ebe4e1`  
**Date:** 2026-10-03  
**Axis:** Safety; independent, read-only focused re-verdict  
**Verdict: NO-GO** — three original P2 findings closed; P2-1 and P2-3 remain open in narrower forms. No new architectural root cause was found.

---

## Evidence base

I read the round-1 Safety report, the remediation plan, the corrected design at the pinned renderer commit, its diff from `bb2c78e2179a6ee06144e70d3f1c9060c117b1b6`, and the relevant asset, admission, and publication clauses of the controlling engine and lifecycle designs. I did not use the current Consistency review. All commands were read-only.

At both the start and end, the root and renderer HEADs matched the baseline above. `git status --short` was empty in both checkouts.

## Prior-finding closure table

| ID | Disposition claimed | Verified original counterexample | Status |
| --- | --- | --- | --- |
| P2-1 | Mid-export source change after the first consumed range; leased and staged cases | The paired backing-byte **and registered-fingerprint** change now fails. A backing-byte change while registry metadata remains stale can still escape the specified test. | **Open, narrowed** |
| P2-2 | Valid and intentionally invalid denied submits compared across occupancy states | EX-02 now requires complete envelopes, null sensitive fields, no index read, and no launched work for both payload classes. The validation-order disclosure counterexample fails. | **Closed** |
| P2-3 | Unprovable aliases reject; proven aliases collide across Glade and CLI | The concurrent overwrite counterexample fails because the unprovable alias cannot launch or publish. The test still permits durable acceptance before that rejection, contrary to the admission invariant. | **Open, narrowed** |
| P2-4 | Live retry after destination expiry; exact retired envelopes | EX-01 requires replay without source/destination revalidation. EX-05 requires exact and changed retired submits, null job, no work, and snapshot-free lookup. The original counterexamples fail. | **Closed** |
| P2-5 | Restore storage and restart after fsync failure | EX-04 requires durable reconciliation before ownership release and later claimant admission. Permanent recovery-required ownership cannot pass. | **Closed** |

## Changed-range analysis

The correction adds deterministic source, profile, admission, publication, orphan-process, and storage-restoration barriers, with observable effects rather than status-only assertions. The replay, denial, and fsync changes close their original failure traces. The remaining gaps concern the inputs and ordering of two newly added tests. They require bounded test-design edits; neither reveals a new architectural root cause.

## Findings

### [P2-1] Paired metadata mutation can mask a backing-byte integrity failure

**Location:** Corrected design lines 36, 46, and 69 (`asset-versions`, AS-01).

**Invariant:** The resolver must detect changed source content while an export consumes it; a stale registry must not let mixed bytes verify, publish, or enter the cache (engine design lines 68–72).

**Counterexample:** After the first range, change the backing bytes but leave the registered version and fingerprint unchanged until later. An implementation that checks only registry metadata detects the specified *paired* mutation and passes AS-01, yet accepts the changed second range under the original fingerprint in this state.

**Required correction and closure test:** Add a separate backing-bytes-only mutation at `after_first_pinned_source_range`, keeping registered metadata unchanged through the next read or lease renewal. Assert that consumed bytes are verified against the pinned identity and that the old request cannot verify, publish, or seed a reusable artifact with mixed content. Retain the paired metadata-change case.

### [P2-3] Alias rejection is not required before durable acceptance

**Location:** Corrected design line 76 (EX-05), read with the admission assertions at line 85.

**Invariant:** If canonical destination identity cannot be proved, *admission* fails. Canonical ownership is obtained before an accepted ID and job are committed (engine design line 102; lifecycle design line 21).

**Counterexample:** A supervisor commits an accepted ID, job, and journal reservation for an unprovable alias, then returns a typed failure before launching a tool or recording publication intent. It passes EX-05’s stated “before launch or publication intent” check. The rejected request now has a durable ID that may replay or retire, and the caller cannot correct the destination under the same request ID.

**Required correction and closure test:** For the unprovable-alias case, assert rejection before `before_admission_commit`: no accepted ID, job, attempt, tombstone, reservation, or durable destination owner exists after retry or restart. A corrected request using the same ID must be freshly admissible.

## Invariant analysis

The corrected design now tests the required live replay, retired-ID outcomes, unauthorized invalid-payload denial, cross-process alias collision, and storage recovery. The two open cases still allow a passing harness to miss source-content substitution or to reserve an ID for a request that the governing contract says must fail admission.

## Risks and next action

Amend AS-01 with a backing-only mutation and EX-05 with explicit pre-admission rejection assertions. Recheck those two original findings at a new settled tuple. The design remains a test plan; this review claims no runtime implementation or pass.
