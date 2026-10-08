---
semantic-links:
  skill-links:
    - triage-github-security-findings
    - catalog-security-vulnerabilities
  related-artifacts:
    - "issue #2493"
    - docs/security/public-scanner-findings.md
    - docs/security/analysis/github-security/reviews/2026-10-08.md
    - "issue #2497"
    - "issue #2496"
    - "issue #2495"
    - "issue #2499"
---

# GitHub Security Findings Catalog

This catalog records durable dispositions for public findings from GitHub Code Scanning and Code
Quality. Dated reviews preserve the source-alert inventory; this file preserves the root-cause
clusters so a recurring alert is not analyzed from scratch.

The catalog follows the priority model in [`docs/security/README.md`](../../README.md). Scanner
severity is evidence, not repository priority.

Trivy findings currently carry two Code Scanning alert numbers, one per scan trigger, and only one
series is open at a time. Cite both numbers until issue #2499 stabilizes alert identity; the dated
review lists the pairing.

## Current Clusters

| ID | Finding class | Alerts | Scanner severity | Disposition | Surface | Priority | Follow-up |
| --- | --- | ---: | --- | --- | --- | --- | --- |
| GSF-001 | Missing explicit `GITHUB_TOKEN` permissions | 27 | medium | Hardening | Release supply chain and CI/build | `p3` | Split into #2496 and #2495 |
| GSF-002 | Test assertion bodies reported as cleartext logging | 4 | high | Non-affecting | Development/test | None | Recheck if helpers process production secrets |
| GSF-003 | Fixed test-only cookie cipher byte arrays | 2 | critical | Non-affecting | Development/test | None | Recheck if test constructors become production-callable |
| GSF-004 | Fixed disposable qBittorrent E2E credentials | 2 | critical | Non-affecting | Development/test | None | Recheck if reused outside disposable E2E stacks |
| GSF-005 | Previously cataloged glibc DNS findings | 2 | medium | Non-affecting | Runtime | None | Continue per-CVE catalog reviews |
| GSF-006 | Native zlib CVE reachability | 2 | medium | Needs investigation | Runtime | `p2` | #2497 |
| GSF-007 | libstdc++ findings in C++ APIs unused by the tracker | 8 | medium | Non-affecting | Runtime | None | Recheck if affected C++ APIs become reachable |
| GSF-008 | Historical low-severity glibc findings | 7 | low | Non-affecting | Runtime | None | Recheck on relevant API or deployment changes |
| GSF-009 | Function- or deployment-specific glibc 2026 findings | 12 | medium; low for CVE-2026-97399 | Non-affecting | Runtime | None | Recheck on relevant API or deployment changes |
| GSF-010 | Unclosed `/dev/null` handle in the active merge tool | 1 | warning | Non-affecting for security | Development tooling | None | Optional reliability cleanup only |
| GSF-011 | Unclosed handle in an immutable historical artifact | 1 | warning | Non-affecting | Historical documentation | None | Do not edit the historical record |

## Disposition Rationale

### GSF-001 — Explicit workflow permissions

The affected jobs inherit the repository default `GITHUB_TOKEN` permissions. The current default
is read-only, and fork pull requests do not receive write tokens or environment secrets, so the
alerts do not demonstrate a current privilege escalation. Explicit permissions are still useful
defense in depth against a future repository-default change.

The cluster is split by trust boundary:

- seven release-publishing alerts belong to the release-supply-chain draft;
- twenty validation and CI alerts belong to the CI/build draft.

Recheck immediately if the repository token default becomes write-capable, a job gains a new
secret, a shared runner boundary changes, or a validation job begins producing a published
artifact.

### GSF-002 — Cleartext logging in test assertions

Alerts 96–99 point to test-only assertion helpers that render HTTP response bodies when tests
fail. They do not run in the tracker application and do not process production credentials.

Recheck if these helpers are moved into production code, used against a deployed environment, or
receive real secrets.

### GSF-003 — Fixed cookie-cipher values

Alerts 146–147 point to deterministic values in cookie-cipher tests. The constructors and values
are test-only, and production cookie keys are generated through the production entropy path.

Recheck if the fixed constructor loses its test-only boundary or becomes callable by production
composition code.

### GSF-004 — qBittorrent E2E credentials

Alerts 100–101 point to fixed credentials for a disposable qBittorrent E2E stack. They are not
tracker production credentials and do not protect a persistent shared service.

Recheck if the credentials are reused outside the disposable stack or the test service becomes
persistent or externally exposed.

### GSF-005 — glibc DNS findings

Alerts 13 and 16 (push-scan series: 1 and 4) are already analyzed in
[`CVE-2026-5435.md`](../production/CVE-2026-5435.md) and
[`CVE-2026-6238.md`](../production/CVE-2026-6238.md). The affected specialized resolver paths are
not used by the tracker.

### GSF-006 — Native zlib reachability

Alerts 24 and 59 (push-scan series: 12 and 60) cannot retain a non-affecting verdict solely on the claim that system zlib is not
used: the production image copies the `libz.so.1` linked by the release binary. No call to the
affected APIs has been established, so this remains an investigation rather than a confirmed
vulnerability.

Issue #2497 must establish why native zlib is linked, whether the affected symbols are reachable,
and whether the dependency can be removed.

### GSF-007 to GSF-009 — Runtime CVE groups

The grouped runtime CVEs are documented in
[`2026-10-08_github-code-scanning-runtime-cves.md`](../production/2026-10-08_github-code-scanning-runtime-cves.md).
Their affected APIs or deployment conditions are absent from the tracker runtime. Each group has
explicit recheck triggers.

### GSF-010 and GSF-011 — Python file handles

The active merge tool opens one write-only `/dev/null` descriptor and uses it for the lifetime of a
short-lived merge process. Process exit closes it, and it does not affect merge integrity, signing,
or pushing. The second finding is in an immutable historical issue artifact. Neither is a security
problem; an optional reliability cleanup of the active tool can be considered separately.

## Reviews

| Review date | Code Scanning | Code Quality | Total | Result |
| --- | ---: | ---: | ---: | --- |
| [2026-10-08](reviews/2026-10-08.md) | 66 | 2 | 68 | 11 clusters; 3 focused drafts; 39 non-affecting findings |

## Next Review

Review again by **2027-01-08**, before the next release cycle if earlier, or immediately when a
recheck trigger in this catalog occurs.
