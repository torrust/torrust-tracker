---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2278
github-issue: null
spec-path: docs/issues/drafts/2278-fetch-all-review-threads/ISSUE.md
branch: "{issue-number}-2278-fetch-all-review-threads"
related-pr: null
last-updated-utc: "2026-09-24 08:01"
semantic-links:
  skill-links:
    - create-issue
    - fetch-review-threads
    - process-pr-review
  related-artifacts:
    - "issue #2278"
    - "issue #2003"
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md
    - docs/issues/closed/2318-2278-port-review-thread-scripts-to-rust/ISSUE.md
    - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Fetch All Pull Request Review Threads

**Parent EPIC:** #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

Subissue 4 of #2278. It implements adopted friction-register items F17, F18, and F32 on the Rust
review-thread tool delivered by #2318: the author's evidence view must include resolved and
outdated threads, with the resolver and line needed to audit an earlier resolution.

## Goal

Make the review-thread tool collect and present every pull-request review thread that
`process-pr-review` needs, while keeping an explicit unresolved-only view for reply and resolution
actions.

## Background

`process-pr-review` already requires the GraphQL thread source of truth to include resolved and
outdated threads, because re-raise detection and the self-audit compare a new comment with what was
resolved before. The `fetch-review-threads` helper still describes and filters to unresolved
threads, and its GraphQL query omits `resolvedBy` and `line`, so that evidence is hidden from the
author (F17, F32) and cheap resolver evidence is never captured (F18).

Subissue 3 ports the helper to a tested Rust tool with parity. This subissue changes the tool's
behavior with fixture tests and updates the skill contract; it does not touch the bulk-resolve
action guard, which is correctly unresolved-only.

## Scope

### In Scope

- Extend the GraphQL query with each thread's `resolvedBy { login }` and `line`, preserving all
  existing fields so the JSON file remains a superset consumed unchanged by
  `resolve-all-unresolved-threads.sh`.
- Make the `list` and `show` subcommands return every fetched thread by default, exposing
  `isResolved`, `isOutdated`, `resolvedBy`, `path`, `line`, and comment data; offer an explicit
  `--unresolved-only` flag for action triage.
- Keep `reply-status` unresolved-only, since it guards a resolution action.
- Add fixture tests for resolved, unresolved, outdated, resolved-by-a-login, and null
  resolver/line threads, and for the `--unresolved-only` flag.
- Rewrite the `fetch-review-threads` skill so it describes evidence collection over all threads,
  names `resolvedBy` and `line` as self-audit evidence, and reserves unresolved-only filtering for
  reply or resolution actions; update the completion checklist accordingly.
- Perform a real GitHub capture from a pull request with at least one resolved and one outdated
  thread and record it in issue-local manual verification evidence.
- Update the parent EPIC's subissue table and AC2 evidence.

### Out of Scope

- Resolving, reopening, replying to, or changing the state of any GitHub review thread.
- The self-audit gate, audit template, and audit validator (subissues 5, 7, 8).
- Changing `resolve-review-threads` scripts or their unresolved-only semantics.
- Pagination beyond the first 100 threads; a follow-up only if the manual capture shows a real
  pull request exceeding it.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`,
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
- ADRs to create: `None known`. This is a behavior change within the tool and placement decided
  by subissue 3.

## Design and Ownership Review

Not applicable. The trait seam, process ownership, and failure paths are defined by subissue 3;
this issue adds fields to the query and projection functions only.

## Bug-Fix Process

Not applicable. The helper contract was incomplete rather than broken at runtime; the change is
governed by the EPIC's approved matrix rather than a defect reproduction.

## Regression Test Strategy

Not applicable as bug work. Fixture tests assert that a resolved and an outdated thread appear in
default `list` and `show` output and that `--unresolved-only` and `reply-status` exclude them.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Extend the query and projections | `resolvedBy` and `line` captured; `list` and `show` return all threads with `--unresolved-only`. |
| T2 | TODO | Add behavior fixture tests | Resolved, outdated, resolved-by, null resolver/line, and flag cases pass; `reply-status` unchanged. |
| T3 | TODO | Rewrite the skill contract | `fetch-review-threads` states the all-thread evidence rule and the action-only filter. |
| T4 | TODO | Verify and record completion evidence | Manual capture, automatic checks, acceptance review, parent EPIC row and AC2 updated. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 + T2 | Query fields, projections, and their fixture tests | One signed `feat(dev-tools)` commit after focused validation and test-design review. |
| T3 | Skill contract | One signed `docs(pr-reviews)` commit. |
| T4 | Evidence and EPIC tracking | One signed `docs(issues)` commit. |

For T2, use the `write-unit-test` skill and complete the prose-first Arrange-Act-Assert review
before committing: each fixture is named for the thread state that makes it appear or disappear,
the projection call is visible, and each expected field is asserted independently.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2278-fetch-all-review-threads/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created, linked as a sub-issue of #2278, and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, crate tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-23 10:15 UTC - GitHub Copilot - Drafted from the approved #2278 matrix entries F17, F18, and F32 as a change to the Bash scripts; local inspection confirmed that `process-pr-review` requires all threads while the helper's display scripts filter `isResolved == false`.
- 2026-09-23 12:10 UTC - GitHub Copilot - Maintainer chose Rust for the helper; redrafted on top of the parity port in subissue 3 and renumbered as subissue 4. Awaiting maintainer review.

## Acceptance Criteria

- [ ] AC1: The `fetch-review-threads` skill states that evidence collection includes resolved and outdated threads and reserves unresolved-only filtering for reply or resolution actions.
- [ ] AC2: The GraphQL query records `isResolved`, `isOutdated`, `resolvedBy`, `path`, and `line` for every returned thread, and the JSON file remains a superset accepted by `resolve-all-unresolved-threads.sh`.
- [ ] AC3: Default `list` and `show` output includes resolved, unresolved, and outdated threads; `--unresolved-only` excludes resolved ones.
- [ ] AC4: `reply-status` still evaluates unresolved threads only.
- [ ] AC5: Fixture tests cover resolved, unresolved, outdated, resolved-by-login, and null resolver/line cases and the flag.
- [ ] AC6: A manual GitHub capture shows a resolved and an outdated thread with their resolver and line.
- [ ] `linter all` exits with code `0`.
- [ ] Crate tests and pre-push checks pass.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `cargo test --package <crate>` and `cargo clippy --package <crate> -- -D warnings`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=text`
- Pre-push checks before opening the implementation pull request.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Capture all threads | Run `fetch` and default `list` against a pull request with resolved and outdated feedback. | Resolved, unresolved, and outdated threads present on GitHub all appear. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Inspect resolution evidence | Inspect a resolved thread in `show` output. | `resolvedBy`, `path`, and `line` identify who resolved it and where; absent values are explicit nulls. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Action-only views | Run `list --unresolved-only` and `reply-status` on the same capture. | Only unresolved threads appear. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Downstream consumer | Run `resolve-all-unresolved-threads.sh --dry-run --threads-file` on the new file. | The script still lists the unresolved thread IDs. | TODO | `manual-verification-evidence.md` section V4 |

Record the Rust toolchain used for every command result. No disposable verification script is
planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Skill diff |
| AC2 | TODO | Fixture tests; M2 and M4 |
| AC3 | TODO | Fixture tests; M1 and M3 |
| AC4 | TODO | Fixture tests; M3 |
| AC5 | TODO | Fixture tests |
| AC6 | TODO | `manual-verification-evidence.md` |

## Risks and Trade-offs

- Showing every thread makes action triage noisier. Mitigation: `--unresolved-only` and the
  unchanged `reply-status` guard keep the action view narrow.
- A thread can lack a resolver or line (review-body threads, deleted users). Mitigation: keep
  explicit nulls and never synthesize evidence.
- Adding fields to the JSON file could break a consumer that validates shape strictly. Mitigation:
  M4 exercises the only known consumer; fields are additive.

## Implementation Completion Review

- Retrospective: `Not yet assessed`.
- Create `implementation-retrospective.md` if the real capture exposes pagination or API-shape
  constraints that change scope.
- Otherwise, record why no retrospective was needed in the progress log.
- When an independent reviewer receives this folder-style specification, record the result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2278; grandparent EPIC: #2003.
- Decision input: `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md` (F17, F18, F32).
- Prerequisite: #2318, `docs/issues/closed/2318-2278-port-review-thread-scripts-to-rust/ISSUE.md`.
- Consuming workflow: `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`.
