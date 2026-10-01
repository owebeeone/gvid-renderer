# Render Engine Design — SAFETY-AXIS RE-REVIEW

**Review object:** Draft [render-engine-design.md](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md) and its single remediation patch, including the renderer schema, requirements status, Taut bindings, fixtures, tests, reports and plan; root architecture and composite-preview ADR. The design remains unaccepted.  
**Baseline:** Renderer `3e6e4883de5f43391e3df842dbd5a450dac635db`; gvid-wz root `567da536bf08a0d84b169c5408ff8dc019b0e87e`. I read clean working files at those HEADs and inspected the diffs from the prior tuple. Both HEADs matched at the start and end; both working trees were clean.  
**Date:** 2026-10-02  
**Axis:** Safety; independent, adversarial, read-only.  
**Verdict: NO-GO** — one P1 and two P2 findings. Bounded corrections could support GO after their counterexamples are re-tested.

---

## Prior-finding closure table

| Prior ID | Disposition claimed | Original counterexample re-traced on corrected tuple | Status |
| --- | --- | --- | --- |
| Consistency P2-1 | Composite-preview ADR and architecture revision | The architecture now assigns sequence composition to Preview/renderer and one leased resource to the presenter ([gvid-arch.md:64-66](D:/projects/gvid-wz/dev-docs/gvid-arch.md), [ADR:12-18](D:/projects/gvid-wz/dev-docs/adr-sequence-preview-composite.md)). | Closed for the design contract |
| Consistency P2-2 | Separate governed-host and standalone-import provenance | The core and ExportJobs contracts distinguish the variants without inventing a Glade incarnation ([design:25-27](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md), [schema design:82](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-graph-schema-design.md)). | Closed for the design contract |
| Consistency P2-3 | Binding-set consuming identity and checked cross-set reuse | The consuming key includes set ID/revision; reuse requires validation of contributing slots ([design:90](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). | Closed for the design contract |
| Consistency P2-4 | New planned attempt on hardware fallback | Failed-attempt artifacts are invalidated and dependent keys recalculated ([design:34,60](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). | Closed for the design contract |
| Consistency P2-5 | CLI journal and restart reconciliation | The CLI has a local durable journal and both supervisors reconcile processes and artifacts ([design:84,94-96](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). The original single-job crash sequence is addressed; concurrent publication exposes a separate defect below. | Closed for original sequence; new P1-1 |
| Safety P2-1 | Governed ExportJobs wire | Submit, event, status and cancel shapes now exist ([Taut:59-69,360-408](D:/projects/gvid-wz/gvid-renderer/ir/gvid_render_graph.taut.py)). An acknowledgement lost across reopen remains unrecoverable under a permitted interpretation of idempotency. | Open as P2-1 |
| Safety P2-2 | Pinned profile definition | ID, version, digest and effective settings remain pinned through preparation, cache, verification and publication ([design:27,90,96](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). | Closed for the design contract |
| Safety P2-3 | Durable job/artifact ownership and restart | Ownership precedes writing, PID reuse is checked, and publication has a journal/backup rule ([design:72,94-96](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). The original isolated crash points are addressed; same-destination jobs can invalidate each other’s recovery. | Closed for original sequence; new P1-1 |
| Safety P2-4 | Metering and hard grants | Metering and quotas were added, but the stated fallback when a hard resource limit is unavailable still permits one underestimated export to consume the preview reservation ([design:92](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). | Open as P2-2 |

## Changed-range analysis

The renderer correction changes [render-engine-design.md:3,21,25-34,60,70-72,80-96,103-104](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md), [render-graph-schema-design.md:15,29,80-104](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-graph-schema-design.md), the ExportJobs definitions at [gvid_render_graph.taut.py:59-69,108-117,360-408](D:/projects/gvid-wz/gvid-renderer/ir/gvid_render_graph.taut.py), generated bindings, three fixtures and contract tests. The root changes architecture preview ownership and adds the [composite-preview ADR](D:/projects/gvid-wz/dev-docs/adr-sequence-preview-composite.md); root `gwz.conf` lock and marker changes are workspace bookkeeping outside the substantive review object. These changes follow the remediation plan’s scope.

**NEW ARCHITECTURAL root causes:** Publication lacks per-destination transaction isolation (P1-1), and ExportJobs does not define recovery of an accepted request across authority-incarnation change when its acknowledgement is lost (P2-1). P2-2 is the still-open prior resource-enforcement root cause.

## 0. Evidence base

I compared the revised design and schema with the renderer requirements and the root architecture, requirements and UI design. I read both prior reviews and the merged remediation plan. No `CurrentProgramCheckpoint.md`, `AgentProcessRules.md` or `GwzProcessOptimization.md` was found under root `dev-docs`, including an unignored search. The permitted `python -B -m unittest discover -s tests -p 'test_*.py'` command passed: six tests. The new ExportJobs test at [test_taut_contract.py:261-338](D:/projects/gvid-wz/gvid-renderer/tests/test_taut_contract.py) proves message round trips; it does not exercise host authorization, idempotency across reopen, publication races, or resource enforcement.

## 1. Findings

### [P1-1] Concurrent exports can roll back a separately successful publication

**Root cause and location:** The publication journal specifies candidate verification, backup, atomic replacement and recovery, but no exclusive destination ownership or destination-generation check across jobs ([render-engine-design.md:80,94-96](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). Multiple batch requests and independent results are required ([render-requirements.md:103-107](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-requirements.md)).

**Violated invariant:** A failed or interrupted job must not remove another job’s successfully published output or leave its success record pointing at different destination bytes.

**Reproduction:** Jobs A and B target the same existing destination D. Both verify candidates and record publication intents; both retain D’s original bytes as their rollback copy before either replacement. A replaces D with A’s output, commits success and deletes its backup. B replaces D with B’s output, then crashes before committing its export record. On restart, B’s specified recovery restores its saved “prior destination,” the original D. A remains succeeded, but its output has vanished. The text permits this ordering and supplies no destination lock or generation test that would stop it.

**Impact:** Loss of a completed export and a false success record. The race crosses the journal and filesystem transaction boundary.

**Required correction:** Serialize publication per canonical destination, or use a durable compare-and-swap generation protocol. Give each job a uniquely owned backup; recovery may restore or complete only while its transaction still owns the destination generation. Define how a second job waits, conflicts or replaces a first committed result.

**Closure test:** Run two verified jobs against one destination. Crash each at intent, backup, replacement and record-commit boundaries in both orderings. After reconciliation, every succeeded record must match the destination generation it committed, and a failed job must not erase a separately succeeded output.

### [P2-1] Lost submit acknowledgement cannot be reliably recovered after project reopen

**Root cause and location:** Idempotency is keyed by project and request ID with a “canonical logical payload,” but the rule does not say whether the request’s authority incarnation is excluded. The wire embeds that incarnation in `ExportJobContext`; status and events require a job ID, with no request-ID lookup ([render-engine-design.md:80-82](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md), [render-graph-schema-design.md:82-88](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-graph-schema-design.md), [gvid_render_graph.taut.py:62-69,360-375,392-408](D:/projects/gvid-wz/gvid-renderer/ir/gvid_render_graph.taut.py)). The neighboring graph-edit contract explicitly excludes incarnation from logical retry identity ([render-graph-schema-design.md:54](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-graph-schema-design.md)); ExportJobs does not.

**Violated invariant:** Once submit is durably accepted, an authorized caller must be able to recover its stable job ID and terminal outcome after a lost acknowledgement and reopen, without starting a duplicate job.

**Reproduction:** Under incarnation `open-1`, submit request X; the gateway durably accepts job J, but its acknowledgement is lost. Reopen at `open-2`. The client does not know J, so cannot use `export.status` or `export.events`. Retrying the original request carries stale `open-1` and may be rejected as stale. Updating the request to `open-2` changes a field in the submitted payload; a canonical whole-request comparison may reject it as changed. Both behaviors satisfy the current words, leaving J undiscoverable to its requester. The round-trip fixture tests retries only with the unchanged original context ([test_taut_contract.py:287-295](D:/projects/gvid-wz/gvid-renderer/tests/test_taut_contract.py)).

**Impact:** A live or completed export can become an orphan from the client’s perspective; the user cannot safely determine whether to retry or cancel it.

**Required correction:** Define immutable idempotency fields explicitly, excluding delivery incarnation while preserving the accepted job’s original context; authorize a new-incarnation retry and return J without re-executing. Alternatively, provide an authorized status lookup by project/request ID. Specify replay ordering relative to profile and destination-reference expiry so an already accepted job remains discoverable.

**Closure test:** Drop the first accepted acknowledgement, reopen the project, and recover J from the new incarnation. Confirm a changed substantive payload conflicts, an old incarnation cannot start new work, and J’s historical result never becomes the new session’s current result.

### [P2-2] The no-hard-limit fallback does not protect interactive capacity

**Root cause and location:** The scheduler must enforce resource grants, yet where a hard limit is unavailable the design permits merely reducing export concurrency or rejecting a competing export ([render-engine-design.md:92](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). That does not bound the single remaining native process.

**Violated invariant:** Batch work must not indefinitely block playback, seeks or exact frames, even when estimates are wrong ([render-requirements.md:107,128](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-requirements.md)).

**Reproduction:** On a host without an enforceable per-process I/O cap, admit one export whose estimate understates sustained source and spool I/O. Reducing concurrency to one satisfies the stated fallback. While it saturates the device, request an exact frame. The supervisor can observe overuse, but the text does not require bounded preemption, suspension or rejection of that single export when preview’s reservation cannot be enforced. The preview may wait indefinitely.

**Impact:** The prior Safety P2-4 starvation case remains legal on a degraded host.

**Required correction:** For every admitted resource class, require an enforceable limit or a bounded preemption mechanism that protects interactive work. If neither exists, reject or defer export admission while interactive service must be available; define the capacity transition rather than treating concurrency reduction alone as protection.

**Closure test:** On a host configured to lack one hard resource limit, run one deliberately underestimated export, then request a late exact frame. Prove bounded preview progress and an explicit export capacity/defer outcome; repeat for memory, disk and I/O limits.

## 2. Invariant analysis

The corrected contract addresses the original single-job provenance, profile, source-lease, PID-reuse and crash paths in text. Source staging records ownership before writing and rejects interrupted or changed-source reads ([design:70-72](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). Cache reuse retains consuming binding identity and validates contributing slots; provider fallback creates a new attempt and keys ([design:34,90](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)). The gateway uses opaque destination references and protects credentials and raw media from portable records ([design:56,68,80,96](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md)).

The remaining safety failures arise at boundaries among otherwise valid jobs: acknowledgement loss across incarnations, simultaneous publication to one destination, and a single native process exceeding a reservation where hard enforcement is unavailable. The current Taut round-trip tests cannot detect those state sequences; the design proof points at [render-engine-design.md:103-104](D:/projects/gvid-wz/gvid-renderer/dev-docs/render-engine-design.md) should add them.

## 3. Risks and next action

Resolve P1-1 and P2-1 as explicit durable transaction and idempotency rules, and close P2-2’s degraded-host admission path. Re-run the three state sequences against the corrected contract and its proof points before a Safety GO. No files were modified during this review.
