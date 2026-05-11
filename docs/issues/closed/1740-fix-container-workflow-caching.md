# Fix Container Workflow Caching

## Overview

The `container` workflow (`.github/workflows/container.yaml`) has a step-ordering bug and a
cache-scoping gap that prevent the GHA Docker layer cache from working reliably.

- GitHub issue: [#1740](https://github.com/torrust/torrust-tracker/issues/1740)
- Related workflow: [`.github/workflows/container.yaml`](../../.github/workflows/container.yaml)
- Related: [#1726 — Reduce Build Times with sccache](../open/1726-reduce-build-times-sccache/ISSUE.md)

## Background

The `test` job builds the container image with `docker/build-push-action` and uses
`cache-from: type=gha` / `cache-to: type=gha` to persist Docker layer cache between runs.
The intent is that the `cargo chef cook` layer (dependency compilation, the slow part) is
only rebuilt when `Cargo.lock` or `Cargo.toml` files change.

In practice the cache provides little benefit because of several problems described below.

## Problems

### 1. `actions/checkout` runs after the build step (bug)

The current step order in the `test` job is:

```text
setup-buildx → build-push-action → inspect → checkout → compose
```

`docker/build-push-action` resolves `./Containerfile` relative to the **workspace root**, which
is only populated after `actions/checkout`. On a cold cache the job will either fail (no
`Containerfile`) or silently use a stale checked-out tree from a previous run.

The correct order is:

```text
checkout → setup-buildx → build-push-action → inspect → compose
```

### 2. Both matrix targets share one cache namespace

The `test` job runs two targets in parallel — `debug` and `release` — and both write to the
same GHA cache scope. The two jobs race to update the cache; whichever finishes last overwrites
the other's entries. On the next run, only one target gets a warm cache.

GitHub's GHA cache is also capped at **10 GB per repository**. The debug and release Docker
layer caches for a Rust workspace of this size can easily exceed that limit together, causing
evictions.

Scoping the cache per target with `scope=${{ matrix.target }}` isolates the two caches:

```yaml
cache-from: type=gha,scope=${{ matrix.target }}
cache-to: type=gha,scope=${{ matrix.target }},mode=max
```

### 3. Final compilation step is never cached (expected limitation)

Even with the above fixes, the `cargo nextest archive` step that compiles workspace crates will
recompile on every source change. This is expected: the `cargo chef` pattern intentionally
separates dependency compilation (cached) from workspace-crate compilation (not cached). On
GitHub's shared 2-core runners this step takes ~15–25 minutes for a full Rust workspace.

Reducing that cost is tracked separately in
[#1726](../open/1726-reduce-build-times-sccache/ISSUE.md).

### 4. `docker-e2e` job in `testing.yaml` builds the image without BuildKit cache

The `docker-e2e` job in `.github/workflows/testing.yaml` also builds the tracker container
image, but it does so indirectly through two Rust binaries:

- `e2e_tests_runner` calls `Docker::build("./Containerfile", tag)` which runs plain
  `docker build -f ./Containerfile -t <tag> .`
- `qbittorrent_e2e_runner` calls `compose.build()` which runs `docker compose build`

Neither path goes through BuildKit with the GHA cache backend (`type=gha`), so the image is
always built from scratch on every run. `docker/setup-buildx-action` is not present in that
job, so the GHA cache backend is never available to the plain `docker` CLI calls.

**Proposed fix**: add an explicit pre-build step to the `docker-e2e` job using
`docker/setup-buildx-action` + `docker/build-push-action` with `cache-from/cache-to: type=gha`
before the Rust runners execute. The runners accept a `--tracker-image` flag, so they can be
pointed at the pre-built image tag instead of rebuilding it themselves. This avoids modifying
the Rust source code.

The step order would become:

```text
checkout → setup-buildx → build-tracker-image (cached) → run-e2e-tests → run-qbt-e2e-tests
```

The pre-build step produces a local image tag (e.g. `torrust-tracker:e2e-local`) that the
runners consume via `--tracker-image torrust-tracker:e2e-local`. A `--no-build` flag (or
equivalent) would need to be added to the runners, or alternatively the runners can be made
to skip their own build when the image already exists in the local daemon cache.

### 5. `.dockerignore` does not exclude non-build files, causing unnecessary cache busting

The `.dockerignore` was created in the original container overhaul and has never been updated.
It correctly excludes `target/`, `.git/`, `storage/`, `.github/`, and a handful of top-level
files, but leaves several directories and files in the build context that have no role in
compiling or testing Rust code:

| Path                                                    | Size   | Effect                                   |
| ------------------------------------------------------- | ------ | ---------------------------------------- |
| `docs/`                                                 | 3.6 MB | Any doc edit busts `COPY . /build/src`   |
| `.coverage/`                                            | 888 KB | Coverage artifacts bust the source layer |
| `integration_tests_sqlite3.db`                          | 60 KB  | Runtime DB busts the source layer        |
| `AGENTS.md`                                             | 24 KB  | AI agent instructions not needed         |
| `.githooks/`                                            | 8 KB   | Git hooks not needed at build time       |
| `codecov.yaml`, `compose.*.yaml`                        | small  | CI config not needed                     |
| `.markdownlint.json`, `.yamllint-ci.yml`, `.taplo.toml` | small  | Linter config not needed                 |
| `project-words.txt`                                     | small  | Spell-checker dictionary not needed      |

Because `COPY . /build/src` appears in the `recipe`, `build_debug`, `build`, `test_debug`, and
`test` stages, any file change in the unfiltered context invalidates those layers, triggering a
full `cargo nextest archive` recompile even when no Rust source changed.

Additionally, the existing entry `/cSpell.json` is incorrectly cased — the actual file is
`cspell.json` (lowercase) — so it is not excluded on case-sensitive Linux filesystems.

### 6. `publish_development` and `publish_release` jobs are missing `actions/checkout`

The `publish_development` and `publish_release` jobs in `container.yaml` have a worse variant
of the checkout bug from Problem 1: `actions/checkout` is **absent entirely**. The step order
in both jobs is:

```text
meta → login → setup-buildx → build-and-push
```

`docker/build-push-action` therefore cannot find `./Containerfile` on a cold runner and will
fail or use a stale workspace from a previous run.

Both publish jobs also write to the default unscoped GHA cache (`type=gha` with no `scope=`
parameter), sharing the cache namespace with the `test` matrix jobs and with each other.

### 7. All jobs share the same GHA cache namespace

Even after applying Fix 2 (scoping the `test` job by `${{ matrix.target }}`), the
`publish_development` and `publish_release` jobs still write to the default unscoped namespace.
A cache write from `publish_release` (which builds the `release` target) overwrites the entry
written by the `test` `release` matrix target, and vice versa.

Using a consistent workflow-prefixed naming scheme for every `scope=` parameter prevents all
cross-job and cross-workflow collisions:

| Job                                       | Recommended scope name      |
| ----------------------------------------- | --------------------------- |
| `container.yaml` `test` debug             | `container-debug`           |
| `container.yaml` `test` release           | `container-release`         |
| `container.yaml` `publish_development`    | `container-publish-dev`     |
| `container.yaml` `publish_release`        | `container-publish-release` |
| `testing.yaml` `docker-e2e` (after Fix 3) | `testing-docker-e2e`        |

GitHub's GHA cache is capped at **10 GB per repository**. With multiple workflows and build
targets, the cache can grow quickly. Using isolated scopes ensures that each layer cache is
retained independently and unaffected by other jobs, preventing unnecessary evictions.

## Proposed Changes

### Fix 1 — Move `checkout` to the first step

In the `test` job, move the `checkout` step before `setup-buildx`:

```yaml
steps:
  - id: checkout
    name: Checkout Repository
    uses: actions/checkout@v6

  - id: setup
    name: Setup Toolchain
    uses: docker/setup-buildx-action@v4

  - id: build
    name: Build
    uses: docker/build-push-action@v7
    with:
      file: ./Containerfile
      push: false
      load: true
      target: ${{ matrix.target }}
      tags: torrust-tracker:local
      cache-from: type=gha,scope=container-${{ matrix.target }}
      cache-to: type=gha,scope=container-${{ matrix.target }},mode=max

  - id: inspect
    name: Inspect
    run: docker image inspect torrust-tracker:local

  - id: compose
    name: Compose
    run: |
      ...
```

### Fix 2 — Scope the cache per matrix target

Replace the unscoped `cache-from`/`cache-to` entries (in all jobs that build the image) with
workflow-prefixed scoped ones:

```yaml
cache-from: type=gha,scope=container-${{ matrix.target }}
cache-to: type=gha,scope=container-${{ matrix.target }},mode=max
```

### Fix 3 — Pre-build the tracker image in `docker-e2e` using BuildKit cache

Add `docker/setup-buildx-action` and a `docker/build-push-action` pre-build step to the
`docker-e2e` job in `.github/workflows/testing.yaml`, scoped to the `release` target
(the only target needed by the E2E runners):

```yaml
- id: setup-buildx
  name: Setup Buildx
  uses: docker/setup-buildx-action@v4

- id: build-tracker-image
  name: Build Tracker Image
  uses: docker/build-push-action@v7
  with:
    file: ./Containerfile
    push: false
    load: true
    target: release
    tags: torrust-tracker:e2e-local
    cache-from: type=gha,scope=testing-docker-e2e
    cache-to: type=gha,scope=testing-docker-e2e,mode=max
```

Then pass `--tracker-image torrust-tracker:e2e-local --skip-build` to both runners. A
`--skip-build` flag must be added to `e2e_tests_runner` (which calls `Docker::build()`) and
`qbittorrent_e2e_runner` (which calls `compose.build()`) to skip their internal image builds
when the image already exists locally.

### Fix 4 — Extend `.dockerignore` to exclude non-build files

Add all paths that do not contribute to building or testing the Rust workspace:

```text
/AGENTS.md
/codecov.yaml
/compose.*.yaml
/cspell.json
/docs/
/integration_tests_sqlite3.db
/project-words.txt
/.coverage/
/.githooks/
/.markdownlint.json
/.taplo.toml
/.yamllint-ci.yml
```

Also remove the stale `/cSpell.json` entry and replace it with the correctly-cased
`/cspell.json` above.

### Fix 5 — Add `actions/checkout`, explicit target, and scoped cache to publish jobs

Add `actions/checkout` as the first step in both `publish_development` and `publish_release`,
add an explicit `target: release`, and replace the unscoped cache entries:

```yaml
steps:
  - id: checkout
    name: Checkout Repository
    uses: actions/checkout@v6

  - id: meta
    name: Docker Meta
    uses: docker/metadata-action@v6
    # ...

  - id: login
    name: Login to Docker Hub
    uses: docker/login-action@v4
    # ...

  - id: setup
    name: Setup Toolchain
    uses: docker/setup-buildx-action@v4

  - name: Build and push
    uses: docker/build-push-action@v7
    with:
      file: ./Containerfile
      push: true
      target: release
      tags: ${{ steps.meta.outputs.tags }}
      labels: ${{ steps.meta.outputs.labels }}
      cache-from: type=gha,scope=container-publish-dev
      cache-to: type=gha,scope=container-publish-dev,mode=max
```

For `publish_release`, use `scope=container-publish-release` instead to keep the caches
isolated.

### Fix 6 — Use workflow-prefixed scope names for all GHA cache entries

Update the `scope=` parameter in Fix 2 and Fix 3 to use the full workflow-prefixed names
from Problem 7, so that no two jobs in any workflow can collide:

- `test` job: `scope=container-${{ matrix.target }}` (expands to `container-debug` or
  `container-release`)
- `publish_development`: `scope=container-publish-dev`
- `publish_release`: `scope=container-publish-release`
- `docker-e2e` job: `scope=testing-docker-e2e`

## Goals

- [ ] Move `actions/checkout` to the first step in the `test` job
- [ ] Add `scope=container-${{ matrix.target }}` to `cache-from` and `cache-to` in the `test` job
- [ ] Verify that a second run on the same branch shows a cache hit for the
      `cargo chef cook` layer in the build log
- [ ] Confirm the `compose` step still works correctly after the reorder
- [ ] Add `docker/setup-buildx-action` + `docker/build-push-action` pre-build step to the
      `docker-e2e` job with `scope=testing-docker-e2e` GHA cache
- [ ] Add `--skip-build` flag to `e2e_tests_runner` and `qbittorrent_e2e_runner` so the
      pre-built image is used instead of rebuilding
- [ ] Pass `--tracker-image torrust-tracker:e2e-local --skip-build` to all three
      `qbittorrent_e2e_runner` invocations in `docker-e2e`
- [ ] Verify that the build logs show cache hits for layers by reviewing the workflow execution
      in the GitHub Actions tab after rerunning the jobs
- [ ] Update `.dockerignore` to exclude non-build files (`docs/`, `.coverage/`, compose
      files, linter configs, `AGENTS.md`, `integration_tests_sqlite3.db`, etc.) and fix the
      stale `/cSpell.json` entry (wrong case; actual file is `cspell.json`)
- [ ] Add inline comments to the two non-obvious Containerfile patterns discovered from git
      history:
  - The `cargo nextest archive ... ; rm -f /build/temp.tar.zst` line in
    `dependencies_debug` and `dependencies` — explain that it is a deliberate pre-linking
    warm-up step: running the linker during the cached dep layer means the subsequent
    `build` stage link step is shorter on a cache hit; it is not a mistake or leftover.
  - The `COPY ./share/ ...` + `sqlite3 ... "VACUUM;"` block in `tester` — explain that the
    default SQLite database must be initialized in the base image because tests depend on it
    at runtime, so it cannot be deferred to the `test`/`test_debug` stages.
- [ ] Add `actions/checkout` as the first step in `publish_development` and `publish_release`
- [ ] Add `target: release`, `cache-from: type=gha,scope=container-publish-dev` and
      `cache-to: type=gha,scope=container-publish-dev` to `publish_development`; use
      `container-publish-release` scope for `publish_release`
- [ ] Use workflow-prefixed scope names throughout all jobs: `container-debug`,
      `container-release`, `container-publish-dev`, `container-publish-release`,
      `testing-docker-e2e`
- [ ] Verify both publish jobs build and push successfully after the checkout and scope fixes

## References

- `docker/build-push-action` caching docs:
  <https://docs.docker.com/build/ci/github-actions/cache/>
- GHA cache backend for BuildKit:
  <https://github.com/moby/buildkit?tab=readme-ov-file#github-actions-cache-experimental>
- `cargo-chef` repository: <https://github.com/LukeMathWalker/cargo-chef>
- `docker/setup-buildx-action`: <https://github.com/docker/setup-buildx-action>
- Related workflow: [`.github/workflows/testing.yaml`](../../.github/workflows/testing.yaml)
