# GVid Renderer Runtime Verification Design — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/render-runtime-verification-design.md` at renderer `bb2c78e2179a6ee06144e70d3f1c9060c117b1b6`; Draft, added by this commit  
**Baseline:** GWZ root `270b0ada3f21e2d5b8a76c4a31330127ff285bd4`; renderer `bb2c78e2179a6ee06144e70d3f1c9060c117b1b6`. The review object was read with `git show HEAD:...`; controlling documents were read from the clean renderer checkout.  
**Date:** 2026-10-03  
**Axis:** Safety; independent adversarial read-only review  
**Verdict: NO-GO** — five P2 findings. Each has a bounded test-design correction; GO is possible on a corrected revision after focused re-review.

---

## 0. Evidence base

I read the workspace `AGENTS.md` and `AGENTS_GWZ.md`, the review-loop skill, the review object, the accepted integrated design and decision, and the relevant engine, lifecycle, owner-scope, graph-schema, and requirements clauses. The specified program checkpoint and process documents are absent from this GWZ root; I did not infer their contents. I ran inspection commands only.

At both the start and end, `git rev-parse HEAD` returned the exact root and renderer SHAs above. `git status --short` returned no changes in either checkout.

## 1. Findings

### [P2-1] A source can change during export without failing the planned check

**Location:** Runtime design lines 36, 46, and 67 (`asset-versions`, AS-01).

**Invariant:** The Glade resolver must detect a changed source version or fingerprint *while consuming an export* and fail or require an explicit new binding set (engine design lines 70–72, 111).

**Counterexample:** The AS-01 test changes bytes before resolution, observes a typed failure, and passes. During a separate export, the source changes after Prepare and after the first range read. The renderer consumes later bytes from the new version and publishes a mixed-source result under the old fingerprint. The plan names a fingerprint-changing capability but no barrier during consumption or assertion on the final file and evidence.

**Correction and closure test:** Add a deterministic mid-export barrier after a pinned source range has been consumed. Change the registered fingerprint and backing bytes before the next range or lease renewal. Require failure with no publication or, where policy allows, a new explicit binding set and new export. Assert that no mixed-source artifact becomes verified or reusable. Exercise leased Glade access and bounded staged access separately.

### [P2-2] The denial oracle omits intentionally invalid payloads

**Location:** Runtime design line 71 (EX-02).

**Invariant:** Unauthorized scope requests must be denied before accepted-ID lookup or proposed-payload validation can reveal another owner’s ID occupancy. The owner-scope contract explicitly requires byte-identical live, retired, and unused probes with both valid and intentionally invalid payloads (owner-scope design line 43; lifecycle design line 53).

**Counterexample:** A host denies valid payloads uniformly, so EX-02 passes. For an invalid payload, it validates before the scope check only when the request ID is unused, returning `invalid_request`; a matching live or retired ID returns `unauthorized`. An unauthorized caller can distinguish occupancy.

**Correction and closure test:** Require the three-state, complete-envelope comparison twice, with a valid payload and an intentionally invalid one. Assert identical non-echo denial fields, null job and diagnostic IDs, `replayed=false`, no accepted-ID read, and no work launched.

### [P2-3] Unprovable destination aliases have no rejection case

**Location:** Runtime design lines 48, 74, and 83 (`publication`, EX-05, alias collisions).

**Invariant:** Publication requires a canonical destination identity shared by Glade and CLI; admission must fail when aliases cannot be proven to identify the same target. A competing active claimant must receive destination-busy without launching work (engine design line 102).

**Counterexample:** The fixture uses aliases already known to resolve to one canonical target, and the collision test passes. A different alias whose identity cannot be proved is admitted under a separate lock. Two processes then replace the same final path, bypassing exclusive ownership and generation checks.

**Correction and closure test:** Add an alias whose canonical equivalence cannot be established at admission. Require a typed rejection before job launch or publication intent. Also assert a typed destination-busy result and zero launched resources for an active claimant using a proven alias, including a Glade-versus-CLI cross-process case.

### [P2-4] Accepted-ID retirement can pass without the required replay outcomes

**Location:** Runtime design lines 70 and 74 (EX-01, EX-05).

**Invariant:** A live exact retry must replay the original job despite an expired original destination reference. After pruning, both exact and changed-payload retries must return `retired_request` with null job ID and no execution; lookup must return retirement without a snapshot (lifecycle design lines 18, 37, 51).

**Counterexample:** The suite retries live jobs only while the destination reference is valid. It tests a tombstone conversion crash and checks merely that an “old retry” does not rerun or overwrite. An implementation can reject the live retry as `destination_expired`, or return a fabricated success/job for a retired ID while doing no work, and still meet those written observations.

**Correction and closure test:** After a lost acceptance acknowledgement, expire the original destination reference, reopen, then require exact replay of the original retained job without destination revalidation. After durable retirement, submit both exact and changed payloads and assert the precise `retired_request` envelope, null job ID, `replayed=false`, no source/destination validation or execution, and a snapshot-free retired lookup.

### [P2-5] The fsync fault may leave permanent recovery-required ownership and still pass

**Location:** Runtime design lines 73 and 83 (EX-04 and state assertions).

**Invariant:** A physical write/fsync failure must retain ownership while storage is unavailable, then reconcile durably after storage is restored before ownership release (lifecycle design line 41).

**Counterexample:** The harness injects an fsync failure, observes that success and ownership release are withheld, and passes. The supervisor never retries reconciliation after storage is restored. The accepted job and destination remain stuck indefinitely, blocking later legitimate work.

**Correction and closure test:** After the injected failure, restore writable storage and restart the supervisor. Require durable reconciliation of process, artifacts, accepted ID, publication intent, and destination generation, followed by the appropriate terminal or recoverable state. Only then may it release ownership and admit a later claimant. Assert that the same test remains fail-closed while storage is still unavailable.

## 2. Invariant analysis

The draft provides named crash barriers, process-tree observations, final probing, scoped authorization fences, and durable destination assertions. Those controls address many failure paths. The five gaps above are cases where an implementation can satisfy the stated scenario observations while violating a governing invariant: mixed source bytes, request-ID disclosure, alias-based ownership bypass, incorrect accepted-ID replay, or permanent recovery blockage.

## 3. Risks and next action

Add the five negative controls and their exact expected envelopes, resource effects, and recovery end states to the runtime matrix. Then re-review the corrected committed design at a new settled tuple. No implementation or runtime pass is claimed by this review.
