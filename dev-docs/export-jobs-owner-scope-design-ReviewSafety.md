# ExportJobs Owner-Scoped Request Namespace — SAFETY-AXIS REVIEW

**Review object:** `dev-docs/export-jobs-owner-scope-design.md` and companion `HEAD^..acf12b4` changes, Draft, 2026-10-02.  
**Baseline:** Renderer `acf12b40a5aeaab715df188297ee002d24b01459`; root product documents read with `git show fa0db729054a9aeb0d3d855cbdea273bc509f419:<path>`. Renderer HEAD matched at the start and end; its working tree remained clean.  
**Date:** 2026-10-02  
**Axis:** Safety: degraded paths, disclosure, revocation, durable ordering and stuck states. Independent, adversarial, read-only. Nothing here relies on the parallel review. Filed verbatim by the lane owner.

**Verdict: NO-GO** — two P2 findings block. I pre-commit to GO on a revision that resolves P2-1 and P2-2 as specified, provided the corrected tuple introduces no new blocker.

---

## 0. Evidence base

Read the complete owner-scope design, especially §§1–4; the changed lifecycle, render-engine and graph-schema clauses; `ir/gvid_render_graph.taut.py:370–446`; `ir/validate_export_owner_scope.py:16–66`; `ir/validate_export_responses.py:12–36`; the changed fixtures and `tests/test_taut_contract.py:435–505`. Checked the lifecycle review decision and renderer requirements. Read pinned root `gvid-arch.md`, `ui-design-v2.md` and `GvidTautProto-Feedback.md` from `fa0db729`. `python -B -m unittest discover -s tests -p 'test_*.py'` passed 8 tests. This suite tests wire and reference-validator shapes; it does not execute a host authority or durable store.

## 1. Findings

### [P2-1] Scope authorization is not linearized with index access or durable admission

**Location:** `dev-docs/export-jobs-owner-scope-design.md:9,11,21–25,33,35,41–43`; corresponding lifecycle clauses at `dev-docs/export-jobs-lifecycle-design.md:17–21,31,51`.

**Violated invariant:** Revocation must stop further access, and retirement must stop new acceptance. Checking a grant *before* index access does not establish that invariant when grants and accepted IDs change concurrently.

**Reproduction:** B has a delegated grant to A’s scope and starts `export.lookup`. The gateway checks B’s grant. The authority then revokes B. A subsequently accepts a previously unused request ID. B’s already authorized lookup reads the index and returns `found` for that new ID. This result cannot be linearized before revocation: the ID did not exist then. The same gap lets a submit pass its grant check, have its scope retired, then reach durable new-key admission. The text specifies a delivery linearization point for events but no equivalent fence for submit or lookup.

**Impact:** A revoked caller can learn a later private job’s context, or a retired scope can accept work after retirement. Both violate the proposed authority partition under the document’s own ordering.

**Required correction:** Specify and enforce a common ordering between scope grant/revocation/retirement and scoped index read or acceptance. A transaction, lock, or version fence is sufficient if a response reflecting a post-revocation ID cannot pass and a post-retirement acceptance cannot commit. Apply the rule to replay, conflict and tombstone reads as well as new admissions.

**Closure test:** Pause B immediately after a successful grant check; revoke B; let A accept a new ID; resume B’s lookup and submit probes and require occupancy-independent denial with no job data. Separately pause a new submission before durable acceptance, retire its scope, resume, and require no accepted ID or work. Repeat after restart.

### [P2-2] The submit-denial validator permits an unauthorized response containing private job context

**Location:** `ir/validate_export_owner_scope.py:61–66`, `ir/gvid_render_graph.taut.py:370–387`, and `tests/test_taut_contract.py:473–479`; contract at `dev-docs/export-jobs-owner-scope-design.md:21,37,41`.

**Violated invariant:** An unauthorized acknowledgement must echo the caller-supplied context and be identical across another owner’s live, retired and unused IDs. It must not carry resolved job facts.

**Reproduction:** Start with `export_job_ack_scope_denied.json`. Replace its `context` with A’s private accepted job context while retaining `status="unauthorized"`, `job_id=null`, `replayed=false` and `diagnostic_id=null`. `ExportJobAck` permits the context, and `validate_export_submit_denial` accepts it because it examines only those four fields. The changed test mutates job ID, diagnostic ID and replay flag, but never compares acknowledgement context with the request.

**Impact:** An adapter following the reference check can return another owner’s graph, sequence, revision, binding or profile identifiers in a denial. Occupancy-dependent context makes the complete response a privacy oracle even though the job ID is null.

**Required correction:** Bind denial validation to the submitted request and require exact context echo; use that check before sending and after decoding. Keep host conformance tests for grant decisions and complete response equivalence.

**Closure test:** For the same inaccessible scope, compare full acknowledgements for live, retired and unused IDs under valid and semantically invalid payloads. Inject an acknowledgement with a different accepted job’s context; both the semantic validator and adapter conformance gate must reject it.

## 2. Invariant analysis

The proposed key includes the owner scope, and its declared ordering authorizes that scope before consulting an accepted-ID index. This blocks the prior project-wide collision probe in a sequential execution. Unknown, retired and inaccessible scopes share a specified submit denial. Authorized exact replay, changed-payload conflict and tombstone responses preserve their distinct outcomes within a scope. Standalone imports retain their independent namespace and null owner scope. The schema appends optional tags without changing existing tags; the 8 reference tests pass.

Those checks do not close the concurrency gap in P2-1 or the forbidden denial envelope in P2-2. Job-ID status, events and cancel specify unknown/inaccessible indistinguishability; event delivery additionally specifies a revocation barrier. Production authentication, store and runtime parity remain deferred as directed.

## 3. Risks and next action

A scope grant intentionally exposes all historical jobs and IDs in that scope, including on delegation; this is a stated policy, not a finding. Complete host privacy and crash conformance remain implementation gates.

The next action is one bounded correction covering the authorization fence and denial echo validation, followed by focused re-review of the two counterexamples on a new pinned renderer revision.
