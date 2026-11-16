---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md
last-updated-utc: 2026-11-16 08:10
---

<!-- cspell:ignore Autocompact dataviz -->

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-15 20:48 (V0)
- Artifact under test: V0: `develop` at `abd7a10ff` (`Merge torrust/torrust-tracker#2863`), before
  any change of this issue.
- Operating system / environment: Linux 7.0.0-43-generic; Claude Code 2.1.318
  (`claude --version`); model reported by `/context`: `claude-sonnet-5-5`, 1M context window.
- Prerequisites and setup performed: a fresh local clone of `develop` in a scratch folder with no
  `.claude/` folder (`ls .claude` failed with "No such file or directory");
  `find . -name SKILL.md -not -path './.git/*' | wc -l` printed `53`.

## Verification Processes

### V0 - Reproduce the defect before T3 (M0)

- Goal: show, in the real repository, that Claude Code does not discover the repository's skills.
- Initial state: fresh clone at `abd7a10ff`, detached `HEAD`, clean tree.
- Status: `DONE` (outcome: **Reproduced**)

#### Steps Performed

The two commands ran one after the other in one shell, at the clone's root. The times are the
output files' birth and last-modification times.

1. 20:48:11.17 to 20:48:13.86 UTC: `claude -p "/context" > m0-context.md`
2. 20:48:14.77 to 20:48:26.22 UTC: `claude -p "/create-issue" > m0-create-issue.txt`

#### Observed Result

Step 1, the category table, the Memory Files table and the whole Skills table of the output,
verbatim; the long scratch path of the clone in the Memory Files row is shortened to `<clone>`:

```text
### Estimated usage by category

| Category | Tokens | Percentage |
|----------|--------|------------|
| System prompt | 2.6k | 0.3% |
| System tools | 18.4k | 1.8% |
| MCP tools | 559 | 0.1% |
| MCP server instructions | 1.6k | 0.2% |
| MCP tools (deferred) | 13.1k | 1.3% |
| System tools (deferred) | 22.7k | 2.3% |
| Memory files | 5k | 0.5% |
| Skills | 4.6k | 0.5% |
| Messages | 10 | 0.0% |
| Free space | 934.2k | 93.4% |
| Autocompact buffer | 33k | 3.3% |

### Memory Files

| Type | Path | Tokens |
|------|------|--------|
| Project | <clone>/AGENTS.md | 5k |

### Skills

| Skill | Source | Tokens |
|-------|--------|--------|
| dataviz | Built-in | ~480 |
| artifact-design | Built-in | ~70 |
| artifact-diagramming | Built-in | ~70 |
| artifact-capabilities | Built-in | ~250 |
| update-config | Built-in | ~240 |
| keybindings-help | Built-in | ~80 |
| code-review | Built-in | ~280 |
| simplify | Built-in | ~60 |
| fewer-permission-prompts | Built-in | ~60 |
| loop | Built-in | ~120 |
| schedule | Built-in | ~130 |
| claude-api | Built-in | ~360 |
| workflow-authoring | Built-in | ~80 |
| run | Built-in | ~120 |
| plugin-authoring | Built-in | ~90 |
| init | Built-in | ~20 |
| security-review | Built-in | ~30 |
| docs | claude.ai sync | ~340 |
| docx | claude.ai sync | ~320 |
| google-workspace | claude.ai sync | ~330 |
| import-memory | claude.ai sync | ~60 |
| morning | claude.ai sync | ~120 |
| pdf | claude.ai sync | ~150 |
| pptx | claude.ai sync | ~330 |
| skill-creator | claude.ai sync | ~120 |
| xlsx | claude.ai sync | ~320 |
```

The Skills table has 26 rows: 17 with source `Built-in` and 9 with source `claude.ai sync`, and
none with source `Project` (`sed -n '/### Skills/,$p' m0-context.md | grep -c '| Project |'`
printed `0`). The only `Project` row in the whole output is the memory file `AGENTS.md`
(`grep -c 'Project' m0-context.md` printed `1`). The MCP Tools table between the category table
and the Memory Files table is left out: it lists connector tools only. The
4.6k-token Skills total equals the empty-repository baseline in the spec's pre-spec checks.

Step 2, the first two sentences of the reply:

```text
The repository has a `create-issue` skill (`.github/skills/dev/planning/create-issue/SKILL.md`). Your `/create-issue` command didn't run in this session, and the skill needs a description of the issue to draft.
```

#### Conclusion

Reproduced. No repository skill is discovered, and `/create-issue` does not run as a command. The
model found the skill file by reading the repository instead, which is step 2 of the spec's
"bug in plain terms".

### T2 - The placement rule fails on the tree before the move (red)

- Goal: show that the regression test of the spec's Regression Test Strategy fails for the defect.
- Initial state: branch `2866-move-skills-to-claude-skills` at `4342dd654` (`Merge
  torrust/torrust-tracker#2869`), with the T2 change in the working tree and no skill moved.
- Status: `DONE` (outcome: **Red**, as expected)

#### Steps Performed

1. 2026-11-16 08:03 UTC, stable Rust toolchain:
   `cargo run -q -p frontmatter-validator --bin frontmatter-validator -- --skill-layout 2> t2-red.ndjson`

#### Observed Result

The command exited `1` and wrote 53 records, all of kind `diagnostic`, severity `error` and
category `misplaced-skill`, one for each `SKILL.md` under `.github/skills/`; no record names
another path. The first record, verbatim:

```json
{"kind":"diagnostic","path":".github/skills/add-new-skill/SKILL.md","severity":"error","category":"misplaced-skill","field_path":null,"message":"A skill must be at `.claude/skills/<name>/SKILL.md`, where Claude Code discovers it."}
```

#### Mutations

Each mutation was made in the working tree, compiled, run with
`cargo test -q -p frontmatter-validator --lib skills`, and restored:

| Mutation in `src/skills.rs`                                         | Failing test case                                   |
| ------------------------------------------------------------------- | --------------------------------------------------- |
| Accept a skill folder that contains `/` (one folder too deep)       | `..._would_not_discover::case_2_one_folder_too_deep` |
| Skip paths under `.github/skills/`                                  | `..._would_not_discover::case_1_under_the_copilot_folder` |

After the restore, the 9 `skills` tests passed.

#### Test-Design Review

- `it_should_report_a_skill_that_claude_code_would_not_discover` and
  `it_should_accept_a_path_that_is_a_discoverable_skill_or_no_skill`: the path is the whole
  initial state and the one causal difference, named by each `case`. The Act is the production
  call `misplaced_skill(path)`; the expected result is written in the test (the
  `misplaced-skill` category, or no finding). Written with temporary Arrange, Act and Assert
  comments, the Arrange was empty and the other two restated the single line under them, so the
  comments were removed. No fixture or helper.
- `it_should_check_the_skill_layout_of_the_index_only_with_skill_layout` (`tests/cli.rs`): the
  binary decides which files the rule sees, so this test owns only that. The one causal
  difference is whether a misplaced skill is in the index; both skills are misplaced, so the
  library's placement decision is not what the test observes. The first draft also staged a
  correctly placed skill; that tested the library's rule a second time, so it was removed. The
  repository fixture holds only incidental mechanics (a disposable git repository).
- Unit-test level: the placement decision is pure and observable from `misplaced_skill`, so it is
  a unit test; the index selection needs a real git index, so it is a command-boundary test.

## Failures and Follow-up

None.
