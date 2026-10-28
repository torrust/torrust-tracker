---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2482
github-issue: 2670
spec-path: docs/issues/closed/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md
branch: "2670-2482-build-shipped-artifacts-without-test-features"
related-pr: 2676
last-updated-utc: "2026-10-28 09:37"
semantic-links:
  skill-links:
    - create-issue
    - implement-workflow
    - add-workspace-member
    - write-unit-test
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/closed/2618-2482-decide-test-support-rules/manual-verification-evidence.md
    - Containerfile
    - .dockerignore
    - .github/workflows/testing.yaml
    - .github/workflows/deployment.yaml
    - .github/workflows/deployment-packages.yaml
    - .github/workflows/container.yaml
    - tests/common/native_tracker/command.rs
    - docs/containers.md
---

<!-- skill-link: create-issue -->

# Issue #2670 - Build the shipped container binary without test features

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 9 of its
Subissues table. Draft written by subissue 1 (#2618) after its pre-PR review and the maintainer's
decision of 2026-10-22; refreshed and its design questions settled with the maintainer on
2026-10-27. Execution order: after row 8 and before row 3, the first move that adds a `testing`
feature.

## Goal

Make every build that produces a shipped artifact (the container image's binaries first) enable
only the features meant for production, by building it on its own with explicit features, and fail
CI if a `testing` feature ever reaches it. This keeps the guarantee of
[rule A.5 of the ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
true for the main shipped artifact.

## Background

### Why a `testing` feature can reach the shipped binary

The ADR chose to share test support through a `testing` feature that is off by default (option O1),
so test support code lives in production crates and is compiled only when something enables the
feature. Cargo enables it through three routes: `--all-features`, `--tests` / `--all-targets` (which
activate the features that dev-dependencies request, evidence V1 and V4c of #2618), and feature
unification inside one build. So the feature stays out of a binary only if the binary is built by
an invocation that uses none of those routes.

### Current state (verified on `develop` at 72d0bca6a, 2026-10-27)

The release image gets `torrust-tracker` and `http_health_check` from the extracted test archive:
the `Containerfile` stage "Build Archive (release)" runs
`cargo nextest archive --tests --all-features ... --release`, and the `test` stage copies
`target/release/torrust-tracker` and `target/release/http_health_check` out of it into `/app/bin`.
The debug image does the same with `target/debug/torrust-tracker` from the `test_debug` stage. The
archive is built over the root `[workspace] default-members` (the root crate, the four server
packages, `configuration`, `events`, `http-core` and other packages; the list is in `Cargo.toml`).

Before row 3 nothing changes in the binary, because no `testing` feature exists yet (the root crate
has no `[features]` table). `--all-features` is not a no-op today: it enables `tracker-core`'s
`db-compatibility-tests` feature, which compiles the MySQL and PostgreSQL driver test modules into
the test archive. That is the reason this subissue keeps `--all-features` in the test archives
(decision D5).

The in-container tests are mostly harness executables that link the library crates, not the shipped
binary. The exception is the root crate's native-tracker tests (`tests/lifecycle/`,
`tests/common/native_tracker/`), which start the real `torrust-tracker` executable through
`tracker_binary()` in `tests/common/native_tracker/command.rs`. That function reads
`NEXTEST_BIN_EXE_torrust-tracker` first; nextest sets that variable itself, so it cannot carry an
override. ADR 20260603 records that the in-container tests run "the exact binary" that ships; this
subissue narrows that statement (decision D2).

### Audit of the commands that build or could build shipped artifacts

| Command | File | Test flags | Produces a shipped artifact |
| --- | --- | --- | --- |
| `cargo chef cook --tests --workspace --all-features` (debug and release, third-party and full recipe) | `Containerfile` | `--tests`, `--all-features` | No: warms the cache for the test archive |
| `cargo nextest archive --tests --all-features` (discarded warm-up, debug and release) | `Containerfile` | `--tests`, `--all-features` | No |
| `cargo nextest archive --tests --all-features --release` ("Build Archive (release)"), then `cp -l` of `torrust-tracker` and `http_health_check` from its extraction | `Containerfile` | `--tests`, `--all-features` | **Yes**: the release image |
| `cargo nextest archive --tests --all-features` ("Build Archive (debug)"), then `cp -l` of `torrust-tracker` | `Containerfile` | `--tests`, `--all-features` | **Yes**: the debug image |
| `docker/build-push-action` with `target: release` | `container.yaml`, `testing.yaml` (docker-e2e) | none of its own | Inherits the `Containerfile` (published by `container.yaml`) |
| `docker build -f Containerfile .` | `security-scan.yaml`, `contrib/dev-tools/containers/docker-build.sh` | none of its own | Inherits the `Containerfile` |
| `cargo test --tests --benches --examples --workspace --all-targets --all-features` | `deployment.yaml` | all | No: tests before publishing |
| `cargo publish -p torrust-tracker` | `deployment.yaml` | none | Publishes source; its verification build uses default features, so the shipped crate is not a feature-enabled binary |
| `cargo test -p <crate> --all-targets --all-features`; `cargo publish -p <crate>` | `deployment-packages.yaml` | `--all-targets`, `--all-features` | No (a test run, and source publication) |
| `cargo build --verbose` | `os-compatibility.yaml` | none | No: a compatibility check |

So the only shipped binaries built with test flags are the two in the `Containerfile` archives.
Task T1 re-verifies this table when the implementation starts.

## Scope

### In Scope

- A shipped-build path in the `Containerfile`, for the release and the debug images: a cook layer and
  a build stage that run `cargo build` for the shipped binaries (`torrust-tracker` and, in the
  release image, `http_health_check`) without `--tests`, `--all-targets`, `--examples`, `--benches`
  or `--all-features`, naming the features explicitly (none today). The image's `/app/bin` takes its
  binaries from that stage.
- The container `test` and `test_debug` stages keep running the nextest archive, and run the
  root crate's native-tracker tests against the shipped binary through a new override variable read
  by `tracker_binary()`.
- A new Rust dev-tool crate, a check that fails when a workspace package's `testing` feature is
  enabled in a shipped package's normal and build dependency tree, run as a job in `testing.yaml`.
- An amendment note in ADR 20260603 and a short description of the builds, the override variable
  and the check in `docs/containers.md`.
- Manual verification evidence, including red and green runs of the check and before and after
  build times.

### Out of Scope

- Moving any test support code (rows 2 to 7) and the coupling tool (row 8).
- Replacing `--all-features` in the test archives by explicit features (decision D5); a follow-up
  after rows 3 to 7 when the feature names are known.
- Changing what the container tests verify, the runtime image layout, or the E2E suite.
- Checking third-party crates' features named `testing` (decision D3).
- Test-only workflows that use `--all-features` on purpose (`testing.yaml` unit job, coverage,
  `deployment*.yaml` test steps): they build and run tests, not shipped artifacts.

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rule A.5: a shipped artifact is built by an invocation of its own, with explicit features and a
  check that no `testing` feature is enabled) and
  [`20260603000000`](../../../adrs/20260603000000_keep_unit_tests_inside_container_build.md) (unit
  tests stay in the container build; amended by D2).
- ADRs to create: None. ADR 20260603 gets an amendment note (T6) because its "exact binary"
  statement narrows.

### Decisions settled with the maintainer (2026-10-27)

#### D1 - How the shipped binary is built

| Option | Strengths | Weaknesses | Result |
| --- | --- | --- | --- |
| Ship the binary built with the test features (today's shape) | One build; nothing changes; fat LTO removes most unreachable code | Extra crates (`mockall`, test environment dependencies) enter `cargo audit`, Trivy and the SBOM; enabling `testing` can also enable features of shared third-party crates and change real behaviour; contradicts ADR rule A.5 | Rejected |
| Two builds: a test archive with the features and a shipped build without them | Exactly the production feature set ships; the check can prove it | Build time grows; the tests no longer run the exact shipped library code | **Chosen** |
| Keep the archive for shipping and add only the check | Smallest change | Rule A.5 requires an invocation of its own; the check would fail as soon as row 3 lands | Rejected |
| Move test support into a separate test-only crate | The leak cannot happen: binaries do not link dev-dependencies | Decided against by the ADR (dev-dependency cycles, type identity, public API of items now `pub(crate)`, versioning of one more crate) | Not revisited here |

#### D2 - Keeping what the tests verify close to what ships

Building twice means the in-container unit tests run a slightly different build of the library
code. Three measures narrow the gap:

1. The E2E suite in `container.yaml` runs against the final `release` image, so the shipped binary
   in the distroless runtime keeps its coverage. Unchanged.
2. `tracker_binary()` reads a new variable, `TORRUST_TRACKER_BINARY`, before
   `NEXTEST_BIN_EXE_torrust-tracker`. The `test` and `test_debug` stages set it to the shipped
   binary, so the black-box native-tracker tests exercise exactly what ships, in the build
   environment. Everywhere else the variable is unset and behaviour is unchanged.
3. Both builds use the same image, compiler, profile and `RUSTFLAGS`; the check proves the feature
   set is the only difference.

The `test` stage still gates the image: no `test` stage, no `release` stage.

#### D3 - How the check is implemented

| Option | Strengths | Weaknesses | Result |
| --- | --- | --- | --- |
| Shell over `cargo tree` | Small | Parsing text in shell is hard to unit test; the repository policy sends non-trivial logic to Rust | Rejected |
| Rust crate over `cargo metadata` | Structured JSON | **Not accurate**: `cargo metadata` unifies features across the whole workspace and counts dev-dependency requests. On the scratch workspace of V1 (package `b` depends on `a` and dev-depends on `a` with `features = ["testing"]`) it reports `testing` on `a`, while `cargo build -p b` does not enable it (see "Design evidence") | Rejected |
| Rust crate over `cargo tree -p <package> -e normal,build -f "{p}\|{f}"` | Resolves features for the selected package with dev-dependencies excluded, the way `cargo build -p` does; testable with fixtures built from real output | Parses text; depends on `cargo tree`'s output format | **Chosen** |
| `cargo build --unit-graph` | Exact | Needs nightly (`-Z unstable-options`) | Rejected |

The maintainer chose "a Rust dev-tool crate over `cargo metadata`" on 2026-10-27 before the design
evidence above existed; the evidence shows `cargo metadata` cannot answer the question, so the
crate keeps the maintainer's choice of a Rust dev-tool and uses `cargo tree` as its input. The
spec approval confirms that refinement.

What the check flags: any enabled feature named `testing` (configurable, default `testing`) on a
**workspace package** (a package whose `cargo tree` line shows a local path) in the normal and
build dependency tree of each shipped package. Third-party crates are not checked: a `testing`
feature of a third-party crate is outside this repository's rules, and `cargo deny` and
`cargo audit` cover third-party risk.

The check passes vacuously today, because no `testing` feature exists. That is why it has its own
negative test on a scratch fixture (real crates cannot get a `testing` feature in this row): an
automated test builds a scratch workspace in a temporary directory with path-only dependencies,
runs the real `cargo tree`, and expects failure when the shipped package enables the feature
through a normal dependency, and success when only a dev-dependency does.

#### D4 - Where the check runs

| Option | Strengths | Weaknesses | Result |
| --- | --- | --- | --- |
| Host job in `testing.yaml` | Fast, no image build, runs on every pull request, fits the `testing` group of `ci-gate`, follows the `layer-bans` and `documented-clippy-allows` jobs | The job and the `Containerfile` must name the same package and feature flags | **Chosen** |
| Inside the `Containerfile` build | Checks the real build | Slower; the checker must be built in the image | Rejected |
| Both | Strongest | Duplicate maintenance | Rejected |

The drift risk is mitigated by comments in the `Containerfile` and the workflow that name each
other, and by `docs/containers.md`.

#### D5 - Test archive features

The test archives keep `--all-features` for now. A.5 limits `--all-features` for shipped artifacts;
test builds may keep it, and replacing it now would change which test modules compile in the image
(`db-compatibility-tests`, above), which needs its own analysis. A follow-up after rows 3 to 7
names the features explicitly "where practical".

#### D6 - Cache layers

The shipped build gets its own cook layers (`cargo chef cook --release -p torrust-tracker`, and the
debug equivalent, without `--tests` or `--all-features`), stacked on the existing cook layers of
the same profile so that third-party artifacts whose features are identical are reused and only
the differing ones are rebuilt. T5 measures cold and warm builds against a baseline taken from
`develop` before the change. If stacking invalidates caches unexpectedly, the fallback is an
independent chain from the `chef` stage, which BuildKit builds in parallel.

## Design and Ownership Review

The `Containerfile` owns how shipped binaries are built; the root crate's manifest owns which
features they enable (none today). The new check crate under `contrib/dev-tools/checks/` owns the
rule "no workspace `testing` feature in a shipped binary". It is a workspace member but not a
default member, so the container tests do not build it; `.dockerignore` must admit it (interim
list of dev-tool crates) and `add-workspace-member` governs the registration. No process or I/O
ownership changes. After the first working image build, review the cache layering and the check
before the workflow is touched.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

The check and the override are new logic and get tests first (`write-unit-test`): unit tests for the
`cargo tree` parser from fixtures built from real output, an automated scratch-workspace test for
the red and green cases, and unit tests for the binary resolution order. The container build and
its test stage are the regression suite for everything else.

Test development follows the progressive loop of the `write-unit-test` and `create-issue` skills:

- Each test increment is the smallest behaviour-focused one (a resolution-order case; a parser
  case; the red or green scratch-workspace case).
- After an increment passes its focused validation, and before the next increment or its commit, the
  author performs and records a test-design review: the test exposes the one causal initial-state
  difference, its fixture owns only incidental mechanics, and the production Act and the
  independently specified expected result stay visible. The review uses the prose-first
  Arrange-Act-Assert comparison: write temporary prose for each section, refactor until the code
  expresses it, remove redundant prose, and record the result in
  `manual-verification-evidence.md`.
- The review is completed for every test-producing task (T2 and T3) before the next one starts.
- After the final test increment of each test-producing task (T2, then T3), work stops for the
  maintainer's review before final verification, the commit that lands the task, or the
  implementation pull request.
- The independent Task Reviewer makes the final pre-PR review of the completed issue, not a review
  of each subtask.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Re-verify the audit table against `develop`; record a baseline: the list and count of the tests the image's `test` stage runs, and cold and warm build times | Table and baseline in the evidence (V0, V1); the list lets the later comparison show that every baseline test is still present |
| T2 | DONE | Add `TORRUST_TRACKER_BINARY` to `tracker_binary()`, test first | Unit tests for the resolution order; default behaviour unchanged |
| T3 | DONE | Create the check crate, test first | Parser tests from real `cargo tree` output; scratch-workspace red and green tests; registered as a workspace member and admitted in `.dockerignore`; `cargo machete` and `cargo deny` pass |
| T4 | IN_PROGRESS | Add the check as a job in `testing.yaml` | Job runs the check for `torrust-tracker`; follows `implement-workflow` |
| T5 | DONE | Add the shipped cook and build stages for the release and debug images; point `test` and `test_debug` at the shipped binary; take `/app/bin` from it | Both targets build; every baseline test is still present and passing, and the delta for new tests (T2 adds some, compiled into the targets that include `tests/common/native_tracker`) is recorded; times recorded |
| T6 | DONE | Amend ADR 20260603; update `docs/containers.md` | Docs describe the two builds, the override variable and the check |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Audit and baseline evidence | One commit. |
| T2 | Override and its tests | Commit after the unit tests pass, the test-design review is recorded and the maintainer has reviewed T2's test increments. |
| T3 | The check crate, its registration and tests | Commit after the red and green tests behave as expected, the test-design review is recorded and the maintainer has reviewed T3's test increments. |
| T4 | Workflow job | Commit after the job passes in the fork or the pull request. |
| T5 | `Containerfile` stages | Commit after a local image build succeeds for both targets. |
| T6 | Docs and ADR note | One commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/closed/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer (approved by the maintainer through `ask_user` and chat on 2026-10-27, including the `cargo tree` refinement of D3)
- [x] GitHub issue created, linked as a sub-issue of #2482 (`parent_issue_url` verified), and its number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (PR #2671)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all` through the pre-commit gate on every commit, `cargo test` of the check crate and the native-tracker targets, and the full pre-push suite on 2026-10-27)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md` (M1 to M6)
- [x] Acceptance criteria reviewed after implementation and updated with evidence (AC1 to AC4 and the final verification criterion)
- [x] Evidence-based implementation completion review recorded (retrospective created)
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/` (issue closed by PR #2676; archived by the pull request that carries this entry)

### Progress Log

- 2026-10-22 17:20 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted after the
  pre-PR review of #2618 found that the release image's binary is built by
  `cargo nextest archive --tests --all-features --release`, and after the maintainer's decision of
  2026-10-22 to build shipped artifacts with explicit features and fail CI on an unintended one.
- 2026-10-27 08:35 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - Refreshed against
  `develop` at 72d0bca6a. The maintainer settled the design in `ask_user` rounds: two builds with
  the native-tracker tests run against the shipped binary (D1, D2), a Rust check in a host job
  (D3, D4), `--all-features` kept in the test archives (D5) and a dedicated shipped cook (D6). A
  scratch workspace showed `cargo metadata` cannot be the check's input, so D3 uses `cargo tree`;
  the maintainer confirms that refinement at spec approval.
- 2026-10-27 08:43 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - The maintainer
  approved the spec, including the `cargo tree` refinement of D3. Created issue #2670, linked it as
  a sub-issue of #2482 (`parent_issue_url` verified) and moved the spec to `docs/issues/open/`.
- 2026-10-27 09:04 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - PR #2671 review round 1, F1: the Regression Test Strategy and the Commit Points now require the recorded prose-first Arrange-Act-Assert test-design review after each test increment and the maintainer-review stop after the final increment of T2 and of T3.
- 2026-10-27 09:05 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - PR #2671 review round 1, F2: T1 now records the list and count of the baseline tests, and T5, AC2 and M2 require every baseline test to remain present and passing with the delta for new tests recorded, instead of an equal count (T2's tests are compiled into the targets that include `tests/common/native_tracker`).
- 2026-10-27 09:06 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - PR #2671 review round 1, F3: scenario M1 now builds both targets with `--file Containerfile`, as `docs/containers.md` does, because the repository root has no `Dockerfile`.
- 2026-10-27 12:43 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - Started the implementation. T1 is done: the audit table still holds on `develop` after PR #2671 (V0), and the baseline is 1531 tests in 40 binaries with a cold `release` build of 4 min 59 s, a cold `debug` build of 1 min 55 s and a warm `release` rebuild of 2 min 53 s (V1, list in `baseline-test-list.txt`).
- 2026-10-27 12:44 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - T2 is done: `tracker_binary()` reads `TORRUST_TRACKER_BINARY` before the nextest and Cargo variables, with three unit tests (red with the override discarded, green on the `lifecycle-signals` and `cli-configuration` targets). The test-design review is recorded in the evidence; the maintainer (josecelano) reviewed the increment and approved it through `ask_user` before the commit.
- 2026-10-27 14:55 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - T3 is done: the `shipped-features-check` crate parses `cargo tree -e normal,build` output and fails on a workspace `testing` feature in a shipped package's tree (9 parser tests and 7 scratch-workspace tests, each red before green, two mutations for the negative tests). The real workspace passes vacuously (M4) and the scratch fixture fails and passes as it should (M5). The crate is a workspace member but not a default member, is admitted in `.dockerignore`, and is listed in the `AGENTS.md` catalogue and the CLI output contract ADR. The maintainer (josecelano) reviewed the increments and approved them through `ask_user` before the commit.
- 2026-10-27 14:57 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - Added the `Shipped Features` job to `testing.yaml` (T4), which runs `cargo run --package shipped-features-check -- --package torrust-tracker`; `linter yaml` and `git diff --check` pass and the same command passes locally (V4). The spec's commit point asks for the job to pass in a fork or pull request first, but the implementation pull request is the first hosted run, so T4 stays IN_PROGRESS and M6 stays TODO until that run is recorded.
- 2026-10-27 20:13 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - T5 is done: the `Containerfile` has shipped cook and build stages for the release and debug images (`cargo build` without test flags or features), the `test` stages run the native-tracker tests against the shipped binary through `TORRUST_TRACKER_BINARY`, and `/app/bin` takes the shipped binaries. Both targets build; 0 of the 1531 baseline tests are missing and 6 were added (T2's tests); cold builds cost +17 s (release) and +18 s (debug) and the warm release rebuild +8 s (V2). The override is honoured: pointing it at a missing file fails the native-tracker tests (V3). An interrupted warm run and a fully cached rerun were discarded and the warm run repeated with a different change.
- 2026-10-27 20:14 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - T6 is done: `docs/containers.md` describes the test build, the shipped build, `TORRUST_TRACKER_BINARY` and the check, and ADR 20260603 carries an amendment note that narrows its "exact binary" claim. Ticked AC1, AC2 and AC4 (validated by the AI assistant against V0 to V3). AC3 stays open until the `Shipped Features` job has run on the implementation pull request (M6).
- 2026-10-27 21:00 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - The independent Task Reviewer's pre-PR review (PASS WITH FINDINGS, no blocking defect) led to three fixes: the check now fails when `cargo tree` output does not list the shipped package as a workspace package, it reads a checkout path that contains the separator, and an empty `TORRUST_TRACKER_BINARY` is ignored; the maintainer approved the test increments through `ask_user`. Rebased onto `develop`, rebuilt the final image (1539 tests, binaries identical to the `build_shipped` stage, healthy; V2b) and ran the full pre-push suite. Wrote the implementation completion review and the retrospective. M6 and AC3 stay open until the `Shipped Features` job has run on the pull request.
- 2026-10-28 07:07 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - Opened PR #2676. The `Shipped Features` job ran green on a hosted runner (M6, V6), so AC3 and M6 are done and the status is `in-review`. Copilot's review found that a relative `TORRUST_TRACKER_BINARY` broke the configuration test that changes the child's working directory (F1 of the audit of PR #2676): reproduced (V7) and fixed by anchoring a relative path at the test working directory, which also fixes the example of `docs/containers.md`. The container workflow criterion stays open until that job is green.
- 2026-10-28 09:37 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 9) - PR #2676 merged on 2026-10-28 (merge commit `09263a46c`) and the maintainer asked for the archive: moved the spec to `docs/issues/closed/`, set the status to `done`, repaired the references, and set row 9 of the #2482 Subissues table to DONE.

## Acceptance Criteria

- [x] AC1: Every command that produces a shipped artifact is listed with its flags (the audit
      table), and none of the shipped-artifact builds uses `--all-features`, `--all-targets`,
      `--tests`, `--examples` or `--benches`.
- [x] AC2: The image's shipped binaries are built by `cargo build` invocations of their own with
      explicit features; the `test` and `test_debug` stages still run the nextest archive, every
      baseline test (T1) is still present and passing, the delta for new tests is recorded, and
      the native-tracker tests run against the shipped binary.
- [x] AC3: The check fails when a workspace `testing` feature is enabled in a shipped package's
      normal or build dependency tree (shown by an automated scratch-workspace test), passes when
      only a dev-dependency enables it, and passes on the real workspace; the `testing.yaml` job
      runs it.
- [x] AC4: `docs/containers.md` describes the builds, the `TORRUST_TRACKER_BINARY` variable and the
      check; ADR 20260603 carries its amendment note.
- [x] `linter all` exits with code `0`; pre-push checks pass; the container workflow is green (pre-commit gate on every commit, pre-push on 2026-10-27, and `Test (Docker) (release)` green on the merged head of PR #2676; validated by the AI assistant).

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies, except that this subissue moves no test code. The workflow changes follow the
`implement-workflow` skill.

### Automatic Checks

- `linter all`, `hadolint` on the `Containerfile` (part of pre-commit), `cargo machete`,
  `cargo deny check bans`, pre-push checks, the unit and scratch-workspace tests of T2 and T3, and
  the container and testing workflows on the pull request.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Baseline before the change | Build the `release` and `debug` targets of `develop`'s `Containerfile` (`docker build --target release --tag torrust-tracker:release --file Containerfile .` and the same with `--target debug --tag torrust-tracker:debug`) and record the `test` stage's test list and count and the cold and warm times | Numbers recorded | DONE | `manual-verification-evidence.md` V1 |
| M2 | The image builds and its tests run | Build both targets after the change | Both build; every baseline test is still present and passing; the delta for new tests is recorded (the count may rise by the tests T2 adds); times recorded | DONE | V2 |
| M3 | The override is honoured | In the `test` stage, point `TORRUST_TRACKER_BINARY` at a missing file, build, then restore it | The native-tracker tests fail with the missing path (red), then pass (green) | DONE | V3 |
| M4 | The shipped binary has no `testing` feature | `cargo run -p shipped-features-check -- --package torrust-tracker` and `cargo tree -p torrust-tracker -e normal,build -f "{p} {f}"` | Passes; no `testing` feature (vacuous today, since none exists) | DONE | V4 |
| M5 | The check fails when it should | Run the scratch-workspace red case (a shipped package enabling a `testing` feature through a normal dependency) | The check fails and names the package and feature | DONE | V5 |
| M6 | The workflow job runs | Open the pull request and read the `testing.yaml` job result | The job is green on the real workspace | DONE | V6 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | V0 (audit) and V2 (build log); validated by the AI assistant |
| AC2   | DONE                   | V2 (builds, 0 of 1531 baseline tests missing, 6 added) and V3 (override); validated by the AI assistant |
| AC3   | DONE                   | V4, V5 and V6 (hosted run); validated by the AI assistant |
| AC4   | DONE                   | `docs/containers.md` and the ADR 20260603 amendment; validated by the AI assistant |

## Design evidence

Scratch workspace (members `a` and `b`; `a` has `[features] testing = []`; `b` depends on `a` and
dev-depends on `a` with `features = ["testing"]`), Cargo 1.99.1, run on 2026-10-27 under `/tmp`
and removed afterwards:

```text
$ cargo tree -p b -e normal,build -f '{p}|{f}' --prefix none
b v0.1.0 (/tmp/r9fx/b)|
a v0.1.0 (/tmp/r9fx/a)|

$ cargo tree -p b -f '{p}|{f}' --prefix none
b v0.1.0 (/tmp/r9fx/b)|
a v0.1.0 (/tmp/r9fx/a)|testing
a v0.1.0 (/tmp/r9fx/a)|testing

$ cargo metadata --format-version 1   (features of the resolved node for `a`)
['testing']
```

`cargo tree` with the same edges as the shipped build shows `a` without `testing`; `cargo metadata`
reports `testing`, so it would flag a build that does not have the feature.

## Risks and Trade-offs

- Building the shipped binary separately lengthens a cold container build. Mitigation: stacked
  cook layers (D6), a measured baseline, and the recorded result.
- The shipped build and the tests can drift apart: the in-container unit tests run the library
  code built with test features. Mitigation: D2 (shipped binary under the black-box tests, E2E
  against the release image).
- The check passes vacuously until row 3. Mitigation: the scratch-workspace red test proves it can
  fail.
- The `testing.yaml` job and the `Containerfile` could name different packages or flags.
  Mitigation: comments that name each other, and `docs/containers.md`.
- The check parses `cargo tree` output, whose format is not a stable API. Mitigation: fixtures from
  real output, the scratch-workspace test running the real `cargo tree` on every CI run.
- Rows 3 to 7 assume this row landed first. Mitigation: the execution order puts it before row 3.

## Implementation Completion Review

Reviewed by the AI assistant against the acceptance criteria and the evidence on 2026-10-27, after an
independent Task Reviewer pre-PR review (evidence, "Pre-PR review"); the maintainer reviews it in the pull
request.

- Scope: the change is in `Containerfile` (shipped cook and build stages, test stages), `tests/common/native_tracker/`
  (the override), the new `contrib/dev-tools/checks/shipped-features-check` crate (registered in `Cargo.toml` and
  `.dockerignore`, listed in `AGENTS.md` and the CLI output contract ADR), the `Shipped Features` job of
  `testing.yaml`, `docs/containers.md` and the amendment note of ADR 20260603. No test support code moved, no
  `testing` feature was added and the test archives keep `--all-features` (D5).
- Design: the shipped binaries come from `build_shipped` stages stacked on the third-party test cooks (D6); the
  measured cost is +17 s (release) and +18 s (debug) cold and +8 s on a warm rebuild. The check reads
  `cargo tree -e normal,build` because `cargo metadata` counts dev-dependency features (D3). It follows the global
  CLI output contract.
- Known limits: the check covers workspace packages only (third-party features are outside it) and only the
  features `cargo build -p torrust-tracker` enables on the host, so target-specific dependencies of another
  platform are not seen. The job names the shipped package, and the `Containerfile` names it too; only two comments
  keep them in step (D4, review finding 6). The shipped build names no feature today. The `Shipped Features` job
  has run green on the pull request (M6, V6) and the check passes vacuously on the real workspace because no
  `testing` feature exists yet; the scratch tests prove it fails when one reaches a shipped package.
- Retrospective: created, [`implementation-retrospective.md`](implementation-retrospective.md): `cargo metadata`
  proved unusable for the check, and the independent review found that the check could pass on output it could not
  read.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADRs above
- Verification evidence V1 and V4c: [`manual-verification-evidence.md`](../../closed/2618-2482-decide-test-support-rules/manual-verification-evidence.md)
