---
semantic-links:
  skill-links:
    - catalog-security-vulnerabilities
  related-artifacts:
    - Containerfile
    - docs/security/analysis/production/
    - docs/security/analysis/build/
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
    - docs/security/docker/scans/torrust-tracker.md
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
├── production/                # CVEs in the production runtime image (release stage)
│   └── CVE-{id}.md            # Per-CVE files
├── build/                     # CVEs in build-stage images (chef, tester, gcc)
│   ├── CVE-{id}.md            # Per-CVE files
│   └── {date}_{source}.md     # Bulk scan/event files (for bulk triage)
└── affecting/                 # (future) Vulnerabilities that DO affect us
```

## Catalog Strategy

We use **one catalog** for all vulnerability sources (Docker scans, cargo-audit, dependabot,
etc.), organized by the impact context of the affected image. A vulnerability is a
vulnerability regardless of origin, but its risk profile depends on whether it appears in
the production runtime or in an ephemeral build stage.

### Per-CVE Files (preferred)

Individual CVEs from container scans are documented in their own file under the appropriate
subdirectory:

```text
production/
├── CVE-2026-5435.md   # glibc TSIG — production runtime
├── CVE-2026-5450.md   # glibc scanf — production runtime
├── CVE-2026-5928.md   # glibc ungetwc — production runtime
├── CVE-2026-6238.md   # glibc DNS response — production runtime
└── CVE-2026-27171.md  # zlib CRC32 — production runtime

build/
├── CVE-2026-20889.md  # libraw — chef/tester/gcc build stages
└── ...
```

**Advantages**:

- `grep -r CVE-2026-5435` finds it instantly.
- Fast to check "have we seen this before?" on any new scan.
- Each file carries its own `date-analyzed`, `review-cadence`, and `requires-recheck-when`
  in frontmatter.
- Impact context is immediately visible from the directory name.

### Bulk Scan/Event Files

For bulk triage (e.g. a full Docker Scout report with dozens of CVEs from build stages),
a single event-based file can be used instead of creating individual CVE files. Example:

```text
build/
└── 2026-06-10_containerfile-trixie-cves.md  # Bulk triage of 100+ build-stage CVEs
```

## Process

### When a security warning appears

1. **Check the catalog**: `grep -r '<CVE-ID>' docs/security/analysis/` to
   see if this vulnerability has already been analyzed. If it has, verify the
   `requires-recheck-when` conditions still hold. If they do, you're done.

2. **If not yet cataloged**: create a new per-CVE analysis document in the appropriate
   subdirectory (`production/` or `build/`) following the template below.

3. **If it DOES affect us**: escalate immediately. Create an issue and a fix. The analysis
   document should describe the impact, affected components, and remediation plan.

### Recheck Policy

Non-affecting verdicts can become stale when dependencies or code change. Each per-CVE file
has a `requires-recheck-when` field that specifies the conditions under which the verdict
must be re-evaluated.

**Triggers for recheck**:

- A change to the `Containerfile` base image.
- A new system dependency added (e.g., via new `FROM` stage or package install).
- Any change that affects the `requires-recheck-when` condition documented in the CVE file.
