---
doc-type: issue
issue-type: enhancement
status: planned
priority: p1
github-issue: 1780
spec-path: docs/issues/open/1780-refactor-pre-push-checks-performance-and-verbosity.md
branch: "1780-refactor-pre-push-checks-performance-and-verbosity"
related-pr: null
last-updated-utc: 2026-05-13 19:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/dev-tools/git/hooks/pre-push.sh
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - .github/workflows/testing.yaml
    - .github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md
    - .github/skills/dev/git-workflow/run-pre-push-checks/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #1780 - Refactor pre-push checks for output-mode parity and clearer failure feedback

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
- Add `TORRUST_GIT_HOOKS_LOG_DIR` env var for configurable per-step log directory (see
  [Design Decisions](#design-decisions)).
- Update `pre-commit.sh` to recognize `TORRUST_GIT_HOOKS_LOG_DIR` as a fallback (after
  `PRE_COMMIT_LOG_DIR`) for backward compatibility.
- Preserve existing pre-push validation steps, including E2E.
- Create a new `run-pre-push-checks` skill (parallel structure to `run-pre-commit-checks`).
- Update `run-pre-commit-checks` skill to document `TORRUST_GIT_HOOKS_LOG_DIR`.
- Update `AGENTS.md` to reference the new env var and pre-push output modes.

### Out of Scope

- Changing which checks run in pre-push.
- Moving E2E out of pre-push.
- CI workflow redesign.
- Broader hook framework rewrite into Rust CLI (future option only).

## Design Decisions

Decisions agreed with maintainer during planning (2026-05-13):

| Decision                                | Choice                                                                                         | Rationale                                                                                                   |
| --------------------------------------- | ---------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Log directory env var                   | `TORRUST_GIT_HOOKS_LOG_DIR` (shared across all hooks)                                          | `TORRUST_` prefix keeps tracker namespace clean; `GIT_HOOKS_` infix distinguishes from tracker runtime vars |
| `pre-commit.sh` backward compat         | Keep `PRE_COMMIT_LOG_DIR` as higher-priority override; `TORRUST_GIT_HOOKS_LOG_DIR` as fallback | Avoids breaking existing users of `PRE_COMMIT_LOG_DIR`                                                      |
| Skill docs strategy                     | New `run-pre-push-checks` skill (parallel to `run-pre-commit-checks`)                          | Keeps skills focused; mirrors pre-commit/pre-push symmetry                                                  |
| `--format=json` + `--verbosity=verbose` | JSON only; verbosity flag silently ignored in JSON mode                                        | Consistent with pre-commit behavior; keeps JSON output machine-parseable                                    |
| Failure behavior                        | Fail-fast — stop on first failure                                                              | Consistent with pre-commit; saves time on a broken state                                                    |

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                   | Notes / Expected Output                                                      |
| --- | ------ | ------------------------------------------------------ | ---------------------------------------------------------------------------- |
| T1  | DONE   | Define pre-push CLI/output contract                    | Decisions captured in [Design Decisions](#design-decisions)                  |
| T2  | TODO   | Refactor `pre-push.sh`                                 | Adds format/verbosity/log-dir parity; mirrors `pre-commit.sh` implementation |
| T3  | TODO   | Update `pre-commit.sh` for `TORRUST_GIT_HOOKS_LOG_DIR` | Add as fallback after `PRE_COMMIT_LOG_DIR`; update usage text                |
| T4  | TODO   | Create `run-pre-push-checks` skill                     | Parallel structure to `run-pre-commit-checks`; documents all output modes    |
| T5  | TODO   | Update `run-pre-commit-checks` skill                   | Add `TORRUST_GIT_HOOKS_LOG_DIR` fallback to env var docs                     |
| T6  | TODO   | Update `AGENTS.md`                                     | Document `TORRUST_GIT_HOOKS_LOG_DIR` and pre-push output modes               |
| T7  | TODO   | Validate behavior in pass and fail paths               | Text concise/verbose + JSON tested with exit-code verification               |
| T8  | TODO   | Run quality checks and finalize evidence               | `linter all` exits `0`; shellcheck passes on both hook scripts               |

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
- 2026-05-13 19:00 UTC - Copilot - Agreed design decisions with maintainer: `TORRUST_GIT_HOOKS_LOG_DIR` shared env var, new `run-pre-push-checks` skill, JSON-only in `--format=json`, fail-fast behavior. Implementation plan refined into T1–T8.

## Acceptance Criteria

- [ ] AC1: `pre-push.sh` supports `--format=<text|json>` and `--verbosity=<concise|verbose>` with `--verbose` alias.
- [ ] AC2: `--format=text --verbosity=concise` prints high-signal per-step summary; failures include log path and short tail.
- [ ] AC3: `--format=json` emits one valid JSON document to stdout with step-level status and timing.
- [ ] AC4: Invalid/unknown flags fail with exit code `2`, usage hint, and stderr diagnostics.
- [ ] AC5: Existing pre-push check ownership is preserved (including E2E in pre-push).
- [ ] AC6: `TORRUST_GIT_HOOKS_LOG_DIR` is the shared log-directory env var for all hooks, defaulting to
      `/tmp`. `pre-push.sh` uses it. `pre-commit.sh` uses it as a fallback after `PRE_COMMIT_LOG_DIR`.
      Both hooks document it in their usage text and in skill docs.
- [ ] AC7: `--format=json` emits JSON only regardless of `--verbosity` value (verbosity silently
      ignored in JSON mode).
- [ ] AC8: On first step failure, the hook stops immediately (fail-fast) and reports the failing
      step; subsequent steps are not run.
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
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |

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
