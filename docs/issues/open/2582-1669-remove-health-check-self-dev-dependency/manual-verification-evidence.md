---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md
last-updated-utc: 2026-10-21 09:55
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

Relevant output is embedded verbatim below; absolute local paths are shortened to `<repo>`.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-20 17:34 to 17:35
- Artifact under test: `develop` at merge commit `5b54bb0f4` (Merge torrust/torrust-tracker#2577)
- Operating system / environment: Linux; stable Rust toolchain, `rustc 1.99.0 (b940084d7 2026-09-28)`,
  `cargo 1.99.0 (5f94df478 2026-08-27)`; crates.io index reachable.
- Prerequisites and setup performed: clean worktree (`git status --short` listed only the
  untracked draft folder of the publish tracking issue, outside every package).

## Verification Processes

### V1 - Reproduce: the workspace cannot be packaged

- Goal: observe the failure a maintainer meets when preparing the first publish.
- Initial state: unmodified tree at `5b54bb0f4`.
- Status: `DONE` (outcome: **Reproduced**)

#### Steps Performed

1. 17:34:24 `cargo package --workspace --no-verify` (stable Rust toolchain); ended 17:34:46.

#### Observed Result

The command exited `101` after packaging 25 members in dependency order (25 `Packaged` lines):

```text
   Packaging torrust-tracker-axum-rest-api-server v0.1.0 (<repo>/packages/axum-rest-api-server)
    Updating crates.io index
    Packaged 53 files, 339.1KiB (74.0KiB compressed)
   Packaging torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)
    Updating crates.io index
error: failed to prepare local package for uploading

Caused by:
  no matching package named `torrust-tracker-axum-health-check-api-server` found
  location searched: crates.io index
  required by package `torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)`
```

#### Conclusion

Reproduced. `packages/axum-health-check-api-server/Cargo.toml` lists the crate itself in
`[dev-dependencies]` with `version = "0.1.0"`. `cargo package` resolves versioned
dev-dependencies against crates.io plus the crates it has already packaged in this run, and a crate
is never packaged before itself, so the requirement cannot be met. `cargo publish` of the crate
would fail the same way.

### V2 - Feasibility probe: the crate without the self dev-dependency

- Goal: check, before planning the fix, that removing the line is enough and breaks no test.
- Initial state: same tree; the line is removed temporarily and restored after the probe.
- Status: `DONE`

#### Steps Performed

1. 17:34:46 `sed -i '/^torrust-tracker-axum-health-check-api-server = /d' packages/axum-health-check-api-server/Cargo.toml`
   (`git diff --stat`: 1 file changed, 1 deletion).
2. `cargo test -p torrust-tracker-axum-health-check-api-server` (stable Rust toolchain); exited `0`
   at 17:34:50.
3. `cargo package --workspace --no-verify` exited `101` at 17:35:03 only because the tree was
   dirty (`error: 1 files in the working directory contain changes that were not yet committed into git`);
   the edit was restored and the step was repeated as step 5.
4. 17:35:03 `git checkout -- packages/axum-health-check-api-server/Cargo.toml Cargo.lock`, then
   `git diff --quiet HEAD -- packages/axum-health-check-api-server/Cargo.toml Cargo.lock` exited `0`.
5. 17:35:11 the same `sed` edit, then `cargo package --workspace --no-verify --allow-dirty`
   exited `0` at 17:35:28 and packaged all 32 workspace members
   (32 `Packaging` lines).
6. 17:35:28 the same `git checkout`, then the same `git diff --quiet HEAD -- ...` exited `0`.

#### Observed Result

```text
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-9fb5f44dbd97c5bb)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/integration.rs (target/debug/deps/integration-66a0ce008e79654d)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
   Doc-tests torrust_tracker_axum_health_check_api_server
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Conclusion

Removing the line is sufficient: the crate's tests still pass (integration tests import the
package under test without declaring it) and the whole workspace packages. This probe is not the
final recheck; the implementation repeats V1 after the fix.

### V3 - Red: the CI packaging command on the unfixed tree, locally

- Goal: show that the regression check added to CI fails for the defect before the fix.
- Initial state: branch `2582-1669-remove-health-check-self-dev-dependency` at the commit
  "docs(issues): [#2582] record the maintainer's decisions for the packaging check" (`develop` at
  `afbd5dbf1` plus that spec-only commit); clean tree (`git status --short` printed nothing).
- Environment: stable Rust toolchain, `cargo 1.99.0 (5f94df478 2026-08-27)`,
  `rustc 1.99.0 (b940084d7 2026-09-28)`.
- Status: `DONE` (outcome: **red**, as expected)

#### Steps Performed

1. 2026-10-21 09:29:40 UTC `cargo package --workspace --no-verify` (the exact command of the new
   `Workspace Packaging` CI job); ended 09:30:04 UTC.

#### Observed Result

Exit `101` after 25 `Packaged` lines; the tail of the output:

```text
    Packaged 53 files, 339.1KiB (74.0KiB compressed)
   Packaging torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)
    Updating crates.io index
error: failed to prepare local package for uploading

Caused by:
  no matching package named `torrust-tracker-axum-health-check-api-server` found
  location searched: crates.io index
  required by package `torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)`
```

#### Conclusion

The regression check is red for this defect, with the same error as V1.

### V4 - Red: the `Workspace Packaging` CI job on the unfixed tree, in the fork

- Goal: show that the new CI job fails for the defect on a hosted runner (AC4).
- Initial state: the branch pushed to the fork with the commit
  "ci(testing): [#2582] package every workspace crate on each run" on top of V3's state, and
  without the fix.
- Environment: GitHub-hosted `ubuntu-latest` runner, `dtolnay/rust-toolchain@stable` installed
  `rustc 1.99.1 (58cf8f9c3 2026-10-13)`.
- Status: `DONE` (outcome: **red**, as expected)

#### Steps Performed

1. `git push -u josecelano 2582-1669-remove-health-check-self-dev-dependency`, which completed at
   about 09:33:54 UTC, when the fork created the run below (the installed pre-push hook passed; it
   did not contain the packaging step yet).
2. The push started the fork's `Testing` workflow,
   [run 39577940822](https://github.com/josecelano/torrust-tracker/actions/runs/39577940822); its
   `Workspace Packaging` job
   ([job 119148033669](https://github.com/josecelano/torrust-tracker/actions/runs/39577940822/job/119148033669))
   ran from 09:34:01 to 09:34:26 UTC and concluded `failure`.
3. The job log was downloaded with
   `gh api repos/josecelano/torrust-tracker/actions/jobs/119148033669/logs`, and ANSI codes were
   stripped.

#### Observed Result

25 `Packaged` lines, then (timestamps kept, runner paths unchanged):

```text
2026-10-21T09:34:14.7734486Z ##[group]Run cargo package --workspace --no-verify
2026-10-21T09:34:14.7734725Z cargo package --workspace --no-verify
...
2026-10-21T09:34:23.0998746Z     Packaged 53 files, 339.1KiB (74.0KiB compressed)
2026-10-21T09:34:23.1251947Z    Packaging torrust-tracker-axum-health-check-api-server v0.1.0 (/home/runner/work/torrust-tracker/torrust-tracker/packages/axum-health-check-api-server)
2026-10-21T09:34:23.1348391Z     Updating crates.io index
2026-10-21T09:34:23.6449707Z error: failed to prepare local package for uploading
2026-10-21T09:34:23.6449901Z
2026-10-21T09:34:23.6449983Z Caused by:
2026-10-21T09:34:23.6450179Z   no matching package named `torrust-tracker-axum-health-check-api-server` found
2026-10-21T09:34:23.6450419Z   location searched: crates.io index
2026-10-21T09:34:23.6450798Z   required by package `torrust-tracker-axum-health-check-api-server v0.1.0 (/home/runner/work/torrust-tracker/torrust-tracker/packages/axum-health-check-api-server)`
2026-10-21T09:34:23.6581707Z ##[error]Process completed with exit code 101.
```

#### Conclusion

The CI job is red for this defect on a hosted runner. The step itself took about 9 seconds; the
whole job, including checkout and toolchain setup, 25 seconds.

### V5 - Red: the pre-push packaging step on the unfixed tree

- Goal: show that the new pre-push step fails for the defect (AC5) and that `--allow-dirty` lets it
  run with uncommitted edits.
- Initial state: V4's commits plus the uncommitted pre-push step and documentation edits of the
  later commit "chore(git-hooks): [#2582] check workspace packaging before pushing", and the
  uncommitted V3 evidence; the manifest still unfixed. The tree was dirty on purpose.
- Environment: same as V3.
- Status: `DONE` (outcome: **red**, as expected)

#### Steps Performed

1. 2026-10-21 09:34:54 UTC `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-push.sh`;
   exited `1` at 09:35:26 UTC.

#### Observed Result

```text
Running pre-push checks...
Note: these checks can take several minutes. If Git reports a closed SSH connection after they pass, see <repository root>/docs/git-hooks.md.

[Step 1/6] Checking format with nightly toolchain ... PASS (1s)
[Step 2/6] Checking workspace with nightly toolchain ... PASS (0s)
[Step 3/6] Building documentation with nightly toolchain ... PASS (5s)
[Step 4/6] Building library documentation with nightly toolchain ... PASS (3s)
[Step 5/6] Checking that every workspace crate can be packaged ... FAIL (23s)  log: .tmp/pre-push-checking-that-every-workspace-crate-can-be-packaged-mKk07r.log
        Updating crates.io index
        Packaged 53 files, 339.1KiB (74.0KiB compressed)
       Packaging torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)
        Updating crates.io index
    error: failed to prepare local package for uploading

    Caused by:
      no matching package named `torrust-tracker-axum-health-check-api-server` found
      location searched: crates.io index
      required by package `torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)`
    (10 lines shown - full log: .tmp/pre-push-checking-that-every-workspace-crate-can-be-packaged-mKk07r.log)

==========================================
FAILED: Pre-push checks failed!
Fix the errors above before pushing.
==========================================
```

#### Conclusion

The pre-push step is red for this defect and stops the hook before the full test suite. It ran
despite the dirty tree, so `--allow-dirty` works as intended. The `.tmp/` log is not kept; the
relevant lines are above.

### V6 - Green and like-for-like recheck after the fix

- Goal: repeat V1 exactly on the fixed tree (M2), and show the crate's tests, the pre-push step and
  the CI job green (AC2, AC3, AC4, AC5).
- Initial state: branch at the commit
  "fix(axum-health-check-api-server): [#2582] remove the self dev-dependency", which is the last
  production change; clean tree (`git status --short` printed nothing).
- Environment: same as V3 locally; the hosted runner as in V4.
- Status: `DONE` (outcome: **green**)

#### Steps Performed

1. 2026-10-21 09:44:14 UTC `cargo package --workspace --no-verify` (V1's command); exited `0` at
   09:44:39 UTC.
2. 09:45:11 UTC `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-push.sh`;
   exited `0` at 09:46:36 UTC.
3. 09:46:43 UTC `cargo test -p torrust-tracker-axum-health-check-api-server`; exited `0`.
4. `git push josecelano 2582-1669-remove-health-check-self-dev-dependency` started the fork's
   `Testing` workflow,
   [run 39579798653](https://github.com/josecelano/torrust-tracker/actions/runs/39579798653); its
   `Workspace Packaging` job
   ([job 119154295112](https://github.com/josecelano/torrust-tracker/actions/runs/39579798653/job/119154295112))
   ran from 09:48:28 to 09:48:51 UTC on the fix commit and concluded `success`. The log was
   downloaded as in V4.

#### Observed Result

Step 1: 32 `Packaging` and 32 `Packaged` lines (every workspace member), no error. The health check
crate now packages:

```text
   Packaging torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)
    Updating crates.io index
    Packaged 18 files, 203.9KiB (54.3KiB compressed)
```

The output also has seven `warning: manifest has no license, license-file, documentation, homepage
or repository` (or `no documentation, homepage or repository`) lines. Each precedes the
`Packaging` line of one of the seven `contrib/dev-tools/` crates (`agent-review-report-contract`,
`ci-gate`, `clippy-allow-reasons`, `frontmatter-validator`, `github-review-threads`,
`package-coverage-check`, `workspace-coupling`), which all set `publish = false`. They are
warnings, not errors, and out of scope.

Step 2:

```text
Running pre-push checks...
Note: these checks can take several minutes. If Git reports a closed SSH connection after they pass, see <repository root>/docs/git-hooks.md.

[Step 1/6] Checking format with nightly toolchain ... PASS (0s)
[Step 2/6] Checking workspace with nightly toolchain ... PASS (1s)
[Step 3/6] Building documentation with nightly toolchain ... PASS (7s)
[Step 4/6] Building library documentation with nightly toolchain ... PASS (4s)
[Step 5/6] Checking that every workspace crate can be packaged ... PASS (37s)
[Step 6/6] Running all tests ... PASS (36s)
==========================================
SUCCESS: All pre-push checks passed! (1m 25s)
==========================================

You can now safely push your changes.
```

Step 3:

```text
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-9fb5f44dbd97c5bb)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/integration.rs (target/debug/deps/integration-66a0ce008e79654d)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
   Doc-tests torrust_tracker_axum_health_check_api_server
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Step 4: 32 `Packaging` lines; the step's excerpt:

```text
2026-10-21T09:48:40.4198651Z ##[group]Run cargo package --workspace --no-verify
...
2026-10-21T09:48:46.6241625Z    Packaging torrust-tracker-axum-health-check-api-server v0.1.0 (/home/runner/work/torrust-tracker/torrust-tracker/packages/axum-health-check-api-server)
2026-10-21T09:48:46.6355574Z     Updating crates.io index
2026-10-21T09:48:46.8948976Z     Packaged 18 files, 203.9KiB (54.3KiB compressed)
...
2026-10-21T09:48:49.1288330Z    Packaging workspace-coupling v0.1.0 (/home/runner/work/torrust-tracker/torrust-tracker/contrib/dev-tools/analysis/workspace-coupling)
2026-10-21T09:48:49.1410235Z     Updating crates.io index
2026-10-21T09:48:49.1499878Z     Packaged 8 files, 45.1KiB (11.8KiB compressed)
```

#### Conclusion

The V1 symptom is gone: the whole workspace packages, the crate's tests pass, and both new checks
that were red in V3, V4 and V5 are green. If a later commit changes production code, this recheck is
repeated and the commit it ran on is named here.

#### Rerun after rebasing onto `develop`

V3 to V6 above ran on the branch based on `develop` at `afbd5dbf1`. Before opening the pull
request the branch was rebased onto `develop` at `97c689fe1` (no conflicts, no change to this
issue's commits). On the rebased fix commit, with a clean tree:

1. 2026-10-21 09:53:24 UTC `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-push.sh`
   passed at 09:54:50 UTC:

   ```text
   [Step 1/6] Checking format with nightly toolchain ... PASS (0s)
   [Step 2/6] Checking workspace with nightly toolchain ... PASS (3s)
   [Step 3/6] Building documentation with nightly toolchain ... PASS (12s)
   [Step 4/6] Building library documentation with nightly toolchain ... PASS (8s)
   [Step 5/6] Checking that every workspace crate can be packaged ... PASS (21s)
   [Step 6/6] Running all tests ... PASS (42s)
   SUCCESS: All pre-push checks passed! (1m 26s)
   ```

2. `cargo test -p torrust-tracker-axum-health-check-api-server` passed again (3 unit, 8
   integration, 0 doc tests).

The pull request's own `Workspace Packaging` job checks the rebased commits in CI.

## Failures and Follow-up

V2 step 3 failed for an unrelated reason (dirty tree without `--allow-dirty`) and was rerun as
step 5.
