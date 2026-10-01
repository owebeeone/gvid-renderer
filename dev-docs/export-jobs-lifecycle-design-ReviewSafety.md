# ExportJobs Lifecycle Design — SAFETY-AXIS REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/export-jobs-lifecycle-design.md` and companion design, schema, bindings, fixtures, and tests at renderer `c58006bb7edacb872b326cc2e2cc36df961a25a5` and GWZ root `23b4a543fcb9b945505a66ea6f79637f2cf186e6`. The draft is not accepted.  
**Baseline:** Renderer diff since `d9de406e3292766c9d42b650cd0aa3ac729e014e`; root diff since `5a82b1f0c3def3626612aa36eae3f5a9901d7408`. Both HEADs matched the review tuple at start and end; both working trees were clean.  
**Date:** 2026-10-02  
**Axis:** Safety, independent adversarial read-only; parallel Consistency report not consulted.  
**Verdict: NO-GO** — four P2 findings; no P0, P1, or P3 findings.

---

## 0. Evidence base

I read root `AGENTS.md` and `AGENTS_GWZ.md`; the controlling amendment, renderer requirements, render engine and graph schema designs, prior render-engine ReviewDecision and final round-3 reports; the ExportJobs Taut declarations, companion diff, fixtures and tests; and the named root architecture, requirements, UI design and Taut feedback. `CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, and `GwzProcessOptimization.md` were absent from root `dev-docs`.

The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` passed six tests. These establish schema and golden-wire behavior, not runtime admission, authorization, pruning, or publication.

## 1. Findings

### [P2-1] A nonterminal job can lose its recovery record during retirement

**Root cause and location:** `export-jobs-lifecycle-design.md:25,29` permits pruning a full job record once publication reconciliation completes, but does not require the job to be durably terminal or its running attempt, process, leases, and artifacts to be resolved. A queued or running job can have no publication intent to reconcile. The companion design requires those ownership details in the durable job record (`render-engine-design.md:96`).

**Violated invariant:** An accepted job must remain recoverable until execution and publication have a durable terminal outcome.

**Credible sequence:** A long export is running when an age-based retention or compaction pass reaches its record. No publication has begun, so there is no unresolved publication intent. The pass converts the accepted-ID entry to a tombstone and deletes the full record. After a host crash, the supervisor lacks the recorded process and artifact ownership needed for safe cleanup or verified resume. Lookup can report only `retired_request`, and status is unavailable.

**Impact:** Accepted work can become unobservable and unrecoverable; an eventual valid output may be lost or left orphaned.

**Required correction:** Make durable terminal state and completed attempt, resource, artifact, and destination reconciliation explicit preconditions for full-record retirement. Keep the record while any of those obligations remains active.

**Closure test:** Run a job past the configured pruning threshold in queued, running, and verifying states. Invoke compaction and crash/restart. Confirm its full record remains until a verified terminal outcome and ownership cleanup, then confirm retirement preserves the accepted ID.

### [P2-2] The event subscription has no defined caller-provenance context

**Root cause and location:** The amendment says status, events, and cancel use the same provenance-specific authorization (`export-jobs-lifecycle-design.md:21`), but `export.events` accepts only `project_id`, `job_id`, and `after_event_sequence` (`ir/gvid_render_graph.taut.py:65-67`). Unlike lookup, status, and cancel, it supplies neither a current governed incarnation nor a local import ID. No transport binding that supplies those values is specified.

**Violated invariant:** A subscriber must prove current authority in the job’s provenance namespace before receiving retained or live events.

**Credible sequence:** A governed project reopens. An old client that still has ordinary project authentication subscribes using a known job ID. The event request contains no incarnation to reject as stale. Similarly, a machine-protocol caller can name a standalone job without presenting its local import context. An implementation following the declared request shape can check project/job visibility but cannot perform the stated caller-context comparison.

**Impact:** The contract permits delivery to a stale or wrong-branch subscriber, including event context, diagnostics, and eventual result identifiers.

**Required correction:** Add a discriminated caller context to event subscription, or explicitly define an authenticated transport binding that supplies the current incarnation or authorized import and is checked on subscription and during continued delivery after revocation.

**Closure test:** Attempt subscriptions with the correct branch, wrong branch, stale governed incarnation, inaccessible import, and revoked access during an open stream. Only the currently authorized subscription may receive events.

### [P2-3] `export.status` cannot represent its specified unavailable states

**Root cause and location:** A pruned job’s status is specified as unavailable (`export-jobs-lifecycle-design.md:27`), while `export.status` returns `ExportStatusSnapshot` directly (`ir/gvid_render_graph.taut.py:68-70`). That message requires a historical context, job ID, state, and sequence (`:404-408`); it has no unavailable, stale-context, or unauthorized outcome. No transport-error mapping is defined.

**Violated invariant:** A failed status lookup must not fabricate a job snapshot or imply a historical outcome.

**Credible sequence:** A client knows job J, its record is pruned, and the client calls `export.status` after a feed gap. The service cannot emit the prescribed unavailable response in the declared wire type. The same problem applies to an unknown job or a caller whose authority has been revoked.

**Impact:** Implementations must invent incompatible error behavior or return a false snapshot, breaking recovery and potentially disclosing job information.

**Required correction:** Define a status-result envelope with a snapshot only for `found`, or specify exact typed transport errors for unavailable, stale, and unauthorized cases. Preserve non-disclosure for unknown versus unauthorized jobs.

**Closure test:** Exercise status for a retained job, pruned job, unknown ID, stale caller, and revoked caller through each generated binding. Negative cases must carry no fabricated context, state, or result.

### [P2-4] Cancel’s distinct unknown and unauthorized statuses expose job existence

**Root cause and location:** `ExportCancelStatus` distinguishes `unknown` from `unauthorized` (`ir/gvid_render_graph.taut.py:123-124`), and the companion design expressly requires distinct unknown and unauthorized targets (`render-graph-schema-design.md:92`). Lookup instead merges unknown and unauthorized into `unavailable` (`:88`).

**Violated invariant:** A caller without a job’s read authority must not learn whether that job exists by choosing another ExportJobs method.

**Credible sequence:** A caller has a current project context but lacks read authority for another user’s job. Canceling a candidate real job ID returns `unauthorized`; canceling a nonexistent ID returns `unknown`. Repeating the probe enumerates accepted jobs without obtaining their details.

**Impact:** Job existence and activity can be disclosed despite lookup’s privacy rule.

**Required correction:** Give callers without job read authority the same cancel response for unknown and inaccessible jobs. Reserve any more specific already-terminal response for a caller authorized to see that job.

**Closure test:** From a caller lacking another job’s read authority, compare cancel responses for an existing job and a nonexistent ID; they must be indistinguishable and must not alter either job.

## 2. Invariant analysis

The two prior P2 counterexamples are closed **as written design rules**. A fresh standalone import uses its durable local import namespace without a Glade incarnation (`export-jobs-lifecycle-design.md:9-13,19`). After an accepted job record is pruned, its ID remains a durable tombstone, so exact and changed retries return `retired_request` and cannot replace a later destination generation (`:25-29`). A lost acknowledgement can recover the full snapshot while the record exists and can establish prior acceptance after retirement.

The admission rule also requires atomic accepted-ID persistence before acknowledgement and directs concurrent claimants to reread the winner (`:19`). Quota exhaustion rejects new work before acceptance rather than evicting accepted IDs (`:27`). Namespace deletion relies on permanent nonreuse of project and import IDs. These are implementable obligations still requiring crash, concurrency, and restart proof; the six wire tests do not provide that proof.

## 3. Risks and next action

Resolve the four contract defects before acceptance, then rerun focused safety review at a new exact tuple. Runtime conformance remains separately due for authorization, durable transitions, and publication ownership. No files or git state were modified.
