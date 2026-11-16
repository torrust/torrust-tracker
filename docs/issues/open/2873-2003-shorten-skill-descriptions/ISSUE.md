---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p1
epic: 2003
github-issue: 2873
spec-path: docs/issues/open/2873-2003-shorten-skill-descriptions/ISSUE.md
branch: "2873-shorten-skill-descriptions-spec"
related-pr: null
last-updated-utc: "2026-11-16 08:53"
semantic-links:
  skill-links:
    - create-issue
    - add-new-skill
    - fix-bug
    - write-markdown-docs
  related-artifacts:
    - .github/skills
    - contrib/dev-tools/checks/frontmatter-validator
    - docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/open/2873-2003-shorten-skill-descriptions/manual-verification-evidence.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - "issue #2866"
---

<!-- skill-link: create-issue -->

# Issue #2873 - Shorten the agent skill descriptions so they fit Claude Code's skill listing budget

Parent EPIC: #2003 - Overhaul: Automation Tools and AI Agent Guardrails (order 22).

## Goal

Shorten and retune the `description` of each of the 53 project skills, so the skill listing
Claude Code loads into every session fits its budget with room to spare, while each skill still
loads for the tasks it is meant for. Fix the `write-markdown-docs` description, which YAML cuts
short, and add a check so no description is cut or grows too long again.

## Background

Issue #2866 moves the skills to `.claude/skills/<name>/SKILL.md`, where Claude Code discovers
them. Once discovered, every skill's name and description is loaded into every session. The
maintainer decided on 2026-11-15 that shortening the descriptions is this separate issue, so the
move stays a reviewable rename (#2866, task T1).

### The listing budget

The Claude Code documentation says the skill listing budget is 1% of the model's context window.
Every skill name is always kept; when the listing is over budget, the descriptions of the
least-invoked skills are dropped first. Each entry is capped at 1,536 characters.

Issue #2866 measured the cost with `/context` in a scratch repository holding the 53 `SKILL.md` files,
flat. This issue's draft repeated that measurement on 2026-11-16 at `develop` `a35960402`, with
Claude Code 2.1.318 (`claude -p "/context"`, default model Sonnet 5.5), and got the same numbers:

| Measurement                                                                                  | Value                   |
| -------------------------------------------------------------------------------------------- | ----------------------- |
| Context window reported                                                                      | 1M tokens               |
| Skills category, no project skills                                                           | 4.6k tokens             |
| Skills category, with the 53 project skills                                                  | 9.9k tokens (1.0%, cap) |
| The 53 project skill entries alone (sum of rows)                                             | ~6,980 tokens           |
| Other skills whose descriptions were dropped (`claude.ai sync` source, under 20 tokens each) | 7 of 26                 |
| Description length: total / mean / max (characters)                                          | 19,489 / 368 / 724      |
| Descriptions of 300 characters or more                                                       | 42 of 53                |
| Descriptions of 200 characters or less                                                       | 4 of 53                 |

The description lengths are the YAML-parsed `description` values (Python `yaml.safe_load` of the
frontmatter, then `len`), the text Claude Code reads.

The listing is already at the cap on a 1M window, and the project skills push other skills'
descriptions out. On a 200k window the budget is about 2k tokens, less than a third of what the
53 project entries cost today, so most descriptions would be dropped.

A linear fit of the 53 rows gives about 0.33 tokens per character of name plus description, about
3 characters per token. The names average 20 characters. If every description were 200
characters, the 53 entries would cost about 3,900 tokens; at a mean of about 170 characters, about
3,300. That roughly halves today's cost and leaves the 1M-window listing well under its cap. It
does not fit a 200k window by itself; see Out of Scope.

### A description that YAML cuts short

The `write-markdown-docs` description is a plain (unquoted) YAML scalar. It contains `#NUMBER`
after a space, and YAML reads a `#` after a space as the start of a comment. A YAML parser keeps
only its first 123 of 578 characters, which end mid-sentence (`... especially the critical`),
before the `Use when` clause and the trigger list (`write docs`, `#number`, `autolink`, `GFM`).
`/context` confirms it: the skill's row costs about 50 tokens, where a 578-character description
would cost about 200. Claude Code sees nothing that says when to load the skill. No other
description is cut.

The `add-new-skill` skill says a description may be up to 1,024 characters, and nothing checks
the length or the quoting.

## Scope

### In Scope

- Fix the `write-markdown-docs` description so YAML reads all of it.
- Rewrite the 53 descriptions to the target in [Description rules](#description-rules).
- A `frontmatter-validator` rule for skill descriptions: the length limit and a description that
  YAML cuts at a comment.
- Update `add-new-skill` (the description rules and the limit) so new skills follow them.
- Measure with `claude -p "/context"` before and after, and check that sample skills still load
  for their trigger phrases.

### Out of Scope

- Moving the skills; that is #2866. This issue edits the descriptions at their new paths.
- The skill bodies, except where a sentence removed from a description has no other home and
  belongs in the body.
- Fitting the full listing into a 200k window. If that is wanted, possible follow-ups are fewer
  skills (merging ones that overlap) or marking maintainer-only skills so they are not listed.
  Each is a separate decision; this issue only records the measured gap.
- Skills outside the repository (built-in, plugin and `claude.ai sync` skills).

## Timing

The description edits land after #2866's implementation PR merges, on
`.claude/skills/<name>/SKILL.md`. Editing them at the old paths first would conflict with every
file that PR moves. The validator rule also belongs next to the skill rules #2866 adds to
`frontmatter-validator`. This spec can merge before; implementation starts when #2866 is closed.

## Description Rules

A description says **what** the skill does and **when** to load it. Claude Code chooses a skill
from its name and description, so the words a task uses must be in it.

- **Length:** 200 characters or less, the YAML-parsed value. This is the target and the
  validator's limit.
- **Shape:** one sentence on what the skill does, then `Use when ...` naming the tasks. Start with
  a verb or the object, not `Guide for` or `This skill`.
- **Trigger words:** keep the two to four words or phrases a task is most likely to contain and
  that set this skill apart from its neighbors (for example `#NUMBER` and `autolink` for
  `write-markdown-docs`). Drop generic words every skill could claim (`documentation`, `project`,
  `torrust-tracker`) and long `Triggers on "..."` lists. A phrase can stay inside the `Use when`
  clause instead of a separate list.
- **No duplication:** details, steps, warnings and option lists go in the body, which loads only
  when the skill does.
- **Quoting:** quote the value (double quotes) when it contains `#` or a colon followed by a space, or starts with a
  character YAML treats specially. The validator rule catches a description cut by a comment.
- **Related skills:** when two skills share trigger words (for example `process-pr-review`,
  `process-pr-review-feedback` and `process-copilot-suggestions`; or `run-pre-commit-checks` and
  `run-pre-push-checks`), each description says what sets it apart.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
  (context capability governance), and the ADR #2866 creates for the `.claude/skills/` location.
- ADRs to create: none known. A description length limit is a convention, recorded in
  `add-new-skill` and enforced by the validator.
- ADR review record: the governance ADR's Review cadence asks for a review when a tracked skill is
  materially changed. Rewriting all 53 descriptions changes when each skill loads, so T6 adds a
  dated row to that ADR's `Review records` table.

## Design and Ownership Review

Not applicable: no child processes, asynchronous I/O, network readiness or reusable fixtures.

## Bug-Fix Process

The cut `write-markdown-docs` description is a defect, so its fix follows the `fix-bug` skill
(`.claude/skills/fix-bug/SKILL.md` after #2866):

- Analysis: a plain YAML scalar that contains a space followed by `#` ends at the `#`; the rest is a comment.
- Reproduction: done while drafting and classified **Reproduced** in
  `manual-verification-evidence.md` section V0: PyYAML keeps 123 of 578 characters, and `/context`
  shows the row at about 50 tokens.
- Regression-test boundary: the validator rule's unit test (T2).
- Red evidence: T2's rule, run against the tree before the fix, fails on `write-markdown-docs` as
  cut at a comment.
- Fix: quote the value (T3), then shorten it with the rest (T4).
- Green evidence and like-for-like recheck: the rule passes, and the parser and `/context` show the
  whole description.

## Regression Test Strategy

Whether a skill loads for a task depends on Claude Code and a model, so it cannot be a maintained
deterministic test; manual scenario M3 samples it. The deterministic boundary that fails for the
defect is the skill file's frontmatter: the `frontmatter-validator` rule (T2) fails when a skill's
`description` is cut at a YAML comment or is longer than 200 characters. Its unit tests use the
`write-markdown-docs` description as it is today as the failing case. It runs on every commit and
in CI with #2866's skill rules.

The rule needs the raw frontmatter text, because a YAML parser drops comments: for example, it
fails when the `description:` line is a plain scalar whose value contains a space followed by `#`, or when the parsed
value is shorter than the raw value. The exact detection is settled in T2 and recorded here.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                              | Notes / Expected Output                                                                                                                                                                                                                                   |
| --- | ------- | --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | BLOCKED | Baseline after #2866              | Waits for #2866 to merge. On that `develop` commit, record per skill the parsed description length and its `/context` row, and the category totals, in issue-local `description-measurements.md`. Run the M3 trigger prompts and record which skill loads. |
| T2  | TODO    | Description rule, proven red      | The regression test: the `frontmatter-validator` rule for skill descriptions (cut at a comment; over 200 characters) with unit tests, next to #2866's skill rules. The rule is opt-in (off unless explicitly selected) until T4 ends: the pre-commit hook already runs `frontmatter-validator --staged`, so a rule on by default would reject the commits that still carry long descriptions. Its opt-in run on the tree before T3 is recorded red in `manual-verification-evidence.md`: it fails on `write-markdown-docs` as cut at a comment and lists every over-long description. |
| T3  | TODO    | Fix `write-markdown-docs` quoting | Quote the value. The rule no longer reports it as cut, and the parser and `/context` show the whole description: the green run and like-for-like recheck of V0, recorded in `manual-verification-evidence.md`. |
| T4  | TODO    | Rewrite the descriptions          | All 53 descriptions to the [Description rules](#description-rules), one commit per area of the `.claude/skills/README.md` index. Each description's old and new text and length go in `description-measurements.md`. The rule from T2 passes, and T4's last commit turns it on in the existing staged and whole-tree gates (pre-commit hook and CI).            |
| T5  | TODO    | Update `add-new-skill`            | The description rules and the 200-character limit replace "max 1024 chars".                                                                                                                                                                             |
| T6  | TODO    | Measure again and verify          | `claude -p "/context"` after the change, same Claude Code version and model as T1 where possible; M3 trigger prompts again; the governance ADR's review-record row; `linter all`; acceptance review.                                                                                               |

## Commit Points

| Task | Coherent change set                         | Commit policy                                                                                                          |
| ---- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| T1   | Baseline evidence                           | One commit with `description-measurements.md` and the baseline sections of `manual-verification-evidence.md`.           |
| T2   | Validator rule and its unit tests           | One commit per reviewed test increment; the red run is recorded in the evidence, not committed as a failing check. The rule stays opt-in, so no commit fails on it, until T4's last commit turns it on, the first one it passes on. |
| T3   | `write-markdown-docs` quoting               | One commit; the bug fix stays reviewable on its own.                                                                   |
| T4   | Rewritten descriptions                      | One commit per index area, each with its rows in `description-measurements.md`.                                        |
| T5   | `add-new-skill`                             | One commit.                                                                                                            |
| T6   | Evidence, acceptance verification, log      | One commit.                                                                                                            |

T2 produces tests. They use the `write-unit-test` skill and add one behavior at a time. After each
increment passes focused validation, and before the next one starts, review its design and record
the review in the task evidence: write temporary prose for the Arrange, Act and Assert sections,
refactor until the code says it, and remove the redundant prose. The review confirms that the test
exposes the one causal initial-state difference, that its fixture owns only incidental mechanics,
and that the production Act and the independently specified expected result stay visible. Judge
helper boundaries by meaningful named actions, not caller count. Stop for maintainer review after
the last increment, before T6. Commits use Conventional Commit messages and are signed.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-shorten-skill-descriptions/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] #2866 merged (implementation starts after it)
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-16 08:22 UTC - AI assistant (Claude Code, Claude Opus 5.5) - Drafted this spec as #2866's task T1; repeated #2866's `/context` measurement at `develop` `a35960402` with Claude Code 2.1.318 and got the same numbers
- 2026-11-16 08:26 UTC - AI assistant (Claude Code, Claude Opus 5.5) - The maintainer approved the draft; created issue #2873 as josecelano-bot. Linking it as a sub-issue of #2003 returned HTTP 404 for the bot, so the maintainer links it
- 2026-11-16 08:47 UTC - AI assistant (Claude Code, Claude Opus 5.5) - Addressed da2ce7's review 5917183183 on PR #2875: the regression test (now T2) comes before the fix (now T3); the reproduction is recorded and classified in `manual-verification-evidence.md` V0; T6 adds the governance ADR's review-record row
- 2026-11-16 08:53 UTC - AI assistant (Claude Code, Claude Opus 5.5) - Addressed Copilot's review 5917272531 on PR #2875: T2's rule stays opt-in until T4's last commit turns it on in the existing staged and whole-tree gates, since the hook already runs `frontmatter-validator --staged`. Correction: the entry stamped 08:47 UTC is later than its commit, made at 08:46:59 UTC; it is kept unchanged

## Acceptance Criteria

- [ ] AC1: Every skill `description` is 200 characters or less, YAML-parsed.
- [ ] AC2: Every description says what the skill does and when to use it, and keeps its
      distinguishing trigger words, as the [Description rules](#description-rules) say;
      `description-measurements.md` shows each old and new text.
- [ ] AC3: The `write-markdown-docs` description is read whole by a YAML parser and by Claude
      Code.
- [ ] AC4: A `frontmatter-validator` rule fails on a skill description cut at a YAML comment or
      longer than 200 characters, with unit tests for both and for a passing description, and runs
      on every commit and in CI.
- [ ] AC5: The 53 project entries cost at most about half of today's ~6,980 tokens in `/context`,
      and the Skills category is below its 1% cap on the same model.
- [ ] AC6: For each M3 sample prompt, the same skill loads after the change as before (or, for
      `write-markdown-docs`, loads where it did not).
- [ ] AC7: `add-new-skill` states the description rules and the limit.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test` for `frontmatter-validator` (stable Rust toolchain)
- The validator run on the whole tree, as the pre-commit hook and CI run it
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                           | Human-oriented command/steps                                                                                                                                                       | Expected Result                                                                                         | Status | Evidence                                     |
| --- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Listing cost before and after      | In a fresh checkout of the commit, run `claude -p "/context"` before (T1) and after (T6); record the Claude Code version, model and window, the Skills category and every Project row | After: Skills below the cap, project rows sum to about half of before, no project description dropped   | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | `write-markdown-docs` read whole   | Parse its frontmatter with a YAML parser and compare with the file; check its `/context` row                                                                                        | Before: 123 of 578 characters, ~50 tokens. After: the whole (new) description                           | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Skills still load for their tasks  | For about eight skills (including `write-markdown-docs`, `create-issue`, `process-pr-review`, `run-pre-push-checks`), run a task-like prompt with `claude -p --output-format stream-json --verbose` and record which skill the `Skill` tool loads, before and after | The same skill loads after as before; `write-markdown-docs` loads for a Markdown task after the fix | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | The rule catches a cut description | In a scratch commit, put a space followed by `#` into one unquoted description, and separately make one longer than 200 characters; run the validator                                              | Each run fails and names the file and the problem                                                        | TODO   | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the Claude Code version and model with every `/context` and trigger result; both can
  change the numbers. Repeat M3's prompts once if a result differs, since model choice is not
  deterministic, and record both runs.
- `manual-verification-evidence.md` was created from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  and keeps its headings. It already holds V0, the reproduction made while drafting; add V1 to V4
  during implementation.

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

## Risks and Trade-offs

- A shorter description can stop a skill from loading for a task it used to catch. Mitigation: the
  trigger-word rule, the M3 before-and-after prompts, and each old text kept in
  `description-measurements.md` so a lost phrase can be restored.
- 200 characters is a judgment, not a measured optimum. It halves the cost and is easy to check;
  a skill that cannot be described in 200 characters is a sign it does two things.
- `/context` numbers and skill choice depend on the Claude Code version and model. Mitigation:
  record both, and compare runs made with the same ones.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a progress-log
  entry saying why none was needed.
- An independent reviewer that receives this specification records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2003 (parent EPIC), #2866 (moves the skills; its task T1 created this issue)
- Related PRs: #2869 (#2866's spec)
- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
