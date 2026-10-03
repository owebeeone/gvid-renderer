# GVid Renderer Runtime Verification Design — Remediation Plan

**Status:** Round 1 response to the independent [Consistency](render-runtime-verification-design-ReviewConsistency.md) and [Safety](render-runtime-verification-design-ReviewSafety.md) NO-GO reports on renderer `bb2c78e2179a6ee06144e70d3f1c9060c117b1b6` and GWZ root `270b0ada3f21e2d5b8a76c4a31330127ff285bd4`. This is one bounded design-document correction; no runtime implementation or test result is claimed.

The two blind reviewers independently found the invalid-payload owner-scope denial gap (Consistency P2-5; Safety P2-2). All blocking findings are accepted and assigned to one revised document patch. The reviewers must retrace the original counterexamples on the corrected committed tuple before closure.

| Finding | Disposition in the design | Closure test required of the future harness |
| --- | --- | --- |
| Consistency P2-1 | Split EX-05 at the admission commit barrier; no accepted ID before commit, exactly one after. | Crash on each side, reopen and retry; pre-commit retry is fresh, post-commit retry replays. |
| Consistency P2-2 | Expand RG-01 with exact ripple split/shift, atomic slot and binding, unsupported placement/overlap, substitution policy and footprint oracles. | Mutants for append-instead-of-split, non-atomic slot, unsupported placement and omitted dependent interval each fail. |
| Consistency P2-3 | Add B-frame/fractional-rate and mixed sample-rate/channel-layout fixtures with A/V sync and encoder-delay measurement. | Wrong presentation frame and shifted audio window each fail. |
| Consistency P2-4 | Add a barrier after profile pinning; mutate/remove the named registry entry during Run and retry. | A mutable-name reread fails; original pinned definition or typed mismatch is required. |
| Consistency P2-5 | Cross inaccessible/retired/unknown scopes and live/retired/unused IDs with valid and invalid payloads; compare full denial envelopes and effects. | Payload validation before scope denial fails. |
| Consistency P2-6 | Add two fresh imports of identical graph bytes, journal loss and bridge use to the CLI scenario. | Graph-hash-derived or caller-recreated local import identity fails. |
| Consistency P2-7 | Add host-only crash leaving a native child and a reused-PID/changed-token case, alongside whole-tree kill. | Missing startup orphan reconciliation or killing an unrelated reused PID fails. |
| Safety P2-1 | Add a mid-export source-change barrier for leased access and bounded staging. | Mixed-source output under old fingerprint cannot verify, publish or enter cache. |
| Safety P2-2 | Same merged correction as Consistency P2-5. | Valid and invalid denied requests remain occupancy-independent with no index access or launched work. |
| Safety P2-3 | Add unprovable alias rejection and cross-process Glade/CLI proven-alias collision. | Ambiguous identity rejects before launch; proven active collision is destination-busy with zero launched work. |
| Safety P2-4 | Specify live replay after destination-reference expiry, and exact retired submit/lookup envelopes for exact and changed payloads. | Wrong live expiry result, fabricated retired job/snapshot, or execution after retirement fails. |
| Safety P2-5 | Restore storage and restart after fsync failure; assert durable reconciliation before release and later admission. | Permanent recovery-required ownership or premature release fails. |

**Round boundary:** Apply all corrections to the single draft. File both original reports unchanged. Commit the corrected design, this plan and the reports as a new renderer checkpoint; update the GWZ lock via GWZ. Send that exact tuple back to the same reviewers for focused re-verdicts. If the correction changes a shared interface or architecture instead of test design, begin a new numbered dual review rather than relying on focused closure. Stop after the skill's remediation cap if new architectural root causes demand redesign.
