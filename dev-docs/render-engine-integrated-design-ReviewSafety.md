# GVid Integrated Renderer Design — SAFETY-AXIS REVIEW

**Review object:** Draft `dev-docs/render-engine-integrated-design.md` and its stated package at renderer `1800cd317f61afb0a1c74b9a0c4a67ea8c843710`.  
**Baseline:** Renderer HEAD matched that SHA at the start and end of review; `git status --short` was clean. Root product documents were read with `git show` at `4f75d5814f6033282e09f985e86dfca51d84e548`.  
**Date:** 2026-10-02  
**Axis:** Safety. Independent, adversarial, read-only. Nothing here relies on the parallel reviewer’s current report.

**Verdict: GO** — no P0–P3 findings. This is a design-contract verdict, not runtime acceptance.

---

## 0. Evidence base

I read the integrated draft; relevant requirements, engine, graph-schema, ExportJobs lifecycle and owner-scope sections; the Taut declarations for preview resources and ExportJobs; the graph and ExportJobs reference validators; and the historical engine, lifecycle and owner-scope decisions and Safety reports. I checked the named root architecture, UI, Taut-feedback and composite-preview ADR content at the pinned root commit.

`git diff --stat 8bd582b..1800cd3` showed only the new integrated draft. The permitted Python contract suite passed: **8 tests**. It checks draft wire fixtures and selected semantic validation, not host authorization, durable stores, media execution or cross-language runtime parity. I made no file or Git changes.

## 2. Invariant analysis

- **Offline CLI and accepted-ID retirement:** The CLI uses a durable standalone-import namespace without a fabricated Glade incarnation (`export-jobs-lifecycle-design.md:11–13`). Admission, replay and lookup use that branch; terminal pruning leaves a permanent scoped tombstone (`:35–43`). Replaying an old ID cannot become a new export after its full job record is pruned.
- **Private IDs, revocation and job-ID operations:** The owner scope is checked before accepted-ID lookup, and its submit grant includes the whole scope’s accepted history (`export-jobs-owner-scope-design.md:9–27`). The durable fence also covers job-ID status/cancel classification, cancel commit and positive handoff (`:17,37`). Revocation before those points blocks disclosure or action; negative responses have no job payload.
- **Publication and crash recovery:** One canonical destination owner is held through reconciliation. Verified-candidate intent precedes replacement; backup restoration is limited to the generation still owned by the interrupted transaction (`render-engine-design.md:102–104`). Admission and publication reserves preserve recovery records despite ordinary journal exhaustion (`export-jobs-lifecycle-design.md:39–43`).
- **Preview, source access and resource pressure:** A late seek follows contributing dependencies and declared handles; stale results require matching request, graph, binding and lease context (`render-engine-design.md:34–38`; `render-graph-schema-design.md:72–76`). Glade-only sources require authorized access or bounded materialization, with lease and fingerprint failure explicit (`render-engine-design.md:64–74`). Export resource use requires enforceable limits or bounded preemption where interactive service is promised (`:96`).
- **Provider and cache isolation:** Hardware fallback creates a new attempt and recalculates affected keys; failed partial artifacts cannot seed it (`render-engine-design.md:34`). Consuming cache identity includes binding, source, profile, fidelity, provider and format semantics, and reuse requires artifact validation (`:94`).

These traces did not reveal a contract-permitted path to a false current preview, inaccessible-ID disclosure, duplicate accepted-ID execution, or rollback over a newer committed destination generation.

## 3. Risks and next action

Host fencing, crash recovery, source lease handling, resource enforcement, decoded-pixel parity and Rust/TypeScript runtime parity remain implementation proof obligations explicitly identified by the draft. The next action is to merge this verdict with the independent Consistency verdict for the same renderer SHA.
