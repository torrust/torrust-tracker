---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

# Agent Review Reports - Migrate the custom agents to Claude Code and add an explicit orchestrator

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-11-17 17:19 UTC - Task reviewer (Claude Code subagent, Claude Opus 5.5, `review-task` skill)

- Invocation scope: Read-only review of task T2 before its pull request, on branch
  `2904-t2-pointer-claude-md`: the root pointer `CLAUDE.md`, the ten scoped pointer files, the
  evidence of scenario M4 with the specification updates, and the late EPIC Progress Log line.
  The Test Design section of the checklist did not apply; no test changed.
- Inputs: the four commits of the branch; the specification (D11, the T2 rows of the
  Implementation Plan and of Commit Points, AC12, M4); `manual-verification-evidence.md`; the
  EPIC; the page <https://code.claude.com/docs/en/memory>, which the reviewer fetched.
- Evidence: `git ls-files` lists 11 `AGENTS.md` and 11 `CLAUDE.md` files in the same
  directories, and `od -c` shows each `CLAUDE.md` is exactly `@AGENTS.md` and a newline. The
  documentation says relative import paths resolve from the file that holds the import, and the
  reviewer's own session loaded `docs/CLAUDE.md` with `docs/AGENTS.md`. The step order and
  times of the evidence are consistent, and its table matches its conclusion. The subjects are
  Conventional Commits with no closing keyword. `linter markdown` and `linter cspell` exited 0.
  `git merge-tree --write-tree torrust/develop HEAD` merged with no conflict.
- Findings:
  - Minor: ADR `20261116082711` says "there is no repository `CLAUDE.md`", which T2 makes
    untrue, and no task says that line is amended.
  - Minor: the evidence said a correct quote could not come from reading the file because the
    prompt allows one call; only probes 9 to 12 record the tool calls, and probe 3 was neither
    recorded at that level nor repeated.
  - Nit: "Artifact under test" called the pointer files "added by" the two commits; the probes
    ran on the files before they were committed.
  - Nit: the evidence commit also ticks the checkpoint "Spec-only PR merged", which its message
    does not mention.
  - Nit: the frontmatter still had `status: planned` and the specification branch.
  - Nit: the branch was 10 commits behind `develop`.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the pull request: reworded the two evidence passages; set
    `status: in-progress` and the T2 branch; added the ADR line to the T10 row; rebased on
    `develop`. The checkpoint nit is left as it is: the tick is true, and it is named here.

### 2026-11-18 17:25 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T3, increment 1, on branch `2904-t3-reply-schema`
  before its commit: the new crate `agent-reply-validator` (workspace member, `.dockerignore`
  entry) and its file-name parser `src/name.rs` with the unit tests. Two rounds: a review, then
  a re-review of the fixes.
- Inputs: D18 and P1 of the specification; `.claude/skills/write-unit-test/SKILL.md`;
  `AGENTS.md`; the uncommitted crate.
- Evidence: `cargo test -q -p agent-reply-validator` passed with 23 tests in the first round
  and 34 in the second. In the second round the reviewer checked, by reading the code, that
  each of the author's seven mutations has a failing case: the digit check, the leading-zero
  check, `u8::try_from` replaced by `as u8`, a plain `parse` on the `pr` branch, the
  `number >= 1` and `number <= MAX` checks of `Round::new`, and the `-` after the agent. The
  author ran the seven mutations; each made at least one test fail.
- Findings:
  - Major (round 1): no test guarded the digit-only check, which alone rejects a leading `+`
    (`t+2`, `r+1`); the `pr#2920` case did not guard it.
  - Minor (round 1): no zero or leading-zero case on the `pr` branch.
  - Minor (round 1): `r256` did not guard `u8::try_from`; `as u8` would turn it into 0, which
    is still rejected.
  - Minor (round 1): the round-trip test covered only a `pr` subject.
  - Minor (round 1): no Arrange/Act/Assert comments and no `_when_` condition in the names.
  - Nit (round 1): the error cases repeat the file name in the expected value.
  - Nit (round 1): `rstest` without `default-features = false`, unlike some sibling crates.
  - Suggestion (round 1): keep the file name in every `NameError` variant.
  - Suggestion (round 1): the `!text.is_empty()` clause is redundant.
  - Nit (round 2): the `Round::new` assertion could be plainer; the `pr#2920` case still
    guards no clause of its own.
- Verdict: REVIEW FAILED in round 1; REVIEW PASSED in round 2.
- Follow-up actions:
  - Author, done before the commit: added the cases `t+2`, `r+1`, `pr0`, `pr02`, `pr+2`,
    `PR2920` and `r257`; made the round-trip test cover a task and a pull request; added the
    comments and `_when_` names; removed the redundant clause. A mutation showed the
    `number >= 1` clause unguarded, because `r0` is rejected first by the leading-zero rule, so
    an rstest on `Round::new` was added.
  - Not changed, with the reviewer's agreement in round 2: the repeated file name (the explicit
    expected value is clearer than a helper), the file name in `NameError` (the validator
    reports the path with every diagnostic), and `rstest` (as in `frontmatter-validator`). The
    round 2 nits are left as they are.
  - Author, after round 2: the spell checker rejected the case name that joins the agent and the
    subject with no separator, so that case now uses `task-reviewer_t2-r1.json`, a wrong separator
    instead. It guards the same clause: removing the `-` check makes it fail.

### 2026-11-18 17:35 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T3, increment 2, on branch `2904-t3-reply-schema`
  before its commit: `src/reply.rs`, the strict serde model of a reply file's JSON content,
  with its unit tests and the new dependencies `serde`, `serde_json` and
  `serde_path_to_error`. Two rounds: a review, then a re-review of the fixes.
- Inputs: D18 of the specification and the author's choices it leaves open (every field
  required, no unknown field, `schema-version` the number 1, no blank text, `created-utc` as
  `YYYY-MM-DD HH:MM` with calendar validity, the review-audit severities, no rule joining two
  fields); `.claude/skills/write-unit-test/SKILL.md`; `AGENTS.md`.
- Evidence: `cargo test -p agent-reply-validator` passed with 89 tests in round 1 and 96 in
  round 2; clippy printed nothing. The reviewer checked by reading, and once with `rustc` that
  `u16::from_str` accepts a leading `+`. The author ran 28 mutations; each made at least one
  test fail.
- Findings:
  - Major (round 1): no test guarded the digit clause of the time layout, which alone rejects
    `2026-+1-18 17:25`.
  - Minor (round 1): only April stood for the 30-day months, and no 31-day month but December
    was tested at day 31.
  - Minor (round 1): the rejection table asserted only the path, not the reason.
  - Minor (round 1): no test of malformed JSON.
  - Suggestion (round 1): a lower bound for the year.
  - Suggestion (round 1): record that no rule joins two fields; no test of a duplicate key.
  - Nit (round 1): `std::ops::Range` written in full; a `clone` only for an error message;
    the serde impls of the name types live in `reply.rs`, and the format mixes three casings.
  - Nit (round 2): the time cases all assert the same reason, so they do not say which check
    failed; each is shown to fail under its own mutation.
- Verdict: REVIEW FAILED in round 1; REVIEW PASSED in round 2.
- Follow-up actions:
  - Author, done before the commit: added cases for a `+` in the time, day 31 of June,
    September and November, an accepted 31 January, a duplicate key and a truncated object;
    every rejection case asserts a reason; the module doc says no rule joins two fields; one
    `is_blank` helper; `Range` imported.
  - Not changed, with the reviewer's agreement in round 2: no year floor (any floor is
    arbitrary, and the agent writes the time from `date -u`); the serde impls stay with the
    reply format; the round 2 nit.

### 2026-11-18 17:39 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T3, increment 3, on branch `2904-t3-reply-schema`
  before its commit: `src/file.rs`, the checks of one reply file from its repository-relative
  path and its text (location, name, content, and agreement of the name with the `agent`,
  `subject` and `round` fields), with its unit tests.
- Inputs: D18 of the specification and the author's choices (the location read from the path
  alone, every problem reported, agreement checked only when name and content are valid, one
  problem per disagreeing field); `.claude/skills/write-unit-test/SKILL.md`; `AGENTS.md`.
- Evidence: `cargo test -p agent-reply-validator` passed with 116 tests. The reviewer checked by
  reading that each of the author's 14 mutations has a failing case.
- Findings:
  - Minor: the messages of the content and location problems, and the empty field path of the
    location and name problems, were not asserted.
  - Minor: `.` and `..` passed as the specification folder.
  - Suggestion: paths starting with `./`, absolute paths and `\` separators fail with a
    confusing name problem; the command must pass repository-relative paths with `/`.
  - Suggestion: decide in the command increment which files it discovers, since the location
    check only matters if misplaced files can reach it.
  - Suggestion: a trailing slash and a file named `agent-replies` give acceptable outcomes.
  - Nit: two test names say the agreement is skipped, which the types already enforce; the
    fixture's evidence text named a stale test count.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the commit: the location, name and content tests assert the whole
    problem (the content message only up to its position, which depends on the key order);
    `.` and `..` are rejected as the specification folder, with a case each; the fixture's
    evidence text no longer names a count. 118 tests pass, and four more mutations (each dot
    folder, the content message, the content field path) each made a test fail.
  - For the command increment: build the paths from the repository root with `/`, and choose
    the discovery rule there. The nits are left as they are.
