# Manual Verification Evidence

## V1 — Security Workflow Routing

**Status:** DONE

Compared the four finding sources against the routing table in
`triage-github-security-findings` and the canonical process:

| Source | Workflow |
| --- | --- |
| GitHub Code Scanning and Code Quality | `triage-github-security-findings` |
| Public CVE, dependency, and container warnings | `catalog-security-vulnerabilities` |
| Manual production image scan | `run-manual-docker-security-scan` |
| Confidential or embargoed report | `docs/security/vulnerability-remediation.md` |

The workflows have distinct publication and evidence boundaries. No confidential workflow creates
public issues or catalog entries.

## V2 — Initial Finding Coverage

**Status:** DONE

The API review returned:

- 66 open Code Scanning alerts;
- 2 open Code Quality findings;
- 68 total findings.

The dated review maps all 68 findings to 11 catalog clusters. Disposition totals reconcile:

| Disposition | Count |
| --- | ---: |
| Hardening | 27 |
| Needs investigation | 2 |
| Non-affecting | 39 |
| Confirmed affecting | 0 |
| **Total** | **68** |

Evidence:

- `docs/security/analysis/github-security/reviews/2026-10-08.md`
- `docs/security/analysis/github-security/README.md`

## V3 — Issue Granularity

**Status:** DONE

The actionable findings produced three independently deliverable issues:

1. #2497 — native zlib reachability investigation for alerts 24 and 59.
2. #2496 — explicit permissions for seven release-publishing alerts.
3. #2495 — explicit permissions for twenty CI/build alerts.

The two workflow drafts are split because release publishing and CI validation have different
trust boundaries, owners, safe verification paths, and consequences. The zlib investigation is
separate because it requires runtime linkage and symbol evidence rather than workflow changes.

The 39 non-affecting findings remain in the catalog with recheck triggers and do not create
remediation issues.
