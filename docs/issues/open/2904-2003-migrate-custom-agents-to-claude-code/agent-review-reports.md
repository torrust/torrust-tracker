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

### 2026-11-18 17:48 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T3, increment 4, on branch `2904-t3-reply-schema`
  before its commit: the command `agent-reply-validator` (`src/bin/agent-reply-validator/`),
  its command-boundary tests `tests/cli.rs`, and `is_reply_file_candidate` in `src/file.rs`.
  Two rounds: a review, then a re-review of the fixes.
- Inputs: D18 of the specification; the CLI output contract ADR
  `20260519000000_define_global_cli_output_contract.md`, sections 1 to 3; the
  `frontmatter-validator` command as the pattern; `.claude/skills/write-unit-test/SKILL.md`;
  `AGENTS.md`.
- Evidence: `cargo test -p agent-reply-validator` passed in both rounds (153 tests in round 2).
  The reviewer checked the ADR contract (empty stdout, NDJSON records with `kind` on stderr,
  exit codes 0, 1 and 2) and the git edge cases (a staged file deleted from disk, `git rm
  --cached`, a relative `GIT_INDEX_FILE`) by reading.
- Findings:
  - Major (round 1): no test showed that path mode checks a file whatever its name.
  - Major (round 1): `fs::canonicalize` followed a symbolic link, so a link was checked under
    its target's place and name.
  - Minor (round 1): no unit tests for a failing stderr, `report` and `repository_relative`; no
    test of the key order of the error records, of `--version`, of non-UTF-8 content, of
    `GIT_INDEX_FILE` or of an empty index; one non-UTF-8 path anywhere in the index stopped
    `--all`; a non-UTF-8 path was reported as outside the repository; a merge conflict stops
    `--all`.
  - Suggestion (round 1): `--all` checks every reply file, not only those of the pull request;
    the stop hook must run inside the repository; `git.rs` copies the one of
    `frontmatter-validator`.
  - Nit (round 1): a path given twice is checked twice; two tests asserted on one record field
    by index.
  - Minor (round 2): no test put a non-UTF-8 path in the index; the author's report named a
    mutation of the UTF-8 branch in words that read as the non-UTF-8 one.
  - Minor (round 2): path mode read through a link, while `--all` reads the link's stored target
    path, so the two modes disagreed on a link.
  - Nit (round 2): one test kept only the `check` fields; one parsed stderr with `contains`.
- Verdict: REVIEW FAILED in round 1; REVIEW PASSED in round 2.
- Follow-up actions:
  - Author, done before the commit: the tests and fixes above, with `serde_json`
    `preserve_order` for the key order; the index is read as byte paths and only a candidate
    path that is not UTF-8 stops the run, with a Unix test for each case; path mode rejects a
    symbolic link, with a test; the two round 2 nits fixed. 155 tests pass. Mutations of the
    candidate filter in path mode, of the link check, and of each non-UTF-8 branch each made a
    test fail. A dedupe of conflicted paths was tried and removed as an equivalent mutant: a
    conflicted candidate has no staged content, so the run stops with a runtime error either
    way, as the doc comment says.
  - Not changed: `--all` checks every reply file (CI runs it on the pull request's tree, and
    `develop` only receives checked trees); the stop hook's working directory is for T5 to
    confirm; a path given twice; the copy of `git.rs`, which a shared crate in the
    harness-workspace work of EPIC #2003 can remove.

### 2026-11-18 17:55 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T3, increment 5, on branch `2904-t3-reply-schema`
  before its commit: the JSON Schema `docs/schemas/agent-reply-v1.schema.json` generated from the
  reply model with `schemars` (`src/schema.rs`), its unit tests, the `agent-reply-schema`
  binary that writes it, and the drift test `tests/schema.rs`. Two rounds: a review, then a
  re-review of the fixes.
- Inputs: D18 of the specification; `docs/schemas/README.md` and the `frontmatter-schema`
  binary as precedent; the CLI output contract ADR; `.claude/skills/add-rust-dependency/SKILL.md`;
  `.claude/skills/write-unit-test/SKILL.md`.
- Evidence: `cargo test -p agent-reply-validator` passed in both rounds. In round 1 the
  reviewer ran a probe crate against the model: a reply written as a JSON array was accepted,
  `1.0` and a duplicate key were rejected, a text holding only a byte order mark (U+FEFF) was
  accepted and one holding only a next-line character (U+0085) rejected.
- Findings:
  - Major (round 1): serde reads a struct from an array of its fields, so a reply or a finding
    written as a JSON array was accepted while the schema says `object`. The gap came from
    increment 2.
  - Minor (round 1): the module doc named one difference between the schema and the model;
    there are four (the calendar, `1.0` as an integer, a duplicate key, and Unicode against
    ECMA-262 white space for U+0085 and U+FEFF).
  - Minor (round 1): the pattern tests use the Rust `regex` crate, not ECMA-262.
  - Minor (round 1): the agent test built its expectation from `Agent::ALL`; the binary had no
    test and wrote relative to the working directory; neither binary was in the CLI ADR table.
  - Suggestion (round 1): validate sample replies against the schema with a JSON Schema crate.
  - Nit (round 1): two tests checked two things each; `docs/schemas/README.md` lacked the
    entry.
  - Nit (round 2): bare `unwrap` in the new test.
  - Suggestion (round 2): say that the second parse of the text is deliberate.
- Verdict: REVIEW FAILED in round 1; REVIEW PASSED in round 2.
- Follow-up actions:
  - Author, done before the commit: `Reply::from_json` rejects a reply or finding written as an
    array, with a case each and a mutation of each branch failing a test; the four differences
    are in the module doc, the white-space gap documented rather than changed; literal agent
    names; the binary writes to a path built from `CARGO_MANIFEST_DIR`, with a test of its
    refusal of arguments; the tests split, and the reply and finding tests also check
    `"type": "object"`; the round 2 nit and suggestion applied.
  - Not changed: a JSON Schema validation crate (a heavy dev-dependency; the drift test and the
    pattern tests cover the projection); a test of the binary's write failure (its path is fixed
    at build time). The ADR table and `docs/schemas/README.md` entries are in the documentation
    commit of T3.

### 2026-11-18 18:01 UTC - Task reviewer (Claude Code subagent, Claude Opus 5.5, `review-task` skill)

- Invocation scope: Read-only review of task T3 as a whole on branch `2904-t3-reply-schema`,
  before its pull request: the seven commits of the branch (five reviewed test increments of the
  crate `agent-reply-validator`, the wiring into the pre-commit hook, the Docs Lint workflow and
  the `run-pre-commit-checks` skill, and the documentation) and the uncommitted specification
  update (T3 `DONE`, the branch, a Progress Log line). The per-increment test-design reviews
  above were not redone.
- Inputs: the T3 rows of the Implementation Plan and of Commit Points, D18, P1 and the
  Verification Plan of the specification; `.claude/skills/review-task/SKILL.md`;
  `.claude/skills/add-workspace-member/SKILL.md`.
- Evidence: `cargo test -p agent-reply-validator` passed; `cargo clippy -p agent-reply-validator
  --all-targets -- -D warnings` exited 0; `agent-reply-validator --all` exited 0 with
  `{"kind":"summary","files":0,"problems":0}`; `linter markdown` and `linter cspell` exited 0;
  `git merge-tree --write-tree torrust/develop HEAD` merged with no conflict. The reviewer
  checked by reading the commit order against Commit Points, the hook step order, the workspace
  member and `.dockerignore` entry, the code against D18, and the claims of the documentation
  (the drift test runs in the pre-push hook and in the `Testing` workflow).
- Findings:
  - Minor: the `docs/index.md` row of `schemas/README.md` named only the frontmatter schema.
  - Minor: no record of `docker build --target recipe --file Containerfile .`, which
    `add-workspace-member` asks for after adding a member.
  - Nit: the Progress Log line for T3 did not name this task-level review.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the pull request: the index row names both schemas;
    `docker build --target recipe --file Containerfile .` exited 0 at 18:01:16Z (`date -u`);
    this entry records the review, and the Progress Log line names it.

### 2026-11-19 08:44 UTC - Task reviewer (Claude Code subagent, Claude Opus 5.5, `review-task` skill)

- Invocation scope: Read-only review of task T4 on branch `2904-t4-pr-review-handler`, before its
  pull request: the six commits of the branch (the agent `.claude/agents/pr-review-handler.md`,
  one paragraph each in `wait-for-pr-review` and `process-pr-review`, the inventory
  `removed-profile-rules.md`, sections V1 and V2 of `manual-verification-evidence.md`, and the
  specification's progress). Scenario M3 had not run, by plan: it runs on the first review round of
  this task's pull request.
- Inputs: the T4 rows of the Implementation Plan and of Commit Points, D3, D4, D9, D13, D14, D16,
  D18, P1, P2, the ownership table, AC1 to AC3, AC6 and M1 to M3 of the specification;
  `.github/agents/copilot-suggestions-handler.agent.md`; `.claude/skills/review-task/SKILL.md`;
  the probe run's debug log, result and transcript files.
- Evidence: `linter markdown`, `linter cspell`, `cargo run -q -p agent-review-report-contract`,
  `frontmatter-validator` on the changed files, `--skill-layout` and `--skill-descriptions`, and
  `lychee --offline` on the changed Markdown files exited 0. `frontmatter-validator --all` exited 1
  on 13 diagnostics in specifications that T4 does not touch, which neither the hook nor CI runs.
  Every claim of V1 and V2 was compared with the transcript, metadata, request-cache and debug
  files; all matched but one (the first Minor finding). Each "already held it" row of the
  inventory was compared with the named section of the skill.
- Findings:
  - Minor: V1 said the agent's transcript holds the four skills before its task; they come right
    after it.
  - Minor: the `wait-for-pr-review` paragraph did not say who brings the branch to the event's
    commit, the first part of step 2.
  - Nit: the agent's `description` named only open pull requests, while its body handles merged
    and replaced ones.
  - Nit: the agent file restated commit rules (signed, Conventional, the gate) that `AGENTS.md` and
    `commit-changes` hold.
  - Nit: the agent file and the `wait-for-pr-review` paragraph listed different parts of what the
    session keeps.
  - Suggestion: AC2 is fully evidenced by V2 and its row can say so.
  - Suggestion: re-check the `Grep` and `Glob` entries of `tools` when the build changes.
  - The reviewer judged the M3 plan sound, with five cautions: the GPG passphrase must already be
    cached for a non-interactive session; the session must be allowed the contributor's GitHub
    and push commands, passed to the agent as inputs; no other session acts on the same event
    during the run; the model is read from the agent's transcript, with
    `CLAUDE_CODE_SUBAGENT_MODEL` unset; and the round is the first count of AC9.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the pull request: V1 says "right after its task"; the session brings the
    branch to the event's commit before it delegates, in the skill paragraph and the agent file;
    the description names open, merged and replaced pull requests; the agent file points to
    `AGENTS.md` and `commit-changes` for commits; the agent file points to the skill's list of what
    the session keeps; AC2 is checked with its evidence.
  - Not changed: `Grep` and `Glob` stay in `tools`, as V1 records, for the builds that have them. The M3
    cautions apply to its run.

### 2026-11-24 07:56 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T5, increment 1, on branch `2904-t5-task-reviewer`
  before its commit: the transcript reader `src/transcript.rs` of the crate
  `agent-reply-validator`, with its unit tests. One round, then the author's fixes.
- Inputs: D17 ("How the hook works") and section V8 of `manual-verification-evidence.md`, which
  give the record format the fixtures must model; `.claude/skills/write-unit-test/SKILL.md`;
  `AGENTS.md`; the uncommitted file.
- Evidence: `cargo test -q -p agent-reply-validator --lib transcript` passed with 20 tests in the
  review and 21 after the fixes; clippy with the workspace lints and nightly rustfmt were clean.
  The author ran nine mutations of the production code, and each made at least one test fail:
  accepting any origin kind, accepting `isMeta` records, dropping the type check, reading the
  first request instead of the latest, matching the prefix anywhere in a line, taking the first
  line of a request, no trim, accepting an empty path, and treating a null origin as present. Two
  of them first survived or did not compile: the type check was killed by no case, because the copy
  of a request has block content, which the string check already rejects, and the origin mutation
  left a constant unused. The author added a case whose record is not of type `user` and has
  string content, and reran both. After the review's fixes the author reran the trim, type and
  `isMeta` mutations, and each was killed.
- Findings:
  - Minor: the `windows_line_ending` case could not fail on the trim rule, because `lines()` strips
    `\r\n` before the trim. Fixed: the case is a lone `\r` at the end of the text.
  - Minor: the case named `preloaded_skill_text` used the shape of a harness reminder (a string),
    not the two text blocks of V8. Fixed: it is now `harness_reminder`, and `preloaded_skill_text`
    builds the two-block record.
  - Minor: the case of the copy of a request cannot fail for the reason its name gives, and the
    type check is pinned only by the agent message. Fixed in the comments of both fixtures.
  - Minor: no assertion had a message. Fixed in the groups and the multi-record tests, where the
    case label alone does not say which record won.
  - Nit: the null-origin test pins a tolerance that V8 did not observe. Fixed: its doc comment
    says so.
  - Nit: the helper `transcript` was shadowed by the variable of the same name in every test.
    Fixed: the helper is `jsonl`.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the commit: the six fixes above; the word `rfind` added to
    `project-words.txt`.

### 2026-11-24 08:06 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T5, increment 2, on branch `2904-t5-task-reviewer`
  before its commit: the decision logic of the stop hook, `src/guard.rs` of the crate
  `agent-reply-validator`, with its unit tests, which run against an in-memory `Workspace`. One
  round, then the author's fixes.
- Inputs: D17 ("How the hook works"); `.claude/skills/write-unit-test/SKILL.md`; `AGENTS.md`; the
  uncommitted file.
- Evidence: `cargo test -q -p agent-reply-validator --lib guard` passed with 41 tests in the review
  and 53 after the fixes; clippy with the workspace lints and nightly rustfmt were clean. The
  author ran thirteen mutations of the production code before the review, and each made at least
  one test fail: the limit off by one in both directions, the count not cleared when the reply is
  valid, the missing-folder branch dropped, the key ignoring the agent, the key ignoring the file
  name, a draft treated as outside, `..` allowed, a junk suffix after the issue number allowed, a
  failed count write turned into a refusal, and a task with no line still enforced. Some of them
  first did not compile, because they left a variable unused, and were rewritten. After the fixes
  the author ran five more: a path of any depth under the replies folder, any folder name before
  the file, the validator's messages replaced, a literal line in the refusal, and a literal line in
  the reason of an unreadable file. Each was killed.
- Findings:
  - Major: `classify` returned a specification for any path below its folder, so a reviewer given a
    path of the wrong shape (deeper, or directly in the folder) was refused three times for a path
    it cannot change, and the validator would refuse it whatever the file held. Fixed: under
    `open/` and `closed/` only `<specification>/agent-replies/<file>` is a specification; every
    other shape is outside, and the hook lets the agent finish with a message. D17 says so (a
    consequence of the maintainer's decision for a path outside the folders, not a new
    decision); the cases moved to the outside group, with new ones.
  - Minor: a check on the missing-folder message could not fail, because the path already contains
    the folder. Fixed: it looks for the folder in backticks.
  - Minor: the helper `key` hid the issue number and the file name that the count tests depend
    on. Fixed: two constants, next to the specification and the reply.
  - Minor: the test of the literal line looped over seven runs, stopped at the first failure and
    missed the invalid-content and unreadable cases. Fixed: a test with nine named cases.
  - Minor: two tests copied the wording of the validator's dependency. Fixed: they compute the
    validator's messages for the same content and check that each appears.
  - Nit: an assertion message about the draft path said it has no issue number, although it has one.
    Reworded.
  - Nit: the case of another issue adds little; kept.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the commit: the six fixes above, and the sentence of D17.

### 2026-11-24 08:11 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T5, increment 3, on branch `2904-t5-task-reviewer`
  before its commit: the file-system workspace `src/workspace.rs` of the crate
  `agent-reply-validator` (the count of refusals in `.agents/issues/<issue-number>/reply-guard/`
  and the reading of the reply file), with its unit tests on a disposable folder. One round, then
  the author's fixes.
- Inputs: D17 ("The count of refusals"); `src/guard.rs` for the trait;
  `.claude/skills/write-unit-test/SKILL.md`; `AGENTS.md`; the uncommitted file.
- Evidence: `cargo test -q -p agent-reply-validator --lib workspace` passed with 31 tests in the
  review and 32 after the fixes; clippy with the workspace lints and nightly rustfmt were clean.
  The author ran fourteen mutations before the review, and each made at least one test fail:
  `exists` instead of `is_dir`, a symbolic link read through, a folder not rejected up front, a
  garbage count read as one, the key ignoring the run, the file name or the issue, a slash allowed
  in a name, a leading dot kept, clearing doing nothing, the state folder not created, the failure
  reason not relative to the root, and invalid UTF-8 read with substitute characters. One of them first did not
  compile, because it left a function unused, and was rewritten. After the fixes the author ran
  two more, appending instead of replacing the count and not reducing the names, and each was
  killed.
- Findings:
  - Minor: no test replaced a count that was already there, so writing by appending would have
    passed every test, and the file `1` then `2` would parse as no count. Fixed: a test writes
    twice and reads the file.
  - Minor: one assertion of the test about names that are paths could not fail (the folder
    `escape` was never the target). Fixed: the test compares the folder's listing with the one
    expected name.
  - Minor: the issue number of the keys was hidden in the fixture. Fixed: a constant, with the
    expected paths spelling the number out.
  - Nit: the assertion of the test that clears a count never kept was true from the start. Kept,
    with a message that says the test exists for the absence of a panic.
  - Nit: the test of the failure reason did not check that a reason follows the file name. Fixed.
  - The reviewer judged that real disk use is justified here, since the file system is the
    behaviour under test, each test has its own folder, and the pure part (the safe names) is
    tested without disk.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the commit: the four fixes above.

### 2026-11-24 08:17 UTC - Test-design reviewer (Claude Code subagent, Claude Opus 5.5, `write-unit-test` skill)

- Invocation scope: Read-only review of task T5, increment 4, on branch `2904-t5-task-reviewer`
  before its commit: the binary `agent-reply-guard` (`src/bin/agent-reply-guard/main.rs`) with
  its unit tests, the command-boundary tests `tests/guard.rs` (the built binary in a disposable
  git repository, fed hook input on standard input and a real transcript file), and the move of
  the git-isolation helper of `tests/cli.rs` into `tests/support/mod.rs`. One round, then the
  author's fixes.
- Inputs: D17 ("How the hook works"); `.claude/skills/write-unit-test/SKILL.md`; `AGENTS.md`; the
  uncommitted files.
- Evidence: `cargo test -q -p agent-reply-validator --test guard --bin agent-reply-guard` passed
  with 18 and 4 tests in the review, and 15 and 5 after the fixes; the `cli` tests still pass;
  clippy with the workspace lints and nightly rustfmt were clean. The author ran nine mutations
  of `main.rs` before the review, and each made at least one test fail: a refusal exiting 1, a
  failure exiting 2, the message key renamed, the transcript ignored, the input directory
  ignored, the agent id ignored, the refusal on standard output, arguments accepted, and an empty
  agent id accepted. After the fixes the author ran three more: a strict UTF-8 transcript, the
  process directory ignored, and a failed read of standard input ignored. Each was killed. The
  mutation of the input directory first did not compile (the field became unused) and was
  rewritten.
- Findings:
  - Minor: two command-boundary tests repeated unit tests of `main.rs` (an argument given, input
    that is not JSON). Fixed: the command-boundary copies were deleted, and the unit tests now share
    one assertion of the contract for a failure of the hook itself (exit 1, empty standard output,
    a reason on standard error).
  - Minor: the fallback to the process directory, when the input has no directory, was untested,
    and the subfolder test could not tell which directory was used. Fixed: a test without a
    directory in the input, and the subfolder test starts the process at the root.
  - Minor: a transcript that is not UTF-8 was untested, so a strict decoding would have survived.
    Fixed: a test with an invalid byte in another record, and a unit test for standard input that
    is not UTF-8.
  - Minor: text assertions were tied to wording owned by the committed decision logic. Fixed: they
    check the exit code, the stream, the file the test chose and the count files, and keep the one
    property that belongs to the boundary, that the refusal never carries the literal line.
  - Minor (optional): the table of five paths at the boundary re-tested decision rules. Fixed: two
    cases remain, a draft (the shape of the message) and a path that leaves the repository (nothing
    outside is read).
  - Nit: two failure tests did not check that standard output is empty; one test name was
    imprecise. Fixed.
  - Not tested, as the reviewer accepted: a failed write of the message to standard output, which
    exits 1.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the commit: the fixes above.

### 2026-11-24 13:33 UTC - Test-design reviewer (Claude Code subagent `general-purpose`, Claude Sonnet 5.5, `write-unit-test` conventions)

- Invocation scope: Test-design review of the uncommitted change that makes `agent-reply-guard` also
  the program of a `PreToolUse` hook on `SubagentHandback` (decision D17, "Two registrations of the
  program"): `subagent_transcript_path` in `src/transcript.rs`, `HookInput` and
  `agent_transcript_path` in the binary, and the new command-boundary tests of `tests/guard.rs`.
  One round, then the author's fixes. The reviewer inherited the session's model, which is
  weaker than the Opus 5.5 of the earlier entries; the review is independent of the author's
  reasoning but not of its model.
- Inputs: `.claude/skills/write-unit-test/SKILL.md`; `AGENTS.md`; the uncommitted files.
- Evidence: `cargo test -q -p agent-reply-validator` passed before and after the fixes (277 and
  22 tests after, among others); clippy with the workspace lints and nightly rustfmt were clean.
  The author ran five mutations before the review (the `subagents` folder renamed, `.` allowed in
  an id, an empty id allowed, the session's transcript preferred, the id left out of the file
  name), and each made a test fail.
- Findings:
  - Major: the precedence test passed for a guard that read neither file. Fixed: a table of two
    cases, the named file valid and the derived one missing, and the reverse, each asserting the
    exit code and an empty standard output.
  - Major: the choice between the two paths was tested only at the process level. Fixed: unit tests
    of `agent_transcript_path` for both fields, only the session's, and neither, and for a `null`
    agent transcript.
  - Minor: the session folder was a literal repeated in two places. Fixed: a constant and a helper.
  - Minor: the shared count was tested in one order and never to the limit. Fixed: a test in which
    the two hooks take turns, three refusals and a fourth attempt that is allowed with the message.
  - Minor: the id and session-path cases were incomplete. Fixed: a backslash, a newline and a
    non-ASCII id; an empty session path, a relative one, one without an extension and one with a
    dotted stem.
  - Minor: the failure test did not name the path the session leads to. Fixed.
  - Nit: a test name still said the input lacks the transcript path; the valid hand-back test did
    not check that no state is kept. Fixed. An empty `agent_transcript_path` is pinned as a failure
    that names the empty path.
  - Not applied: the overlap between `../x` and `a/b` in different layers was kept, because the
    boundary case pins the message and the unit cases pin the rule.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - Author, done before the commit: the fixes above.
