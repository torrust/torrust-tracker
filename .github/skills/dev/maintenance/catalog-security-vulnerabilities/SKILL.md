---
name: catalog-security-vulnerabilities
description: Guide for cataloging security vulnerability warnings (e.g. Docker DX CVEs) that do NOT affect the project. Covers the process of checking the existing catalog, creating a new analysis document with rationale, and escalating if a vulnerability is found to be affecting. Use when handling Docker DX warnings, CVE analysis, vulnerability scanning results, or security audit findings. Triggers on "Docker DX", "vulnerability warning", "CVE analysis", "security scan", "catalog vulnerability", "non-affecting CVE", or "container CVE".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/security/analysis/README.md
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

## Quick Reference

```text
docs/security/analysis/
  README.md              ← Process + template
  production/            ← CVEs in the production runtime image (catalog)
  build/                 ← CVEs in build-stage images (catalog)
  affecting/             ← CVEs that DO affect us (create when needed)
```

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
3. If **non-affecting**: create a dated file in the appropriate subdirectory following the
   template in the README. Include rationale, future actions, and review cadence.
4. If **affecting**: escalate immediately (see Step 3).

### Step 3: Escalate if Affecting

If a vulnerability **does** affect us (rare — the runtime is distroless):

1. Confirm it is already public or approved for disclosure. Otherwise stop this workflow and
   use `docs/security/vulnerability-remediation.md`.
2. Create the `docs/security/analysis/affecting/` directory if it does not exist, then create a file there with the same template.
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
