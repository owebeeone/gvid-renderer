# GVid Renderer Runtime Verification Design — Remediation Plan 2

**Status:** Second and final bounded correction under the review-loop cap. [Safety re-review](render-runtime-verification-design-ReviewSafety-2.md) on renderer `f4860c74fb7ee58ea34f5451a7d6a9bcc8ebe4e1` kept two narrowed P2 findings open; [Consistency re-review](render-runtime-verification-design-ReviewConsistency-2.md) reported GO on that tuple. No new architectural root cause was reported.

| Open finding | Disposition in the single corrected document | Closure test |
| --- | --- | --- |
| Safety P2-1 | Split the mid-export source mutation into two variants: paired backing-byte/registry-fingerprint change and backing-byte-only change while registry version/fingerprint stay unchanged through the next consumed read or lease renewal. Require consumed content to validate against pinned identity, with no verification, publication or cache reuse of mixed bytes. | An implementation that trusts unchanged registry metadata while reading modified backing bytes fails the backing-only case for both leased and bounded staged access. |
| Safety P2-3 | Make unprovable canonical destination identity a pre-admission rejection, before the admission commit barrier. Require no accepted ID, job, attempt, tombstone, journal reserve or durable destination owner after retry/restart; a corrected destination under the same request ID is fresh. | An implementation that commits an accepted ID before returning destination-identity failure fails; the corrected same-ID request can enter admission. |

The design change is test-plan precision only. File both round-2 reports verbatim, commit this plan and the corrected design at one renderer checkpoint, update the GWZ lock with GWZ, then ask both axes to verdict the same new tuple. If any new architectural root cause appears, stop for redesign instead of drafting another remediation.
