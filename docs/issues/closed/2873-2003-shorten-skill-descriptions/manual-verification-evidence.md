---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2873-2003-shorten-skill-descriptions/ISSUE.md
last-updated-utc: "2026-11-17 08:58"
---

# Manual Verification Evidence

<!-- cspell:ignore splitlines startswith -->

## Purpose

Record real, human-oriented verification for issue #2873. V0 reproduces the cut
`write-markdown-docs` description, as the `fix-bug` skill asks before review. Sections V1 to V4
follow the manual scenarios M1 to M4 of the spec; the T2 section records the description rule's red
run, its mutations and its test-design review.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-16, steps stamped below
- Artifact under test: V0: the skill files on `develop` `a35960402`, before #2866 moved them. V1
  and V3 before: `develop` `9e463cb0c`, the merge of #2866's PR #2886, the first commit with the
  skills at `.claude/skills/<name>/SKILL.md`. T2: branch `2873-shorten-skill-descriptions` at
  `feat(frontmatter-validator): [#2873] add the opt-in --skill-descriptions mode`, whose skill files are still those of `9e463cb0c`. V2 after T3: `fix(skills): [#2873] quote the write-markdown-docs description so YAML reads all of it`. V1, V3
  and V4 after: `docs(skills): [#2873] state the skill description rules in add-new-skill`, the branch after task T5
- Operating system / environment: Linux; Python 3.14.4 with PyYAML 6.0.3; Claude Code 2.1.318
  with its default model `claude-sonnet-5-5` and a 1M-token window; stable Rust
- Prerequisites and setup performed: V1 and V3 run in a detached `git worktree` of the commit in a
  scratch folder, which has no parent `CLAUDE.md`

## Verification Processes

### V0 - Reproduce the cut `write-markdown-docs` description

- Goal: show that a YAML parser, and Claude Code, read only part of the description.
- Initial state: `.github/skills/dev/planning/write-markdown-docs/SKILL.md` as on `develop`
  `a35960402`; its `description` is a plain YAML scalar containing a space followed by `#NUMBER`.
- Status: `DONE`
- Classification: **Reproduced** for the parser: the wrong outcome, the cut value, was observed in
  its output. Claude Code's `/context` corroborates it by inference only: it shows a token count,
  not the description text.

#### Steps Performed

1. 08:21 UTC. In a scratch repository holding the 53 `SKILL.md` files flat under
   `.claude/skills/<name>/` (copied from `.github/skills/` at `a35960402`), ran
   `claude -p "/context"` with Claude Code 2.1.318.
2. 08:45:16 UTC. In the checkout at `a35960402`, parsed the frontmatter with PyYAML and compared
   the raw text after `description:` with the parsed value:

   ```python
   import yaml
   p='.github/skills/dev/planning/write-markdown-docs/SKILL.md'
   fm=open(p).read().split('---')[1]
   raw=[l for l in fm.splitlines() if l.startswith('description:')][0][len('description: '):]
   d=yaml.safe_load(fm)['description']
   print('raw', len(raw)); print('parsed', len(d)); print('parsed text:', d)
   ```

#### Observed Result

Step 1, the lines of the `/context` output that bear on this issue (model `claude-sonnet-5-5`,
`27.8k / 1m (3%)` in total):

```text
| Skills | 9.9k | 1.0% |
| write-markdown-docs | Project | ~50 |
```

Step 2:

```text
raw 578
parsed 123
parsed text: Guide for writing Markdown documentation in this project. Covers GitHub Flavored Markdown pitfalls, especially the critical
```

#### Conclusion

The parser keeps 123 of the 578 characters and stops before `#NUMBER pattern`: a space
followed by `#` starts a YAML comment; this is observed. The `/context` row of about 50 tokens
matches 123 characters plus the name at the measured rate of about 3 characters per token, where
the whole 578 characters would cost about 200. That Claude Code lists the same cut text, without
the `Use when` clause and the trigger phrases, is inferred from that cost: `/context` does not
show the text.

### V1 - Listing cost before and after (M1)

- Goal: measure what the 53 project skill entries cost in every session, before and after the
  descriptions are shortened.
- Initial state: before: a fresh checkout of `develop` `9e463cb0c`, clean tree. After: a fresh
  checkout of `docs(skills): [#2873] state the skill description rules in add-new-skill`.
- Status: `DONE`

#### Steps Performed

1. 17:35:47 UTC. In the checkout, ran `claude -p "/context"` with Claude Code 2.1.318.
2. Read the Skills category and every Project row; tabulated the rows against each skill's parsed
   description length in `description-measurements.md`.
3. 18:04:51 UTC. Repeated step 1 in the checkout of `docs(skills): [#2873] state the skill description rules in add-new-skill`, same version and model.
4. 19:02:47 UTC. After da2ce7's review of PR #2893 reworded 16 descriptions, repeated step 1 in a
   fresh checkout of the commit `docs(skills): [#2873] word every skill description to the Use when shape`,
   same version and model.

#### Observed Result

Model `claude-sonnet-5-5`, `32.9k / 1m (3%)` in total. Category rows that bear on this issue:

```text
| System prompt | 2.6k | 0.3% |
| Skills | 9.9k | 1.0% |
```

The Skills table has 53 Project rows, summing to 6,980 tokens; the descriptions total 19,489
characters (mean 368, max 724; 42 of 53 at 300 or more, 4 at 200 or less). Seven `claude.ai
sync` skills show `< 20` tokens, their descriptions dropped. `write-markdown-docs` costs `~50`.
Per-skill numbers: `description-measurements.md`.

After (step 3), `30.7k / 1m (3%)` in total:

```text
| Skills | 7.8k | 0.8% |
```

The 53 Project rows sum to 3,220 tokens; the descriptions total 8,306 characters (mean 157, max
187, all 53 at 200 or less). No skill in the table shows `< 20` tokens: the seven `claude.ai sync`
descriptions dropped before are listed again. `write-markdown-docs` costs `~70`.

After the rewording (step 4), `32.9k / 1m (3%)` in total, Skills `7.9k`, `0.8%`. The 53 Project
rows sum to 3,270 tokens; the descriptions total 8,431 characters (mean 159, max 187). No skill
shows `< 20` tokens. `description-measurements.md` holds this run.

The two after runs differ by 2.2k in total (30.7k and 32.9k) but only 0.1k in Skills. The other
2.1k is the MCP rows: the 19:02:47 UTC run lists `MCP tools` (559) and `MCP server instructions`
(1.6k), as the before run does, and the 18:04:51 UTC run lists neither; `System tools` is 15.2k and
15.1k. So an MCP connection differed between the runs; it does not touch the Skills category or
its rows. Against the before run, `System tools` is larger in both after runs: 13.1k before,
15.2k and 15.1k after. That 2.0k is why the before run and the reworded run both total 32.9k
although Skills fell 2.0k. The totals depend on the session (tools and connections loaded), so the
conclusions compare only the Skills category and its Project rows.

#### Conclusion

The baseline after #2866 equals the one the spec measured at `a35960402` in a flat scratch copy:
the move changed no description, and the listing is at its 1% cap. After the rewrite the project
entries cost 47% of before (3,270 of 6,980 tokens, final wording; 46% at the first after run), the Skills category is below its cap, and no
description is dropped: M1's expected result holds.

### V2 - `write-markdown-docs` read whole (M2)

- Goal: show that the T3 fix makes a YAML parser read the whole description; the like-for-like
  recheck of V0.
- Initial state: the branch at `fix(skills): [#2873] quote the write-markdown-docs description so YAML reads all of it` (T3, the value quoted) and at `docs(skills): [#2873] state the skill description rules in add-new-skill` (after T4).
- Status: `DONE` (outcome: **Green**)

#### Steps Performed

1. 17:51:45 UTC. At T3, parsed the frontmatter with PyYAML as in V0 step 2, and ran the opt-in
   `--skill-descriptions` mode before and after staging the file.
2. 17:52:27 UTC. In a fresh checkout of `fix(skills): [#2873] quote the write-markdown-docs description so YAML reads all of it`, ran `claude -p "/context"`.
3. At `docs(skills): [#2873] state the skill description rules in add-new-skill`, read the parsed length and the `/context` row from V1's after run.

#### Observed Result

| Commit      | Parsed by PyYAML        | Rule finding for the file            | `/context` row |
| ----------- | ----------------------- | ------------------------------------ | -------------- |
| `9e463cb0c` | 123 of 578 characters   | `skill-description-cut-by-comment`   | `~50`          |
| `fix(skills): [#2873] quote the write-markdown-docs description so YAML reads all of it` | 578 of 578 characters   | `skill-description-too-long` only    | `~200`         |
| `docs(skills): [#2873] state the skill description rules in add-new-skill` | 179 of 179 characters   | none                                 | `~70`          |

At `fix(skills): [#2873] quote the write-markdown-docs description so YAML reads all of it` the parsed text ends `... "markdown pitfall", or "GFM".`, the end of the file's text.

#### Conclusion

Observed: the parser reads the whole description once it is quoted, and the rule no longer reports
it as cut. Inferred: Claude Code reads it whole too, since its row grew from about 50 to about 200
tokens, what 578 characters cost at the measured rate of about 3 characters per token, and then
fell to about 70 for 179 characters. `/context` does not show the text.

### V3 - Skills still load for their tasks (M3)

- Goal: check that a task-like prompt loads the same skill after the descriptions are shortened.
- Initial state: before: the same checkout of `9e463cb0c` as V1. After: the same checkout of
  `docs(skills): [#2873] state the skill description rules in add-new-skill` as V1.
- Status: `DONE`

#### Steps Performed

1. 17:36 to 17:38 UTC. For each of eight skills, ran one prompt with
   `claude -p --output-format stream-json --verbose --tools Skill,Read,Grep,Glob --max-turns 4`.
   Only read-only tools are available, so a prompt cannot change anything; `--max-turns 4` stops
   it soon after the skill is chosen. The default model was used.
2. Read the `Skill` tool calls from each stream.
3. 18:05 to 18:07 UTC. Repeated steps 1 and 2 with the same prompts in the checkout of
   `docs(skills): [#2873] state the skill description rules in add-new-skill`.
4. 19:03 to 19:05 UTC. Repeated steps 1 and 2 in the checkout of V1's step 4, after the
   rewording.

The prompts:

| Skill expected          | Prompt                                                                                                                                         |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `write-markdown-docs`   | Add a section to docs/containers.md that mentions issue #2873 and the PR that fixed it, and make sure the issue numbers render as links on GitHub. |
| `create-issue`          | I want a new GitHub issue for adding a rate limit to the health check endpoint. Start the issue for it.                                       |
| `process-pr-review`     | Copilot and da2ce7 left review comments on my pull request 2875. Go through each finding and address it.                                       |
| `run-pre-push-checks`   | I am about to push my branch to GitHub. Run the checks I need before pushing.                                                                 |
| `commit-changes`        | I have finished my changes. Commit them with a proper commit message.                                                                          |
| `fix-bug`               | The UDP tracker sends the wrong announce interval to clients. Investigate and fix this bug.                                                    |
| `add-rust-dependency`   | Add the uuid crate as a dependency of the tracker-core package.                                                                                |
| `write-unit-test`       | Write a unit test for the function that parses the announce request query string.                                                              |

#### Observed Result

| Skill expected        | Before: skill loaded  | After: skill loaded   | Model               |
| --------------------- | --------------------- | --------------------- | ------------------- |
| `write-markdown-docs` | `write-markdown-docs` | `write-markdown-docs` | `claude-sonnet-5-5` |
| `create-issue`        | `create-issue`        | `create-issue`        | `claude-sonnet-5-5` |
| `process-pr-review`   | `process-pr-review`   | `process-pr-review`   | `claude-sonnet-5-5` |
| `run-pre-push-checks` | `run-pre-push-checks` | `run-pre-push-checks` | `claude-sonnet-5-5` |
| `commit-changes`      | `commit-changes`      | `commit-changes`      | `claude-sonnet-5-5` |
| `fix-bug`             | `fix-bug`             | `fix-bug`             | `claude-sonnet-5-5` |
| `add-rust-dependency` | `add-rust-dependency` | `add-rust-dependency` | `claude-sonnet-5-5` |
| `write-unit-test`     | `write-unit-test`     | `write-unit-test`     | `claude-sonnet-5-5` |

Each run loaded exactly one skill, its first tool call. Step 4, after the rewording, loaded the
same eight skills. No result differed, so no prompt was repeated.

#### Conclusion

Every sample prompt loads the same skill after the change as before. Before the change
`write-markdown-docs` already loaded: with its description cut to 123 characters, its name and the
first sentence were enough for this prompt. So the spec's expectation that it "loads where it did
not" does not apply to this sample; AC6 holds as "the same skill loads after as before" for all
eight. Eight prompts on one model are a sample, not a guarantee for every task.

### V4 - The rule catches a cut description (M4)

- Goal: show that the check fails, naming the file and the problem, when a description breaks a
  rule.
- Initial state: a scratch `git worktree` of `docs(skills): [#2873] state the skill description rules in add-new-skill`.
- Status: `DONE`

#### Steps Performed

1. 18:05:10 UTC. Replaced the `create-adr` description with the unquoted
   `Create an ADR for issue #2873. Use when recording a decision.`, made a scratch commit (no hook,
   unsigned), and ran `frontmatter-validator --skill-descriptions`. Reset the commit.
2. Replaced it with a 201-character unquoted description (`Create an ADR.` and 186 `x`), made a
   scratch commit, ran the same command, and reset the commit.

#### Observed Result

```text
{"kind":"diagnostic","path":".claude/skills/create-adr/SKILL.md","severity":"error","category":"skill-description-cut-by-comment","field_path":"description","message":"YAML cuts this plain `description` at white space followed by `#`, a comment; quote the value."}
exit=1
{"kind":"diagnostic","path":".claude/skills/create-adr/SKILL.md","severity":"error","category":"skill-description-too-long","field_path":"description","message":"The skill's `description` has 201 characters; the limit is 200. Keep what the skill does and when to use it; move details to the body."}
exit=1
```

#### Conclusion

Each run fails and names the file and the problem. The pre-commit hook and the Docs Lint workflow
run this same command since `ci(skills): [#2873] check the skill descriptions in the pre-commit hook and Docs Lint`.

### T2 - The description rule fails on the tree before the fix (red)

- Goal: show that the regression test of the spec's Regression Test Strategy fails for the defect.
- Initial state: branch `2873-shorten-skill-descriptions` at `feat(frontmatter-validator): [#2873] add the opt-in --skill-descriptions mode`, with the rule (commits
  `feat(frontmatter-validator): [#2873] report a skill description that YAML cuts at a comment` and `feat(frontmatter-validator): [#2873] report a skill description longer than 200 characters`) and the opt-in `--skill-descriptions` mode (`feat(frontmatter-validator): [#2873] add the opt-in --skill-descriptions mode`); the skill
  files are those of `9e463cb0c`.
- Status: `DONE` (outcome: **Red**, as expected)

#### Steps Performed

1. 17:46:58 UTC. Ran
   `cargo run -q -p frontmatter-validator --bin frontmatter-validator -- --skill-descriptions`.

#### Observed Result

Exit code `1`, 50 diagnostic records: 49 `skill-description-too-long` (the 53 skills minus the 4
already at 200 characters or less) and one `skill-description-cut-by-comment`:

```text
{"kind": "diagnostic", "path": ".claude/skills/write-markdown-docs/SKILL.md", "severity": "error", "category": "skill-description-cut-by-comment", "field_path": "description", "message": "YAML cuts this plain `description` at white space followed by `#`, a comment; quote the value."}
```

`write-markdown-docs` is not among the too-long ones: YAML reads only 123 of its characters.

#### Detection Settled in T2

The rule reads the raw frontmatter: the `description:` line and its continuation lines (blank or
indented) up to the next top-level line. The first non-white-space character of that text gives
the style: `"` or `'` is quoted, `|` or `>` is a block scalar, anything else is plain. A plain
value is reported when any of those lines contains a space or a tab followed by `#`. The length
rule counts the characters (not bytes) of the parsed value.

A tab before `#` is real for the validator's parser: `serde_yaml` reads
`description: Writes Markdown, avoiding the<TAB>#NUMBER pattern.` as `Writes Markdown, avoiding
the`. PyYAML instead rejects that line as a scanner error. The other unit-test fixtures were
checked with PyYAML: the reported ones are cut, the accepted ones are read whole.

#### Mutations

Each clause of the rule was broken on its own in the working tree, the tests run, the file
restored. Every mutation compiled (the first attempts of L4, C1 and C2 did not, for an unused item,
and were rewritten until they did).

| ID  | Clause broken                                       | Tests that failed                                                                                                                      |
| --- | --------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | A tab before `#` is a comment (only a space checked) | `..._cuts_at_a_comment::case_2_tab_before_hash`                                                                                        |
| M5  | A space before `#` is a comment (only a tab checked) | `case_1_space_before_hash`, `case_3_on_a_continuation_line`, `case_4_continuation_line_starting_with_hash`, `..._cut_at_a_comment_and_still_too_long` |
| M2  | Continuation lines are read (first line only)       | `case_3_on_a_continuation_line`, `case_4_continuation_line_starting_with_hash`                                                         |
| M3  | Quoted and block values are not reported            | `it_should_accept_a_description_that_yaml_reads_whole` cases 3 to 6 (double, single, folded, literal)                                 |
| M4  | The description ends at the next top-level line     | `it_should_not_read_a_comment_in_the_following_field_as_part_of_the_description`                                                      |
| L1  | The limit itself is accepted (`>=` for `>`)          | `..._no_longer_than_the_limit::case_1_at_the_limit`, `it_should_count_the_characters_yaml_reads_not_the_raw_text_or_its_bytes`         |
| L2  | Characters are counted, not bytes                   | `it_should_count_the_characters_yaml_reads_not_the_raw_text_or_its_bytes`                                                              |
| L3  | The parsed value is counted, not the raw text       | `case_1_at_the_limit`, `it_should_count_the_characters_yaml_reads_not_the_raw_text_or_its_bytes`                                       |
| L4  | Over-long is reported at all                        | `it_should_report_a_description_longer_than_the_limit`, `..._cut_at_a_comment_and_still_too_long`                                     |
| C1  | Only skill files are checked (every Markdown file)  | `it_should_check_the_description_of_each_skill_in_the_index_only_with_skill_descriptions`                                              |
| C2  | The index is read, not the working tree             | `it_should_read_the_skill_description_from_the_index_with_skill_descriptions`                                                          |

After Copilot's review of PR #2893 (F1, F2), the rule also reads a quoted or spaced key and finds
the value's style after tags, anchors and a comment on the key's line. Its new fixtures were checked
with PyYAML (each reported one is cut to `Avoids the`, each accepted one is read whole) and its new
clauses mutated the same way:

| ID  | Clause broken                                           | Tests that failed                                                                                         |
| --- | ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| N1  | A quoted key is read (only `description` matched)        | `..._however_its_key_and_properties_are_written::case_2_double_quoted_key`, `case_3_single_quoted_key`    |
| N2  | White space before the `:` is allowed                    | `..._however_its_key_and_properties_are_written::case_1_white_space_before_the_colon`                     |
| N3  | Tags and anchors are skipped before the style is read    | `it_should_accept_a_whole_description_...::case_1_double_quoted_after_a_tag`, `case_2_folded_after_an_anchor` |
| N4  | A comment ending the key's line is not the value         | `it_should_accept_a_whole_description_...::case_3_folded_after_a_comment`                                 |
| N5  | The key needs its `:` (any line starting `description`) | `it_should_accept_a_whole_description_...::case_5_another_key_with_the_same_start`                        |
| O1  | A value that is only a comment is reported               | `..._however_its_key_and_properties_are_written` cases 7 to 9 (comment on the key line, on the next line, after a tag) |

A first N5 mutation, of a guard that required white space after the `:`, made no test fail: no
line the key match accepts reaches it. The guard was removed instead of tested. The N5 row first
recorded a second mutation as N5 that stopped every key from matching, a different clause; da2ce7's
second review of PR #2893 (his F5) found it. N5 was run again as written in its row. O1 guards the
fix for his F4: after the reader learned to skip a comment on the key's line, a description that
is only a comment, which YAML reads as null, was no longer reported. PyYAML reads the three O1
fixtures as `None`, `None` and an empty string.

#### Test-Design Review

Each increment's tests were reviewed with the prose-first Arrange, Act and Assert pass of the
`write-unit-test` skill before the next increment started:

- Comment rule (`description_tests` in `src/skills.rs`). Arrange: a skill file whose
  `description` is written in one way; Act: `description_findings`; Assert: the categories. The
  one causal difference is the description text, which each case names and shows whole; the
  fixture `skill_with_description` owns only the incidental frontmatter around it, including a
  following `metadata` field so the end of the description is exercised. The prose matched the
  code, so none was kept.
- Length rule. `description_of_length` names the one causal input, the length, against
  `MAX_DESCRIPTION_CHARACTERS`. The raw-versus-parsed test keeps a two-line comment: why a folded
  block of 99 and 100 characters parses to exactly the limit cannot be read from the code alone.
- CLI mode (`tests/cli.rs`). Arrange and Assert comments name the staged, working-tree and
  untracked files, as the neighboring `--skill-layout` tests do.
- Unit level: both rules are pure functions of a file's text, so they are tested there; only file
  selection and reading the index need the binary.

## Failures and Follow-up

None so far. V0's parse first ran while drafting, in the script that measured all 53
descriptions, which gave 123 parsed against 578 raw characters for this file; the 08:45:16 UTC run
in step 2 repeated it for this record with the same result. Neither run failed.
