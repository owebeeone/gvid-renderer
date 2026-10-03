# GVid Renderer Runtime Verification Design — CONSISTENCY-AXIS REVIEW

**Review object:** `gvid-renderer/dev-docs/render-runtime-verification-design.md` at `bb2c78e2179a6ee06144e70d3f1c9060c117b1b6`; committed DRAFT, newly added relative to its parent  
**Baseline:** GWZ root `270b0ada3f21e2d5b8a76c4a31330127ff285bd4`; renderer `bb2c78e2179a6ee06144e70d3f1c9060c117b1b6`. Sources were read from the named directories using read-only Git and file-inspection commands.  
**Date:** 2026-10-03  
**Axis:** Consistency; independent adversarial read-only review  
**Verdict: NO-GO** — 7 P2 findings; 0 P0, P1 or P3 findings

---

## 0. Evidence base

The review compared the committed draft with `render-requirements.md`, `render-graph-schema-design.md`, `render-engine-design.md`, `export-jobs-lifecycle-design.md`, `export-jobs-owner-scope-design.md`, and the accepted integrated design and decision. The GWZ workspace instructions were read. `CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, and `GwzProcessOptimization.md` were absent at the checked root locations, so no content was inferred from them. No tests, builds or writes were performed.

Both SHAs were checked again at the end and remained exactly as stated above. `git status --short` was empty in both the root and renderer.

## 1. Findings

### [P2-1] The pre-acceptance crash oracle requires an ID that must not exist

**Location:** Runtime draft lines 70, 74 and 81–83. **Violated invariant:** A crash before the atomic admission commit leaves no accepted ID or tombstone; only a committed admission reserves one. This is required by `export-jobs-lifecycle-design.md` lines 21 and 35 and `export-jobs-owner-scope-design.md` line 27.

**Reproduction and impact:** Crash at `before_admission_commit`. EX-05 says *each recovery state* has one accepted ID or tombstone, so a correct implementation with no ID fails the proposed test. Creating an ID to satisfy that oracle would violate the admission contract and could strand or falsely retire a request ID.

**Required correction:** Split EX-05’s oracle at the commit boundary: before commit, assert no accepted ID, job, attempt or tombstone and permit a fresh admission after reconciliation; after commit, assert exactly one durable ID and recoverable job. **Closure test:** Crash on both sides of admission commit, reopen, retry the same key and verify the corresponding distinct outcomes.

### [P2-2] The graph/editor pass condition does not test the mutation semantics it claims

**Location:** Runtime draft lines 44, 46, 59 and 98. **Violated invariant:** Initial ripple insertion must split a containing span and shift later spans atomically; registered-asset insertion must create the graph slot and binding together; unsupported placement and overlap cases must reject. Binding-only rebind must validate substitution policy and publish a conservative dependency footprint. See `render-requirements.md` lines 39, 44, 57, 79, 83 and 173–178, and `render-graph-schema-design.md` lines 58, 60 and 68–70.

**Reproduction and impact:** Implement `insert_source_span` as an append that advances one revision, then serialize and reload that wrong graph. RG-01’s stated round-trip and revision checks can pass despite wrong timeline meaning. Likewise, a binding change with an empty footprint can pass its stated new-set check while leaving a dependent preview current.

**Required correction:** Give RG-01 exact expected graph and binding records for an insertion inside a span, a registered-asset insertion, undo/redo after reopen, rejected placement/overlap cases, substitution-policy decisions and binding footprints. **Closure test:** Mutants that omit the split, fail atomic slot creation, accept unsupported placement, or omit a dependent interval must each fail.

### [P2-3] The media fixtures omit required timing and synchronization cases

**Location:** Runtime draft lines 45, 47, 65 and 87–89. **Violated invariant:** The acceptance scenarios require B-frame and fractional-rate cuts, mixed input sample rates and channel layouts, and output audio/video synchronization including encoder delay. See `render-requirements.md` lines 143 and 151–152.

**Reproduction and impact:** A decoder that handles simple VFR timestamps but chooses the wrong presentation frame around B-frame reordering, or an audio path that mishandles sample-rate conversion or encoder delay, can pass the specified `timing-audio` landmarks and gain/fade windows. The release gate would then claim media timing evidence while a required case remains untested.

**Required correction:** Add independently generated B-frame/fractional-rate and mixed-rate/layout fixtures, with cut-boundary frame identities and measured output audio/video alignment and encoder-delay oracles. Tolerances may be versioned later, but the measurements and acceptance method must be specified now. **Closure test:** Deliberate one-frame reordering and one audio-sample-window offset must fail.

### [P2-4] No scenario changes a named profile between Prepare and Run or retry

**Location:** Runtime draft lines 47, 65–66, 68 and 77. **Violated invariant:** The profile’s ID, version, definition digest and effective settings are pinned at admission; Run, retry, verification and publication must use that definition or reject. `render-engine-design.md` lines 25–27 and 111 explicitly require this proof.

**Reproduction and impact:** Admit profile `P` at definition v1, pause after Prepare, change the host registry entry named `P` to v2, then resume or retry. An implementation that looks up the mutable name again can render v2 and pass the draft’s ordinary two-profile tests. The output no longer represents the admitted request.

**Required correction:** Add a barrier after profile resolution/Prepare, mutate or remove the named profile, and assert every later stage uses the original pinned definition or returns a typed mismatch without publication. **Closure test:** A provider or verifier that rereads v2 must fail the scenario.

### [P2-5] The scope-denial matrix misses payload-validation ordering

**Location:** Runtime draft lines 49, 70–71 and 98–99. **Violated invariant:** An inaccessible, retired or unknown governed scope must be denied independently of both accepted-ID occupancy and proposed payload validity. `export-jobs-owner-scope-design.md` lines 23 and 43 and `export-jobs-lifecycle-design.md` line 53 require complete-response comparisons for valid and intentionally invalid payloads.

**Reproduction and impact:** Make an inaccessible-scope submit with a malformed profile. A gateway that validates the profile before checking scope returns a distinct invalid-payload response; valid denied requests against live, retired and unused IDs can still pass EX-02. The differing response reveals processing order and fails the required occupancy-independent denial contract.

**Required correction:** Cross the live/retired/unused ID states with valid and intentionally invalid payloads, plus inaccessible, retired and unknown scopes, comparing complete denial envelopes and external effects. **Closure test:** A gateway that returns a payload error before scope denial must fail.

### [P2-6] The CLI tests do not prove durable import namespace non-reuse

**Location:** Runtime draft lines 29, 67–70 and 97–99. **Violated invariant:** Each offline import receives a new non-reused local import ID, even when importing the same portable graph again; an absent journal/import cannot be reconstructed from a caller-supplied old ID. See `export-jobs-lifecycle-design.md` lines 11–13 and `export-jobs-owner-scope-design.md` line 15.

**Reproduction and impact:** Derive `local_import_id` from graph bytes. A fresh import, restart, lookup and render all pass the draft’s single-import path. Importing that graph a second time then aliases the first namespace, causing an old request ID to replay or conflict across separate imports.

**Required correction:** Exercise two fresh imports of identical graph/package bytes, assert distinct persistent IDs and independent accepted-ID histories, then remove or make one journal unavailable and prove its old ID cannot be recreated by request. Repeat with a Glade asset bridge while retaining standalone provenance. **Closure test:** Graph-hash-derived or caller-recreated import IDs must fail.

### [P2-7] Killing the whole process tree cannot prove startup orphan reconciliation

**Location:** Runtime draft lines 34, 72 and 81–83. **Violated invariant:** After a host crash, startup must identify a surviving native process by recorded identity and creation token, terminate a verified orphan without targeting a reused PID, and reconcile its artifacts. See `render-engine-design.md` lines 98 and 112 and `render-requirements.md` line 127.

**Reproduction and impact:** The prescribed crash kills the supervised host/CLI *and its entire process tree*. EX-03’s “no orphan continues” then passes even if restart reconciliation contains no orphan detection at all. A host-only abrupt death that leaves a native child running exposes that defect.

**Required correction:** Keep the whole-tree kill case, and add a controlled host-only crash with a surviving supervised child, plus a reused-PID/changed-creation-token case. Record the process token and verify termination only of the owned child before journal reuse. **Closure test:** Disabling startup orphan reconciliation, or terminating the reused unrelated PID, must fail.

## 2. Invariant analysis

The draft correctly distinguishes independent source-preview identity from sequence graph/binding identity in PV-01/PV-03, and it keeps Taut control separate from authorized HTTP media ranges in CL-01. It also states the binding-only graph-digest rule, new-attempt provider fallback, governed versus standalone provenance, owner-scope fences, tombstone retention, journal reserve and destination-generation ownership. Those statements do not close the narrower proof gaps above.

The accepted integrated design’s proof matrix remains runtime work. In particular, a passing set of the current scenario rows would not establish the omitted editor semantics, named-profile pin, complete owner-scope denial ordering, offline import namespace, media timing cases or orphan recovery. No implementation pass is claimed by the draft.

## 3. Risks and next action

Correct the seven pass conditions and fixtures before converting the rows to executable cases. The corrections are bounded document changes; the revised design needs a focused re-review against these counterexamples at a new settled tuple.
