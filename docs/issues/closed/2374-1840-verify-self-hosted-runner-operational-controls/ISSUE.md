---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p1
epic: 1840
github-issue: 2374
spec-path: docs/issues/closed/2374-1840-verify-self-hosted-runner-operational-controls/ISSUE.md
branch: "2374-1840-verify-self-hosted-runner-operational-controls"
related-pr: 2379
last-updated-utc: "2026-09-30 07:31"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/self-hosted-runner.md
    - docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md
    - docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
    - docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/manual-verification-evidence.md
    - docs/issues/closed/2374-1840-verify-self-hosted-runner-operational-controls/manual-verification-evidence.md
    - docs/issues/closed/2374-1840-verify-self-hosted-runner-operational-controls/agent-review-reports.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
---

<!-- skill-link: create-issue -->

# Issue #2374 - Verify the Self-Hosted Runner's Offline Recovery and Untrusted-Code Routing

Parent EPIC: #1840 - Improve PR Workflow Performance

## Goal

Collect real evidence for the two runner controls that #2323 left unverified: an offline runner
leaves jobs visibly queued and the documented recovery unblocks them (M5), and Dependabot and
external-contributor code cannot reach the persistent self-hosted runner without review (M7). With
that evidence, complete the review of #2323's acceptance criteria AC1, AC3, and AC4.

## Background

Issue #2323 moved the `Test (Docker)` and `Docker E2E` jobs to the self-hosted Hetzner runner
`torrust-runner-01` (PR #2352, merged as `432d4e69`; evidence PR #2355, merged as `17b5e9eb`).
The issue was closed on 2026-09-27 and its specification archived in
`docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/`. Scenarios M1 to M4 and M6 are done.
Scenarios M5 and M7 and acceptance criteria AC1, AC3, and AC4 were left unchecked on purpose.

The runner is persistent, and job code gets root-equivalent access to it through the `docker`
group. The ADR accepts that only while unreviewed code rarely reaches the runner, so the routing
controls are the security boundary, and the offline procedure is the availability fallback. Both
are documented in `docs/self-hosted-runner.md` but have not been exercised.

State on 2026-09-29 (read-only checks):

- The runner is online and idle, label `torrust-hetzner`, version 2.337.0.
- Fork-PR approval policy: `all_external_contributors`. Organization 2FA required: `true`.
- The only jobs that reference secrets or an environment (`dockerhub-torrust`) are
  `publish_development` and `publish_release` in `container.yaml`; both use `runs-on:
  ubuntu-latest`.

Evidence that already exists (to be recorded, not repeated):

- Dependabot PR #2369 (opened 2026-09-28, after the routing change): `Test (Docker)` ran on a
  GitHub-hosted runner (`GitHub Actions 1000091160`, run `36474323310`). The `Docker E2E` job of
  the push to its `dependabot/` branch also ran GitHub-hosted (run `36474314420`).
- Dependabot PR #2338, re-run by Dependabot on 2026-09-28: `Test (Docker)` ran GitHub-hosted (run
  `36408286110`, which failed for a reason unrelated to routing). Its `Docker E2E` push run
  `36408280318` also ran GitHub-hosted.
- PR #2376 from first-time contributor `josecelano-bot` had no workflow run before maintainer
  approval; Docs Lint started after approval. This proves the external-contributor gate.
- No push to `main` or `releases/**` has happened since the change (the latest `main` commit is
  from 2024), so that routing can only be checked from the workflow expressions for now.

## Scope

### In Scope

- M5: exercise the runner-offline condition and the documented recovery, in a window approved by
  the maintainer.
- M7: record the Dependabot routing evidence above; observe a Dependabot PR after a maintainer
  updates its branch from `develop`; observe a real external-contributor PR waiting for approval.
- Review AC1, AC3, and AC4 of #2323 against the evidence, including the static checks of routing
  that cannot be observed yet (`main`, `releases/**`).
- Record detailed results in the archived #2323 evidence file (new sections V5 and V7), and retain
  an issue-local manual-evidence summary with the commands, outcomes, and links. Update #2323's
  scenario and acceptance tables with links to this issue.
- Make the secrets boundary explicit in `docs/self-hosted-runner.md` (a Security section listing
  which jobs use secrets and how to re-check it).
- Fix `docs/self-hosted-runner.md` if the observed behavior differs from what it describes.

### Out of Scope

- Runner capacity, extra instances, and server sizing (separate #1840 draft; see Related Work).
- The unit-test critical path (#2323 Scenario G), which needs its own #1840 subissue.
- Any change to the routing expressions, the approval policy, or the 2FA setting, unless a
  scenario fails; a failure is diagnosed and planned before any remedy.
- Creating a fake external contributor or weakening an organization setting to exercise M7.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
  (security controls, offline runner, and fallback).
- ADRs to create: none expected. If a control fails and the remedy changes the security model,
  update that ADR.

## Design and Ownership Review

- Operational owner: the maintainer (@josecelano) owns the server and approves every action that
  stops the runner, updates a Dependabot branch, or approves an external workflow run. The agent
  prepares commands, captures observations, and writes the evidence.
- Secrets: the agent never handles SSH passphrases, tokens, or other credentials. Server commands
  run through the maintainer's loaded SSH agent key, or the maintainer runs them.
- Normal and failure paths for M5: the runner must end in its prior state (service `active`,
  GitHub status `online`, idle). If it does not come back within the window, apply the documented
  fallback (a reviewed PR that sets `runs-on: ubuntu-latest`) and record it.
- Deadline: the disposable PR lowers the job's `timeout-minutes` to 2 in its own branch, so the
  runner is offline for about 10 minutes instead of more than 90 (see M5 Plan).

## Bug-Fix Process

Not applicable. This is operational verification of existing behavior. If a scenario fails, the
failure becomes a bug: record it and follow `.github/skills/dev/debugging/fix-bug/SKILL.md`.

## Regression Test Strategy

Not applicable unless a scenario fails. The controls are workflow and GitHub-setting behavior,
verified by real runs.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                   | Notes / Expected Output                                                                                                                                                                  |
| --- | ------ | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Record existing M7 Dependabot evidence | V7 part 1 in the #2323 evidence file: PRs #2369 and #2338, run and job URLs, runner names.                                                                                             |
| T2  | DONE   | Observe a maintainer branch update     | #2369 updated by `josecelano`: its Dependabot PR head `7b3a3245` ran `Test (Docker)` on GitHub Actions with `ubuntu-latest`. See #2323 evidence V7 part 2.                           |
| T3  | DONE   | Observe the external-contributor gate  | #2376 from `josecelano-bot` had no Actions run before approval; after approval Docs Lint started on GitHub Actions. See #2323 evidence V7 part 3. |
| T4  | DONE   | Run M5                                 | #2378 queued past its 2-minute limit (293 seconds), ran after restart, and its fallback run started on GitHub Actions. See #2323 evidence V5.                                         |
| T5  | DONE   | Review AC1, AC3, and AC4               | #2323 AC1, AC3, and AC4 are DONE: observed V1-V7 evidence plus static `main` and release routing checks.                                                                              |
| T6  | DONE   | Document the secrets boundary          | `docs/self-hosted-runner.md` Security section: per-job secrets table, what a job can still reach, and the re-check commands (verified 2026-09-29).                                    |
| T7  | DONE   | Correct the operations guide if needed | Corrected the inaccurate fork-routing statement: approved fork PRs targeting `develop` can select the self-hosted runner.                                                            |

### M5 Plan (for maintainer approval)

What must be shown: `timeout-minutes` (90 for `Test (Docker)`) limits only how long a job runs
after a runner picks it up. It does not limit how long a job waits in the queue, where GitHub keeps
a self-hosted job for up to 24 hours. So an offline runner shows up as a check stuck in "Waiting for
a runner", not as a timed-out job. Proving it with the real 90-minute limit would block every
non-documentation PR for more than 90 minutes. Instead, the disposable PR lowers the limit in its
own branch, because a `pull_request` run uses the workflow file from the PR:

1. Choose a quiet window: no open PR is waiting for `Test (Docker)`.
2. On a branch of the maintainer's fork (a member, so no approval is needed), set
   `timeout-minutes: 2` on the `test` job in `container.yaml`. Do not merge it.
3. Stop the runner:
   `sudo systemctl stop actions.runner.torrust-torrust-tracker.torrust-runner-01.service`.
   A deliberate stop is not restarted by the `Restart=on-failure` drop-in.
4. Open the draft PR. Record the job's `status` (`queued`), `created_at`, the runner's `offline`
   status from the runners API, and the "Waiting for a runner" check text.
5. Wait at least 5 minutes (more than the 2-minute limit) and record that the job is still
   `queued`, not cancelled or failed.
6. Fallback check, without touching `develop`: push a commit to the same PR that sets the `test`
   job's `runs-on` to `ubuntu-latest`. Record that the new run starts on a GitHub-hosted runner.
   Also record what happens to the queued run of the first commit, and whether **Re-run** of a
   queued or failed job would use the updated workflow. (A re-run reuses the commit of its original
   event, so the documented step 2 of "When the Runner Is Offline" may not pick up a fallback;
   T7 corrects the guide if so.)
7. Restart the runner: `sudo systemctl start ...`. Record whether the first run's job is picked
   up (`started_at`, runner name) or was already cancelled.
8. Confirm the runner is `online` and idle again, cancel the remaining runs, close the PR, and
   delete the branch.

Other PRs opened during the window also queue and resume when the runner restarts; hence the quiet
window. The runner is offline for about 10 minutes.

### M7 Plan

- Part 1 (Dependabot PR): done by existing evidence (T1).
- Part 2 (maintainer update): the maintainer clicks **Update branch** on an open Dependabot PR.
  The new `Container` run has `github.actor` equal to the maintainer, but the PR author is still
  `dependabot[bot]`, so the routing expression must still choose `ubuntu-latest`.
- Part 3 (external contributor): `josecelano-bot` opens a harmless PR from its own fork (for
  example a one-line comment change in a Rust file). No workflow run must exist until a maintainer
  approves it. Approve only after reviewing the diff, as the Security rules require, then record
  the approved workflow. Close the PR afterwards.

## Commit Points

| Task   | Coherent change set                                          | Commit policy                                   |
| ------ | ------------------------------------------------------------ | ----------------------------------------------- |
| T1, T2 | V7 Dependabot evidence in the #2323 evidence file            | Commit after the observations are reviewed.     |
| T3     | V7 external-contributor evidence                             | Commit after the observation is reviewed.       |
| T4     | V5 runner-offline evidence                                   | Commit after the observation is reviewed.       |
| T5     | #2323 scenario and acceptance tables                         | Commit after maintainer review.                 |
| T6     | `docs/self-hosted-runner.md` Security section                | Commit after maintainer review.                 |
| T7     | `docs/self-hosted-runner.md` correction (only if needed)     | Commit after maintainer review.                 |

Use Conventional Commits (`docs(issues)`, `docs(self-hosted-runner)`) and sign every commit with GPG.

T3 does not depend on an outside contributor anymore, so all tasks can go in one PR.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1840-verify-self-hosted-runner-operational-controls/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Issue linked as a GitHub sub-issue of EPIC #1840
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, pre-commit checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-29 14:59 UTC - GitHub Copilot - Drafted from the #2323 operational-verification hand-off; recorded existing Dependabot routing evidence (PRs #2369, #2338) and the static secrets check - this file
- 2026-09-29 15:30 UTC - GitHub Copilot - Maintainer answers applied: M5 uses a 2-minute `timeout-minutes` on a disposable branch and checks the fallback there; `josecelano-bot` provides the external PR for M7; evidence goes to the #2323 file; T6 added the Security section to `docs/self-hosted-runner.md` - this file
- 2026-09-29 16:02 UTC - GitHub Copilot - Maintainer approved the spec; created #2374, linked it as a sub-issue of #1840, and moved the spec to `docs/issues/open/` - this file
- 2026-09-29 16:15 UTC - GitHub Copilot - T1 DONE: Dependabot PRs #2369 and #2338 ran `Test (Docker)` and their branch-push `Docker E2E` jobs on GitHub-hosted runners - #2323 `manual-verification-evidence.md` V7 part 1
- 2026-09-29 16:50 UTC - GitHub Copilot - T3 DONE: #2376 from first-time contributor `josecelano-bot` had no run before maintainer approval; Docs Lint started after approval - #2323 `manual-verification-evidence.md` V7 part 3
- 2026-09-29 18:39 UTC - GitHub Copilot - T4 DONE: #2378 queued for 293 seconds despite its 2-minute timeout, ran after the service restarted, and its `ubuntu-latest` fallback started in 2 seconds; T7 DONE with no guide change - #2323 `manual-verification-evidence.md` V5
- 2026-09-29 18:57 UTC - GitHub Copilot - T2 DONE: maintainer updated #2369 with a merge commit; its Dependabot PR head started `Test (Docker)` on GitHub Actions with `ubuntu-latest` - #2323 `manual-verification-evidence.md` V7 part 2
- 2026-09-29 18:59 UTC - GitHub Copilot - T5 DONE: #2323 AC1, AC3, and AC4 reviewed against V1-V7 and static routing checks; no retrospective or operations-guide correction is needed because every observed control behaved as documented - #2323 `ISSUE.md`
- 2026-09-29 19:48 UTC - GitHub Copilot - Corrected `docs/self-hosted-runner.md`: approved fork PRs targeting `develop` can select the self-hosted runner. The maintainer update created no Dependabot branch-push run, so the post-update `Docker E2E` observation remains pending; reopened AC2 and the completion review - #2323 evidence V7
- 2026-09-29 21:12 UTC - GitHub Copilot - M7 DONE: temporary signed trigger and revert commits on the updated Dependabot branch produced `Docker E2E` job `109621539097` on GitHub Actions with `ubuntu-latest`; source content restored - #2323 evidence V7
- 2026-09-29 21:16 UTC - GitHub Copilot - Final full pre-commit gate passed, including `linter all`, staged Markdown frontmatter, dependency checks, formatting, Containerfile linting, and documentation tests - `contrib/dev-tools/git/hooks/pre-commit.sh`
- 2026-09-29 21:26 UTC - GitHub Copilot - Independent review clarified that the PR merge update observed `Test (Docker)`, while a later temporary branch push observed `Docker E2E`; recorded the reusable discovery in `implementation-retrospective.md` - this file
- 2026-09-30 06:10 UTC - Task Reviewer - Final read-only review passed AC1-AC5; reviewer and committer checkpoints recorded, with GitHub issue closure and archive pending PR merge - current branch
- 2026-09-30 07:03 UTC - GitHub Copilot - Addressed PR #2379 review: restored V1 status and evidence timestamp, added issue-local manual evidence and review report, linked the retrospective, recorded PR metadata, and narrowed the runner credential invariant - #2379
- 2026-09-30 07:31 UTC - GitHub Copilot - PR #2379 merged and GitHub closed #2374; archived this completed specification with its evidence and review records - #2379

## Acceptance Criteria

- [x] AC1: M5 shows that a job waits as queued ("Waiting for a runner") while the runner is
      offline, is not ended by its `timeout-minutes`, and runs after the restart; that the
      `runs-on: ubuntu-latest` fallback starts a GitHub-hosted run; and that the runner ends online
      and idle.
- [x] AC2: M7 shows that Dependabot `Test (Docker)` runs GitHub-hosted after a maintainer updates
  its PR branch, and `Docker E2E` runs GitHub-hosted after a subsequent Dependabot-branch
  push.
- [x] AC3: M7 shows that a PR from a non-member fork (`josecelano-bot`) waits for maintainer
      approval before any job runs.
- [x] AC4: #2323 AC1, AC3, and AC4 are reviewed against the evidence and their tables updated,
      with static workflow evidence named as such where a path could not be observed.
- [x] AC5: `docs/self-hosted-runner.md` states which jobs use secrets and how to re-check that no
      self-hosted job does.
- [x] `linter all` exits with code `0`
- [x] Manual verification scenarios are executed and documented in the #2323 `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated if observed behavior differs from `docs/self-hosted-runner.md`

## Verification Plan

### Automatic Checks

- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`

No code changes are planned, so no additional tests are required.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

The detailed evidence remains in #2323's V5 and V7; this folder keeps an issue-local summary, so
the scenario IDs retain #2323's numbering.

| ID      | Scenario                    | Human-oriented command/steps                                                                                             | Expected Result                                                                   | Status | Evidence                                    |
| ------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------- | ------ | ------------------------------------------- |
| M5      | Runner offline              | M5 Plan above; `gh api repos/torrust/torrust-tracker/actions/runs/<run-id>/jobs` and the runners API during the window | Job queued 293 seconds past its 2-minute limit; fallback GitHub-hosted; job ran after restart | DONE   | [`manual-verification-evidence.md`](manual-verification-evidence.md) V1; #2323 V5 |
| M7 (1)  | Dependabot PR               | Jobs API for the Dependabot PR runs listed in Background                                                                 | `Test (Docker)` and `Docker E2E` on GitHub-hosted runners                        | DONE   | [`manual-verification-evidence.md`](manual-verification-evidence.md) V2; #2323 V7 |
| M7 (2)  | Maintainer updates branch   | **Update branch** on #2369; then signed branch trigger; jobs API                                                         | Updated PR `Test (Docker)` and later-push `Docker E2E` GitHub-hosted            | DONE   | [`manual-verification-evidence.md`](manual-verification-evidence.md) V2; #2323 V7 |
| M7 (3)  | External contributor        | PR from `josecelano-bot`; inspect runs before and after approval                                                         | No workflow run before approval; run starts after approval                       | DONE   | [`manual-verification-evidence.md`](manual-verification-evidence.md) V2; #2323 V7 |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | DONE   | [`manual-verification-evidence.md`](manual-verification-evidence.md) V1; #2323 V5 |
| AC2   | DONE   | [`manual-verification-evidence.md`](manual-verification-evidence.md) V2; #2323 V7 |
| AC3   | DONE   | [`manual-verification-evidence.md`](manual-verification-evidence.md) V2; #2323 V7 part 3 |
| AC4   | DONE   | #2323 `ISSUE.md` acceptance verification: observed V1-V7 evidence and static routing checks |
| AC5   | DONE   | `docs/self-hosted-runner.md` Security section; re-check commands run on 2026-09-29: every `secrets.`/`environment:` match is in a publish job, and only `container.yaml` and `testing.yaml` use `torrust-hetzner` |

## Implementation Completion Review

The reusable routing and verification discoveries are recorded in
[`implementation-retrospective.md`](implementation-retrospective.md).

The independent final review is recorded in
[`agent-review-reports.md`](agent-review-reports.md).

## Related Work

- #2323 - Offload the container test job to a self-hosted Hetzner runner (closed; this issue
  completes its verification).
- Runner capacity, server sizing, and resilience: a separate #1840 subissue, drafted but not yet
  opened; it starts after this issue is merged.

## Decisions (2026-09-29)

1. M5 queue wait: lower `timeout-minutes` to 2 on the disposable branch instead of a 90-minute
   outage (see M5 Plan).
2. Recovery: restart the service; the `runs-on: ubuntu-latest` fallback is also accepted, because
   it keeps PRs moving when no maintainer can log in to the server. Its mechanics are checked on
   the disposable branch, without changing `develop`. A second runner would add resilience as
   well as concurrency; that belongs to the capacity draft.
3. External contributor: `josecelano-bot` opens the M7 part 3 PR.
4. Evidence: appended to the archived #2323 evidence file as V5 and V7.
