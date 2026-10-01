# ExportJobs Lifecycle Design — Safety Review 3

**Review object:** ExportJobs lifecycle DRAFT and companion renderer contract at renderer `8540f07fc7189a91b9235395681c742d378673ae`.  
**Baseline:** Final remediation diff from renderer `7445b3fb6ee9fb97ac0301105267870addcabbf8`; prior reviewed GWZ root `2868caeb9775fb79df43bbfebb5e08d7f67a1ae8`. Controlling root documents are pinned to commit `0f2f1320be97b007b3e4d5276cbd7df9357fdcc7`.  
**Date:** 2026-10-02  
**Axis:** Independent, peer-blind, read-only Safety review.  
**Verdict: NO-GO** — one P2 finding; no P0, P1, or P3 findings.

The revised tuple rule treats the root commit as an immutable document reference, not as a moving HEAD. The renderer HEAD matched the reviewed SHA at review start and end; the pinned root commit object existed at both checks.

## Prior-finding closure table

| Prior finding | Result |
| --- | --- |
| Safety P2-1 — nonterminal retirement | **Closed as a contract rule.** Full records remain until durable terminal state and reconciliation of process, lease, artifact, resource, and publication ownership (`export-jobs-lifecycle-design.md:31`). Runtime proof remains due. |
| Consistency P2-1 / Safety P2-2 — event caller provenance | **Closed at the wire and contract level.** `ExportEventsQuery` carries the current governed incarnation or local import ID; authorization precedes replay and is rechecked at delivery (`ir/gvid_render_graph.taut.py:403–407`; lifecycle draft `:25`). Runtime revocation-barrier proof remains due. |
| Consistency P2-2 / Safety P2-3 — typed status unavailable | **Closed at the wire and contract level.** `ExportStatusResult` represents `found`, `unavailable`, and `stale_context`; negative results have no snapshot (`ir/gvid_render_graph.taut.py:422–426`; lifecycle draft `:23`). |
| Safety P2-4 — cancel existence oracle | **Closed for cancel.** Unknown and inaccessible job IDs return `unavailable` without a terminal state (`export-jobs-lifecycle-design.md:27`; `ir/gvid_render_graph.taut.py:125–127`). The finding below concerns submission. |
| Round-2 Safety P2-1 — accepted-job journal capacity | **Closed as a contract rule.** Admission reserves a permanent accepted-ID slot and physically backed capacity for bounded failure, terminal, cleanup, and destination reconciliation writes. Ordinary history cannot consume that reserve; physical write failure retains recovery ownership (`export-jobs-lifecycle-design.md:35–37,45`). Runtime quota/crash proof remains due. |
| Round-2 Consistency P2-1 — negative diagnostic disclosure | **Closed as a contract rule.** Unavailable and stale lookup, status, and event responses have null diagnostic and payload fields. The reference validator rejects negative responses carrying either (`export-jobs-lifecycle-design.md:21–25,43`; `ir/validate_export_responses.py:14–38`). |

## Changed-range analysis

The final renderer range adds the round-2 reports and remediation plan, revises the lifecycle draft and two companion designs, and adds a semantic response validator and tests. It does not change Taut field layout. The root changes since the pinned commit are workspace bookkeeping and unrelated UI work; the named controlling root documents had no diff from the pinned commit at final verification.

The revised text closes both round-2 counterexamples as specified obligations. The finding below is an inherited, previously unfiled authority-partition defect in the whole lifecycle contract. It is **NEW ARCHITECTURAL** because the shared request-ID namespace and per-job read authority interact to create the disclosure.

## 0. Evidence base

I inspected the controlling lifecycle draft, both companion designs, Taut declarations, validator, fixtures, tests, prior filed ExportJobs reviews and remediation plans through round 2, renderer requirements, and the prior render-engine ReviewDecision. I checked root `AGENTS.md`, `AGENTS_GWZ.md`, and the named requirements, architecture, UI, and Taut-feedback documents against pinned root commit `0f2f1320be97b007b3e4d5276cbd7df9357fdcc7`. The named root checkpoint and process documents were absent.

The permitted Python suite passed: **7 tests**. It checks schema and response shapes, not host authorization, quota recovery, or publication behavior. Inspection made no filesystem or Git changes. Renderer HEAD was `8540f07fc7189a91b9235395681c742d378673ae` at start and end; the pinned root commit object was present at both checks.

## 1. Finding

### [P2-1] NEW ARCHITECTURAL — submit reveals an inaccessible accepted request ID

**Root cause and exact location:** The governed request-ID namespace is shared across a project (`export-jobs-lifecycle-design.md:9,13`). An existing live ID requires the caller to have that job’s read authority before replay or conflict handling (`:17`), and an inaccessible tombstone returns unauthorized/unavailable (`:33`). A *new* ID instead requires current project and destination authority, then validates inputs and can be accepted (`:19`). `ExportSubmitStatus` exposes distinct `accepted`, `invalid`, and `unauthorized` outcomes (`ir/gvid_render_graph.taut.py:112–115`). The draft does not require a project member who may create exports to have read authority for every other member’s job.

**Violated invariant:** A caller lacking a job’s read authority must not determine whether a candidate request ID was accepted in the shared project namespace.

**Credible sequence:** Two collaborators may create exports in one governed project. A submits request ID `R`; B retains project read and destination write authority but cannot read A’s job. B submits a valid export with `R`. The accepted-ID lookup finds A’s record, so B receives an unauthorized result. B submits the same valid export under an unallocated ID `R2`; the new-ID branch can return `accepted`. B can therefore test candidate IDs for private accepted jobs. The stated lookup, status, event, and cancel non-disclosure rules do not close this submit path. A well-formed request with an invalid pinned profile can also distinguish the branches without launching work if the specified index-before-profile-validation order is followed.

**Impact:** An authorized project exporter can enumerate or confirm other users’ accepted export request IDs, including retained tombstones, despite lacking job read authority. The result also varies with an ID’s existence after job-record retirement.

**Required correction:** Define a stable request-ID authority partition or an admission policy under which every caller allowed to create in a shared namespace has the same visibility to collisions in that namespace. Specify indistinguishable submit behavior for absent and inaccessible accepted IDs without permitting an inaccessible collision to launch or replace work. Carry the rule through live records, tombstones, and concurrent admission.

**Closure test:** Give B project export authority but deny B read access to A’s job. Compare B’s complete submit responses for A’s live ID, its retired tombstone ID, and an unallocated ID using equivalent valid payloads; B must learn no collision distinction, and no submission may launch work for an inaccessible accepted ID. Repeat with concurrent submit and after restart.

## 2. Invariant analysis

The final quota rule preserves capacity for an accepted job’s durable failure and cleanup path, while physical write failure retains ownership for recovery. Retirement preserves accepted IDs and does not prune jobs with unresolved obligations. Negative lookup, status, and event responses now suppress job data and diagnostics; cancel merges unknown and inaccessible targets.

Submission remains a separate observable operation. Its existing-ID authorization branch and new-ID admission branch have different outcomes for a caller who can create jobs but cannot read the owner’s job. No runtime implementation is required for this counterexample: the written contract permits it, and the Taut status enum represents it.

## 3. Risks and next action

**NO-GO.** The lane owner should classify P2-1 under the review loop’s final-round architectural-root-cause rule and seek the required operator redesign-or-accept decision. This review does not authorize another routine remediation patch.
