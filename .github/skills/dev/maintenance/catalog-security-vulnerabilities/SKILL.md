---
name: catalog-security-vulnerabilities
description: Guide for cataloging public container, dependency, and CVE warnings that do not affect the project, with escalation for affecting vulnerabilities. Use for Docker DX, Trivy, cargo audit, RustSec, dependency advisories, CVE analysis, non-affecting CVEs, or container CVEs. For GitHub Code Scanning or Code Quality findings, use triage-github-security-findings instead.
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/security/analysis/README.md
      - docs/templates/SECURITY-ANALYSIS.md
      - .github/skills/dev/maintenance/triage-github-security-findings/SKILL.md
---

# Catalog Security Vulnerabilities

This skill guides you through evaluating and documenting security vulnerability warnings
(such as Docker DX extension flags or scanner output) that appear in the project's
dependencies or infrastructure.

The authoritative process document is `docs/security/analysis/README.md` — this skill
provides a quick reference.

This skill applies only to public scanner findings and vulnerabilities already approved for
disclosure. For a privately reported or embargoed vulnerability, do not create a public
catalog record or issue; follow `docs/security/vulnerability-remediation.md`.

For source-level findings from GitHub Code Scanning or Code Quality, use
[`triage-github-security-findings`](../triage-github-security-findings/SKILL.md). That workflow
clusters repeated alerts and maintains their dispositions; this skill remains focused on public
CVE, container, and dependency vulnerability analysis.

## Quick Reference

```text
docs/security/analysis/
  README.md              ← Process and catalog placement
  production/            ← CVEs in the production runtime image (catalog)
  build/                 ← CVEs in build-stage images (catalog)
  reports/               ← Handled coordinated-disclosure reports (created at disclosure)
  affecting/             ← CVEs that DO affect us (create when needed)
```

`reports/` is written by the confidential remediation process, not by this skill; consult it
when a scanner or reviewer flags code that was already the subject of a handled report (grep
the path or CWE).

## Process (3 Steps)

### Step 1: Check the Catalog

Before analyzing a new warning, check `docs/security/analysis/production/` and
`docs/security/analysis/build/` to see if it has already been evaluated. Every file there
documents why a set of CVEs is non-affecting. If found, the analysis is already done —
link the existing document in any related issue or PR comment.

### Step 2: Analyse and Document (if not cataloged)

If the vulnerability is **not yet cataloged**:

1. Determine whether it affects us (see criteria examples in the README).
2. Determine the impact context: production runtime (`production/`) or build stage
   (`build/`).
3. If **non-affecting**: create a dated file in the appropriate subdirectory using
   [docs/templates/SECURITY-ANALYSIS.md](../../../../../docs/templates/SECURITY-ANALYSIS.md).
   Include rationale, future actions, and review cadence.
4. If **affecting**: escalate immediately (see Step 3).

### Step 3: Escalate if Affecting

If a vulnerability **does** affect us (rare — the runtime is distroless):

1. Confirm it is already public or approved for disclosure. Otherwise stop this workflow and
   use `docs/security/vulnerability-remediation.md`.
2. Create the `docs/security/analysis/affecting/` directory if it does not exist, then create a
   file there using [docs/templates/SECURITY-ANALYSIS.md](../../../../../docs/templates/SECURITY-ANALYSIS.md).
3. Open a GitHub issue with the `security` and `bug` labels.
4. Notify maintainers — these are high priority.

## Review Cadence

All analysis documents have a `review-cadence` field in their frontmatter. The default
is `quarterly` — re-check whether upstream CVEs have been fixed and whether the
assessment is still valid.

## Policy

- Never ignore a vulnerability warning without documenting why.
- The runtime image (`gcr.io/distroless/cc-debian13:debug`) is the critical trust boundary.
  Build-stage CVEs are generally non-affecting unless they involve code execution during
  build that could compromise the output binary.
