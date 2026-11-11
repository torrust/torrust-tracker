---
schema-version: 1
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: 2003
github-issue: 2822
spec-path: docs/issues/open/2822-2003-wait-for-pr-review-gh-option/ISSUE.md
branch: "2822-wait-for-pr-review-gh-option-spec"
related-pr: 2826
last-updated-utc: "2026-11-11 11:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/main.rs
    - .github/skills/dev/pr-reviews/wait-for-pr-review/tool/src/gh.rs
    - .github/skills/dev/pr-reviews/wait-for-pr-review/tool/tests/cli.rs
    - docs/issues/closed/2581-2003-pr-review-wait-poller/ISSUE.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
---

<!-- skill-link: create-issue -->

# Issue #2822 - `wait-for-pr-review`: add a `--gh-command` option to name the GitHub CLI command

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Add a `--gh-command <command>` option (default `gh`) to the `wait-for-pr-review` poller, so a caller can point it
at another executable, and make the identity the poller runs as visible in its output.

## Background

The poller reads a pull request through the GitHub CLI. `main.rs` builds `GhCli::new("gh", ...)`, so the
program is always `gh`, resolved through `PATH`. Before every wait, `wait` calls `check_authenticated`, which
runs `gh auth status`.

A caller that must run as a GitHub identity other than the default `gh` login has only one way to do it today:
put a `gh` on `PATH` that runs that identity's own command. When that stand-in itself calls `gh`, it calls
itself, without end. This crashed a desktop session four times. The poller needs an explicit way to be told
which command to run, so no stand-in is needed.

An AI agent must also be able to tell which identity it runs as (an external contributor or the project
maintainer). A bare default and an unexpressive option name do not tell it. The concern is stated on its merits:
an agent that cannot see its identity can act under the wrong one without noticing.

The tool does not name any particular command, account or role; the caller chooses.

The need comes from the maintainer's backlog for the autonomous PR review loop (item A1, "run the poller as any
identity"), which lives outside this repository; the sibling items are named in words below.

## Scope

### In Scope

- A `--gh-command <command>` option on `wait-for-pr-review`, default `gh`, documented in `--help` and in the skill.
- Using that command for every call the poller makes to GitHub.
- The authentication pre-check (`auth status`) for a non-default command.
- The `gh_command` and `gh_login` fields of the `wait_started` record, and the best-effort lookup behind
  `gh_login`.
- Documenting the option and the reported fields in the skill's text about the poller.
- Maintained tests that pass a fake command through the option, with no `PATH` shim.

### Out of Scope

- The flaky fake-`gh` scenarios of `tests/cli.rs`: a separate issue. These tests must not become less
  reliable.
- New poller events for workflow runs that wait for approval, and the review-loop wording of the skill
  (reviewer deadlines, re-requesting reviews, the hand-over state): separate sub-issues.
- Changing the CLI output contract. The exit codes `0`, `1`, `2` and the JSON-only stderr stay.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md` (decided and
  merged; its scope names this poller) and `docs/adrs/20260519000000_define_global_cli_output_contract.md`. The
  contract asks each stderr record for a `kind`; fields added to `wait_started` fit it and no existing field
  changes.
- ADRs to create: `None known`.

## Design and Ownership Review

`GhCli` already owns its child process, its absolute deadline and its signal handling, and `GhCli::new` already
takes the program, so the option adds no new ownership. The new element is one best-effort `api user` call, which
is a child process and so needs an explicit bound and cancellation:

- **Responsibility.** `main` builds two `GhCli` handles from the same program and the same `interrupted` flag:
  the ordinary one with the hard deadline `started + timeout`, and a lookup one whose deadline is
  `min(started + 10 s, started + timeout)`. The lookup handle runs `api user` and nothing else.
- **Lifetime.** A signal kills a running lookup through the shared `interrupted` flag, exactly as for any call.
  A lookup that is still running at its own deadline is killed and yields `gh_login: null`. No lookup child
  outlives the process.
- **Budget.** The lookup has its own short bound, so a stalled lookup cannot consume the wait's budget: it ends at
  most 10 seconds in, and the ordinary calls then run on the original deadline. The one exception to "a failed
  lookup never changes the result" is a `--timeout-seconds` smaller than 10: then the lookup deadline is the hard
  deadline, and a stalled lookup consumes the whole budget. This is the documented price of a timeout shorter
  than the lookup bound.
- **Place.** `wait_started` is written first, with `gh_command` only, as today, so callers and the signal
  scenarios that read it as "running" are not delayed. The lookup then runs and its result is written as a
  second stderr record `gh_identity` with `gh_command` and `gh_login`. (Decision D2.)
- **Checkpoint.** A design review after the first passing vertical slice (T3), before the tests are extended.

## Bug-Fix Process

Not applicable: this is an enhancement, not a defect.

## Regression Test Strategy

Not applicable: this is an enhancement, not a defect.

### Decisions

2026-11-11, AI assistant (Claude Code, Claude Sonnet 5.5), on the maintainer's instruction in the session of
this issue to use what the assistant considers best and document it with pros and cons. The maintainer's
concern: an AI agent must be able to tell which identity it runs as (an external contributor or the project
maintainer), and default values or unexpressive names can confuse an agent. The tool itself names no role or
account.

**D1. Make the identity explicit.** Four options were weighed.

| Option | Pros | Cons |
| ------ | ---- | ---- |
| A. Expressive name `--gh-command` instead of `--gh` | The name says what the value is. No run-time cost. | A name does not say which identity runs. Deviates from the backlog item A1 (`--gh`) in the name only. The default is still ambiguous. |
| B. Report what ran: `gh_command` and, best effort, `gh_login` | States facts that an agent or a person can check against the intended role. Works even when the wrong command is passed. Data in the output, not prose that can be skipped. Names no role or account. | One extra API call (`api user`) with its own bound. Adds fields and a record to stderr; the output contract asks each record for a `kind`, so this fits it, but it is new surface. The login is information: the poller cannot know whether it is the right one. |
| C. Always pass the command explicitly in the skill text; the code default stays `gh` | Agents learn the command of their role from instructions they read. Nothing existing breaks. | Depends on agents following text. The review-loop wording belongs to other sub-issues. The default remains for anyone who omits the option. |
| D. Required option, no default | Most explicit; no identity by omission. | Breaks every existing call and skill example. Forces a choice on callers that correctly use `gh`. A required option still does not tell the agent which identity it chose. |

**Decision: A + B, with C limited to documenting the option and the reported fields; D is rejected for this
issue.** B is the strongest answer to the concern: it makes the identity visible in the output rather than
relying on a name or a default. A is free and makes the intent clearer. D is too disruptive for a tight
sub-issue and can be its own issue once the skill text exists.

**D2. Details.**

- Option: `--gh-command <command>`, default `gh`. The command is one program name or path handed to
  `Command::new` as-is: never split on spaces, never run through a shell. It receives the same arguments `gh`
  receives today, so it must accept the `gh` command line.
- `gh auth status` runs only for the default `gh`, as today. A custom command skips it. An unusable or
  logged-out command is then reported by the first call that is not the lookup (the lookup swallows its own
  failure): `gh_unavailable` if it cannot start, `gh_api_error` if it fails. Both keep their kinds and exit code
  `1`. Message change: `gh_unavailable` already names the program; `gh_api_error` is changed to name the command
  too, and `gh_unavailable` stops calling any program "the GitHub CLI".
- Identity record: `wait_started` gains `gh_command` (always, no lookup needed). A separate `gh_identity` record
  follows with `gh_command` and `gh_login` (the login from `api user`; `null` when the lookup fails or times
  out). Putting the login in its own record keeps `wait_started` immediate (see the design review). The lookup is
  best effort and bounded as described above.
- Tests pass a fake script's path through `--gh-command` while `PATH` does not contain it, so the option is the
  only route to it. The existing scenarios keep their `PATH`-based fake and are not rewritten. No `PATH` shim
  is ever used.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                    | Notes / Expected Output                                                     |
| --- | ------ | ----------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| T1  | DONE   | Record the design decisions with pros and cons                          | D1 and D2 recorded.                                                         |
| T2  | TODO   | Add the failing tests for `--gh-command` (progressive loop)             | Red run recorded in `manual-verification-evidence.md`.                      |
| T3  | TODO   | Add the option, the auth pre-check rule, `gh_command` and the lookup    | Tests green; `--help` documents the option; design review after this slice. |
| T4  | TODO   | Document the option and the reported fields in the skill; run M1 to M3  | `SKILL.md` describes `--gh-command`, `gh_command` and `gh_identity`; evidence recorded. |

Test development uses the `write-unit-test` skill. Each test is the smallest behaviour-focused increment. After
each passes focused validation, a recorded design review (prose-first Arrange-Act-Assert: temporary prose per
section, refactor until the code expresses it, remove redundant prose, keep only irreducible context) is done
before the next test-producing subtask. The review records whether the shared `Sandbox` fixture of
`tests/cli.rs` still owns only incidental mechanics. The work stops for maintainer review after the final test
increment, before final verification, commit and pull request.

## Commit Points

| Task   | Coherent change set                              | Commit policy                                        |
| ------ | ------------------------------------------------ | ---------------------------------------------------- |
| T1     | The decisions in this spec                       | Done in the spec PR.                                 |
| T2, T3 | The tests and the option                         | Commit after focused validation and required review. |
| T4     | Skill wording and evidence                       | Commit after focused validation.                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted
- [x] GitHub issue created (#2822) and issue number added to this spec (created at the coordinator's
      instruction, before spec review, because the maintainer approved the sub-issue and its order)
- [x] Design decisions recorded
- [ ] Spec reviewed and approved by the maintainer (PR #2826)
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-11 11:53 UTC - AI assistant (Claude Code, Claude Sonnet 5.5) - Drafted the specification, created
  issue #2822 and recorded decisions D1 and D2 after the maintainer asked that agents can tell which identity
  they run as. The GitHub sub-issue link to EPIC #2003 could not be set with the bot account (the API answered
  404), so the issue text names the parent and the maintainer links it. Squashed the earlier drafts into one
  commit when addressing the first review of PR #2826.

## Acceptance Criteria

- [ ] AC1: `--gh-command <command>` makes the poller run that command for every GitHub call; with no option it
      runs `gh` as before.
- [ ] AC2: With a custom command, the poller does not run `auth status`; with the default it does, as before.
- [ ] AC3: The exit codes `0`, `1`, `2`, the existing record kinds and the JSON-only stderr are unchanged; a
      command that cannot be started is reported as `gh_unavailable` with exit code `1`.
- [ ] AC4: `wait_started` carries `gh_command`, and a `gh_identity` record carries `gh_command` and `gh_login`
      (`null` when the lookup fails or times out). A failed lookup does not change the result or the exit code
      when `--timeout-seconds` is at least the lookup bound; a stalled lookup ends at its own bound.
- [ ] AC5: The tests reach the fake command only through `--gh-command`, with no `PATH` shim, and the existing
      scenarios are no less reliable.
- [ ] AC6: `--help` and the skill document the option and the reported fields.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p wait-for-pr-review`
- `cargo clippy -p wait-for-pr-review --all-targets`
- Pre-push checks

### Manual Verification Scenarios

| ID | Scenario                                | Human-oriented command/steps                                                                                              | Expected Result                                                                 | Status | Evidence |
| -- | --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ------ | -------- |
| M1 | A fake command passed through the option | `wait-for-pr-review --pr-number 7 --gh-command <path to fake script> --timeout-seconds 5 \| jq .` with `PATH` lacking the script | One JSON result on stdout, no `auth status` call, exit code `0`                 | TODO   |          |
| M2 | A command that does not exist           | `wait-for-pr-review --pr-number 7 --gh-command /nonexistent > out.json 2> err.ndjson; echo "exit=$?"`                       | `out.json` empty, `gh_unavailable` in `err.ndjson`, `exit=1` (stdout is not a terminal, so no `tty_refusal`) | TODO   |          |
| M3 | The reported identity                   | Run M1's command with a fake that answers `api user` with a login, then with one that fails it                              | `gh_identity` shows `gh_command` and the login, then `gh_login: null`; the result and exit code are the same | TODO   |          |
| M4 | A stalled lookup                        | Run M1's command with a fake whose `api user` call sleeps past 10 seconds                                                   | `gh_login: null` after about 10 seconds; the wait continues and the result is unchanged | TODO   |          |

Evidence goes in issue-local `manual-verification-evidence.md`, naming the toolchain of each command.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- Skipping `auth status` for a custom command moves the "not logged in" report to the first non-lookup call,
  which yields `gh_api_error` instead of `gh_not_authenticated` for that case.
- A command that is not `gh`-compatible fails on the first call, not at start-up.
- The `gh_identity` record is new stderr surface. It fits the decided output contract (every record has a
  `kind`) and changes no existing record, but later contract changes must account for it.
- `gh_login` is information, not proof of the right role: the poller cannot know which login is intended.
- With `--timeout-seconds` below the lookup bound (10 seconds), a stalled lookup can use the whole budget.

## Implementation Completion Review

At close-out, record the outcome of the implementation: deviations from the plan, unexpected validation
findings and reusable lessons. Create an issue-local `implementation-retrospective.md` when there is any of
these; otherwise add a progress-log entry that explains why none is needed.

## References

- Related issues: #2581 (the poller), #2003 (parent EPIC), #2715 (CLI output contract scope, closed)
