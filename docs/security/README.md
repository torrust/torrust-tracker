# Security Overview

This directory documents security considerations for the Torrust Tracker project, organized by priority level.

## Priority Levels

Security effort should be distributed according to exposure and risk. The highest-priority areas are those that directly affect end users in production.

### Priority 1 — Production Docker Image (Critical)

**Directory**: [`docker/`](docker/)

The production Docker image (`release` stage based on `gcr.io/distroless/cc-debian13`) is the most critical security surface. It is exposed to the internet and runs continuously. Any vulnerability here directly affects tracker users.

**Scope**:

- The tracker binary
- OS base layer (distroless/cc-debian13)
- Transitive library dependencies (glibc, zlib)

**Scan history**: [`docker/scans/`](docker/scans/)

---

### Priority 2 — Vulnerability Analysis (Important)

**Directory**: [`analysis/`](analysis/)

When a security issue is detected (Docker Scout, dependabot, manual audit, container image scans), we create an analysis document to evaluate whether it actually affects the tracker.

**Scope**:

- Evaluating CVEs reported by scanning tools
- Documenting non-affecting vulnerabilities
- Tracking affecting vulnerabilities with remediation plans

**Documents**:

- [Analysis README](analysis/README.md) — process and document index
- [Non-affecting CVEs](analysis/production/) — analyzed and accepted vulnerabilities in the production runtime image
- [Build-stage CVEs](analysis/build/) — analyzed and accepted vulnerabilities in build-stage images

---

### Priority 3 — Build Chain Security (Standard)

The build-time images (`chef`, `tester`, `gcc` stages in the `Containerfile`) are a **lower-risk surface** because:

- They run only during CI/CD or local builds
- They are not exposed to the internet
- They produce the final artifact but are not deployed themselves

This priority increases if the build images are ever used in a long-running service.

**Scope**:

- Base build images: `rust:slim-trixie` (chef + tester), `debian:trixie-slim` (gcc)
- Rust dependency vulnerabilities (`cargo audit` / RustSec)
- CI/CD pipeline security

---

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

## Related Documentation

- [Confidential Vulnerability Remediation](vulnerability-remediation.md) — coordinated-disclosure process for privately reported vulnerabilities
- [Docker Image Security](docker/README.md) — scanning instructions and scan history
- [Security Analysis](analysis/README.md) — CVE evaluation process
- [`SECURITY.md`](../../SECURITY.md) — project security policy and reporting
