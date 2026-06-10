---
semantic-links:
  skill-links:
    - catalog-security-vulnerabilities
  related-artifacts:
    - Containerfile
    - docs/security/analysis/non-affecting/
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
---

# Security Analysis

This folder contains security analysis documents for the Torrust Tracker project.

## Purpose

When a security issue is detected (e.g., by Docker Scout, dependabot, manual audit, or
container image vulnerability scanning), we create an analysis document here to:

1. **Evaluate** whether the vulnerability actually affects our project.
2. **Document the decision** so other contributors seeing the same warning can quickly
   determine whether it has been analyzed before.
3. **Track periodic review** — even non-affecting vulnerabilities should be re-evaluated
   periodically to check if the situation has changed.

## Folder Structure

```text
docs/security/analysis/
├── README.md                  # This file — index and process
├── non-affecting/             # Vulnerabilities that do NOT affect us
│   └── {date}_{descriptive-name}.md
└── ...                        # (future) Affecting vulnerabilities go here
```

## Process

### When a security warning appears

1. **Check the catalog**: search `docs/security/analysis/non-affecting/` to see if this
   vulnerability has already been analyzed. If it has, you're done — the document explains
   why it doesn't affect us and what to watch for.

2. **If not yet cataloged**: create a new analysis document in `non-affecting/` (or an
   appropriate subfolder) following the template below.

3. **If it DOES affect us**: escalate immediately. Create an issue and a fix. The analysis
   document should describe the impact, affected components, and remediation plan.

### Analysis Document Template

Each analysis document should include:

- **Date of analysis**
- **Source of the warning** (tool, scanner, CVE database, etc.)
- **Vulnerability summary** — what CVEs, what packages, what severity
- **Why it does not affect us** — a clear rationale tied to our architecture
- **Future actions** — periodic review cadence, conditions that would change the status
- **References** — links to the original warning, Docker Hub layers, CVE entries, etc.

### Review Cadence

Non-affecting vulnerabilities should be reviewed:

- At least **quarterly** (or when the relevant base image is updated).
- Immediately if the affected image begins being used in a **different context** (e.g., if
  a build-stage image becomes part of the runtime).
