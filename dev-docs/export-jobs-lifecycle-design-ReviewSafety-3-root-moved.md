# ExportJobs Lifecycle Design — Safety Review Interrupted

**Review object:** ExportJobs lifecycle DRAFT and companion renderer contract at renderer `8540f07fc7189a91b9235395681c742d378673ae`, with requested GWZ root `0f2f1320be97b007b3e4d5276cbd7df9357fdcc7`.  
**Baseline:** Renderer `7445b3fb6ee9fb97ac0301105267870addcabbf8`; prior reviewed root `2868caeb9775fb79df43bbfebb5e08d7f67a1ae8`.  
**Date:** 2026-10-02  
**Axis:** Independent, peer-blind, read-only Safety review.  
**Disposition:** **No verdict.** The GWZ root HEAD changed during review.

## Prior-finding closure table

| Prior finding | Inspection before tuple change |
| --- | --- |
| Safety P2-1 — accepted-job journal capacity | The revised amendment specifies a durable accepted-ID slot, physically backed capacity for terminal and reconciliation writes, bounded record growth, and a quota/crash conformance case (`export-jobs-lifecycle-design.md:35–37,45`). Closure cannot receive a final verdict on the moved tuple. |
| Consistency P2-1 — negative-response diagnostic disclosure | The amendment requires null diagnostics on unavailable and stale lookup, status, and event responses (`:21–25,43`). The reference validator rejects negative responses carrying a diagnostic or job payload. Closure cannot receive a final verdict on the moved tuple. |

Earlier findings concerning nonterminal retirement, event provenance, typed status, and cancel disclosure remained addressed in the inspected renderer contract. Their runtime conformance is deferred.

## Changed-range analysis

The renderer range adds the round-2 reports and remediation plan, revises the lifecycle amendment and two companion designs, and adds a semantic response validator and tests. It does not change the Taut field layout. The root range includes workspace bookkeeping and an unrelated UI wireframe, outside this review’s scope.

## 0. Evidence base

At review start, root HEAD was `0f2f1320be97b007b3e4d5276cbd7df9357fdcc7` and renderer HEAD was `8540f07fc7189a91b9235395681c742d378673ae`; both working trees were clean. I inspected the process files, controlling documents, prior filed ExportJobs reports and plans, renderer designs, Taut declarations, validator, fixtures, and tests. The named root checkpoint and process documents were absent. The permitted Python suite passed: **7 tests**.

At final verification, renderer HEAD remained `8540f07fc7189a91b9235395681c742d378673ae`, but root HEAD was **`1210f1298ed7daa5defc79b2cee38e4e82fb169b`**. Review stopped at that discrepancy. Inspection made no filesystem or Git changes.

## 1. Findings

No findings are filed because the exact review tuple did not remain stable through final verification.

## 2. Invariant analysis

The inspected renderer text states fail-closed rules for journal exhaustion and physical write failure, preserves accepted IDs through retirement, and forbids job data in negative lookup, status, and event envelopes. These observations are **not a GO verdict** on a settled tuple.

## 3. Risks and next action

Settle and record the intended GWZ root HEAD, then run a Safety review with start and end verification against that exact root and renderer pair.
