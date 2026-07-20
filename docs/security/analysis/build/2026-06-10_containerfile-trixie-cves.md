---
date-analyzed: 2026-07-20
source: Trivy 0.69.3 / Docker DX (docker-language-server)
status: non-affecting
review-cadence: quarterly
requires-recheck-when: any build-stage image (`chef`, `tester`, `gcc`) is used in a runtime context
image-digest: sha256:5c6f46a6e4472ab1ca7ba7d494e6677f2f219ebc02f32025d3986f057635ec9c
semantic-links:
  related-artifacts:
    - Containerfile
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
    - docs/security/docker/scans/build-images.md
---

# Containerfile trixie-based image vulnerabilities

## Context

The VS Code Docker DX extension (docker-language-server) flagged vulnerabilities in the
`Containerfile` on the three `FROM` instructions that use Debian trixie-based base images.
Line numbers drift as the file changes; the stages are the stable reference:

| Image                  | Stage    | Purpose                               |
| ---------------------- | -------- | ------------------------------------- |
| `rust:slim-trixie`     | `chef`   | Install `cargo-chef`, `cargo-nextest` |
| `rust:slim-trixie`     | `tester` | Run unit tests inside container build |
| `debian:trixie-slim`   | `gcc`    | Compile `su-exec` from source         |

## Vulnerability Summary

All three stages use **upstream Docker Official Images** based on Debian trixie. The
implemented stages were rebuilt and scanned together on 2026-07-20 with Trivy 0.69.3 and
the vulnerability database updated at 2026-07-20 13:19:47 UTC.

| Stage    | Debian packages | Critical | High | Medium | Low | Unknown | Total |
| -------- | --------------- | -------- | ---- | ------ | --- | ------- | ----- |
| `chef`   | 145             | 4        | 65   | 309    | 617 | 77      | 1,072 |
| `tester` | 123             | 4        | 51   | 301    | 579 | 79      | 1,014 |
| `gcc`    | 114             | 4        | 51   | 299    | 577 | 77      | 1,008 |

These totals count findings, not unique CVEs. The chef stage also contains installed Cargo
tools, so Trivy scans both OS and language-specific files there. Detailed reproducibility,
image IDs, and base digests are maintained in the consolidated build-image scan report.

## Why This Does NOT Affect Us

These vulnerabilities are **not exploitable in our deployment context** for the following
reasons:

### 1. Build-time-only stages

The `chef`, `tester`, and `gcc` stages are **intermediate build stages**. They exist only
during `docker build` and are never:

- **Pushed to any registry** as a runnable image.
- **Exposed to any network** — no ports are open, no services are listening.
- **Accessible to any external actor** — they run ephemerally in the build process and are
  discarded after the final `release` stage is assembled.

| Stage       | Base image               | Exposed to traffic?   | Persisted after build? |
| ----------- | ------------------------ | --------------------- | ---------------------- |
| `chef`      | `rust:slim-trixie`       | ❌ No                 | ❌ No                  |
| `tester`    | `rust:slim-trixie`       | ❌ No                 | ❌ No                  |
| `gcc`       | `debian:trixie-slim`     | ❌ No                 | ❌ No                  |
| **Runtime** | `distroless/cc-debian13` | ✅ Yes (UDP/HTTP/API) | ✅ Yes                 |

### 2. Runtime image is different

The production runtime image is `gcr.io/distroless/cc-debian13:debug` (the `runtime` stage,
near the end of the Containerfile). This is a Google distroless image based on Debian 13,
which has a much smaller attack surface (~10 packages vs ~600 in the full Rust image). Any
vulnerability scanner warnings on the runtime image would be treated as **high priority** —
but those are not present in this warning.

### 3. Upstream image trust boundary

All three flagged images are **Docker Official Images** (`library/rust`, `library/debian`).
We pull them from Docker Hub's official repository, which is the same trust boundary as
any `FROM` statement in any Dockerfile. The CVEs exist in the upstream images themselves;
they are not introduced by our Containerfile.

### 4. Supply-chain risk is acceptable

The theoretical concern is that a compromised build tool (e.g., a vulnerable `openssl` in
the build image) could produce compromised binaries. However:

- The build stages are **ephemeral** — the vulnerability would need to be actively exploited
  during the ~35-40 minute build window.
- The final runtime image is **independently verified** by E2E tests running against the
  distroless image.
- The `tester` stage runs **unit tests** on the compiled binary produced by `rust:trixie`,
  exercising the same code paths that would run in production. This provides a build-pipeline
  integrity check: unexpected test failures could indicate anomalous behaviour from a
  compromised build tool or dependency. See the Security Rationale section in the ADR
  `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md` for more detail.

## Future Actions

| Action                                                               | Cadence                | Owner |
| -------------------------------------------------------------------- | ---------------------- | ----- |
| Monitor Docker Hub for updated slim Rust and Debian images          | Quarterly              | TBD   |
| Rebuild container image and verify warning count decreases           | After upstream updates | TBD   |
| Re-evaluate if these stages become part of the runtime image         | On architecture change | TBD   |
| Check if Docker fixes these CVEs in fresh `trixie` tags              | Next quarterly review  | TBD   |

## References

- Docker Hub `rust:slim-trixie` (linux/amd64): <https://hub.docker.com/_/rust>
- Docker Hub `debian:trixie-slim` (linux/amd64): <https://hub.docker.com/_/debian>
- Consolidated build-stage scan history: `docs/security/docker/scans/build-images.md`
- ADR: Keep unit tests inside container build: `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md`

  <!-- skill-link: catalog-security-vulnerabilities -->

- Issue draft about pre-built base images: `docs/issues/drafts/1840-workflow-performance-prebuilt-base-images/ISSUE.md`

## Related GitHub Issues

| Issue                                                           | Title                                        | Relationship                                                                                                            |
| --------------------------------------------------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| [#1457](https://github.com/torrust/torrust-tracker/issues/1457) | Docker Security Overhaul EPIC                | Parent EPIC covering all Docker security improvements                                                                   |
| [#1460](https://github.com/torrust/torrust-tracker/issues/1460) | Add hadolint linter step to `container.yaml` | Related: Containerfile linting for best practices                                                                       |
| [#1463](https://github.com/torrust/torrust-tracker/issues/1463) | Consider using `rust:slim-trixie`            | Related: Also analyzes trixie CVEs; found slim-trixie has same vulns; also scanned distroless runtime (0 critical/high) |

## Changelog

| Date       | Change                                           |
| ---------- | ------------------------------------------------ |
| 2026-06-10 | Initial analysis — CVEs determined non-affecting |
| 2026-07-20 | Replaced stale bases and counts after issue #1463 |
