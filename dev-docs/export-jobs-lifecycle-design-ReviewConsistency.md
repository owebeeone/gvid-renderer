# ExportJobs Lifecycle Design — CONSISTENCY-AXIS REVIEW

**Review object:** Draft `gvid-renderer/dev-docs/export-jobs-lifecycle-design.md` and companion design, schema, fixture, generated-binding and test diff. Renderer HEAD `c58006bb7edacb872b326cc2e2cc36df961a25a5`; GWZ root HEAD `23b4a543fcb9b945505a66ea6f79637f2cf186e6`. The design remains unaccepted.  
**Baseline:** Renderer diff from `d9de406e3292766c9d42b650cd0aa3ac729e014e`; root diff from `5a82b1f0c3def3626612aa36eae3f5a9901d7408`. Both HEADs matched the specified tuple at review start and end; both working trees were clean. Inspection was read-only.  
**Date:** 2026-10-02  
**Axis:** Consistency, independent adversarial read-only; parallel Safety report not consulted.  
**Verdict: NO-GO** — two P2 findings.

---

## 0. Evidence base

I read the new amendment, both amended renderer designs, renderer requirements, Taut schema, fixtures, generated Python/Rust/TypeScript bindings and contract tests. I checked the controlling root architecture, requirements, UI design and Taut feedback, plus the prior render-engine ReviewDecision and final round-3 reports. Root `AGENTS.md` and `AGENTS_GWZ.md` were read. `dev-docs/CurrentProgramCheckpoint.md`, `AgentProcessRules.md` and `GwzProcessOptimization.md` do not exist in the root repository.

The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` passed six tests. These exercise Python schema encoding, round trips and golden digests; they do not execute authorization, retirement or cross-language wire interoperability.

## 1. Findings

### [P2-1] The event subscription cannot carry the required caller provenance

**Root cause and location:** The amendment requires status, events and cancel to use the same provenance-specific authorization as lookup (`export-jobs-lifecycle-design.md:21`). Lookup, status and cancel have optional caller-incarnation and local-import fields (`ir/gvid_render_graph.taut.py:400–421`). `export.events` accepts only `project_id`, `job_id` and `after_event_sequence` (`ir/gvid_render_graph.taut.py:65–67`). The other resumable graph, catalog and binding feeds explicitly take an authority incarnation (`ir/gvid_render_graph.taut.py:15–17,34–36,46–48`).

**Violated invariant:** A subscription must identify the current governed incarnation or the authorized standalone import so the gateway can enforce the stated branch-specific access rule at the Taut boundary.

**Reproduction:** Submit job J under governed incarnation `open-1`, then reopen the project as `open-2`. A subscription made with the old context and one made with the new context have identical `export.events(project_id, job_id, after_event_sequence)` wire inputs. The receiving service cannot validate or reject the stale caller incarnation from that contract. The same omission prevents a standalone subscription from naming the local import whose journal access is to be checked.

**Impact:** The specified event authorization is not implementable or testable from the declared service request without an undocumented external binding. A gateway that relies on these fields can deliver job history to a stale or wrong-context subscriber.

**Required correction:** Add a discriminated caller context to `export.events`, or define an explicit, validated Taut envelope that carries equivalent branch and incarnation/import identity. Align the amendment and generated service bindings.

**Closure test:** Encode subscriptions for current governed, stale governed, correct standalone import and wrong standalone import contexts. Semantic service tests must admit only the authorized contexts and must not deliver events for rejected subscriptions.

### [P2-2] The pruned-job status outcome has no Taut representation

**Root cause and location:** The amendment says status for a pruned job is `unavailable` and forbids presenting a historical outcome from its tombstone (`export-jobs-lifecycle-design.md:25–27`). Yet `export.status` returns `ExportStatusSnapshot` directly (`ir/gvid_render_graph.taut.py:68–70`), and that message requires an originating context, job ID, lifecycle state and event sequence, with no availability status (`ir/gvid_render_graph.taut.py:404–408`). In contrast, `export.lookup` has an explicit result status and optional snapshot (`ir/gvid_render_graph.taut.py:413–417`). The root requirements require structured errors across component boundaries to use governed Taut contracts (`gvid-requirements.md:92`).

**Violated invariant:** A known job whose full record has been pruned must yield a typed unavailable outcome without manufacturing a lifecycle state or retained historical context.

**Reproduction:** A caller knows J’s job ID, J reaches a terminal state, and its full record is pruned into the permitted compact tombstone. The caller invokes `export.status` for J. The service cannot construct the declared `ExportStatusSnapshot` from the permitted tombstone, nor encode `unavailable` in that response type. Inventing `interrupted` or another state would falsely describe J; an unspecified transport error would leave this Taut contract incomplete.

**Impact:** Clients cannot implement the specified post-prune status behavior consistently, and the stated recovery proof cannot be satisfied on the declared wire.

**Required correction:** Define a typed status result or governed Taut error envelope with an unavailable branch and no snapshot, then specify its unknown, unauthorized, stale and pruned behavior.

**Closure test:** Prune a terminal job to the specified minimal tombstone. Round-trip and service-test the unavailable status response without a fabricated state or original job context; verify live-job status still returns its retained snapshot.

## 2. Invariant analysis

The two prior P2 counterexamples are addressed in the written admission and retirement sequence. A fresh standalone import has its own durable namespace and needs no Glade incarnation; a governed request needs a current one. Live replay checks the accepted-ID index before expired-reference revalidation, excludes the governed delivery incarnation from logical payload identity, and returns the originating context. After pruning, an accepted-ID tombstone blocks both exact and changed retries; its lifetime and non-reused namespace prevent an old request from replacing a later destination generation.

The superseding paragraphs in `render-engine-design.md:80–84` and `render-graph-schema-design.md:82–90` agree on those rules and limit full status/event availability to the retained-record period. The new enum values appear in all three generated binding sources and the new fixtures encode the intended submit and lookup retirement shapes. The six passing tests establish those wire shapes in Python, not the amendment’s host/CLI behavior, crash recovery or Rust/TypeScript execution parity. Those remain stated proof obligations rather than demonstrated conformance.

## 3. Risks and next action

**NO-GO.** Complete the event caller context and typed post-prune status response, add the closure tests above, and review the corrected tuple. No files or git state were modified.
