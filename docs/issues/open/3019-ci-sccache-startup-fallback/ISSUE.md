---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p1
epic: null
github-issue: 3019
spec-path: docs/issues/open/3019-ci-sccache-startup-fallback/ISSUE.md
branch: "3019-ci-sccache-startup-fallback"
related-pr: null
last-updated-utc: "2026-11-24 17:39"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - implement-workflow
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/implement-workflow/SKILL.md
    - docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md
    - .github/workflows/testing.yaml
    - .github/workflows/os-compatibility.yaml
    - .github/workflows/db-compatibility.yaml
    - .github/workflows/coverage.yaml
    - .github/workflows/db-benchmarking.yaml
---

<!-- skill-link: create-issue -->

# Issue #3019 - CI bare builds fail when the sccache server does not start

## Goal

A bare CI build (a job that compiles Rust on the runner with `sccache` as the compiler wrapper)
keeps building, without the compilation cache, when the `sccache` server cannot start. Today the
job fails before compiling anything.

## Background

[ADR 20260612000000](../../../adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md) added
`sccache` with the GitHub Actions cache backend to every bare CI build. Each job has two steps:
`mozilla-actions/sccache-action` installs the binary, and an `Enable sccache` step exports
`RUSTC_WRAPPER=sccache`, `SCCACHE_GHA_ENABLED=true` and `CARGO_INCREMENTAL=0`.

What happens, step by step:

1. The first `cargo` command of the job (`cargo fetch` in the `unit` job) runs
   `sccache rustc -vV`, because `RUSTC_WRAPPER` is set.
2. No `sccache` server is running yet, so the client starts one and waits for it to report
   that it is ready. The client waits at most 10 seconds (`SERVER_STARTUP_TIMEOUT` in
   `src/commands.rs` of sccache v0.20.0, the version the action installs).
3. With the GitHub Actions cache backend, the server talks to the cache service while it
   starts. When that service is slow or unreachable, the server is not ready in 10 seconds.
4. The client prints
   `sccache: error: Timed out waiting for server startup. Maybe the remote service is unreachable?`
   and exits with status 2. Cargo cannot query `rustc`, exits with status 101, and the job fails.

On 2026-11-23, between 20:00 and 22:04 UTC, this failed these jobs on GitHub-hosted runners:

| Workflow run                     | Attempt | PR   | Job                                   |
| -------------------------------- | ------- | ---- | ------------------------------------- |
| `OS Compatibility` 45253124047   | 2       | 3012 | `Build on ubuntu-latest (nightly)`    |
| `OS Compatibility` 45253124047   | 3       | 3012 | `Build on macos-latest (nightly)`     |
| `Testing` 45253124156            | latest  | 3012 | `Unit (stable)`                       |
| `Testing` 45270620775            | latest  | 3013 | `Unit (stable)`                       |

Other jobs and attempts in the same period passed, and no workflow file had changed on
`develop`. The cause is the cache service, not the repository code.

Expected behavior: the ADR adopted `sccache` to make builds faster. A cache is an optimization, so
an unavailable cache must cost build time, not fail the build. Impact today: PRs show red checks
that have nothing to do with their change, and the maintainer has to re-run jobs (attempt 3 of
run 45253124047 still failed).

### Why `SCCACHE_IGNORE_SERVER_IO_ERROR` is not enough

The sccache v0.20.0 source only reads `SCCACHE_IGNORE_SERVER_IO_ERROR` in `do_compile`, when the
client fails to read a compile response from a server that is already running. The startup path
(`connect_or_start_server`) returns the timeout error with `?`, so the variable does not apply.
The local reproduction in `manual-verification-evidence.md` (V2) confirms it: with the variable
set to `1`, the build still fails with the same error.

## Scope

### In Scope

- A fallback for every bare CI job that enables `sccache`: start the server explicitly, use
  `sccache` only if it started, and otherwise build without it and show a warning annotation.
- One shared local composite action, `.github/actions/enable-sccache/action.yml`, that replaces
  the nine copies of the `Enable sccache` step in `testing.yaml`, `os-compatibility.yaml`,
  `db-compatibility.yaml`, `coverage.yaml` and `db-benchmarking.yaml`. The action sets
  `shell: bash`, so the step also runs in Git Bash on the `windows-latest` jobs.
- Keep the server alive for the whole job (`SCCACHE_IDLE_TIMEOUT=0`), so that the only server
  startup is the checked one. Without it, the server stops after 10 idle minutes and the next
  compile starts it again without the check.
- An amendment to the sccache ADR that records the fallback and why
  `SCCACHE_IGNORE_SERVER_IO_ERROR` was not used.

### Out of Scope

- Making the GitHub Actions cache service more reliable, or changing the cache backend.
- Retrying the server startup. A retry adds up to 10 seconds per attempt on a service that is
  already failing; a build without the cache is a predictable cost.
- Docker builds (`container.yaml`), which do not use `sccache`.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md`
- ADRs to create: none. The fallback refines how the accepted decision is implemented; it is
  recorded as an amendment section in the existing ADR.

## Design and Ownership Review

- The `mozilla-actions/sccache-action` step stays in each workflow. It owns the install and its
  post step (`sccache --show-stats`). Keeping it a direct step of the job avoids depending on how
  a composite action runs the post steps of the actions it nests.
- The new composite action owns the decision to enable `sccache`. It runs the `sccache` that
  the install step puts on `PATH`, the same one `RUSTC_WRAPPER=sccache` uses; its only output
  is `$GITHUB_ENV`.
- `db-compatibility.yaml` and `db-benchmarking.yaml` run only for listed paths, so their path
  filters also list `.github/actions/enable-sccache/**`: a change to the action runs them.
- Backend selection: the server reads its configuration, including the cache backend, when it
  starts. So the startup command itself runs with `SCCACHE_GHA_ENABLED=true` and
  `SCCACHE_IDLE_TIMEOUT=0` in its environment (the step's `env:`), and the server it starts uses
  the GitHub Actions cache. The action writes `RUSTC_WRAPPER=sccache`,
  `SCCACHE_GHA_ENABLED=true` and `SCCACHE_IDLE_TIMEOUT=0` to `$GITHUB_ENV` only after the startup
  succeeded, so later steps, and any server the post step starts, use the same configuration.
  `CARGO_INCREMENTAL=0` is written in both cases, as today.
- Server lifetime: started once by the composite action with `SCCACHE_IDLE_TIMEOUT=0`, it lives
  until the runner ends the job. On failure the action runs `sccache --stop-server` and ignores
  its result, so a server that becomes ready late does not keep running with the cache backend.
- Deadline: the startup wait is bounded by sccache's own 10-second timeout.
- Shell size: an `if`/`else` of a few lines is adapter work under the `implement-workflow` rule,
  so no repository tool is needed.

## Bug-Fix Process

Follows [fix-bug](../../../../.claude/skills/fix-bug/SKILL.md):

1. Analysis: done above. The startup timeout in `connect_or_start_server` is fatal for the
   compile command, and `SCCACHE_IGNORE_SERVER_IO_ERROR` does not cover it.
2. Reproduction: **Reproduced** locally with a real `sccache` (V1 in
   `manual-verification-evidence.md`), forcing the same startup timeout with a 1 ms startup
   budget. The hosted failures are linked as the original symptom.
3. Regression-test boundary: see the next section.
4. Red: the forced-timeout job fails on the current workflow (V3).
5. Fix: the composite action and the workflow changes.
6. Green and recheck: the same forced-timeout job builds with the fix (V4), and the normal jobs
   still use the cache (V5).

## Regression Test Strategy

A maintained automatic test is not practical: the behavior lives in GitHub Actions workflow
steps, and forcing the timeout in a permanent job would turn the cache off for that job on every
run. The strongest evidence is a hosted run of the real workflow with the timeout forced:

- A temporary commit sets `SCCACHE_CONF` to a config file with `server_startup_timeout_ms = 1`
  for the `unit` job. Before the fix it must fail with the original error (red); after the fix
  it must pass with the warning annotation (green).
- The temporary commits live only on throwaway draft PRs (red: #3033, green: #3034), closed
  after their checks ran. They never enter the implementation PR. The run links stay in the
  evidence. The plan was to use verification branches in the `josecelano-bot` fork, but pushes
  there start no workflow run (V3), so the maintainer chose draft PRs instead.

Two jobs that use the action do not run on a pull request to `develop`, and without fork runs
they get their hosted evidence after the merge (V6):

- `coverage.yaml` only runs for pushes to `develop`, so the merge push runs it.
- `testing.yaml` → `docker-e2e` skips pull requests that target `develop` and pushes to
  `develop`. In `torrust/torrust-tracker` it runs its `sccache` steps only on a GitHub-hosted
  runner, which it gets only for Dependabot branch pushes. The next Dependabot branch push runs
  them.
- `db-compatibility.yaml` and `db-benchmarking.yaml` run on the PR because their path filters
  include their own workflow file, which the change edits.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                     | Notes / Expected Output                                                                 |
| --- | ------ | ---------------------------------------- | --------------------------------------------------------------------------------------- |
| T1  | DONE   | Reproduce locally                        | V1 and V2 in `manual-verification-evidence.md`                                          |
| T2  | DONE   | Red: hosted run with the timeout forced  | `unit` job fails with `Timed out waiting for server startup` on the current workflow (V3) |
| T3  | DONE   | Add the `enable-sccache` composite action | Starts the server; exports the variables only on success; warns otherwise              |
| T4  | DONE   | Use it in the five workflows             | Nine `Enable sccache` steps replaced; `if:` conditions kept                             |
| T5  | DONE   | Green: hosted run with the timeout forced | Same job passes and shows the warning (V4); temporary commit dropped                   |
| T6  | TODO   | Normal hosted run                        | PR jobs pass and the sccache stats show cache hits (V5)                                 |
| T6b | TODO   | Hosted runs of jobs the PR does not run  | After the merge: `coverage` and a Dependabot `docker-e2e` pass with cache hits (V6)                     |
| T7  | DONE   | Amend the ADR                            | Fallback and the `SCCACHE_IGNORE_SERVER_IO_ERROR` analysis recorded                     |

## Commit Points

| Task  | Coherent change set                                         | Commit policy                                        |
| ----- | ----------------------------------------------------------- | ---------------------------------------------------- |
| T1    | This spec and `manual-verification-evidence.md`             | Commit after `linter markdown` and `linter cspell`.  |
| T3–T4 | Composite action and the five workflows                     | Commit after `linter yaml` and `git diff --check`.   |
| T7    | ADR amendment                                               | Commit after focused validation.                     |
| T2/T5 | Temporary forced-timeout commits                            | Throwaway draft PRs only; never in the PR.           |
| T6b   | Evidence update with the post-merge run links               | Close-out commit after the merge.                    |
| T6    | Evidence update with the hosted run links                   | Commit after focused validation.                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/ci-sccache-startup-fallback/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-24 07:26 UTC - Claude Code (Claude Opus 5.5) - Confirmed the failure in the logs of
  jobs 138047391738 and 138029839665, read the sccache v0.20.0 startup code, reproduced the
  timeout locally and tested the fallback locally - `manual-verification-evidence.md` V1, V2
- 2026-11-24 09:09 UTC - Claude Code (Claude Opus 5.5) - The maintainer approved the spec and
  asked for a spec-only PR first; created issue #3019 and moved the spec to `docs/issues/open/`
- 2026-11-24 10:16 UTC - Claude Code (Claude Opus 5.5) - Addressed CodeRabbit's review of PR
  #3020: the startup command sets `SCCACHE_GHA_ENABLED=true`; hosted runs move to fork
  verification branches and cover `docker-e2e` and `coverage` (V6)
- 2026-11-24 13:07 UTC - Claude Code (Claude Opus 5.5) - Spec PR #3020 merged. Added the
  `enable-sccache` composite action, used it in the nine `Enable sccache` steps (the spec said
  ten; the `os-compatibility.yaml` matrix makes one step run in three jobs), and amended the
  ADR. Pushing the red verification branch to the `josecelano-bot` fork started no workflow
  run (see V3), so T2, T5 and T6b wait for a way to run them
- 2026-11-24 17:04 UTC - Claude Code (Claude Opus 5.5) - The maintainer chose throwaway draft
  PRs for the red and green runs: opened #3033 (red) and #3034 (green). `coverage` and
  `docker-e2e` evidence (V6) moves to after the merge, because neither job runs on a PR to
  `develop`
- 2026-11-24 17:39 UTC - Claude Code (Claude Opus 5.5) - Red (#3033) and green (#3034) runs
  done: the forced timeout fails the current workflow and passes with the fix, with the
  warning annotation - `manual-verification-evidence.md` V3, V4

## Acceptance Criteria

- [x] AC1: When the `sccache` server does not start, a bare CI job builds without `sccache` and
      passes, and the run shows a warning annotation that says the cache was disabled.
- [ ] AC2: When the server starts, the job uses `sccache` with the GitHub Actions cache backend
      as before (`RUSTC_WRAPPER=sccache`, cache hits in the sccache stats).
- [ ] AC3: Every bare CI job that enabled `sccache` before uses the shared composite action, and
      the `windows-latest` jobs in `os-compatibility.yaml` still pass.
- [x] AC4: The sccache ADR records the fallback and why `SCCACHE_IGNORE_SERVER_IO_ERROR` is not
      sufficient.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all` (includes `linter yaml`)
- `git diff --check`
- The hosted CI checks of the PR

### Manual Verification Scenarios

| ID  | Scenario                                    | Human-oriented command/steps                                                                 | Expected Result                                         | Status | Evidence                                     |
| --- | ------------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Local reproduction of the startup timeout   | `cargo build` with `RUSTC_WRAPPER=sccache` and `server_startup_timeout_ms = 1`               | Build fails with `Timed out waiting for server startup` | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | `SCCACHE_IGNORE_SERVER_IO_ERROR` and fallback, locally | Same build with the variable set; then the fallback commands and the same build     | Variable: still fails. Fallback: warning, build passes  | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | Hosted red run                              | Draft PR #3033: temporary commit forcing the timeout on the current workflow                 | `unit` job fails with the original error                | DONE   | `manual-verification-evidence.md` section V3 |
| M4  | Hosted green run                            | Draft PR #3034: same temporary commit on top of the fix                                      | `unit` job passes with the warning annotation           | DONE   | `manual-verification-evidence.md` section V4 |
| M5  | Hosted normal run                           | PR checks without any temporary commit                                                       | Jobs pass; sccache stats show cache hits                | TODO   | `manual-verification-evidence.md` section V5 |
| M6  | Jobs the PR checks do not run               | After the merge: the `coverage.yaml` run of the merge push; the next Dependabot branch push (`docker-e2e`) | Both jobs pass; sccache stats show cache hits | TODO | `manual-verification-evidence.md` section V6 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | V4       |
| AC2   | TODO                   | V5, V6   |
| AC3   | TODO                   | V5, V6   |
| AC4   | DONE                   | PR diff  |

## Risks and Trade-offs

- A job that falls back is slower (the ADR measured about 480 s instead of about 190 s for the
  `unit` job). That is the intended trade: slower instead of failed. The warning annotation
  makes it visible.
- If a server is already running when the action starts, `sccache --start-server` fails with
  `Address in use` and the job builds without the cache. That cannot happen on the fresh
  GitHub-hosted runners these jobs use; the self-hosted `docker-e2e` path skips `sccache`.
- The action's post step (`sccache --show-stats`) starts a server if none is running. After a
  fallback, `SCCACHE_GHA_ENABLED` is not exported, so that server uses the local disk cache and
  does not need the cache service.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Failed runs: <https://github.com/torrust/torrust-tracker/actions/runs/45253124156>,
  <https://github.com/torrust/torrust-tracker/actions/runs/45253124047>,
  <https://github.com/torrust/torrust-tracker/actions/runs/45270620775>
- Related ADRs: `docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md`
- sccache v0.20.0 source: `src/commands.rs` (`connect_or_start_server`, `do_compile`,
  `Command::StartServer`), `src/server.rs` (`SCCACHE_IDLE_TIMEOUT`)
