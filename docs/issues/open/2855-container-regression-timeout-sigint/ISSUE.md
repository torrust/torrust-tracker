---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: null
github-issue: 2855
spec-path: docs/issues/open/2855-container-regression-timeout-sigint/ISSUE.md
branch: "2855-container-regression-timeout-sigint-spec"
related-pr: 2857
last-updated-utc: "2026-11-13 11:20"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh
---

<!-- skill-link: create-issue -->

# Issue #2855 - Container regression script fails when `timeout` is uutils

## Goal

Make `test-mounted-no-persistence-configuration.sh` stop the tracker container reliably with
SIGINT regardless of which `timeout` implementation (GNU or uutils) the host provides, so the
"Test (Docker) (release)" job of the Container workflow is green again on `develop`.

## Background

The Container workflow step "Run Persistence Transition Regression" fails with exit code 1 on
`develop` and on unrelated PR branches (develop runs 43184363486, 43225433196, 43299605132; last
success 43059027926 on 2026-11-11 16:30 UTC). The diff between the last success and the first
failure touches only `docs/`, so no repository change caused it.

What happens, step by step:

1. `run_tracker` runs `timeout --signal=INT --kill-after=3s 10s docker run --rm ... <image>`.
2. After 10 s `timeout` must deliver SIGINT to the `docker` client, which forwards it to the
   tracker (PID 1 in the container). The tracker then logs `Torrust tracker shutting down (SIGINT)`
   and exits 0; the script accepts exit status 0 or 124.
3. With the **uutils** `timeout` (the default `timeout` on Ubuntu 26.04, `rust-coreutils`), run
   without `--foreground`, the tracker never receives SIGINT: it keeps running, `--kill-after=3s`
   SIGKILLs the `docker run` client after 3 s instead, the status is 137, and the script's
   `test "${exit_status}" -eq 0 -o "${exit_status}" -eq 124` (line 39) fails, ending the step with
   exit 1. The tracker log shows no shutdown line, matching the failing CI logs. Whether the
   signal reaches the client and is not forwarded, or never reaches the client at all, is a
   hypothesis (`manual-verification-evidence.md` V2); the fix below is correct under either
   reading.

The tracker is not at fault: `docker kill --signal=INT` and `--signal=TERM` both stop the image
cleanly with exit 0, and `kill -INT <docker client pid>` does too. GNU `timeout` (with or without
`--foreground`) and uutils `timeout --foreground` also work.

Impact: the Container workflow is red on every `develop` push and PR run on the self-hosted
runner, hiding real regressions and blocking the E2E steps that follow it in the job. Each failed
run also leaves its tracker container running: `docker run --rm` only removes the container on
exit, and `--kill-after` kills the attached client, not the container, so it never exits. The
runner's daily prune (`contrib/dev-tools/self-hosted-runner/docker-ci-prune.sh`) only removes
*stopped* containers, so these accumulate; 26 were running on the runner at the time of writing,
the oldest since before this regression's first red run (`manual-verification-evidence.md` V5).

Why it regressed with no repository change: the job runs on the self-hosted `torrust-hetzner`
runner (`torrust-runner-01`, Ubuntu 26.04.1), whose `timeout` is the uutils one. The runner's
apt history (`/var/log/apt/history.log`) shows `rust-coreutils` upgraded from 0.8.0 to
0.10.0-1ubuntu2~26.04.1 on 2026-11-12 06:36 UTC, between the last green run (2026-11-11 16:30)
and the first red run (2026-11-12 08:36). The failure was reproduced locally with the same
version, 0.10.0. That 0.8.0 passed the script is inferred from the green runs, not tested.

## Scope

### In Scope

- Make the `timeout` invocation in `run_tracker` deliver SIGINT to the docker client on both
  implementations (`--foreground`).
- Print the unexpected exit status when `run_tracker` fails, so the next failure is diagnosable.

### Out of Scope

- Changing tracker shutdown code (verified correct).
- Changing the runner image or pinning its coreutils.
- Removing the leftover containers already accumulated on the runner (`manual-verification-evidence.md`
  V5); that changes shared-runner state and is left to the maintainer or a follow-up.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
- ADRs to create: None known

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follows `.github/skills/dev/debugging/fix-bug/SKILL.md`. Analysis and reproduction (outcome:
**Reproduced**) are recorded in `manual-verification-evidence.md`.

## Regression Test Strategy

The maintained regression test is the script itself, which already fails (exit 1) on the broken
`timeout` invocation. A unit test is not practical because the defect is a host-tool and
process-boundary interaction. Red is proven by the local run before the fix; green by the same
run after it, plus the Container workflow on the fix PR.

## Implementation Plan

| ID  | Status | Task                                                                      | Notes / Expected Output                      |
| --- | ------ | ------------------------------------------------------------------------- | -------------------------------------------- |
| T1  | DONE   | Reproduce and record evidence                                             | `manual-verification-evidence.md` V1, V2     |
| T2  | DONE   | Record red run of the script before the fix                               | Evidence V1: `RUN_TRACKER_STATUS=137`, exit 1 |
| T3  | DONE   | Add `--foreground` to `timeout`; report the unexpected status on failure | Script change only                           |
| T4  | IN_PROGRESS | Green run locally and Container workflow green on the PR             | Evidence V3 done locally; PR #2857 checks pending |

## Commit Points

| Task | Coherent change set                                      | Commit policy                                        |
| ---- | -------------------------------------------------------- | ---------------------------------------------------- |
| T3   | `fix(container): [#2855] stop the tracker with timeout --foreground` (PR #2857; identify by this Conventional Commit subject, not a branch commit id, which a rebase onto `develop` can rewrite) | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2855-container-regression-timeout-sigint/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-13 10:10 UTC - Claude Code (Claude Sonnet 5.5) - Drafted spec, reproduced the failure locally - `manual-verification-evidence.md`
- 2026-11-13 10:40 UTC - Claude Code (Claude Sonnet 5.5) - Runner apt history confirms rust-coreutils 0.8.0 to 0.10.0 on 2026-11-12; issue #2855 created; spec moved to open/ - evidence V4
- 2026-11-13 11:20 UTC - Claude Code (Claude Sonnet 5.5) - Opened implementation PR #2857 (T3 commit); addressed PR #2856 review findings F1-F7: re-ran V1/V2 with recorded repeatable commands, added V3 (green recheck) and V5 (leftover-container evidence), rewrote the Background mechanism claim as a hypothesis, updated Scope, Implementation Plan and checkpoints - `manual-verification-evidence.md` V1-V5

## Acceptance Criteria

- [ ] AC1: `test-mounted-no-persistence-configuration.sh` passes with uutils `timeout` 0.10.0.
- [ ] AC2: It still passes with GNU `timeout`.
- [ ] AC3: The Container workflow "Test (Docker) (release)" job is green on the fix PR.
- [ ] `linter all` exits with code `0`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- The regression script with `timeout` and `gnutimeout`

### Manual Verification Scenarios

| ID  | Scenario                           | Human-oriented command/steps                                              | Expected Result        | Status | Evidence                                     |
| --- | ---------------------------------- | ------------------------------------------------------------------------- | ---------------------- | ------ | -------------------------------------------- |
| M1  | Reproduce the failure              | Run the script against a locally built release image                      | Status 137, exit 1     | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Isolate the cause                  | Run the container under each `timeout` variant                            | Only uutils w/o flag fails | DONE | `manual-verification-evidence.md` section V2 |
| M3  | Recheck after the fix              | Rerun the fixed script with uutils `timeout`, then with GNU `timeout` via `PATH` | Exit 0, both times | DONE | `manual-verification-evidence.md` section V3 |
| M4  | Runner package history             | `ssh torrust-runner-01` apt history, dpkg, and `timeout --version`         | rust-coreutils upgrade 2026-11-12 | DONE | `manual-verification-evidence.md` section V4 |
| M5  | Leftover containers on the runner  | `ssh torrust-runner-01 'docker ps -a'`                                     | Running containers accumulate; prune does not remove them | DONE | `manual-verification-evidence.md` section V5 |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | DONE   | `manual-verification-evidence.md` V3 (uutils run) |
| AC2   | DONE   | `manual-verification-evidence.md` V3 (GNU run via `PATH`) |
| AC3   | TODO   | PR #2857 checks (pending) |

## Risks and Trade-offs

- `--foreground` only signals the direct child; this is what is wanted (the docker client). No
  other process in the script depends on group signalling.
- The 0.8.0 behavior was not tested directly; the fix is correct for both implementations regardless.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Failing runs: 43184363486, 43225433196, 43299605132; last success 43059027926
- Related ADR: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
- Related issues: #2855
- Related PRs: #2856 (spec-only), #2857 (implementation)
