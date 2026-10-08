---
semantic-links:
  skill-links:
    - triage-github-security-findings
    - catalog-security-vulnerabilities
    - run-manual-docker-security-scan
  related-artifacts:
    - docs/security/public-scanner-findings.md
    - docs/security/analysis/README.md
    - docs/security/docker/README.md
    - docs/security/vulnerability-remediation.md
---

# Security Overview

This directory documents security considerations for the Torrust Tracker project. Security work
is prioritized by the affected trust boundary and demonstrated path to users, not by scanner
severity alone.

## Priority Model

Record these dimensions separately during triage:

1. **Scanner severity** — the tool's severity or rule level; useful evidence, not the project
   priority.
2. **Affected surface** — runtime product, release supply chain, CI/build infrastructure, or local
   development/test tooling.
3. **Reachability and exposure** — whether untrusted input can reach the weakness and under which
   deployment or execution conditions.
4. **Privilege and pivot potential** — what credentials, artifacts, runners, caches, or downstream
   systems an attacker could reach.
5. **Repository priority** — the maintainer-assigned urgency after considering the prior four
   dimensions and available mitigations.

A `critical` scanner result can be non-affecting, while a lower-severity workflow finding can be
high priority if it permits release artifact or publishing-token compromise.

### Tier 1 — Runtime Product and Release Supply Chain

This is the highest-priority trust tier because a confirmed compromise can reach production users
or distributed production artifacts.

**Runtime product scope**:

- tracker application code shipped in the final binary;
- the production container image and runtime OS libraries;
- Cargo dependencies linked into or used by the shipped application;
- runtime configuration defaults that create an exploitable production condition.

**Release supply-chain scope**:

- workflows and credentials that publish crates or production-grade container images;
- release branches, artifact provenance, registry authentication, and release-specific caches;
- any CI or build component with a demonstrated path to alter a published artifact.

This repository does not operate a production tracker deployment. Deployment security for demo or
hosted instances belongs to the repository that operates each environment. However, this
repository publishes crates and production-grade Docker images. A compromise of its release/CD
path can therefore affect downstream production users even without a Torrust-operated production
server.

Confirmed affecting findings in this tier normally receive `p1` / `High Priority`. Use `p0` only
for an active or imminent emergency such as exposed publishing credentials, known malicious
artifacts, or exploitation requiring immediate containment.

**Scan history**: [`docker/scans/`](docker/scans/)

### Tier 2 — CI and Privileged Build Infrastructure

CI is important but is not automatically equivalent to production or release publishing.

**Scope**:

- pull-request and branch-validation workflows;
- self-hosted runners, build images, actions, caches, and generated test artifacts;
- CI-only tokens and services that cannot publish or modify release artifacts.

These findings normally receive `p2`. Escalate them to Tier 1 and `p1` when evidence shows a pivot
to publishing credentials, release artifacts, protected branches, shared privileged runners, or
another production-impacting trust boundary.

CI and CD must be assessed separately even when they share workflow files or machines. Shared
runners, caches, credentials, or write permissions can turn an apparently CI-only finding into a
release-supply-chain problem.

### Tier 3 — Local Development, Testing, and Documentation Tooling

This is normally the lowest security priority because the affected code is neither shipped nor
executed in a privileged shared environment.

**Scope**:

- test-only dependencies and fixtures;
- local development scripts and disposable tools;
- documentation-only tooling;
- failures requiring a developer to run an already-untrusted local payload manually.

Confirmed findings usually receive `p2` or `p3`, depending on exploitability and impact.
Non-affecting findings and low-value hardening can remain documented without a remediation issue.
Escalate when the development surface can steal maintainer credentials, poison committed output,
or pivot into CI or release publishing.

## Priority Assignment Rules

| Evidence and impact | Default priority |
| --- | --- |
| Active compromise, exposed release credential, malicious published artifact, or immediate containment required | `p0` |
| Confirmed runtime or release-supply-chain impact that can reach downstream production users | `p1` |
| Confirmed CI/build issue without a release pivot, or meaningful development/test security issue | `p2` |
| Low-impact hardening, defense in depth, or deferred local-tooling improvement | `p3` |
| False positive or non-affecting under documented conditions | No remediation priority; catalog with recheck triggers |

Priority is assigned to each root-cause cluster after impact analysis. Do not automatically give
every issue created from the Security tab `High Priority` or `p1`.

## Vulnerability Analysis

When a security issue is detected through a public source such as Trivy, Dependabot, RustSec,
Code Scanning, or Code Quality, create or update the appropriate analysis record to determine
whether it affects the project and which tier owns the risk.

**Documents**:

- [Analysis README](analysis/README.md) — process and document index
- [Non-affecting CVEs](analysis/production/) — analyzed and accepted vulnerabilities in the production runtime image
- [Build-stage CVEs](analysis/build/) — analyzed and accepted vulnerabilities in build-stage images

## Scan Tooling

| Tool  | Purpose                   | Run Command                                    |
| ----- | ------------------------- | ---------------------------------------------- |
| Trivy | Docker image CVE scanning | `trivy image --severity HIGH,CRITICAL <image>` |

## Current Security Status

### Production Image

See [`docker/scans/README.md`](docker/scans/README.md) for the latest status of the production `release` stage image.

### Vulnerability Analysis

See [`analysis/README.md`](analysis/README.md) for cataloged vulnerability evaluations.

**Non-affecting CVE catalog**: [`analysis/production/`](analysis/production/) —
per-CVE files documenting why each vulnerability does not affect the tracker and what
conditions would change the verdict.

**Build-stage CVE catalog**: [`analysis/build/`](analysis/build/) —
per-CVE and bulk files documenting vulnerabilities in ephemeral build images.

**Handled-report catalog**: [`analysis/reports/`](analysis/reports/) —
one sanitized record per coordinated-disclosure report the project has processed (fixed,
hardened, declined, or non-affecting), created at disclosure time.

## Related Documentation

- [Confidential Vulnerability Remediation](vulnerability-remediation.md) — coordinated-disclosure process for privately reported vulnerabilities
- [GitHub Security Scanner Findings](public-scanner-findings.md) — periodic review and clustering workflow for Code Scanning / Code Quality findings
- [Docker Image Security](docker/README.md) — scanning instructions and scan history
- [Security Analysis](analysis/README.md) — CVE evaluation process
- [`SECURITY.md`](../../SECURITY.md) — project security policy and reporting
