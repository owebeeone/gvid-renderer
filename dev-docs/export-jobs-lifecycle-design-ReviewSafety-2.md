# ExportJobs Lifecycle Design — SAFETY-AXIS RE-REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/export-jobs-lifecycle-design.md` and companion design, schema, bindings, fixtures, and tests at renderer `7445b3fb6ee9fb97ac0301105267870addcabbf8` and GWZ root `2868caeb9775fb79df43bbfebb5e08d7f67a1ae8`. The draft is not accepted.  
**Baseline:** Prior reviewed tuple: renderer `c58006bb7edacb872b326cc2e2cc36df961a25a5`, root `23b4a543fcb9b945505a66ea6f79637f2cf186e6`. Both HEADs matched the review tuple at start and end; both working trees were clean.  
**Date:** 2026-10-02  
**Axis:** Independent adversarial read-only Safety review; no current-round Consistency report consulted.  
**Verdict: NO-GO** — one P2 finding; no P0, P1, or P3 findings.

---

## Prior-finding closure table

| Prior finding | Re-review result |
| --- | --- |
| Safety P2-1 — nonterminal retirement | **Closed as a design rule.** `export-jobs-lifecycle-design.md:31` requires a durable terminal state and reconciliation of process, lease, artifact, resource, and publication ownership before full-record pruning. The companion designs and conformance obligation carry this rule. Runtime proof remains due. |
| Consistency P2-1 / Safety P2-2 — event caller provenance | **Closed as a contract defect.** `ExportEventsQuery` carries the current governed incarnation or local import ID (`ir/gvid_render_graph.taut.py:403–407`). The design requires branch authorization before replay, rechecks at each delivery point, queue clearing on revocation, and stream termination. Current, stale, wrong-branch, and revoked subscriptions are named conformance cases. Runtime barrier proof remains due. |
| Consistency P2-2 / Safety P2-3 — typed status unavailable | **Closed as a contract defect.** `export.status` returns `ExportStatusResult` (`ir/gvid_render_graph.taut.py:68–70,422–426`). The design requires a snapshot exactly for `found`, no snapshot for `unavailable` or `stale_context`, and serialization with record retirement (`export-jobs-lifecycle-design.md:23`). Runtime race proof remains due. |
| Safety P2-4 — cancel existence oracle | **Closed as a contract defect.** Unknown and inaccessible targets both return `unavailable`; former enum codes 3 and 4 are reserved (`ir/gvid_render_graph.taut.py:123–128`). The design checks caller context independently of job existence and permits `already_terminal` only after job read authorization (`export-jobs-lifecycle-design.md:27`). Runtime non-disclosure proof remains due. |

## Changed-range analysis

The renderer change adds the first-round reports and remediation plan, revises three design documents and the Taut schema, regenerates Python/Rust/TypeScript bindings, and adds golden fixtures and contract checks. The GWZ root change contains workspace lock and marker updates, with no product-design change. The new event and status wire shapes are expressible in the generated bindings. The six passing Python tests check schema and golden-wire behavior; they do not exercise service authorization, quota enforcement, crash recovery, or cross-language execution.

The revised text closes the prior queued-event, status/prune, cancel-probing, nonterminal-retirement, accepted-ID conversion, wrong-branch, and destination-owner counterexamples as stated rules. The remaining finding concerns a different failure path introduced by the interaction between permanent accepted-ID retention, nonterminal record retention, and a finite configured journal quota.

## 0. Evidence base

I inspected root `AGENTS.md` and `AGENTS_GWZ.md`; the controlling amendment, prior reports and remediation plan, renderer requirements, render-engine and graph-schema designs, render-engine ReviewDecision, ExportJobs Taut declarations, changed bindings, fixtures and tests, and the named root requirements, architecture, UI design, and Taut feedback. `dev-docs/CurrentProgramCheckpoint.md`, `AgentProcessRules.md`, and `GwzProcessOptimization.md` are absent from the GWZ root.

The permitted command `python -B -m unittest discover -s tests -p 'test_*.py'` passed: six tests. Inspection made no filesystem or git changes.

## 1. Findings

### [P2-1] NEW ARCHITECTURAL — admission does not protect durable space for an accepted job’s failure and cleanup transition

**Root cause and location:** `export-jobs-lifecycle-design.md:19,31–33` requires an accepted job to retain its full record until a durable terminal state and ownership reconciliation, and says a configured quota rejects *new* admissions before acceptance. It does not require admission to reserve or otherwise guarantee durable capacity for the accepted job’s subsequent state, diagnostic, process/artifact reconciliation, publication, and tombstone writes. `render-engine-design.md:94` requires a capacity diagnostic when the supervisor terminates an attempt that exceeds its resource grant, but the durable journal’s capacity failure path is unspecified.

**Violated invariant:** Once accepted, a job must be able to record a recoverable terminal or interrupted outcome and reconcile its owned resources even when its configured storage quota is reached.

**Credible sequence:** Admit a job when the durable store is below its quota. While it is running, retained progress or attempt records consume the remaining allowance. The supervisor then must terminate an over-budget process or handle a crash. Its terminal-state, diagnostic, and cleanup-intent writes are denied by the same quota. The job cannot retire because it is not durably terminal and reconciled; rejecting later admissions frees no space. After restart, the retained running record identifies ownership, but reconciliation still cannot commit its outcome or release the destination obligation safely.

**Impact:** An accepted export can remain stuck without a durable, visible failure outcome. Its destination ownership and temporary-resource obligations can remain unresolved indefinitely, blocking later exports and weakening the stated crash-recovery guarantee.

**Required correction:** Specify a bounded journal-growth and quota policy that preserves enough durable write capacity for every admitted job’s failure, terminal, and ownership-reconciliation path, including crash recovery. A reserved emergency record area or an equivalent fail-closed mechanism is acceptable. State how retained event history is bounded or expired without losing the accepted-ID invariant.

**Closure test:** Accept a job near the configured journal quota, exhaust its remaining ordinary allowance during execution, then force a capacity failure and crash/restart. Verify that the supervisor durably records an actionable outcome, reconciles process/artifact/destination ownership, preserves the accepted request ID, and rejects only new admissions with `capacity`; it must not report false success or silently evict an accepted ID.

## 2. Invariant analysis

The event revocation rule is expressible: authorization is checked before replay and at each delivery point, queued events are cleared on revocation or retirement, and a negative envelope terminates the stream. The guarantee is about the sender’s delivery linearization; bytes already sent before revocation cannot be recalled. The specified queued-event conformance case is therefore necessary runtime proof.

Status and retirement are explicitly serialized, so a query sees a retained snapshot or `unavailable`. Tombstone conversion is specified as atomic and durable, with crash recovery yielding a live record or tombstone and no absent-ID interval. An authorized old-ID retry cannot acquire a destination or launch a second job. Current caller and branch checks precede job lookup; the merged cancel outcome removes the prior enum-based existence probe.

These are design obligations, not demonstrated host or CLI behavior. The quota finding identifies a missing failure rule in those obligations; the passing wire tests cannot close it.

## 3. Risks and next action

**NO-GO.** Add the accepted-job journal-capacity rule and its quota/crash conformance case, then seek a focused Safety re-verdict on a new exact tuple. The lane owner should apply the review loop’s architectural-root-cause cap when merging this finding with the independent current-round report.
