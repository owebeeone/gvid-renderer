# Render Requirements — SAFETY-AXIS REVIEW

**Review object:** `D:/projects/gvid-wz/gvid-renderer/dev-docs/render-requirements.md`, commit `8a1e5a8f6b79ed1ce35eb5da04890d6d42e7388d`, Initial draft, reviewed 2026-10-01.  
**Baseline:** Renderer commit `8a1e5a8f6b79ed1ce35eb5da04890d6d42e7388d`; root commit `ac674e831d18234e23a2a54689256dc6d20c9016`; Taut commit `3c6d07ecb4cdb2496dd1a7524e270f8cc11a7640`. Pinned commits and required blobs verified at start and end; documents read through immutable Git objects.  
**Date:** 2026-10-01  
**Axis:** Safety, independent, adversarial, read-only. Other axis in parallel; nothing here relies on it. Filed verbatim by lane owner.  
**Verdict: NO-GO** — One P2 finding; no P0, P1, or P3 findings. I pre-commit to GO on a revision resolving P2-1 without weakening the other reviewed requirements.

---

## 0. Evidence base

Read the complete renderer requirements and controlling product requirements. Inspected the architecture’s component ownership, contracts, primary flows, persistence, concurrency, security, failure model, relevant detailed-design assignments, and cross-document invariants.

Relevant evidence:

- Renderer requirements:
  - Taut boundaries and compatibility: lines 22–29.
  - Portable graph, bindings, and sharing: lines 35–54.
  - Mutation acceptance and stale requests: lines 60–78.
  - Batch isolation and retries: lines 86–91.
  - Planning, execution, intermediate reuse, and publication: lines 96–127.
  - Acceptance scenarios and design dependencies: lines 129–164.
- Root product requirements:
  - Asset identity and changed-source detection: lines 115–123.
  - Preview fidelity and stale requests: lines 160–169.
  - Rendering and jobs: lines 187–214.
  - Correctness, bounded resources, recovery, security, and compatibility: lines 255–311.
  - Requirement weakening must be proposed explicitly: line 368.
- Root architecture:
  - Cache trust boundary: line 107.
  - Asset and artifact contracts: lines 243–262.
  - Concurrency and failure invariants: lines 444–501.
  - Cache design ownership: lines 588–599.
  - Cross-document invariants: lines 895–911.
- Pinned Taut documentation:
  - `docs/CodecContract.md`, lines 80–111: canonicalization exceptions, decoding bounds, and generated-code unknown-field behavior.
  - `docs/Reference.md`, lines 268–318: optional-field evolution and root-level decoding bounds.
  - `docs/Overview.md`, lines 29–55: delivery shapes and explicit runtime capability rejection.

Read the supplied process authorities, `AGENTS.md`, `AGENTS_GWZ.md`, and the review-loop skill. No current-round peer report was read. No files were modified, and no builds or tests were run.

At both verification points, `git cat-file -e` succeeded for the three pinned commits. `git rev-parse COMMIT:path` returned:

| Object | Verified blob |
| --- | --- |
| Renderer `dev-docs/render-requirements.md` | `05c002d8cc7dcfca4c86b1c9cf48b58a2cf99091` |
| Root `dev-docs/gvid-requirements.md` | `aefbf035948a8ed43980ec42785ca7c0bd3cdf3e` |
| Root `dev-docs/gvid-arch.md` | `ff51b7cce50ff7b8618e780d1aaad28c6679497c` |

## 1. Findings

### [P2-1] Intermediate-cache safety is weakened from a mandatory invariant to a recommendation

**Location:** Renderer requirements line 116, `GVR-RDR-016`; related batch reuse clause at line 87, `GVR-RDR-051`.

**Violated invariant:** Root `GVR-NFR-MNT-005`, line 311, requires every cache artifact to identify every semantic and toolchain input needed for safe reuse. Architecture line 107 permits trusting a cache hit only after its identity and validation metadata match the request. Architecture line 497 states that cache failure must affect performance rather than correctness.

`GVR-RDR-016` instead says reused intermediates **SHOULD** be keyed by every required semantic input and toolchain version. The product baseline explicitly defines SHOULD as permitting a justified exception. The mandatory second sentence covers invalid or corrupt intermediates but does not clearly require complete request-to-artifact identity comparison. A valid media file can contain the wrong composition.

**Credible state sequence:**

1. Render an intermediate for graph revision R, asset slot S bound to source fingerprint A, and profile P.
2. Retain that valid intermediate under a cache key omitting the binding fingerprint, justified as an exception to the SHOULD.
3. Create a new immutable binding set mapping S to source B. B has the same duration, dimensions, streams, and timing properties as A but different content.
4. Render the new request. Its normalized plan and export record identify B, but the incomplete cache key retrieves the intermediate containing A.
5. The intermediate is structurally valid. Final profile probing can pass because the wrong content has the requested stream, timing, dimension, and metadata properties.

The stronger interactive-only reuse rule in `GVR-RDR-047` closes this route for interactive artifacts. It does not repair the weaker general intermediate requirement. Batch isolation and immutable binding records also do not establish that the bytes retrieved from cache match those records.

**Impact:** The renderer requirements provide an apparent exception to a controlling correctness safeguard. An implementation following that exception can publish the wrong source content while recording the requested source fingerprints. Ordinary output-profile verification need not detect the error.

**Required correction:** Make complete semantic and toolchain identity, and validation against the consuming request, mandatory whenever intermediates are reused. Reuse itself may remain optional. Require a cache miss, regeneration, or explicit failure when identity or validation evidence is incomplete or mismatched. A complete key or an equivalent mandatory metadata comparison can satisfy the invariant.

**Closure/regression test:** Add a required acceptance scenario that renders with binding A, then renders the same graph and profile with a different binding B having identical structural media properties. Force availability of A’s cached intermediate. Verify that it cannot satisfy B’s request, that the output contains B, and that export evidence matches the consumed inputs. Repeat with a changed semantic parameter or toolchain version to exercise the same identity invariant.

## 2. Invariant analysis

- **Late and duplicate mutations:** Commands carry expected revisions and stable IDs; acceptance is atomic; duplicates are idempotent; stale revisions require rejection or documented reconciliation (`GVR-RDR-034`–`037`). An out-of-order delivery cannot legitimately bypass revision checks or partially apply a rejected command.

- **Seek cancellation and stale previews:** Results must match current request, revision, time, quality, and asset context. Superseded work may finish, but its result cannot overwrite the current frame (`GVR-RDR-041`, `044`, `047`). This covers a seek followed by an edit or asset relink while older work remains active.

- **Approximation versus final output:** Fidelity must be labeled, unsupported live effects require pre-rendering or a visible reduced-fidelity state, and preview artifacts cannot become completed exports without final verification and publication (`GVR-RDR-043`–`045`, `049`, `022`). The draft does not equate deterministic semantics with identical lossy encoded bytes.

- **Concurrent interactive and batch work:** Immutable per-item inputs and records prevent later edits from retargeting an export. Admission, resource budgets, and fairness protect interactive requests (`GVR-RDR-050`, `053`–`055`, `011`). Numerical thresholds remain assigned to the preview/jobs designs and must be established before release.

- **Missing sources and interrupted exports:** Planning must identify missing or changed inputs; acceptance scenario 4 explicitly tests disappearance after planning begins. Restart detection, retained retry inputs, candidate verification, and atomic publication protect against presenting incomplete exports as successful (`GVR-RDR-003`, `010`, `017`–`019`, `053`; scenarios 4–6).

- **HTTP resources and credential boundaries:** Structured control remains on Taut; media bytes use opaque descriptors and the authenticated loopback endpoint. The controlling baseline additionally requires session authentication, loopback binding, origin/host validation, and no arbitrary-path endpoint. Missing or expired resource recovery must be completed by the named API/media designs under the architecture’s failure model. No remote-media authorization or fallback is implicitly granted by the renderer requirements.

- **Mixed-version Taut:** Taut’s codec success does not establish semantic compatibility. Generated codecs can drop unknown fields without forward-compat generation, and ordinary optional-field additions can reject old writers. The renderer requirements independently demand explicit handling of unsupported semantics, schema/runtime pinning, migration, and cross-language evolution tests (`GVR-RDR-030`, `059`–`062`). Those requirements prevent treating codec defaults alone as proof of safe graph round trips.

## 3. Risks and next action

This is a requirements review, not evidence that scheduling, recovery, decoding limits, publication, or compatibility have been implemented correctly. The deferred contracts still need concrete limits, state transitions, and acceptance evidence. The final graph field layout and downstream schema implementation were not adjudicated.

**Next action:** Amend `GVR-RDR-016` to preserve mandatory cache identity and request validation, add the P2-1 acceptance scenario, and return the revised immutable tuple for focused verification.