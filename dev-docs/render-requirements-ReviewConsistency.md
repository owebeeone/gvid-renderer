# Render Requirements — CONSISTENCY-AXIS REVIEW

**Review object:** `D:/projects/gvid-wz/gvid-renderer/dev-docs/render-requirements.md`, commit `8a1e5a8f6b79ed1ce35eb5da04890d6d42e7388d`, initial draft, reviewed 2026-10-01.

**Baseline:** Renderer `8a1e5a8f6b79ed1ce35eb5da04890d6d42e7388d`; root `ac674e831d18234e23a2a54689256dc6d20c9016`; Taut `3c6d07ecb4cdb2496dd1a7524e270f8cc11a7640`. Pinned commits and specified blobs verified at both start and end. Content read through `git show COMMIT:path`, with numbered output for citations.

**Date:** 2026-10-01

**Axis:** Consistency, independent, adversarial, read-only. Other axis in parallel; nothing here relies on it. Filed verbatim by lane owner.

**Verdict: GO** — 0 P0, 0 P1, 0 P2, 0 P3 findings. This verdict accepts the requirements draft’s consistency, not implementation readiness or the downstream graph schema.

---

## 0. Evidence base

All three pinned commits passed `git cat-file -e` with exit code 0 at the beginning and end. Both verification passes returned:

| Pinned document | Verified blob |
| --- | --- |
| Renderer `dev-docs/render-requirements.md` | `05c002d8cc7dcfca4c86b1c9cf48b58a2cf99091` |
| Root `dev-docs/gvid-requirements.md` | `aefbf035948a8ed43980ec42785ca7c0bd3cdf3e` |
| Root `dev-docs/gvid-arch.md` | `ff51b7cce50ff7b8618e780d1aaad28c6679497c` |

Inspected:

- Renderer requirements, lines 1–164, including every requirement, acceptance scenario, and deferral.
- Root requirements, particularly §§5–7, §8 capability baseline, and §9 acceptance model.
- Root architecture, particularly AD-02–09; §§7–9 ownership/contracts/time; §§10–15 flows, persistence, fidelity, scheduling, and failure; and §§16–20 downstream design ownership and invariants.
- Pinned root `AGENTS.md` and `AGENTS_GWZ.md`, and the specified local review-loop skill.
- Pinned Taut `docs/Overview.md`, `docs/CodecContract.md`, and `docs/Server.md`.
- Relevant numbered passages from the renderer’s downstream `render-graph-schema-design.md`, solely to test coherence around durable command acceptance, semantic identity, cache validation, and protocol recovery.

No files were changed. No builds, tests, network requests, Git mutations, or other-axis report reads were performed.

## 2. Invariant analysis

### Canonical state, graph identity, and exact time

Renderer lines 12, 35–44, and 96–103 preserve the controlling distinction between canonical composition and derived execution plans. They agree with root `GVR-CON-004`, `GVR-REND-004`, `GVR-NFR-MNT-002`, architecture AD-03/05, and architecture §8.1.

I attacked equivalence under different edit order, graph round trips, machine relocation, and time conversion. `GVR-RDR-025`, `027`, `028`, `030`, and `032` require explicit exact time, deterministic semantic normalization, separate asset bindings, preservation of supported render meaning, and derived execution state. They do not require byte-identical lossy output, which would contradict root `GVR-NFR-COR-002`.

The final field layout remains legitimately deferred. The requirements already constrain identity, typed operations, ranges, bindings, request/result context, and compatibility behavior.

### Mutation acceptance and recovery

Renderer lines 60–66 require typed commands, revision preconditions, atomic acceptance, idempotent duplicate delivery, undo/redo, and conservative invalidation. These agree with root `GVR-EDIT-009–011` and architecture §§7 and 10.2.

I considered acknowledgement followed immediately by a host crash, and retry after reconnect. The renderer summary does not restate the complete persistence protocol, but it assigns acceptance to the project authority rather than replacing that authority. Root `GVR-PROJ-003`, `GVR-NFR-REL-002`, and architecture §§10.2/11.1 continue to control durability. The informative downstream graph draft, line 59, explicitly records the accepted batch and revision durably before publishing `AcceptedChange` and `EditAck`. I found no actual permission to weaken the root durability guarantee.

Scenario 12 establishes normal atomicity, duplicate/stale-command handling, undo/redo, and invalidation. Crash recovery and deterministic history replay still require evidence under the root acceptance model; the draft accurately labels its scenarios “initial” and makes no claim that implementation acceptance is complete.

### On-demand preview and stale results

Renderer lines 71–80 agree with root `GVR-PREV-001–010` and architecture AD-07 and §§10.2/12.

The attempted counterexample was a late result arriving after a seek, edit, asset relink, or sequence change. `GVR-RDR-041` and `044` require context matching and prohibit overwriting the current frame. `GVR-RDR-047` permits reuse across revisions only when semantic inputs still match and the displayed result is associated with the current request context.

Scenarios 13–15 can demonstrate direct late-timeline seeking, supersession handling, and local/full-render parity. Fidelity labels and explicit fallback prevent an approximate preview from silently becoming an exact result or final export.

### Asset rebinding, batching, and repeatability

Renderer lines 40–44, 86–91, and 96–127 preserve stable asset identity, explicit substitution, immutable per-item export context, master-media resolution, verified publication, and export provenance. These agree with root `GVR-ASSET-003–005`, `GVR-REND-001–014`, and architecture §19 invariants 7 and 11.

I considered changing bindings during export, retrying after an interruption, and reusing a derivative after a profile or toolchain change. `GVR-RDR-050`, `053`, and `055` preserve original revision/binding identity and require a new binding set and export record for rebinding.

`GVR-RDR-016` recommends comprehensive cache keys; that wording does not remove the controlling requirement that every artifact identify all inputs needed for safe reuse (`GVR-NFR-MNT-005`, architecture §§4/8.3). The downstream draft also requires a full semantic-key match before reuse. I found no concrete conflicting cache policy.

Scenarios 1, 4–8, 10–11, and 16 exercise the principal export, relink, retry, and verification invariants. Publication probing alone cannot establish visual correctness; the separate parity and semantic comparison requirements remain necessary.

### Taut capabilities and HTTP transport

Renderer lines 14 and 22–29 agree with root `GVR-CON-007–009`, `GVR-REND-015–016`, and architecture AD-02. The HTTP exception covers encoded media bytes; it does not introduce a second control API.

Pinned Taut documentation distinguishes delivery-shape catalogue membership from implemented runtime capabilities (`Overview.md`, lines 54–55). Its codec contract documents exact `i64`, bounded decoding, root-level bounds, and target-dependent unknown-field preservation (`CodecContract.md`, §§1, 5–6).

The renderer requirements demand explicit method semantics, compatibility rules, limits, pinned versions, and cross-language tests. They do not claim these application contracts already exist or that codec conformance establishes transport correctness. Scenarios 17–19 are suitable starting evidence for those separate obligations.

## 3. Risks and next action

The important remaining risks are implementation risks: acknowledgement durability under crashes, event recovery after disconnection, complete cache-input validation, native/browser compatibility, and measured preview/export fairness. Their controlling requirements remain intact. Numeric budgets, effect tolerances, substitution policies, and the saved graph format remain declared decisions rather than hidden defaults.

**Next action:** Merge this consistency verdict with the independent safety verdict for the same pinned tuple; retain the root acceptance obligations when assigning downstream design and implementation evidence.