---
name: run-manual-docker-security-scan
description: Guide for running a manual Docker security scan for the tracker runtime image and documenting results. Covers build, Trivy scan, CVE triage, per-CVE catalog updates, and scan history updates. Use when asked to run a manual container scan, triage Docker CVEs, or refresh security scan docs.
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - Containerfile
      - docs/security/README.md
      - docs/security/docker/README.md
      - docs/security/docker/scans/README.md
      - docs/security/docker/scans/torrust-tracker.md
      - docs/security/analysis/README.md
      - docs/security/analysis/production/
      - docs/security/analysis/build/
---

# Run Manual Docker Security Scan

Use this workflow to run and document manual security scans for the tracker production container.

## Scope

- Target image: tracker runtime image built from root `Containerfile`.
- Main severity gate: `HIGH,CRITICAL`.
- Documentation outputs:
  - `docs/security/docker/scans/torrust-tracker.md`
  - `docs/security/docker/scans/README.md`
  - `docs/security/analysis/production/CVE-*.md` (when non-affecting CVEs are analyzed)

## Quick Commands

```bash
# 1) Build runtime image
docker build -t torrust-tracker:local -f Containerfile .

# 2) Gate scan (primary)
trivy image --severity HIGH,CRITICAL torrust-tracker:local

# 3) Full context scan (optional but recommended)
trivy image --severity MEDIUM,HIGH,CRITICAL torrust-tracker:local
```

## Workflow

### Step 1: Check Existing Catalog First

Before analyzing any CVE, search the existing catalog:

```bash
grep -R "CVE-<id>" docs/security/analysis/
```

If already present and `requires-recheck-when` conditions have not changed, reuse the existing verdict.

### Step 2: Build and Scan

- Build local runtime image from `Containerfile`.
- Run the gate scan with `HIGH,CRITICAL`.
- Run optional full scan (`MEDIUM,HIGH,CRITICAL`) to capture trend context.

### Step 3: Update Scan History Docs

Update:

- `docs/security/docker/scans/torrust-tracker.md` with:
  - date/time, Trivy version, totals by severity
  - notable CVEs and rationale
- `docs/security/docker/scans/README.md` summary table with latest status and date.

### Step 4: Document New Non-Affecting CVEs

For any new non-affecting CVE, create `docs/security/analysis/production/CVE-<id>.md` or
`docs/security/analysis/build/CVE-<id>.md` with:

- frontmatter fields:
  - `cve-id`
  - `date-analyzed`
  - `source`
  - `status: non-affecting`
  - `review-cadence`
  - `requires-recheck-when`
- evidence-based explanation tied to tracker architecture
- conditions that would invalidate the current verdict

### Step 5: Escalate Affecting CVEs

If a CVE is affecting:

- create/update a tracking issue
- include impact, affected component, exploitability context, and remediation plan
- update scan docs with current status and owner

## Recheck Triggers

Re-evaluate catalog verdicts when any of these happen:

- `Containerfile` base image changes
- new runtime/system dependency is introduced
- code path changes that satisfy a CVE file's `requires-recheck-when` condition

## Completion Checklist

- [ ] `trivy` gate scan executed (`HIGH,CRITICAL`)
- [ ] scan history files updated
- [ ] new CVEs cataloged or linked to existing catalog entries
- [ ] affecting CVEs escalated
- [ ] `linter all` passes
