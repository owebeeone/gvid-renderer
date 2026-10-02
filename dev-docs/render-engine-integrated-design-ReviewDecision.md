# GVid Integrated Renderer Design — Review Decision

**Status: GO for the integrated renderer draft contract at renderer `21ba5ef9a704ebeeb2857566a27700402944a1cc`, with GWZ root product documents pinned to `4f75d5814f6033282e09f985e86dfca51d84e548`. This accepts design coherence and implementation direction, not a running renderer.**

**Date:** 2026-10-02  
**Controlling draft:** [Integrated renderer design](render-engine-integrated-design.md) and its stated package.  
**Final independent reports:** [Consistency re-review](render-engine-integrated-design-ReviewConsistency-2.md) and [Safety re-review](render-engine-integrated-design-ReviewSafety-2.md), both GO on the same renderer revision.

## Verdict history

| Renderer checkpoint | Consistency | Safety | Lane result |
| --- | --- | --- | --- |
| `1800cd317f61afb0a1c74b9a0c4a67ea8c843710` | NO-GO: source preview was required to carry sequence graph/binding identity | GO: no P0–P3 finding | [One remediation plan](render-engine-integrated-design-RemPlan.md) |
| `21ba5ef9a704ebeeb2857566a27700402944a1cc` | GO: source and sequence preview identities are distinct, prior P2 closed | GO: no new Safety finding | **Accepted at design-contract level** |

## Accepted scope

The integrated object reconciles the previously reviewed engine design with the later ExportJobs lifecycle and owner-scope contracts. The original engine [NO-GO decision](render-engine-design-ReviewDecision.md) recorded missing standalone-CLI provenance and accepted-ID retirement; the lifecycle design now specifies both. The lifecycle [NO-GO decision](export-jobs-lifecycle-design-ReviewDecision.md) recorded a project-wide request-ID privacy oracle; the accepted [owner-scope contract](export-jobs-owner-scope-design-ReviewDecision.md) replaces that namespace and specifies the revocation fence. These earlier decisions remain auditable findings at their historical tuples. They are not current alternatives to this accepted integrated contract.

The design keeps the portable flat Taut graph and asset bindings separate from disposable typed workflows; places graph mutation with project authority; uses one headless render core with pluggable tool and resolver interfaces; separates Glade Taut/HTTP delivery from an offline-capable CLI; and plans requested preview ranges before bounded prefetch while admitting batch work fairly. Sequence preview freshness uses graph revision and binding identity. Independent source preview freshness uses registered asset version, stream and fingerprint without fabricated graph fields. ExportJobs uses provenance-specific durable IDs, scoped visibility, retained tombstones, journal reserve, destination publication ownership and revocation-ordered action/disclosure.

## Evidence and limits

The Python reference contract suite passed **8 tests** at both reviewed checkpoints, including the focused source/sequence wire-shape assertions. `git diff --check` passed on the corrected product/test range. The generated Python, Rust and TypeScript Taut bindings are present. The review judged written contracts and reference wire shapes; it did not execute a Glade gateway, CLI renderer, media toolchain, durable journal, provider parity suite or cross-language runtime integration.

Implementation acceptance still requires the proof matrix in the integrated design: authority and revocation races, concurrent ExportJobs admission and crash recovery, journal reserve and publication rollback, Glade-only asset access, source/sequence stale-result rejection, provider and cache parity, late-seek latency and scheduler fairness, and verified batch output. Those gates must be run against the actual implementation before release. No tag, merge or production release is implied by this decision.
