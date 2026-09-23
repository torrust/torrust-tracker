---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2278
github-issue: 2318
spec-path: docs/issues/open/2318-2278-port-review-thread-scripts-to-rust/ISSUE.md
branch: "2318-2278-port-review-thread-scripts-to-rust"
related-pr: https://github.com/torrust/torrust-tracker/pull/2319
last-updated-utc: "2026-09-23 14:30"
semantic-links:
  skill-links:
    - create-issue
    - fetch-review-threads
    - resolve-review-threads
    - process-pr-review
  related-artifacts:
    - "issue #2278"
    - "issue #2003"
    - "issue #2266"
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
    - .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2318 - Port the Review-Thread Shell Scripts to a Rust Tool

**Parent EPIC:** #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

Subissue 3 of #2278. It replaces the four `fetch-review-threads` Bash and `jq` scripts with one
Rust workspace tool that reproduces their data exactly, so that subissue 4 can change what the
tool collects with fixture tests instead of editing untested `jq` filters.

## Goal

Provide a tested Rust binary under `contrib/dev-tools/` that fetches pull-request review threads
through GitHub CLI GraphQL and reproduces the data of the current `fetch-review-threads` scripts,
then remove those scripts and point the skills at the binary.

## Background

The `fetch-review-threads` skill ships four Bash scripts (`get-pr-review-threads.sh`,
`list-unresolved-threads.sh`, `show-unresolved-thread-bodies.sh`, and
`check-thread-reply-status.sh`) written quickly to unblock the review workflow. They have no tests,
their filtering logic lives in `jq` strings duplicated across files, and subissue 4 must change
that logic. Repository policy prefers Rust once developer tooling is non-trivial or worth testing
independently, and every other Rust developer tool already lives as a non-published workspace crate
under `contrib/dev-tools/` invoked with `cargo run --package`.

The Agent Skills specification allows any executable code in a skill's `scripts/` directory but a
Cargo crate cannot be built from there without becoming a workspace member; the tool therefore
lives with the other developer tools and the skill documents the `cargo run` invocation.

This subissue is parity only. Its value is the test seam and the removal of untested shell logic;
the behavior changes F17, F18, and F32 belong to subissue 4 so each pull request is reviewable on
its own.

## Scope

### In Scope

- Add a non-published workspace crate under `contrib/dev-tools/` with one binary offering
  subcommands equivalent to the four scripts: fetch threads to a JSON file, list unresolved threads,
  show unresolved thread bodies, and report per-thread reply status for a login.
- Reproduce the current GraphQL query fields and the exact JSON file shape written by
  `get-pr-review-threads.sh`, because `resolve-all-unresolved-threads.sh` and the historical
  evidence records consume that file.
- Reproduce the data of each script's stdout at field level (thread ID, path, URL, outdated flag,
  reply flag, summary counts, exit code semantics).
- Follow the global CLI output contract for a new `stdout-result-data` binary: JSON or NDJSON on
  stdout, JSON diagnostics on stderr, TTY refusal, and registration in the ADR classification table.
- Isolate the `gh api graphql` invocation behind a narrow trait so projection and filtering are
  unit-tested with checked-in GraphQL response fixtures without network or `gh` access.
- Delete the four Bash scripts, update `fetch-review-threads` and `resolve-review-threads` skill
  text and `semantic-links` to the binary, and search for live references to the removed paths.
- Record a manual capture against a real pull request and compare it with the last shell output.

### Out of Scope

- Changing which threads are returned or adding `resolvedBy` and `line` (subissue 4).
- Porting `resolve-review-threads` scripts, which mutate GitHub state; they keep consuming the
  JSON file. Their port is a candidate follow-up once this crate exists.
- Pagination beyond the current first 100 threads and 20 comments per thread.
- Replacing `gh` with a direct HTTP client and token handling; `gh` reuses the developer's
  existing authentication and is what the scripts use today.
- Any shared runner, cache, or CI integration for developer tools (EPIC #2003).

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`,
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
- Placement: the non-published `github-review-threads` workspace crate under
  `contrib/dev-tools/github/github-review-threads/` (the proposed directory for GitHub API tools;
  it is not a repository check, so not under `checks/`). This follows the `clippy-allow-reasons`
  and `frontmatter-validator` precedent and does not anticipate the #2266 or #2003 placement
  decisions for check crates.
- Output class: `stdout-result-data`. The human-readable body listing of
  `show-unresolved-thread-bodies.sh` becomes JSON; readers use `jq` for formatting. This is a
  deliberate format deviation recorded here; data parity is the requirement, byte parity is not.
- ADRs to create: `None known`.

## Design and Ownership Review

- Public interface: one binary with subcommands; a `ThreadSource` trait with a single method
  returning the GraphQL response for a pull request; a `GhCli` implementation that runs
  `gh api graphql` synchronously with `std::process::Command`; pure functions that project the
  response into each subcommand's output.
- Ownership: the binary owns the child process for the duration of one synchronous call and
  captures all stdout and stderr; no background tasks, sockets, or temporary files other than the
  requested output file.
- Failure and drop paths: a non-zero `gh` exit, malformed JSON, or an output file that cannot be
  written produces a JSON error on stderr and exit code `1`; nothing is written to stdout on failure.
- Deadline: `gh` applies its own HTTP timeouts; the tool adds none and documents that constraint.
  Tests never invoke `gh`; they inject fixture responses through the trait.
- Design-review checkpoint: after the `fetch` subcommand passes its fixture test and one real
  capture, review the trait boundary before porting the remaining subcommands.

## Bug-Fix Process

Not applicable. This task changes tooling language and test coverage without changing behavior.

## Regression Test Strategy

Not applicable as bug work. Parity is guarded by fixture tests: checked-in GraphQL responses
covering resolved, unresolved, outdated, and multi-comment threads, with expected outputs derived
from running the current shell scripts on the same fixtures before they are deleted.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Capture parity fixtures from the shell scripts | Minimal GraphQL fixture and shell-script golden outputs are committed as test data. |
| T2 | DONE | Add the crate with the `fetch` subcommand | Workspace member, `ThreadSource` trait, `gh` implementation, fixture tests, and ADR table row. |
| T3 | DONE | Port the three read-only projections | `list`, `show`, and `reply-status` subcommands have fixture-tested data parity. |
| T4 | TODO | Retire the scripts and update skills | Scripts deleted; `fetch-review-threads` and `resolve-review-threads` document the binary. |
| T5 | TODO | Verify and record completion evidence | Manual capture, automatic checks, acceptance review, parent EPIC row updated. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Fixture data and recorded shell outputs | One signed `test(dev-tools)` commit. |
| T2 | Crate skeleton, `fetch`, trait seam, ADR row | One signed `feat(dev-tools)` commit after the design-review checkpoint. |
| T3 | Remaining subcommands and tests | One signed `feat(dev-tools)` commit per subcommand when that keeps review small. |
| T4 | Script removal and skill updates | One signed `refactor(pr-reviews)` commit. |
| T5 | Evidence and EPIC tracking | One signed `docs(issues)` commit. |

For each test increment, use the `write-unit-test` skill and complete the prose-first
Arrange-Act-Assert review: the fixture is named for the thread state it contains, the projection
call is visible, and the expected output is stated independently of the code under test.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2278-port-review-thread-scripts-to-rust/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2278, and issue number added to this spec
- [x] Spec-only PR #2319 merged into `develop` before implementation
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

- 2026-09-23 12:10 UTC - GitHub Copilot - Drafted after the maintainer chose Rust for the `fetch-review-threads` helper and approved splitting the work into a parity port (this issue) and the F17/F18/F32 change (subissue 4); awaiting maintainer review.
- 2026-09-23 12:30 UTC - GitHub Copilot - Maintainer approved the specification; created GitHub sub-issue #2318 under EPIC #2278 and moved this specification to `docs/issues/open/2318-2278-port-review-thread-scripts-to-rust/`. Spec-only PR pending.
- 2026-09-23 13:02 UTC - GitHub Copilot - PR #2319 merged as `ef234623`; started implementation on branch `2318-2278-port-review-thread-scripts-to-rust` with T1, the parity-fixture baseline.
- 2026-09-23 13:02 UTC - GitHub Copilot - T1 captured a minimal deterministic GraphQL response fixture and shell-script golden outputs. The fixture covers resolved, unresolved, outdated, multi-comment, reply-present, and reply-missing states. A fixture-backed `gh` shim captured the `fetch` script without network access.
- 2026-09-23 14:25 UTC - GitHub Copilot - T2 added `github-review-threads fetch`, a fixture-tested `ThreadSource` seam, `GhCli` process adapter, response validation, and the ADR output-contract row. The design-review checkpoint passed after a fixture-backed command run and a real PR #2319 capture preserved the downstream GraphQL response shape.
- 2026-09-23 14:30 UTC - GitHub Copilot - T3 added fixture-tested `list`, `show`, and `reply-status` projections. `reply-status` preserves its missing-reply exit code while following the output contract: on missing replies, stdout is empty and stderr contains a JSON diagnostic; its detailed computed summary is covered by the unit test.

## Acceptance Criteria

- [ ] AC1: A workspace crate under `contrib/dev-tools/` provides `fetch`, unresolved `list`, unresolved `show`, and `reply-status` subcommands.
- [ ] AC2: `fetch` writes a JSON file whose shape is identical to the current `get-pr-review-threads.sh` output, and `resolve-all-unresolved-threads.sh` consumes it unchanged.
- [ ] AC3: Fixture tests prove each subcommand's output data matches the recorded shell output for resolved, unresolved, outdated, and multi-comment threads, including the reply-status summary and non-zero exit when a reply is missing.
- [ ] AC4: The binary complies with the CLI output contract (`stdout-result-data`, TTY refusal, JSON diagnostics on stderr) and is listed in the ADR classification table.
- [ ] AC5: No test invokes `gh` or the network; the GraphQL call sits behind a trait with a fixture implementation.
- [ ] AC6: The four Bash scripts are deleted and `fetch-review-threads` and `resolve-review-threads` document the `cargo run --package` invocation; no live documentation references the removed paths.
- [ ] `linter all` exits with code `0`.
- [ ] Crate tests and pre-push checks pass.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `cargo test --package <crate>` and `cargo clippy --package <crate> -- -D warnings`
- `cargo machete`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=text`
- Pre-push checks before opening the implementation pull request.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Fetch parity on a real PR | Run the shell `get-pr-review-threads.sh` and the Rust `fetch` against the same pull request before deleting the script; `diff` the two JSON files. | No difference other than GitHub-side changes between the two calls. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Read-only projections | Run `list`, `show`, and `reply-status` on the M1 capture and compare with the shell outputs. | Same thread IDs, paths, URLs, flags, counts, and exit code. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Downstream consumer | Run `resolve-all-unresolved-threads.sh --dry-run --threads-file` on the Rust-produced file. | The script lists the same unresolved thread IDs as with the shell-produced file. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Output contract | Run `fetch` with stdout attached to a terminal, then redirected. | TTY refusal with a JSON error on stderr; redirected run emits the summary JSON. | TODO | `manual-verification-evidence.md` section V4 |

Record the Rust toolchain used for every command result. No disposable verification script is
planned; parity checks are maintained crate tests.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Crate tests; M2 |
| AC2 | TODO | Fixture test; M1 and M3 |
| AC3 | TODO | Fixture tests |
| AC4 | TODO | M4; ADR table diff |
| AC5 | TODO | Test source review |
| AC6 | TODO | `rg` for removed script names; skill diff |

## Risks and Trade-offs

- `cargo run` adds compile time compared with a shell script. Accepted: the pre-commit checks
  already pay this cost for `clippy-allow-reasons`, and the tests remove a class of silent `jq`
  errors.
- Byte-level parity of the human-readable `show` output is impossible under the JSON-only output
  contract. Mitigation: define parity at field level and record the deviation in the spec.
- Deleting scripts breaks anyone with the old paths in muscle memory. Mitigation: skills are the
  single documented entry point and are updated in the same pull request.
- Shelling out to `gh` couples the tool to the CLI's JSON shape. Accepted for now; the trait seam
  keeps a later HTTP-client swap local to one implementation.

## Implementation Completion Review

- Retrospective: `Not yet assessed`.
- Create `implementation-retrospective.md` if the parity fixtures expose behavior the shell scripts
  produced by accident, or if the trait boundary needs rework after the design-review checkpoint.
- Otherwise, record why no retrospective was needed in the progress log.
- When an independent reviewer receives this folder-style specification, record the result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2278; grandparent EPIC: #2003.
- Decision input: `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md` (rows F17, F18, F32 name the owner; language decision recorded in the EPIC progress log).
- Placement precedent: `contrib/dev-tools/checks/clippy-allow-reasons`, `contrib/dev-tools/checks/frontmatter-validator` (#2266).
- Unblocks: #2278 subissue 4; provides thread data for subissue 9.
