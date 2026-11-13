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
related-pr: null
last-updated-utc: "2026-11-13 10:10"
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
   without `--foreground`, the docker client never forwards the signal: the tracker keeps running,
   `--kill-after=3s` SIGKILLs the client after 3 s, the status is 137, and the script's final
   `test "${exit_status}" -eq 0 -o "${exit_status}" -eq 124` fails, ending the step with exit 1.
   The tracker log shows no shutdown line, matching the failing CI logs.

The tracker is not at fault: `docker kill --signal=INT` and `--signal=TERM` both stop the image
cleanly with exit 0, and `kill -INT <docker client pid>` does too. GNU `timeout` (with or without
`--foreground`) and uutils `timeout --foreground` also work.

Impact: the Container workflow is red on every `develop` push and PR run on the self-hosted
runner, hiding real regressions and blocking the E2E steps that follow it in the job.

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
| T2  | TODO   | Record red run of the script before the fix                               | Evidence V1 (already captured)               |
| T3  | TODO   | Add `--foreground` to `timeout`; report the unexpected status on failure | Script change only                           |
| T4  | TODO   | Green run locally and Container workflow green on the PR                  | Evidence V3 and PR checks                    |

## Commit Points

| Task | Coherent change set                                      | Commit policy                                        |
| ---- | -------------------------------------------------------- | ---------------------------------------------------- |
| T3   | `fix(container): [#2855] stop the tracker with timeout --foreground` | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2855-container-regression-timeout-sigint/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Implementation completion review recorded
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-11-13 10:10 UTC - Claude Code (Claude Sonnet 5.5) - Drafted spec, reproduced the failure locally - `manual-verification-evidence.md`
- 2026-11-13 UTC - Claude Code (Claude Sonnet 5.5) - Runner apt history confirms rust-coreutils 0.8.0 to 0.10.0 on 2026-11-12; issue #2855 created; spec moved to open/ - evidence V4

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
| M4  | Runner package history | `ssh torrust-runner-01` apt history | rust-coreutils upgrade 2026-11-12 | DONE | section V4 |
| M3  | Recheck after the fix              | Rerun the script with both `timeout` implementations                      | Exit 0                 | TODO   | `manual-verification-evidence.md` section V3 |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |

## Risks and Trade-offs

- `--foreground` only signals the direct child; this is what is wanted (the docker client). No
  other process in the script depends on group signalling.
- The 0.8.0 behavior was not tested directly; the fix is correct for both implementations regardless.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Failing runs: 43184363486, 43225433196, 43299605132; last success 43059027926
- Related ADR: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
