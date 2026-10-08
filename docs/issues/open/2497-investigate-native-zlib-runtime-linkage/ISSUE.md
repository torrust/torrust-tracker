---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: null
github-issue: 2497
spec-path: docs/issues/open/2497-investigate-native-zlib-runtime-linkage/ISSUE.md
branch: "2497-investigate-native-zlib-runtime-linkage"
related-pr: null
last-updated-utc: "2026-10-08 18:00"
semantic-links:
  skill-links:
    - create-issue
    - catalog-security-vulnerabilities
  related-artifacts:
    - "issue #2493"
    - "issue #2499"
    - Containerfile
    - docs/security/analysis/production/CVE-2026-27171.md
    - docs/security/analysis/production/CVE-2026-85091.md
    - docs/security/analysis/github-security/README.md
---

# Issue #2497 - Investigate native zlib runtime linkage and CVE reachability

## Goal

Establish why the production tracker binary links native `libz.so.1`, determine whether the
functions affected by CVE-2026-27171 or CVE-2026-85091 are reachable, and either remove unnecessary
linkage or record reproducible evidence supporting the final security verdict.

## Background

GitHub Code Scanning alerts 24 and 59 report two zlib CVEs in the production image. The same
findings appear as alerts 12 and 60 after a push-triggered scan; only one pair is open at a time
until #2499 stabilizes Trivy alert identity. The earlier
CVE-2026-27171 analysis classified system zlib as an unused transitive runtime package. The current
`Containerfile`, however, discovers the release binary's linked `libz.so.1` with `ldd` and copies
it into the final image.

No source call to the affected APIs has been found:

- CVE-2026-27171 requires `crc32_combine()` or `crc32_combine64()`.
- CVE-2026-85091 requires a non-blocking `gzwrite` path followed by `gzprintf` or `gzvprintf`.

The Rust `flate2` dependency resolves through `zlib-rs`, not `libz-sys`. This makes exploitability
unconfirmed but invalidates the previous package-is-unused rationale.

## Scope

### In Scope

- Reproduce native zlib linkage in the final release binary and production image.
- Identify the crate, native library, build input, or linker behavior introducing the dependency.
- Inspect imported and reachable zlib symbols relevant to both CVEs.
- Determine whether attacker-controlled runtime input can reach an affected function.
- Remove native zlib linkage when unnecessary, or document the unreachable boundary with
  reproducible evidence.
- Refresh both CVE analyses and the GitHub Security findings catalog.
- Re-run the production image scan and reconcile alerts 24 and 59, or 12 and 60.

### Out of Scope

- General replacement of compression libraries when unrelated to native linkage.
- Fixing unrelated glibc, GCC runtime, CI, or Code Quality findings.
- Treating package presence or scanner severity alone as proof of exploitability.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None expected unless removing zlib requires a long-lived change to the
  production linking or compression architecture.

## Design and Ownership Review

The investigation must distinguish:

- Rust dependency ownership;
- native linker ownership;
- production image assembly;
- runtime call-path reachability.

If a production change is required, complete a design review after the first reproducible linkage
explanation and before selecting removal or mitigation.

## Bug-Fix Process

Not applicable initially: this is a security reachability investigation. If an affected API is
reachable, reclassify the issue as a bug, escalate priority to `p1`, and follow the `fix-bug`
workflow before implementing remediation.

## Regression Test Strategy

If native zlib is removed, add the smallest maintained check that fails when the final tracker
binary unexpectedly links it again. If linkage is required but the affected symbols are
unreachable, prefer a deterministic binary-symbol or dependency-boundary check over a manual-only
assertion.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce final native linkage | Record build command, image digest, `ldd` output, and copied runtime library. |
| T2 | TODO | Identify the linkage owner | Trace the native dependency to a crate, build script, compiler input, or runtime component. |
| T3 | TODO | Assess affected symbol reachability | Inspect imports, symbols, and runtime call paths for both CVEs. |
| T4 | TODO | Remove or justify the dependency | Remove unnecessary linkage or record a reproducible unreachable-boundary rationale. |
| T5 | TODO | Update security records and scan | Finalize both CVE verdicts and reconcile GitHub alerts 24 and 59, or 12 and 60. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T3 | Investigation evidence without production behavior changes | Commit when evidence is reproducible and independently reviewable. |
| T4 | Native dependency removal or maintained reachability guard | Commit after focused validation and security review. |
| T5 | Catalog, CVE verdicts, and scan evidence | Commit after the final scanner result matches the documented verdict. |

## Progress Tracking

### Workflow Checkpoints

- [x] Draft specification created from the #2493 triage
- [x] Draft reviewed and approved by maintainer
- [x] GitHub issue created and issue number added
- [ ] Investigation evidence completed
- [ ] Final verdict and any remediation completed
- [ ] Automatic and manual verification completed
- [ ] Acceptance criteria reviewed after implementation
- [ ] Independent review completed
- [ ] Issue closed and spec archived

### Progress Log

- 2026-10-08 16:55 UTC - Copilot - Drafted from GitHub Security cluster GSF-006 after native
  linkage evidence invalidated the previous unused-package premise.
- 2026-10-08 17:05 UTC - Maintainer - Approved the focused draft; Copilot created GitHub issue
  #2497 and promoted the specification to the open issue catalog.
- 2026-10-08 18:00 UTC - Copilot - Added the push-scan alert numbers 12 and 60 for the same
  findings, from PR #2498 review finding `review-finding:pr-2498-f10`.

## Acceptance Criteria

- [ ] The exact source of native `libz.so.1` linkage is identified.
- [ ] Reachability of every affected API named by both CVEs is established with reproducible
  evidence.
- [ ] Unnecessary native linkage is removed, or required linkage is protected by a maintained
  check and evidence-based non-affecting rationale.
- [ ] CVE-2026-27171 and CVE-2026-85091 records contain the final verdict and recheck triggers.
- [ ] GitHub Code Scanning alerts 24 and 59, and their push-scan duplicates 12 and 60, are closed
  by a fix or explicitly reconciled with the documented disposition.
- [ ] `linter all` and relevant focused tests pass.

## Verification Plan

### Automatic Checks

- Build the production release binary and image.
- Inspect dynamic dependencies and imported zlib symbols.
- Run the smallest test or maintained dependency check introduced by the solution.
- Run the production image Trivy scan.
- Run `linter all`.

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reproduce linkage | Build the release image and inspect the tracker binary and copied libraries. | Evidence identifies whether and why `libz.so.1` is linked. | TODO | `manual-verification-evidence.md` |
| M2 | Verify affected symbols | Inspect imports and call paths for `crc32_combine*`, `gzwrite`, `gzprintf`, and `gzvprintf`. | Each affected API has an explicit reachable or unreachable verdict. | TODO | `manual-verification-evidence.md` |
| M3 | Re-scan final image | Run the documented Trivy command against the resulting image. | Scanner state agrees with the final catalog verdict. | TODO | `manual-verification-evidence.md` |

## Acceptance Verification

Complete after the investigation. Record image digest, binary hashes, symbol commands, scanner
version, alert state, and the evidence supporting the final priority and disposition.

## Implementation Completion Review

Create `implementation-retrospective.md` if the linkage source or remediation changes the expected
production architecture. Otherwise record why the investigation produced no reusable design
lesson beyond the updated CVE evidence.
