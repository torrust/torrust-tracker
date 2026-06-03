# Keep unit tests inside the container build process

## Description

The Torrust Tracker [Containerfile](../../Containerfile) runs unit tests inside the image build
itself (via `cargo nextest archive` + `cargo nextest run` in the `test` stage). When evaluating
CI performance improvements (issue #1854), one option was to move unit tests out of the
Containerfile and run them on the GitHub Actions host after the container image was built.

This ADR records the decision to keep them, and what they actually guarantee.

## Agreement

**Unit tests continue to run inside the container build process, as one layer of a
defence-in-depth test strategy.**

The test environments involved are:

| Layer                               | Base image                        | What runs there                               |
| ----------------------------------- | --------------------------------- | --------------------------------------------- |
| `tester` stage (unit tests)         | `rust:slim-trixie`                | ~500 unit tests via `cargo nextest run`       |
| `release` stage (production binary) | `gcr.io/distroless/cc-debian13`   | only the two production binaries              |
| E2E tests in `container.yaml`       | against the final `release` image | full E2E suite against the distroless runtime |
| `unit` job in `testing.yaml`        | GHA `ubuntu-latest`               | same unit tests, plus lint/docs               |

**Important caveat:** the `tester` base image (`rust:slim-trixie`) is **not** the production
runtime (`gcr.io/distroless/cc-debian13`). The unit tests therefore do not prove that the
binary executes correctly in the production runtime environment. The distroless runtime
validation is provided exclusively by the E2E tests, which run against the final assembled
`release` image.

What the in-container unit tests do provide that the GHA host does not:

- They run the exact binary that was compiled by `rust:trixie` (same compiler, same linker,
  same `RUSTFLAGS`), extracted from the nextest archive, and verified executable before
  being copied into the final image. This catches build-pipeline failures that would not
  be detected by running a separate `cargo test` on the host.
- They use the same Debian trixie glibc as the distroless runtime image (both are
  `debian13`-based). While this is a weak guarantee compared to running in distroless
  itself, it is stronger than `ubuntu-latest` whose glibc version may diverge.
- The `ldd` + explicit `libz.so.1` copy in the `test` stage verifies the shared-library
  linkage of the extracted binary before it enters the runtime stage.

The three-layer strategy is therefore:

1. **GHA host unit tests** (`testing.yaml` `unit` job) — fast feedback on every push/PR,
   covers all branches including feature branches where the container workflow does not run.
2. **In-container unit tests** (`test` Containerfile stage) — validates the compiled binary
   in the build pipeline environment before it is promoted to the runtime image.
3. **E2E tests against the distroless `release` image** (`container.yaml` `test` job) —
   the only layer that proves the binary works in the actual production runtime.

### Alternatives Considered

**Move unit tests entirely to the GHA host and remove the `tester` stage.**
This would make the container build significantly faster (eliminating ~50 fat-LTO binary
compilations). However, it removes layer 2 above. The decision for now is to keep all three
layers. If the build time becomes unacceptable, this option can be revisited as part of the
LTO optimization work tracked in issue #1840.

### Consequences

The container build remains slow because `cargo nextest archive` compiles all test binaries
(~50 total after workspace exclusions), each linked with fat LTO. This is a separate performance
problem addressed elsewhere (see issue #1840 epic and the LTO optimization drafts).

The CI workflow is structured to avoid running the same work twice where possible
(implemented as part of issue #1854):

- Unit tests run inside the Containerfile build (unchanged).
- E2E tests run in `container.yaml` after the image is built, before any publish step.
- `testing.yaml` `docker-e2e` is skipped when `container.yaml` covers the same trigger
  (PR targeting `develop`/`main`, push to `develop`/`main`/`releases/**`).
- For feature branch pushes where `container.yaml` does not trigger, `testing.yaml`
  `docker-e2e` still runs and provides equivalent coverage.

## Date

2026-06-03

## References

- Issue #1854: [docs/issues/open/1854-1840-workflow-performance-container-test-gating/ISSUE.md](../issues/open/1854-1840-workflow-performance-container-test-gating/ISSUE.md)
- Epic #1840: [docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md](../issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md)
- [Containerfile](../../Containerfile)
- [.github/workflows/container.yaml](../../.github/workflows/container.yaml)
- [.github/workflows/testing.yaml](../../.github/workflows/testing.yaml)
