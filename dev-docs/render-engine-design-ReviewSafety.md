# Render Engine Design — SAFETY-AXIS REVIEW

**Review object:** `D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md` at `e168ecd815df679202cfd4bc60b9330df64e5305`; Draft design committed 2026-10-02  
**Baseline:** Renderer `e168ecd815df679202cfd4bc60b9330df64e5305`; gvid-wz root `a8f88fdbda3108005166a75545cc6c6b7ef5e2d4`. Both HEADs matched at the start and end. Sources were read with `git show` at those SHAs; `AGENTS_GWZ.md` was read directly.  
**Date:** 2026-10-02  
**Axis:** Safety. Independent, adversarial, read-only; no reliance on other axis.  
**Verdict: NO-GO** — four P2 findings. I pre-commit to GO on a revision that resolves P2-1 through P2-4 as specified.

---

## 0. Evidence base

I assessed the design against `render-requirements.md`, `render-graph-schema-design.md`, `ir/gvid_render_graph.taut.py`, and the root architecture, requirements, and UI design at the stated tuple. An unignored search of root `dev-docs` found no `CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, or `GwzProcessOptimization.md`.

## 1. Findings

### [P2-1] Export and job control have no governed wire contract

**Location:** `render-engine-design.md:19,74`; `ir/gvid_render_graph.taut.py:11-63`. The design says the Glade client translates governed Taut export and cancellation messages. The governed schema lists graph, editor, catalog, bindings, preview, and authority services, but no export request, progress, cancellation, or result contract.

**Invariant:** Structured export and job control across Glade or sidecar boundaries must have versioned Taut identities and delivery semantics (`render-requirements.md:22-25`).

**Reproduction:** Implement the Glade export path against this schema. There is no legal export message carrying the pinned graph revision, binding set, profile, destination, and job ID, nor a correlated progress/cancel/result exchange. The implementer must introduce an ungoverned channel or guess how to reuse another service.

**Impact:** Stale-context rejection, cancellation targeting, and result correlation cannot be established at the boundary; Glade export remains unsafe to integrate.

**Remedy:** Define the versioned export/job request, progress, cancel, and result Taut contracts, including immutable context, authorization, lifecycle and recovery semantics. Align the design’s integration claim with those contracts.

**Closure test:** Cross-language wire tests for two concurrent exports with different revisions and bindings, out-of-order progress/results, cancellation of one job, and a stale result after project reopen.

### [P2-2] Profile identity is not pinned across preparation, execution, cache, and publication

**Location:** `render-engine-design.md:23,27-30,34,82-84`. These clauses name a “profile” and “output profile” but do not require an immutable profile version or definition digest in the workflow and evidence. `render-requirements.md:52,113` requires a named, versioned output profile.

**Invariant:** An in-flight export must retain the exact output policy it was prepared and authorized to produce.

**Reproduction:** Prepare an export using a profile name, revise that profile’s stream or color settings, then run or resume the prepared workflow. The text does not require the run, cache lookup, verifier, and export record to use the original definition rather than the current one.

**Impact:** The same request can produce or reuse an output under changed settings while reporting the original intent.

**Remedy:** Resolve the named profile to an immutable versioned definition at request admission. Carry its identity and effective settings through tasks, cache keys, verification, result, and export record; reject a missing or changed definition on retry.

**Closure test:** Change a profile between prepare and run, and again before retry. Confirm the job uses its pinned definition or fails explicitly, never silently adopting the new one.

### [P2-3] Crash recovery is absent from the job and artifact lifecycle

**Location:** `render-engine-design.md:28,68,84`. Cleanup is described for cancellation, expiry, or failure handling, but not for a host crash or restart. `render-requirements.md:127` and root `gvid-requirements.md:213-214` require abandoned-job and temporary-artifact detection on startup.

**Invariant:** A crash must not leave an apparently active job or unbounded incomplete staging and render artifacts.

**Reproduction:** Kill the host after a Glade source has been staged and an export candidate has been partly written, before verification or normal cleanup. Restart. The design defines no durable ownership marker, startup reconciliation, or terminal status for the abandoned job and its files.

**Impact:** Retried work may remain stuck behind stale ownership or disk exhaustion; incomplete artifacts accumulate without a defined recovery path.

**Remedy:** Specify durable job and temporary-artifact ownership, startup scanning, invalidation or safe resumption of verified work, lease expiry handling, and a terminal diagnostic state. Keep final publication conditional on complete verification.

**Closure test:** Crash at staging, intermediate production, verification, and publication boundaries; restart and verify cleanup or safe reuse, visible job status, and preservation of the prior final destination.

### [P2-4] Resource estimates are admitted but actual native use is not bounded

**Location:** `render-engine-design.md:27-28,52,84`. The scheduler accepts estimates and grants, but the design gives no enforcement rule for a native process or artifact sink that exceeds its grant. `render-requirements.md:107,128` requires fair preview progress and execution within admitted resources.

**Invariant:** Concurrent exports must not consume resources reserved for interactive work, even when an estimate is wrong.

**Reproduction:** Admit several exports whose source metadata understates decode memory or output size. Their native tools then exceed estimated memory, I/O, or disk use while a seek is queued. The stated admission policy has no runtime limit, backpressure, or termination transition.

**Impact:** Preview can be starved indefinitely or the host can exhaust memory or storage; staged and partial files may worsen recovery.

**Remedy:** Define enforceable per-job and aggregate limits at native processes and artifact sinks, measured-use accounting, bounded queues, and a clear capacity failure or pause path when grants are exceeded.

**Closure test:** Run concurrent exports with deliberately underestimated memory and output size while requesting exact frames; verify bounded host use, fair preview service, and explicit capacity outcomes.

## 2. Invariant analysis

The design establishes several useful fail-closed boundaries: accepted snapshots and bindings are immutable inputs (`:9,23`); source replacement must fail rather than silently substitute (`:66`); unsupported effect semantics reject (`:54`); final outputs are verified before publication (`:28,84`); and stale preview results are rejected (`:34,74`). The findings concern paths those rules do not yet close: governed export control, profile drift, hard-crash recovery, and runtime resource overrun.

## 3. Risks and next action

Resolve the four interface and lifecycle gaps in one design revision, then rerun focused safety review against the revised tuple. No files were modified during this review.
