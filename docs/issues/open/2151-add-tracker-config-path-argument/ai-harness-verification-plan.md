---
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - docs/templates/ISSUE.md
    - docs/templates/MANUAL-VERIFICATION-EVIDENCE.md
    - docs/testing.md
    - tests/AGENTS.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - .github/agents/
    - docs/issues/open/2151-add-tracker-config-path-argument/ISSUE.md
---

# AI Harness Verification Guidance Plan

## Purpose

Define and document three distinct forms of verification so AI agents produce
durable automated tests, credible manual verification evidence, and auditable
temporary automation without conflating their roles.

This plan is a documentation-and-guidance change. It does not alter tracker
behavior or replace issue #2151's remaining manual verification work. Review
and approve this plan before implementing any task below.

## Definitions

### Automatic Tests

Maintained test code in the repository that makes repeatable claims about
product behavior. It belongs at the lowest suitable test layer, runs through
the normal Rust test toolchain, and remains as regression coverage.

All tracked repository test code is Rust. This includes test runners, fixtures,
assertion helpers, and scripts that automate test inputs or test assertions.

### Manual Verification

A real, human-oriented use of the completed feature or reproduction of the
fixed bug. An agent executes the specified commands or interactions against the
real artifact and records what actually happened: prerequisites, exact steps,
commands, program output, relevant tracker logs, and the resulting status.

Manual verification complements automated tests; it is not simulated output,
and it is not satisfied merely by running a test command. One issue may record
multiple manual-verification processes in its
`manual-verification-evidence.md` artifact.

### Disposable Verification Scripts

Temporary, issue-local automation used to execute or capture a verification
scenario efficiently. It is neither maintained automatic test code nor the
manual-verification evidence itself.

An agent may create such a script only when it explains in the issue
specification why that concrete scenario is better served by temporary
automation than by a maintained Rust automatic test. The script and the
rationale must be tracked inside the issue-specification folder so later
reviewers can inspect what was verified. Python is discouraged: an agent that
selects it over Rust must record why Rust was not a suitable choice for that
specific script.

When a disposable script proves durable product behavior, promote that behavior
to maintained Rust automatic tests when practical, then remove the script. If
the script remains, the issue must document why it is still necessary and who
owns its removal.

## Design Decisions

- `docs/templates/ISSUE.md` remains the source of the mandatory
  manual-verification requirement and points to a standard issue-local evidence
  artifact.
- `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` is the reusable format for
  actual manual runs. The artifact is created only when an issue performs
  manual verification; the template itself does not assert that every issue has
  identical scenarios.
- `tests/AGENTS.md` remains the operational authority for the Rust-only tracked
  test-code rule. `docs/testing.md` links to that authority rather than
  restating its exceptions.
- The `create-issue` skill governs issue-specification structure and evidence.
  The `write-unit-test` skill governs maintained automatic tests. Both must
  distinguish disposable verification scripts from their primary responsibility.
- Custom agents that plan, implement, review, or commit issue work must follow
  the same definitions. They should link to the repository-owned guidance,
  rather than reproduce policy text independently.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status      | Task                         | Expected result                                                                                                                                                                                                                                                                                                     |
| --- | ----------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| H1  | DONE        | Add manual-evidence template | Added `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` with purpose, environment, multiple scenario records, actual steps, observed output/logs, conclusions, and follow-up. It forbids invented evidence.                                                                                                          |
| H2  | DONE        | Update issue template        | `docs/templates/ISSUE.md` now requires issue-local manual evidence, defines real human-oriented manual verification, and records the rationale, location, and ownership rules for disposable scripts.                                                                                                               |
| H3  | DONE        | Update testing guidance      | `docs/testing.md` distinguishes all three verification types; `tests/AGENTS.md` retains the authoritative Rust-only test-code policy and links to that guidance.                                                                                                                                                    |
| H4  | DONE        | Update repository skills     | Updated `create-issue` and `write-unit-test` with manual-evidence, disposable-script-rationale, Rust preference, and durable-test-promotion requirements.                                                                                                                                                           |
| H5  | DONE        | Update custom agents         | Updated Planner, Implementer, Task Reviewer, and Committer: the roles that plan, perform, validate, and finalize issue work. They link to or enforce the canonical verification guidance.                                                                                                                           |
| H6  | IN_PROGRESS | Apply the policy to #2151    | The issue records the removed Python verifier as historical disposable automation, and M1-M5 are reset for real release-style manual runs. Create `manual-verification-evidence.md` and record actual results.                                                                                                      |
| H7  | DONE        | Validate documentation       | `validate-skill-links.sh`, `linter markdown`, `linter cspell`, `linter lychee`, and `git diff --check` passed. Reviewed the resulting guidance: `tests/AGENTS.md` remains the authoritative Rust-only test-code rule; other artifacts link to it or to `docs/testing.md` rather than defining competing exceptions. |

## Acceptance Criteria

- [ ] The repository documents automatic tests, manual verification, and
      disposable verification scripts as distinct activities with clear purposes.
- [ ] New issue specs require actual, issue-local manual-verification evidence
      at `manual-verification-evidence.md` when scenarios are executed.
- [ ] A disposable verification script must be issue-local and accompanied by a
      concrete rationale for using temporary automation instead of a maintained
      automatic test.
- [ ] Python use in disposable scripts requires a recorded case-specific reason
      for not using Rust.
- [ ] Maintained, tracked test code remains Rust-only.
- [ ] Relevant templates, repository skills, and custom-agent instructions link
      to consistent, repository-owned guidance.
- [ ] Documentation linters and local link checks pass.

## Risks and Trade-offs

- Requiring evidence artifacts can create boilerplate. The manual-evidence
  template should therefore allow one concise scenario while retaining enough
  structure to distinguish observed output from an expected result.
- Forcing every temporary investigation into Rust would make some operational
  checks needlessly expensive. The rationale requirement keeps the exception
  available while making its cost and removal explicit.
- Repeating policy in templates, skills, and agents risks divergence. The
  implementation should keep full definitions in the testing guidance and use
  links plus focused workflow requirements elsewhere.

## Validation Plan

Before completing the implementation, create a small representative
`manual-verification-evidence.md` artifact in this issue and verify that it can
record actual commands, output, and logs for more than one scenario. Confirm
that every new or updated disposable-script instruction requires both the
automatic-test rationale and, for Python, the language-choice rationale.

Run the applicable repository validation scripts for updated skills or agents,
then run `linter markdown`, `linter cspell`, and local link checking.
