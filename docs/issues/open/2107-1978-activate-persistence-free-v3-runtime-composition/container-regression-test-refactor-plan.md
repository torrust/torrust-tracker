---
doc-type: refactor-plan
status: deferred
related-issue: 2107
related-pr: 2112
spec-path: docs/issues/open/2107-1978-activate-persistence-free-v3-runtime-composition/container-regression-test-refactor-plan.md
last-updated-utc: 2026-08-31
semantic-links:
  skill-links:
    - write-unit-test
    - run-pre-commit-checks
  related-artifacts:
    - contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh
    - src/console/ci/e2e/
    - src/console/ci/compose.rs
    - .github/workflows/container.yaml
    - share/container/entry_script_sh
    - Containerfile
---

# Refactor Plan — Make Persistence-Transition Container Tests Maintainable and Enforced

## Goal

Replace the Bash script as the sole regression authority for persistence-transition
container behavior with readable, maintainable automated tests. Cover entrypoint
policy with fast non-Docker tests and preserve a small Rust-owned release-image
integration suite in CI, then remove the Bash script after the replacement
provides its required coverage.

Related issue: #2107

## Context and Problem

`contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh`
was valuable during implementation: it directly exercised the release image,
mounted configuration precedence, persistence-free startup, and non-destructive
SQLite target transitions. It caught behavior that unit tests and static checks
could not detect.

It is not sufficient as the long-term protection mechanism because it is manually
invoked and has no automatic test discovery. The script also combines image
building, container lifecycle management, timeout handling, filesystem fixtures,
and several acceptance scenarios in one shell flow. That is acceptable as a
short-term implementation safety net, but makes failures harder to isolate and
future behavior changes easier to miss.

The broader refactor is deliberately deferred and is not part of PR #2112. The
approved interim script readability and CI-enforcement improvement is included
in that pull request; the policy-test extraction, Rust replacement, and
Bash-script removal remain future work. Before changing the entrypoint, the
script, `Containerfile`, or the Docker workflow in future work, contributors
must review this plan and decide whether to implement its affected refactoring
items in that change.

### Interim Delivery

The current Bash regression is refactored in #2107 into named scenario helpers
and runs automatically in the Docker workflow after `torrust-tracker:local` is
built. CI passes `BUILD_IMAGE=false`, avoiding a duplicate release-image build.
This is an intentional incremental improvement, not completion of this deferred
plan: the test remains Bash and Docker-backed, while the policy-test extraction,
Rust replacement, and Bash-script removal remain future work.

The two CI regressions found during #2107 reinforce the need for enforced
release-image coverage:

1. An explicitly selected SQLite storage directory was created after recursive
   ownership setup, leaving it not writable by the runtime user.
2. The qBittorrent SQLite fixture mounted a persistence-enabled configuration
   with an empty storage root and therefore needed to create the configuration's
   SQLite parent directory itself.

## Target Test Architecture

Extract the entrypoint's configuration-selection policy from its side effects so
fast tests can validate it without Docker. The policy tests must use a temporary
directory and mocked system commands where needed; they must not need to build an
image, create a user, invoke `su-exec`, or start the tracker.

Implement a small Rust-owned release-image integration suite using the existing
container and Compose abstractions under `src/console/ci/`. It should execute
against the release image built by `.github/workflows/container.yaml` and report
scenario-specific assertion failures with retained container logs when startup
fails. Docker integration coverage remains necessary for final-image ownership,
volume, binary, health-check, and tracker-startup behavior, but it must not
repeat every entrypoint policy branch.

The test must treat the following boundaries as explicit contracts:

- The image entrypoint owns setup for a fresh image-managed configuration.
- A mounted `tracker.toml` is authoritative and must not be replaced.
- A test fixture that mounts an explicitly selected SQLite configuration owns the
  parent directory required by that configuration.
- Persistence-free startup must not create a database directory solely because
  the image starts.
- Persistence enable/disable and SQLite target changes are restart-only and
  non-destructive. No unselected database target may be altered.

The fast policy tests and automated Rust integration suite become the CI
authority. The Bash script is a temporary implementation safety net and must be
removed once the replacements provide equivalent required coverage.

## Acceptance Criteria

- [ ] Fast non-Docker tests cover configuration selection, mounted-configuration
      precedence, supported driver handling, and SQLite-storage decisions.
- [ ] A Rust container regression covers persistence-free startup and
      non-destructive SQLite transitions.
- [ ] Each scenario has a descriptive test or helper name and a focused failure
      message that identifies the violated container contract.
- [ ] The test uses the release image and the runtime user identity, including
      assertion that entrypoint-created SQLite storage is writable by that user.
- [ ] The test verifies that a mounted no-persistence configuration remains byte
      identical and no SQLite storage directory is created as a side effect.
- [ ] The test proves prior and unselected SQLite targets remain byte identical
      across disable, target-change, and original-target reuse transitions.
- [ ] The Docker workflow runs the release-image regression after the image is
      built and before publishing is eligible.
- [ ] CI does not rebuild an equivalent tracker image solely for the regression
      when `torrust-tracker:local` from the workflow build step is available.
- [ ] The Bash script is removed after the Rust test and CI workflow provide
      equivalent required coverage.
- [ ] Focused Rust tests, the Docker workflow-equivalent command, `linter all`,
      and the mandatory pre-commit gate pass.

## Refactor Items

### 1. [ ] Extract and test entrypoint policy without Docker [High impact / Medium effort]

**Problem**: The entrypoint mixes configuration-selection policy with user,
filesystem, and process-execution side effects. Testing every decision branch
therefore currently requires an expensive release-image build.

**Files**:

- `contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh`
- `share/container/entry_script_sh`

**Change**:

1. Extract the policy that selects a default configuration and decides whether
   SQLite storage is required into a side-effect-free shell unit or a small
   policy module that tests can load.
2. Use fast tests with temporary directories and mocked commands to cover:
   - mounted no-persistence configuration remains authoritative;
   - no override selects the no-persistence default;
   - each supported driver selects its intended fresh-mount configuration;
   - only explicit fresh SQLite selection requests SQLite storage;
   - unsupported drivers fail with the documented diagnostic.
3. Keep user creation, ownership, copying, and `su-exec` invocation in the
   entrypoint execution layer; test that layer only through the smaller
   release-image integration suite.

---

### 2. [ ] Implement a small Rust release-image regression [High impact / Medium effort]

**Problem**: Fast policy tests cannot prove the final distroless image has its
required binaries, correct runtime-user write permissions, volume behavior, or a tracker
that can start and become healthy.

**Files**:

- New Rust container-regression module under `src/console/ci/`
- Existing Docker helpers under `src/console/ci/e2e/`

**Change**:

1. Reuse existing Docker/container helpers rather than adding raw process or
   shell command construction to test code.
2. Introduce helpers that reveal intent, such as
   `assert_mounted_configuration_is_unchanged`,
   `assert_runtime_user_can_write_sqlite_storage`, and
   `assert_file_checksum_is_unchanged`.
3. Model a deliberately bounded tracker run as an explicit successful outcome
   rather than accepting an unexplained process exit code.
4. Keep helpers focused on one contract and avoid generic test frameworks that
   conceal container paths or configuration ownership.
5. Add unit tests for pure filesystem/checksum or configuration-generation
   helpers where that improves diagnostic quality without duplicating the
   release-image integration assertions.
6. Keep Docker scenarios limited to contracts that cannot be proven by the
   policy tests: no packaged SQLite seed, runtime-user SQLite write permission,
   persistence-free startup, and the SQLite transition contract.

---

### 3. [ ] Integrate the Rust regression into container CI [High impact / Low effort]

**Problem**: A manually run script cannot prevent future container or entrypoint
changes from reintroducing the failures it was created to detect.

**Files**:

- `.github/workflows/container.yaml`
- Rust binary or test entry point selected in item 1
- `packages/e2e-tools/` if the existing E2E runner package is the selected home

**Change**:

1. Add a clearly named Docker-workflow step after `Build Tracker Image` and
   before qBittorrent scenarios or publish-eligible work.
2. Pass the image built in the existing workflow, `torrust-tracker:local`, to
   avoid a second image build.
3. Ensure the step is covered by the workflow's failure policy and blocks the
   publish jobs through the existing `test` job dependency.
4. Keep the scenario isolated from qBittorrent transfer coverage: this test owns
   image initialization and persistence transitions, while qBittorrent tests own
   interoperability.

---

### 4. [ ] Remove the superseded Bash regression [Medium impact / Low effort]

**Problem**: Leaving a second implementation after its Rust replacement is
enforced invites behavioral drift, duplicates maintenance, and sends the wrong
signal that manual test execution is an acceptable release safeguard.

**Files**:

- `contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh`
- `contrib/dev-tools/containers/tests/README.md` if a directory index is added
- `docs/containers.md` only if user-facing procedure changes

**Change**:

1. After the Rust regression and CI step are proven, compare its assertions with
   the script line by line.
2. Confirm the Rust entry point provides a documented local invocation and
   retains sufficient failure diagnostics for container troubleshooting.
3. Delete `test-mounted-no-persistence-configuration.sh` in the same change that
   marks the Rust regression as the enforced replacement.
4. Remove references to the deleted script from #2107 documentation and update
   the final evidence to name the Rust test and CI workflow.

---

### 5. [ ] Review all test code as production-quality code [High impact / Low effort]

**Problem**: Container tests influence release safety. Generated test code must
be readable, maintainable, and reviewed with the same standards as production
runtime code.

**Files**:

- All files changed by items 1 through 4

**Change**:

1. Apply the same refactoring cycle used for production code: remove duplication,
   name behavior, isolate side effects, and preserve clear intent.
2. Review fixture ownership explicitly: image entrypoint, mounted configuration,
   and host-side test storage must each have one responsible owner.
3. Confirm every behavior introduced by #2107 has an automated test at the
   appropriate level, with release-image behavior covered by CI rather than a
   voluntary manual command.
4. Record final command evidence in this issue folder and update #2107 only if
   the implementation status or acceptance evidence changes.

## Order of Execution

| Order | Status | Item                                              | Impact | Effort |
| ----- | ------ | ------------------------------------------------- | ------ | ------ |
| 1     | [ ]    | Extract and test entrypoint policy without Docker | High   | Medium |
| 2     | [ ]    | Implement a small Rust release-image regression   | High   | Medium |
| 3     | [ ]    | Integrate regression into container CI            | High   | Low    |
| 4     | [ ]    | Remove superseded Bash regression                 | Medium | Low    |
| 5     | [ ]    | Review all test code as production-quality code   | High   | Low    |

## Validation Plan

1. Run fast policy tests without Docker.
2. Run the focused Rust release-image tests locally.
3. Run the equivalent CI command with `torrust-tracker:local` without rebuilding
   the image.
4. Confirm the existing SQLite, MySQL, PostgreSQL, and qBittorrent E2E scenarios
   continue to pass.
5. Run `linter all` and the mandatory pre-commit gate.
6. Confirm the Docker workflow executes the regression automatically on a pull
   request that changes `Containerfile`, `share/container/`, or the Rust test
   entry point.

## Non-Goals

- Do not broaden the persistence capability matrix or change v3 runtime
  composition behavior.
- Do not reintroduce unconditional SQLite directory creation in the production
  entrypoint.
- Do not make qBittorrent transfer tests responsible for all tracker image
  initialization semantics.
- Do not require contributors or AI agents to remember a manual command as the
  only protection against container regressions.

## Deferral Record

The #2107 implementation remains intentionally focused on the persistence-free
runtime and its discovered release-container defects. This plan records the
required test and entrypoint refactor without expanding the current draft PR's
scope. Implement it in a dedicated follow-up issue and pull request before
making further non-trivial changes to the linked container behavior.
