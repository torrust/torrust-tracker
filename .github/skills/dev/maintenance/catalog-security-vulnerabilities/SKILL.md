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

## Quick Reference

```text
docs/security/analysis/
  README.md              ← Process + template
  non-affecting/         ← CVEs that do NOT affect us (catalog)
  affecting/             ← CVEs that DO affect us (future)
```

## Process (3 Steps)

### Step 1: Check the Catalog

Before analyzing a new warning, check `docs/security/analysis/non-affecting/` to see if
it has already been evaluated. Every file there documents why a set of CVEs is
non-affecting. If found, the analysis is already done — link the existing document in
any related issue or PR comment.

### Step 2: Analyse and Document (if not cataloged)

If the vulnerability is **not yet cataloged**:

1. Determine whether it affects us (see criteria examples in the README).
2. If **non-affecting**: create a dated file in `non-affecting/` following the template
   in the README. Include rationale, future actions, and review cadence.
3. If **affecting**: escalate immediately (see Step 3).

### Step 3: Escalate if Affecting

If a vulnerability **does** affect us (rare — the runtime is distroless):

1. Create a file in `docs/security/analysis/affecting/` with the same template.
2. Open a GitHub issue with the `security` and `bug` labels.
3. Notify maintainers — these are high priority.

## Review Cadence

All analysis documents have a `review-cadence` field in their frontmatter. The default
is `quarterly` — re-check whether upstream CVEs have been fixed and whether the
assessment is still valid.

## Policy

- Never ignore a vulnerability warning without documenting why.
- The runtime image (`gcr.io/distroless/cc-debian13:debug`) is the critical trust boundary.
  Build-stage CVEs are generally non-affecting unless they involve code execution during
  build that could compromise the output binary.
