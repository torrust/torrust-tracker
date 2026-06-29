---
doc-type: issue
issue-type: task
status: planned
priority: p2
github-issue: 1459
spec-path: docs/issues/open/1459-docker-security-overhaul/ISSUE.md
branch: 1459-docker-security-overhaul
related-pr: null
last-updated-utc: 2026-06-29
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/workflows/security-scan.yaml
    - Containerfile
    - .github/workflows/container.yaml
    - .github/skills/dev/maintenance/run-manual-docker-security-scan/SKILL.md
    - docs/security/README.md
    - docs/security/docker/scans/
    - docs/security/docker/README.md
    - docs/security/analysis/non-affecting/
---

# Issue #1459 - Docker Security Overhaul: Set Up Security Scanning Workflow

## Problem

The torrust-tracker Docker image contains known vulnerabilities that need to be regularly scanned and monitored. As demonstrated by the Trivy scan results, the current image has multiple security vulnerabilities including critical, high, and medium severity issues.

## Goal

Implement a scheduled workflow to periodically scan Docker images for vulnerabilities and misconfigurations, ensuring the security posture of the application is maintained.

## Acceptance Criteria

- [ ] A new GitHub Actions workflow is created in `.github/workflows/security-scan.yaml`
- [ ] The workflow runs on a schedule (daily) to scan the Docker image
- [ ] The workflow builds the Docker image and scans it with Trivy
- [ ] Vulnerability findings are reported in both human-readable and SARIF formats
- [ ] The workflow integrates with the existing container build process
- [ ] The README.md badge row includes the new security scan workflow badge
- [ ] `docs/security/docker/scans/` is created with the first baseline scan report
- [ ] `docs/security/docker/README.md` provides scanning instructions
- [ ] `docs/security/README.md` provides a priority-tier security overview
- [ ] Per-CVE analysis files created in `docs/security/analysis/non-affecting/` for each
      MEDIUM vulnerability found in the baseline scan
- [ ] `docs/security/analysis/README.md` documents the catalog strategy and recheck policy
- [ ] A maintenance skill exists at
      `.github/skills/dev/maintenance/run-manual-docker-security-scan/SKILL.md`
      documenting how to run and document manual Docker security scans

## Implementation Plan

### Step 1: Create Security Scan Workflow

Create a new workflow file `.github/workflows/security-scan.yaml` that:

- Runs on a schedule (daily at 6 AM UTC) and on push to main/develop branches
- Builds the Docker image using the Containerfile
- Scans the image with Trivy
- Reports results in both table and SARIF formats

### Step 2: Configure Trivy Scanning

Configure the workflow to:

- Use Trivy to scan the Docker image
- Report vulnerabilities in both human-readable table format and SARIF format for GitHub Code Scanning
- Generate SARIF output for integration with GitHub Security features

### Step 3: Integrate with Existing Workflows

Ensure the security scan workflow integrates properly with the existing container workflow.

### Step 4: Add Workflow Badge to README.md

Add the security scan workflow badge to the README.md header row and consistent reference links at the bottom, following the same pattern as existing workflow badges.

### Step 5: Create Security Documentation and Run Baseline Scan

Create `docs/security/docker/` structure mirroring the deployer's security docs pattern:

- `docs/security/docker/README.md` — scanning instructions and context
- `docs/security/docker/scans/README.md` — scan history index table
- `docs/security/docker/scans/torrust-tracker.md` — detailed scan report with vulnerability analysis

Run the first manual baseline scan of the production `release` stage image and document all findings, including vulnerability analysis and severity assessment.

### Step 6: Create Top-Level Security Overview

Create `docs/security/README.md` providing a priority-tier overview of security areas for the project, mirroring the deployer's top-level security README pattern:

- Priority 1: Production Docker image (critical, internet-exposed)
- Priority 2: Vulnerability analysis (evaluation and tracking)
- Priority 3: Build chain security (lower-risk, build-time only)
- Current security status summary
- Scan tooling reference

### Step 7: Create Non-Affecting CVE Catalog

Create per-CVE analysis files in `docs/security/analysis/non-affecting/` for each
vulnerability found in the baseline scan, following this pattern:

```text
non-affecting/
├── CVE-2026-5435.md   # glibc TSIG
├── CVE-2026-5450.md   # glibc scanf
├── CVE-2026-5928.md   # glibc ungetwc
├── CVE-2026-6238.md   # glibc DNS response
└── CVE-2026-27171.md  # zlib CRC32
```

Each file includes:

- Frontmatter with `cve-id`, `date-analyzed`, `source`, `status`, `review-cadence`,
  and `requires-recheck-when` conditions
- Vulnerability description and severity
- Evidence-based rationale for why it does not affect the tracker
- Conditions that would change the verdict

Update `docs/security/analysis/README.md` to document the catalog strategy (one catalog
for all vulnerability sources, per-CVE files preferred, with recheck policy).

### Step 8: Add Maintenance Skill for Manual Security Scans

Create a new skill at
`.github/skills/dev/maintenance/run-manual-docker-security-scan/SKILL.md` to standardize
how contributors run manual Docker security scans and maintain scan documentation.

The skill should include:

- build and scan commands (`docker build`, `trivy image`)
- triage workflow (check catalog first, then analyze)
- documentation update requirements (`docs/security/docker/scans/*` and
  `docs/security/analysis/non-affecting/CVE-*.md`)
- recheck triggers and escalation path for affecting vulnerabilities

## References

- Original issue: https://github.com/torrust/torrust-tracker/issues/1459
- Related issue #1630
- Trivy documentation for GitHub Actions integration
- Tracker Deployer security scan workflow for reference: https://github.com/torrust/torrust-tracker-deployer/blob/main/.github/workflows/docker-security-scan.yml

## Verification Plan

### Automatic Checks

- [ ] Workflow file is created and syntactically correct
- [ ] Workflow runs successfully on schedule
- [ ] Trivy scan produces expected output

### Manual Verification Scenarios

- [ ] Run workflow manually to verify it scans the image
- [ ] Verify vulnerability reports are generated correctly
- [ ] Confirm workflow integrates with existing container workflow
