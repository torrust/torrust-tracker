---
doc-type: issue
issue-type: enhancement
status: planned
priority: p1
github-issue: null
spec-path: docs/issues/open/1770-refactor-pre-push-checks-performance-and-verbosity.md
branch: "1770-refactor-pre-push-checks-performance-and-verbosity"
related-pr: null
last-updated-utc: 2026-05-13 13:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/dev-tools/git/hooks/pre-push.sh
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - .github/workflows/testing.yaml
    - .github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Refactor pre-push checks for output-mode parity and clearer failure feedback

## Goal

Refactor the pre-push hook to align its operator experience with the new pre-commit behavior:
concise output by default, verbose streaming on demand, and structured JSON output for automation.

## Background

Issue #1769 introduced a stronger CLI and reporting contract for pre-commit, including:

- `--format=<text|json>`
- `--verbosity=<concise|verbose>` and `--verbose` alias
- concise per-step summaries with log-path and failure tail
- optional workspace-local log directory via environment variable

`contrib/dev-tools/git/hooks/pre-push.sh` still uses legacy output behavior. This creates an
inconsistent local workflow and weaker automation ergonomics in the heavier validation gate.

Because pre-push includes nightly checks and E2E, this refactor should keep the check set intact
while improving clarity, observability, and parity with pre-commit.

## Scope

### In Scope

- Add `--format=<text|json>` to pre-push with `text` as default.
- Add `--verbosity=<concise|verbose>` with `concise` as default.
- Keep `--verbose` as alias for `--verbosity=verbose`.
- Add concise failure summaries (step, status, elapsed, log path, failure tail).
- Add JSON output mode with one structured payload to stdout.
- Add configurable per-step log directory env var (follow pre-commit contract).
- Preserve existing pre-push validation steps, including E2E.
- Update docs/skills so pre-commit and pre-push behavior is consistent.

### Out of Scope

- Changing which checks run in pre-push.
- Moving E2E out of pre-push.
- CI workflow redesign.
- Broader hook framework rewrite into Rust CLI (future option only).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                     | Notes / Expected Output                                        |
| --- | ------ | ---------------------------------------- | -------------------------------------------------------------- |
| T1  | TODO   | Define pre-push CLI/output contract      | Final behavior matrix and error handling documented            |
| T2  | TODO   | Implement hook refactor                  | `pre-push.sh` supports format/verbosity/log-dir parity         |
| T3  | TODO   | Validate behavior in pass and fail paths | Text concise/verbose + JSON tested with exit-code verification |
| T4  | TODO   | Update docs and skills                   | Workflow docs aligned with pre-push capabilities               |
| T5  | TODO   | Run quality checks and finalize evidence | `linter all` and targeted checks pass                          |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-13 13:00 UTC - Copilot - Drafted follow-up issue for pre-push parity with #1769 (output modes, summaries, JSON, log-dir configurability).

## Acceptance Criteria

- [ ] AC1: `pre-push.sh` supports `--format=<text|json>` and `--verbosity=<concise|verbose>` with `--verbose` alias.
- [ ] AC2: `--format=text --verbosity=concise` prints high-signal per-step summary; failures include log path and short tail.
- [ ] AC3: `--format=json` emits one valid JSON document to stdout with step-level status and timing.
- [ ] AC4: Invalid/unknown flags fail with exit code `2`, usage hint, and stderr diagnostics.
- [ ] AC5: Existing pre-push check ownership is preserved (including E2E in pre-push).
- [ ] AC6: Log-directory override env var is supported and documented (parity with pre-commit behavior).
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Documentation is updated when behavior/workflow changes

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

- Pre-push is already long-running; additional wrapper logic can increase complexity.
  - Mitigation: keep refactor scoped to output/logging contract, without changing command set.
- JSON/log-tail formatting can drift from pre-commit if implemented separately.
  - Mitigation: explicitly mirror field names and argument semantics.
- In constrained environments, log directory permissions can fail.
  - Mitigation: keep default `/tmp` and support workspace-local override.

## References

- Related issues: #1769
- Related PRs: none
- Related ADRs: none
- Hook scripts: `contrib/dev-tools/git/hooks/pre-commit.sh`, `contrib/dev-tools/git/hooks/pre-push.sh`
