---
date-analyzed: 2026-06-10
source: Docker DX (docker-language-server) / Docker Scout
status: non-affecting
review-cadence: quarterly
image-digest: sha256:19dfb952582d0e17841fdb8cd70febfb6cb0761c4e0cd84f3cb1f07bb3281a8d
semantic-links:
  related-artifacts:
    - Containerfile
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
---

# Containerfile trixie-based image vulnerabilities

## Context

The VS Code Docker DX extension (docker-language-server) flagged vulnerabilities in the
`Containerfile` on the three `FROM` instructions that use Debian trixie-based base images.
Line numbers drift as the file changes; the stages are the stable reference:

| Line (approx.) | Image              | Stage    | Purpose                               |
| -------------- | ------------------ | -------- | ------------------------------------- |
| 6              | `rust:trixie`      | `chef`   | Install `cargo-chef`, `cargo-nextest` |
| 15             | `rust:slim-trixie` | `tester` | Run unit tests inside container build |
| 32             | `gcc:trixie`       | `gcc`    | Compile `su-exec` from source         |

## Vulnerability Summary

All three images are **upstream Docker Official Images** based on Debian trixie
(Debian 13/testing). The scanner reports CVEs in the OS-level packages shipped by
those images, not in anything we add.

| Image              | C   | H   | M   | L   | Unspecified | Total |
| ------------------ | --- | --- | --- | --- | ----------- | ----- |
| `rust:trixie`      | 4   | 26  | 27  | 178 | 27          | 262   |
| `rust:slim-trixie` | 1   | 6   | 6   | 84  | 1           | 98    |
| `gcc:trixie`       | 4   | 31  | 27  | 182 | 27          | 271   |

### Notable critical CVEs

| CVE            | CVSS | Package   |
| -------------- | ---- | --------- |
| CVE-2026-20889 | 9.8  | `libraw`  |
| CVE-2026-21413 | 9.8  | `libraw`  |
| CVE-2026-45447 | 9.8  | `openssl` |
| CVE-2026-33278 | 9.1  | `unbound` |

### Notable high-severity CVEs

| CVE            | CVSS | Package                 |
| -------------- | ---- | ----------------------- |
| CVE-2026-41142 | 8.8  | `openexr`               |
| CVE-2026-42216 | 8.8  | `openexr`               |
| CVE-2026-32740 | 8.8  | `libheif`               |
| CVE-2026-42959 | 8.7  | `unbound`               |
| CVE-2026-7383  | 8.1  | `openssl` (slim-trixie) |

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
| `chef`      | `rust:trixie`            | ❌ No                 | ❌ No                  |
| `tester`    | `rust:slim-trixie`       | ❌ No                 | ❌ No                  |
| `gcc`       | `gcc:trixie`             | ❌ No                 | ❌ No                  |
| **Runtime** | `distroless/cc-debian13` | ✅ Yes (UDP/HTTP/API) | ✅ Yes                 |

### 2. Runtime image is different

The production runtime image is `gcr.io/distroless/cc-debian13:debug` (the `runtime` stage,
near the end of the Containerfile). This is a Google distroless image based on Debian 13,
which has a much smaller attack surface (~10 packages vs ~600 in the full Rust image). Any
vulnerability scanner warnings on the runtime image would be treated as **high priority** —
but those are not present in this warning.

### 3. Upstream image trust boundary

All three flagged images are **Docker Official Images** (`library/rust`, `library/gcc`).
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
| Monitor Docker Hub for updated `rust:trixie` and `gcc:trixie` images | Quarterly              | TBD   |
| Rebuild container image and verify warning count decreases           | After upstream updates | TBD   |
| Re-evaluate if these stages become part of the runtime image         | On architecture change | TBD   |
| Check if Docker fixes these CVEs in fresh `trixie` tags              | Next quarterly review  | TBD   |

## References

- Docker Hub `rust:trixie` (linux/amd64): <https://hub.docker.com/layers/library/rust/trixie/images/sha256-19dfb952582d0e17841fdb8cd70febfb6cb0761c4e0cd84f3cb1f07bb3281a8d>
- Docker Hub `rust:slim-trixie` (linux/amd64): <https://hub.docker.com/layers/library/rust/slim-trixie/images/sha256-7be7e62dbd0954a32c340afe3df951d75dde2859549b2b72fdd4a8c842b37534>
- Docker Hub `gcc:trixie` (linux/amd64): <https://hub.docker.com/layers/library/gcc/trixie/images/sha256-74b6d3e67f73206d3474a9fd8ce21695de3816bbc52616169110460594d66c32>
- ADR: Keep unit tests inside container build: `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md`

  <!-- skill-link: create-adr -->

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
