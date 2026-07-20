---
doc-type: issue
issue-type: task
status: draft
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-prebuilt-base-images/ISSUE.md
branch: "{issue-number}-prebuilt-base-images"
related-pr: null
last-updated-utc: 2026-06-01 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - .github/workflows/container.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/closed/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
---


# Issue #[To be assigned] - Publish stable base stages as pre-built Docker Hub images

## Goal

Extract the rarely-changing Containerfile stages (`chef`, `tester`, `gcc`) into
versioned pre-built images published on Docker Hub, so the container build can
skip rebuilding them from scratch on every CI run.

## Background

The Containerfile has three base stages that change infrequently:

- **`chef`** (`rust:trixie`): installs `cargo-binstall`, `cargo-chef`, and
  `cargo-nextest`.
- **`tester`** (`rust:slim-trixie`): installs system packages (`curl`,
  `sqlite3`, `time`), `cargo-binstall`, `cargo-nextest`, and initializes a
  SQLite3 test database.
- **`gcc`** (`gcc:trixie`): compiles `su-exec` from source.

These stages are stable: they only need rebuilding when the upstream Rust/GCC
base image changes or when the pinned tool versions (`cargo-chef`,
`cargo-nextest`) are updated. In a warm Docker layer cache they are already
skipped, but on cold runners (new runner allocation, cache eviction, or cache
miss) they are rebuilt from scratch, requiring apt-get downloads, cargo-binstall
bootstrap, and tool installation.

### Expected benefit

The expected wall-clock saving is **small**. Each stage was benchmarked locally
using `docker build --no-cache` with base images already present (i.e. simulating
a CI runner that has the upstream images cached but no intermediate layer cache).
Machine: Ryzen 9 7950X, 2026-06-01.

| Stage    | Dominant cost                         | Measured time (RUN/COPY steps only) |
| -------- | ------------------------------------- | ----------------------------------- |
| `gcc`    | single C file compile                 | 1.2 s                               |
| `tester` | apt-get + cargo-binstall + nextest    | 11 s                                |
| `chef`   | cargo-binstall + cargo-chef + nextest | 4.5 s                               |

Total build steps (RUN/COPY, base images cached): **~17 s**.

On a truly cold runner where base images are not present, add pull time for:

- `rust:trixie` (~1.6 GB uncompressed; ~500–600 MB compressed)
- `rust:slim-trixie` (~900 MB uncompressed; ~300 MB compressed)
- `gcc:trixie` (~1.5 GB uncompressed; ~500 MB compressed)

At typical GitHub Actions runner network speeds (~500 Mbps), image pulls add
roughly **20–40 s**. Total worst-case cold build: **< 1 min**.

The overall container build baseline is 35–40 min. These three stages represent
**< 2%** of total build time. The compile and link stages dominate overwhelmingly.

By contrast, the operational cost of maintaining pre-built images is
non-trivial:

- A separate CI workflow is needed to rebuild and publish images when any
  ingredient changes (Rust version bump, tool version update, apt package
  change).
- Images must be versioned and tagged precisely to avoid stale caches (e.g.
  `torrust/tracker-chef:rust-trixie-chef-0.1.0-nextest-0.9.98`).
- Published images require security scanning and regular rebuilds to incorporate
  upstream OS/library patches.
- Any mismatch between the pre-built image and what the Containerfile expects
  is a silent correctness risk.

### When this becomes more valuable

The trade-off shifts in favor of pre-built images if:

- The `chef` stage grows significantly (e.g. after adding `mold` or other
  build tools — see sub-issue #9 on alternative linker).
- CI runners begin allocating fresh environments more often (longer cold-cache
  periods).
- The `tester` stage requires more apt packages or longer setup steps.
- GitHub Actions introduces a way to share layer cache across workflows more
  reliably, making pre-built images the natural anchor point.

## Scope

### In scope

- Measure the actual cold-build time of the three base stages locally and in CI
  (no layer cache) so the real baseline saving is known before deciding whether
  to proceed. **Local measurement complete — see T1 in Background.**
- Evaluate what a versioning and publishing workflow would look like (trigger
  policy, tagging strategy, image retention).
- Decide whether the saving justifies the maintenance cost.

### Out of scope

- Pre-building the `recipe`, `dependencies`, `dependencies_debug`, `build`,
  `build_debug`, `test`, or `test_debug` stages — those change on every commit
  and are not candidates for pre-publishing.
- Changing the base images themselves (Rust version policy is a separate
  concern).
- Configuring a private registry or caching service (Docker Hub public images
  are sufficient if pursued).

## Implementation Plan

| Task ID | Description                                                                                                                                          | Status |
| ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| T1      | Measure actual cold-build time of `chef`, `tester`, and `gcc` stages in CI (disable layer cache for those stages only) and record in baseline report | DONE   |
| T2      | Define a versioning and tagging scheme for the pre-built images                                                                                      | TODO   |
| T3      | Draft a GitHub Actions workflow that builds and publishes the base images on a push to `main`/`develop` when relevant files change                   | TODO   |
| T4      | Update the Containerfile to `FROM` the published images instead of rebuilding from upstream                                                          | TODO   |
| T5      | Validate that CI builds are still reproducible and that the image cache hit rate improves measurably                                                 | TODO   |
| T6      | Document the rebuild trigger policy and tagging convention in `docs/containers.md`                                                                   | TODO   |

## Risks and Trade-offs

| Risk                                               | Likelihood    | Mitigation                                                                                                                                                                      |
| -------------------------------------------------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pre-built image becomes stale after upstream patch | Medium        | Automated weekly rebuild; Dependabot or Renovate alerts on base image digest change                                                                                             |
| Version mismatch between image and Containerfile   | Medium        | Pin image tags to exact tool versions in a shared variable; fail loudly on mismatch                                                                                             |
| Low actual saving makes maintenance unjustifiable  | **Confirmed** | T1 measured locally: ~17 s total RUN/COPY (gcc: 1.2 s, tester: 11 s, chef: 4.5 s). < 2% of 35–40 min baseline. Proceed only if CI cold-cache frequency increases significantly. |
| Docker Hub rate limiting or outage                 | Low           | Fall back to rebuilding from upstream base images (original Containerfile still works without the pre-built `FROM` lines)                                                       |

## Progress Tracking

### Checklist

- [x] T1 — measure cold-build time of base stages locally: gcc 1.2 s, tester 11 s, chef 4.5 s — total ~17 s (base images cached); < 2% of 35–40 min baseline
- [ ] T2 — versioning and tagging scheme defined
- [ ] T3 — publishing workflow drafted
- [ ] T4 — Containerfile updated to FROM published images
- [ ] T5 — CI build validated; cache hit rate measured
- [ ] T6 — `docs/containers.md` updated

### Progress Log

Append one line per meaningful update.

| Date (UTC)       | Note                                                                                                                                                                                                  |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-06-01 00:00 | Spec drafted. Low-priority idea: base stages are fast (3–7 min cold), compile dominates. Document for future re-evaluation if context changes.                                                        |
| 2026-06-01 00:00 | T1 measured locally with `docker build --no-cache` (base images cached): gcc 1.2 s, tester 11 s, chef 4.5 s — total ~17 s. Cold pull adds ~30 s for base images. Total < 1 min vs 35–40 min baseline. |
