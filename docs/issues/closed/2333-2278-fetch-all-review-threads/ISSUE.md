---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2278
github-issue: 2333
spec-path: docs/issues/closed/2333-2278-fetch-all-review-threads/ISSUE.md
branch: "2333-2278-fetch-all-review-threads"
related-pr: 2339
last-updated-utc: "2026-09-26 09:46"
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
    - contrib/dev-tools/github/github-review-threads/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #2333 - Fetch All Pull Request Review Threads

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
resolved before. The `fetch-review-threads` skill still describes and filters to unresolved
threads, the `github-review-threads` `list` and `show` subcommands return unresolved threads only,
and the tool's GraphQL query omits `resolvedBy` and `line`, so that evidence is hidden from the
author (F17, F32) and cheap resolver evidence is never captured (F18). The skill's inline `gh api
graphql` fallback query has also drifted from the tool's query: it reads `path` from comments and
omits `url`, `createdAt`, and `isCollapsed`.

Subissue 3 (#2318, merged in PR #2322) ported the helper to the tested Rust crate
`contrib/dev-tools/github/github-review-threads` with data parity. This subissue changes the tool's
behavior with fixture tests and updates the skill contract; it does not touch the bulk-resolve
action guard, which is correctly unresolved-only.

This subissue inherits the output shape #2318 recorded as deliberate deviations from the retired
scripts, and does not reopen it:

- `list` and `show` emit one JSON object with a `threads` array on stdout (`stdout-result-data`).
- `reply-status --login` is required; a missing reply exits `1` with empty stdout and a
  `missing_reply` stderr record carrying `threads` and `summary`.
- `--help` and usage errors are JSON `usage_error` records on stderr with exit code `2`; the TTY
  check runs first.

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
- Align the skill's inline `gh api graphql` fallback query with the tool's query, including the new
  fields, so both evidence paths return the same data.
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

Not applicable. The `ThreadSource` trait seam, process ownership, and failure paths are defined by
subissue 3; this issue adds fields to the query, the response model, and the projection functions
only, and adds one flag to `list` and `show`.

No new script, crate, or binary is created. The change extends the existing
`github-review-threads` crate in place and does not move it into another crate or a unified
AI-harness binary; that placement remains an EPIC #2003 decision, and the crate's library/binary
split keeps it absorbable by whatever #2003 selects.

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
| T1 | DONE | Extend the query and projections | `resolvedBy` and `line` captured; `list` and `show` return all threads with `--unresolved-only`. |
| T2 | DONE | Add behavior fixture tests | Resolved, outdated, resolved-by, null resolver/line, and flag cases pass; `reply-status` unchanged. |
| T3 | DONE | Rewrite the skill contract | `fetch-review-threads` states the all-thread evidence rule and the action-only filter; its fallback query matches the tool's. |
| T4 | DONE | Verify and record completion evidence | Manual capture, automatic checks, acceptance review, parent EPIC row and AC2 updated. |

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
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2278, and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, crate tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-23 10:15 UTC - GitHub Copilot - Drafted from the approved #2278 matrix entries F17, F18, and F32 as a change to the Bash scripts; local inspection confirmed that `process-pr-review` requires all threads while the helper's display scripts filter `isResolved == false`.
- 2026-09-23 12:10 UTC - GitHub Copilot - Maintainer chose Rust for the helper; redrafted on top of the parity port in subissue 3 and renumbered as subissue 4. Awaiting maintainer review.
- 2026-09-24 15:08 UTC - GitHub Copilot - Refreshed after #2318 merged (PR #2322): Background now names the delivered `github-review-threads` crate and the output-shape deviations this issue inherits; scope adds aligning the skill's drifted inline fallback query. The 2026-09-23 approval predates the #2318 implementation, so re-approval is requested before GitHub issue creation.
- 2026-09-24 16:07 UTC - GitHub Copilot - Maintainer re-approved the refreshed specification, including the note that the change extends the existing crate rather than a new or unified binary. Created GitHub issue #2333, linked it as a sub-issue of #2278, and moved this specification to `docs/issues/open/`. Spec-only PR pending.
- 2026-09-24 17:12 UTC - GitHub Copilot - Spec-only PR #2334 merged as `a39935ca`; implementation started on the reserved `2333-2278-fetch-all-review-threads` branch.
- 2026-09-24 17:28 UTC - GitHub Copilot - T1 and T2 done. The query requests each thread's `line` and `resolvedBy { login }`; `list` and `show` take a `ThreadSelection` (`All` by default, `UnresolvedOnly` through `--unresolved-only`) and emit `isResolved`, `resolvedBy`, and `line` as additive fields; `reply-status` still filters to unresolved threads. The retired-script fixture `tests/fixtures/review-threads.json` stays unchanged because its README pins it to the parity captures; it now also proves that files fetched before this change still parse, with `resolvedBy` and `line` as null. A new `tests/fixtures/thread-states.json` names each thread for its state (resolved by a login, resolved and outdated, resolved without a resolver, unresolved current, unresolved outdated with a null line). The CLI parity tests compare the retired-script fields under `--unresolved-only`, and new CLI tests show that `list` and `show` include a resolved thread by default. Prose-first review: each test's Arrange names the thread state or selection that changes the result, the production `list`/`show`/`reply_status` call stays visible in Act, and assertions check one field or result each; no temporary prose was kept. Proof that the tests guard the change: temporarily reverting `ThreadSelection::includes` to the unresolved-only filter in the working tree failed 5 of 13 unit tests; the filter was then restored. Evidence (stable Rust 1.98.1): `cargo test --package github-review-threads` passes 13 unit and 7 CLI tests; `cargo clippy --package github-review-threads --all-targets -- -D warnings` is clean; `cargo +nightly fmt --all -- --check` is clean. Real-query check ahead of the T4 manual capture: `fetch --pr-number 2322` returned 13 resolved threads, all resolved by `josecelano`, 8 of them outdated with `line: null`.
- 2026-09-24 18:13 UTC - GitHub Copilot - T3 done. The `fetch-review-threads` skill (version 1.1) now: collects every thread as evidence; names `resolvedBy`, `path`, and `line` as self-audit evidence, with nulls recorded as unknown; limits unresolved-only filtering to reply or resolution actions, through `--unresolved-only` or the unchanged `reply-status`; and rewrites the completion checklist to match. The skill had listed `canResolve`, which neither query requests, so it was dropped from the collected fields. The inline `gh api graphql` fallback now carries the tool's query. A whitespace-insensitive `diff` of the two query texts is empty. Run against PR #2322, the fallback query returned the same 13 threads, resolver set (`josecelano`), and 8 null lines as the tool. The only live caller of `list` or `show` is this skill; `resolve-review-threads` uses `reply-status` and the raw file, both unchanged.
- 2026-09-24 18:46 UTC - GitHub Copilot - T4 done. M1-M4 were run read-only against merged PR #2320 and all passed (29 threads: 27 resolved, 2 unresolved, 21 outdated); evidence is in `manual-verification-evidence.md`. Automatic checks (stable Rust 1.98.1; nightly for the pre-push nightly steps): the pre-commit hook, which includes `linter all`, passed on both implementation commits, and `pre-push.sh --format=text` passed all 4 steps, including the full stable test suite. Acceptance criteria AC1-AC6 were re-reviewed against the current tree; see the Acceptance Verification table. Completion review: no retrospective is needed, because the implementation followed the plan without a material design change. The one deviation was dropping `canResolve` from the skill's collected fields (T3 entry). The capture's survey found five merged PRs that still hold unresolved threads (#2290, #2293, #2300, #2313, #2320); they were left untouched under the post-merge rule and reported to the maintainer.
- 2026-09-24 20:11 UTC - GitHub Copilot - Opened implementation PR #2339 targeting `develop`; it closes #2333 when merged.
- 2026-09-25 08:26 UTC - GitHub Copilot - PR #2339 merged into `develop` as `a20e8f3a` after a rebase onto the latest `develop`; GitHub closed #2333. The reviewer (da2ce7) recomputed the acceptance criteria in review 5310586223 before approving, and left three non-blocking inline findings.
- 2026-09-26 09:46 UTC - GitHub Copilot - Those findings were still unprocessed at merge. With maintainer approval (<https://github.com/torrust/torrust-tracker/pull/2339#issuecomment-5844522320>), a follow-up branch from `develop` fixes the two test findings in `test(dev-tools): pin review-thread query fields and explicit nulls` and the evidence finding in `docs(issues): record the exact #2333 evidence commands`. Archived the specification to `docs/issues/closed/` after confirming the GitHub issue is `CLOSED`; EPIC #2278 order 4 marked `DONE`.

## Acceptance Criteria

- [x] AC1: The `fetch-review-threads` skill states that evidence collection includes resolved and outdated threads and reserves unresolved-only filtering for reply or resolution actions; its inline fallback query requests the same fields as the tool.
- [x] AC2: The GraphQL query records `isResolved`, `isOutdated`, `resolvedBy`, `path`, and `line` for every returned thread, and the JSON file remains a superset accepted by `resolve-all-unresolved-threads.sh`.
- [x] AC3: Default `list` and `show` output includes resolved, unresolved, and outdated threads; `--unresolved-only` excludes resolved ones.
- [x] AC4: `reply-status` still evaluates unresolved threads only.
- [x] AC5: Fixture tests cover resolved, unresolved, outdated, resolved-by-login, and null resolver/line cases and the flag.
- [x] AC6: A manual GitHub capture shows a resolved and an outdated thread with their resolver and line.
- [x] `linter all` exits with code `0`.
- [x] Crate tests and pre-push checks pass.
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `cargo test --package github-review-threads` and
  `cargo clippy --package github-review-threads --all-targets -- -D warnings`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=text`
- Pre-push checks before opening the implementation pull request.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Capture all threads | Run `fetch` and default `list` against a pull request with resolved and outdated feedback. | Resolved, unresolved, and outdated threads present on GitHub all appear. | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Inspect resolution evidence | Inspect a resolved thread in `show` output. | `resolvedBy`, `path`, and `line` identify who resolved it and where; absent values are explicit nulls. | DONE | `manual-verification-evidence.md` section V2 |
| M3 | Action-only views | Run `list --unresolved-only` and `reply-status` on the same capture. | Only unresolved threads appear. | DONE | `manual-verification-evidence.md` section V3 |
| M4 | Downstream consumer | Run `resolve-all-unresolved-threads.sh --dry-run --threads-file` on the new file. | The script still lists the unresolved thread IDs. | DONE | `manual-verification-evidence.md` section V4 |

Record the Rust toolchain used for every command result. No disposable verification script is
planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Skill diff in `docs(pr-reviews): collect all review threads as self-audit evidence`; whitespace-insensitive query `diff` is empty (T3 entry). |
| AC2 | DONE | `REVIEW_THREADS_QUERY` requests `line` and `resolvedBy { login }`; `it_should_list_the_resolution_evidence_of_a_resolved_thread`; M2 and M4. |
| AC3 | DONE | `it_should_list_resolved_unresolved_and_outdated_threads_by_default`, `it_should_list_only_unresolved_threads_when_unresolved_only_is_selected`, and the CLI default-view tests; M1 and M3. |
| AC4 | DONE | `it_should_check_replies_on_unresolved_threads_only`; M3 (2 of 29 threads checked). |
| AC5 | DONE | `tests/fixtures/thread-states.json` and the 13 unit and 7 CLI tests; the mutation check failed 5 unit tests. |
| AC6 | DONE | `manual-verification-evidence.md` V2 (PR #2320). |

## Risks and Trade-offs

- Showing every thread makes action triage noisier. Mitigation: `--unresolved-only` and the
  unchanged `reply-status` guard keep the action view narrow.
- A thread can lack a resolver or line (review-body threads, deleted users). Mitigation: keep
  explicit nulls and never synthesize evidence.
- Adding fields to the JSON file could break a consumer that validates shape strictly. Mitigation:
  M4 exercises the only known consumer; fields are additive.

## Implementation Completion Review

- Retrospective: Not needed; see the 2026-09-24 18:46 UTC progress log entry.
- Create `implementation-retrospective.md` if the real capture exposes pagination or API-shape
  constraints that change scope.
- Otherwise, record why no retrospective was needed in the progress log.
- When an independent reviewer receives this folder-style specification, record the result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2278; grandparent EPIC: #2003.
- Decision input: `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md` (F17, F18, F32).
- Prerequisite: #2318 (merged in PR #2322), `docs/issues/closed/2318-2278-port-review-thread-scripts-to-rust/ISSUE.md`.
- Tool: `contrib/dev-tools/github/github-review-threads/`.
- Consuming workflow: `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`.
