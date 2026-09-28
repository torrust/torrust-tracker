---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

# Agent Review Reports - Issue #2281 Frontmatter Validator Command

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-09-26 14:23 UTC - GitHub Copilot Task Reviewer

- Invocation scope: Independent pre-PR verification of issue #2281 on branch
  `2281-frontmatter-validator-command` (26 commits over `torrust/develop`, nothing pushed):
  AC1-AC11, the D9 NDJSON contract and exit codes, D1-D11 and R1-R4 conformance, repository
  conventions, and the completion review.
- Inputs: `ISSUE.md`; `test-design-review.md`; `manual-verification-evidence.md`;
  `implementation-retrospective.md`; crate sources `src/lib.rs`, `src/diagnostic.rs`,
  `src/profile.rs`, `src/syntax.rs`, `src/repository.rs`, `src/bin/frontmatter-validator/`
  (`main.rs`, `discovery.rs`, `git.rs`, `record.rs`), `tests/cli.rs`, `README.md`, and
  `Cargo.toml`; `contrib/dev-tools/git/hooks/pre-commit.sh`; `Containerfile`; the CLI output
  ADR; the `run-pre-commit-checks` skill; the T6 documentation fixes and seven EPIC migrations.
- Evidence:
  - `cargo test --package frontmatter-validator`: exit `0`; 176 tests (library 115,
    `frontmatter-schema` 24, `frontmatter-validator` binary 6, `tests/cli.rs` 31, documentation tests 0),
    matching the recorded count.
  - `cargo clippy --package frontmatter-validator --all-targets -- -D warnings`: exit `0`.
  - `cargo +nightly fmt --all -- --check`: exit `0`.
  - `linter all`: exit `0` (captured with `> log 2>&1; echo $?`).
  - `cargo machete --with-metadata`: exit `0`.
  - `cargo run --quiet --offline --package frontmatter-validator --bin frontmatter-validator --
    --all`: exit `1`, stdout 0 bytes; 44 `error legacy-shape`, 1 `error invalid-allowed-value`
    (#2324 `status`), 2 `warning wrong-scalar-type` (#2295, #2308 `related-pr`), 1
    `warning invalid-reference-syntax` (#2280). The seven D11 EPIC records start with
    `schema-version: 1` and report nothing.
  - `git log --format=%G? torrust/develop..HEAD`: 26 of 26 signed; every subject carries
    `[#2281]`; `torrust/develop` is an ancestor of `HEAD`; no `.github/workflows/` or pre-push
    change.
  - No `allow`/`expect` lint attributes in the crate; the `Cargo.lock` diff adds only `clap`,
    `rstest`, and `serde_path_to_error` to the crate.
  - Every non-test Cargo target added by this branch has a Containerfile stub; the crate is in
    every `--exclude` list.
- Acceptance criteria:
  - AC1 `PASS` - `ArgGroup` `mode` is required and exclusive (`main.rs`); explicit-file,
    directory, `--staged`, and `--all` tests in `tests/cli.rs`; README usage.
  - AC2 `PASS` - `record.rs` serializes the four kinds with the D9 field order and
    `Option` as `null`; key-order assertions for all four kinds; `Outcome::from` asserts empty
    stdout on every CLI run; clap uses `try_parse_from`, so nothing reaches stdout.
  - AC3 `PASS` - `EXIT_SUCCESS`/`EXIT_FAILURE`/`EXIT_USAGE`; warnings-only runs exit `0`; stderr
    write failure exits `1`; covered by `main.rs` and `tests/cli.rs`.
  - AC4 `PASS` - `repository::legacy_shape`, `for_location`, and `experimental_fields` implement
    D8/D10; table tests per location.
  - AC5 `PASS` - `repository_findings` applies status, `spec-path`, artifact, and skill checks
    to drafts/open and only status and `spec-path` (as warnings) to closed; `--staged` builds
    `RepositoryFiles` from the index (`discovery::repository_files`).
  - AC6 `PASS` - `ownership` is applied inside `validate_document` for every mode; D6
    exclusions in `discover` for every mode, tested per mode.
  - AC7 `PASS` - named step `Checking staged Markdown frontmatter` runs `--staged`; no CI or
    pre-push change.
  - AC8 `PASS` - library rejected fixtures and mutation tables (T1-T5) plus the command-boundary
    failure-family table; see finding 1 for its test-design defect.
  - AC9 `PASS` - V1-V7 record real commands, output, and `unshare -rn` offline runs.
  - AC10 `PASS` - reproduced: only `legacy-shape` errors plus the accepted #2324 exception; see
    finding 2 for the breakdown wording.
  - AC11 `PASS` - README usage, migration checklist, and relocation section; ADR row
    `frontmatter-validator` `no-stdout-result`; skill step 3.
- Findings:
  1. Major (test design, blocking) - `tests/cli.rs`
     `it_should_report_each_failure_family_as_one_error_record` writes the four rejected
     fixtures to `docs/issues/open/1-example/ISSUE.md`, but each fixture declares
     `spec-path: docs/issues/open/example/ISSUE.md`. The Arrange comment says the open spec's
     "only defect is the one each case names"; in fact every fixture case carries a second,
     hidden `spec-path-mismatch` defect that the `only_record` assertion tolerates only because
     structural failures short-circuit D7. The same hidden state exists in
     `it_should_exit_one_only_when_a_record_is_an_error` (`WRONG_SCALAR_ISSUE` at the same
     path). The T8 section of `test-design-review.md` records no prose-first Arrange-Act-Assert
     comparison (unlike T1-T5), which would have caught the mismatch.
  2. Minor (evidence accuracy) - V4 in `manual-verification-evidence.md`, the T6 progress entry
     in `ISSUE.md`, and the T5 smoke note in `test-design-review.md` classify the 44
     `legacy-shape` errors as 42 legacy issue specs, the draft EPIC, and one masked draft. The
     reproduced list is 42 primary `ISSUE.md` files (26 drafts, 16 open, including the masked
     `docs/issues/drafts/increase-main-app-integration-test-coverage/ISSUE.md`), the draft EPIC,
     and one supporting file that declares `doc-type: issue`:
     `docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/sample-substantive-bug-spec.md`.
     The T5 note "50 ... matching D10/D11's 42 issues plus 8 EPICs" matches only by
     coincidence (41 unmasked primaries plus the sample file). That sample also has
     `status: draft` in `open/`, so its future migration will hit a lifecycle mismatch.
  3. Minor (documentation) - README "Severity depends on where a document lives" says closed
     specs get advisory warnings and omits that syntax and envelope errors stay errors
     everywhere and that strict v1 records elsewhere report errors (D8).
  4. Minor (completion review) - `implementation-retrospective.md` says the pipe-hidden gate
     lesson was "recorded in agent memory". Under AGENTS.md policy 7 that store is not a source
     of truth; improvements 1 (Containerfile maintenance note) and 3 (captured exit codes) are
     neither applied to a Git-tracked skill or comment nor tracked as follow-ups.
  5. Nit - `tests/cli.rs` `isolated` clears seven `GIT_*` variables but not
     `GIT_CONFIG_PARAMETERS`/`GIT_CONFIG_COUNT` or a global `core.hooksPath`, which
     `Repository::commit` would run in the disposable repository.
  6. Nit - `git.rs` `tracked_files` passes directory arguments as git path patterns, so glob
     characters in a directory name are interpreted; the `:(literal)` prefix would make it exact.
     `discovery.rs` canonicalizes explicit paths, so a symlinked Markdown file is reported and
     classified by its target path.
  7. Nit - the T6 progress entry cites "the four approved fix groups", but only three fix
     commits exist; the fourth decision was to leave #2324 unchanged.
  8. Info, pre-existing and out of scope - the Containerfile has no stubs for
     `contrib/dev-tools/analysis/workspace-coupling/src/lib.rs`,
     `packages/axum-server/examples/token_aware_drain.rs`, or
     `packages/swarm-coordination-registry/examples/bench_peers.rs`.
- Completion review: `implementation-retrospective.md` exists in the folder-style spec and
  covers the container test-stage gap, masked errors, out-of-spec baseline findings, and the
  pipe-hidden gate failure; accepted subject to finding 4.
- Issue-spec updates: none; checkboxes left unchanged as instructed.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Finding 1: write the fixture cases to the fixtures' own `spec-path`
    (`docs/issues/open/example/ISSUE.md`) or set `OPEN_SPEC` to it for all cases, apply the same
    to `it_should_exit_one_only_when_a_record_is_an_error`, and add a T8 prose-first comparison
    (with a mutation check of the new table) to `test-design-review.md`; then re-run the crate
    tests and request a re-review.
  - Finding 2: correct the V4, T6, and T5 breakdowns to name the supporting sample file.
  - Findings 3-4: tighten the README severity summary; move the retrospective's
    exit-code lesson into a Git-tracked skill or record it as a follow-up.
  - Findings 5-7: optional hardening and wording fixes.

### 2026-09-26 21:47 UTC - GitHub Copilot Task Reviewer

<!-- cspell:ignore dirlink Magit pathspec pathspecs PATHSPECS -->

- Invocation scope: Follow-up pre-PR review of issue #2281 on branch
  `2281-frontmatter-validator-command` (29 commits over `torrust/develop`, nothing pushed).
  It verifies the remediation of findings 1-8 from the 2026-09-26 14:23 UTC entry, which
  recorded REVIEW FAILED, and checks the remediation for new defects. The remediation is in
  `fix(frontmatter): [#2281] address pre-PR review findings in the command`,
  `docs(frontmatter): [#2281] apply review and retrospective guidance`, and
  `docs(issues): [#2281] record the failed pre-PR review and its remediation`.
- Inputs: the earlier entry; the three remediation diffs; `tests/cli.rs`; `discovery.rs`;
  `git.rs`; `repository.rs`; the rejected and accepted fixtures; the crate `README.md`;
  `Containerfile`; the `run-pre-commit-checks` skill; `ISSUE.md` (progress log and AC table);
  `manual-verification-evidence.md` (V4); `test-design-review.md` (T5 note and
  "T8 Review Remediation"); `implementation-retrospective.md`.
- Evidence:
  - `cargo test --package frontmatter-validator`: exit `0`; 178 tests (library 115,
    `frontmatter-schema` 24, `frontmatter-validator` binary 6, `tests/cli.rs` 33, documentation
    tests 0), matching the recorded count.
  - `cargo clippy --package frontmatter-validator --all-targets -- -D warnings`: exit `0`.
  - `cargo +nightly fmt --all -- --check`: exit `0`.
  - `linter all > /tmp/l.log 2>&1; echo $?`: `0`.
  - `cargo machete --with-metadata`: exit `0`.
  - `cargo run --quiet --offline --package frontmatter-validator --bin frontmatter-validator --
    --all`: exit `1`, stdout 0 bytes; 44 `error legacy-shape`, 1 `error invalid-allowed-value`
    (#2324), 2 `warning wrong-scalar-type` (#2295, #2308), 1 `warning invalid-reference-syntax`
    (#2280). Legacy-shape paths: 42 `ISSUE.md` (26 drafts, 16 open, including
    `increase-main-app-integration-test-coverage`), 1 `EPIC.md`
    (`docs/issues/drafts/generalize-error-events/EPIC.md`), and 1 other file
    (`docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/sample-substantive-bug-spec.md`).
  - T5-era baseline, reproduced from the tree after
    `feat(frontmatter): [#2281] add repository-aware checks for strict specs` by extracting
    `docs/` and `.github/skills/` into a disposable git repository under `.tmp/` and running the
    built binary with `--all`: 50
    `legacy-shape` records, split into 42 `ISSUE.md` (including
    `docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md`), 7 `EPIC.md`, and the #2230
    sample. This matches the corrected T5 note.
  - Finding-1 check in a disposable repository under `.tmp/`: each rejected fixture, and each
    `open_strict_issue` case written to `docs/issues/open/example/ISSUE.md`, emits exactly its
    named record. With the named defect removed, each exits `0` with no records. The removals
    were: a quoted `github-issue` unquoted, `owner` deleted, `completed` changed to `planned`, the
    URL changed to `"issue #2264"`, `done` changed to `planned`, and `docs/removed.md` replaced by
    a tracked file. The relocated closed case gives one `warning wrong-scalar-type` and exit `0`,
    and clean once fixed. Control: the same repaired fixture at the old
    `docs/issues/open/1-example/ISSUE.md` emits `error spec-path-mismatch`, confirming the hidden
    defect is gone.
  - Edge probes with the built binary in a disposable repository under `.tmp/`: `docs`, `./docs/`,
    `docs/sub/..`, and `..`/`../b.md`/`../..` from a subdirectory resolve correctly. `d*` selects
    only `d*/inside.md`. `link.md` and `./link.md` report `link.md`. A dangling symlink exits `2`
    with "does not exist". `dirlink` and `dirlink/b.md` report the target `docs/...` paths. With
    `GIT_LITERAL_PATHSPECS=1`, `frontmatter-validator docs` exits `0` with no records; see N1.
  - `GIT_LITERAL_PATHSPECS=1 cargo test --package frontmatter-validator --test cli`: exit `101`;
    `it_should_expand_a_directory_to_its_tracked_markdown_files` and
    `it_should_treat_a_directory_argument_literally_rather_than_as_a_glob` fail.
  - `git --literal-pathspecs ls-files -- 'd*' docs`, used as a candidate fix, lists the correct
    files with or without an inherited `GIT_LITERAL_PATHSPECS=1`. With `GIT_GLOB_PATHSPECS=1` it
    fails loudly (`fatal: global 'literal' pathspec setting is incompatible ...`, rc `128`).
  - The `docs(issues): [#2281] record the failed pre-PR review and its remediation` diff leaves
    earlier progress-log lines unchanged. It adds two entries and changes `last-updated-utc` and
    the AC8 table row. V4 and the T5 note were corrected in place, each with a correction note.
  - The three remediation commits are signed (`%G?` = `U`) and carry `[#2281]`.
    `torrust/develop` is an ancestor of `HEAD`, and the working tree was clean before this report.
- Earlier findings, remediation status:
  1. RESOLVED - `OPEN_SPEC` is now the fixtures' `spec-path` (`tests/cli.rs` line 21).
     `relocated_fixture` (lines 31-35) moves the closed case with `status: done`. Both tests
     carry only their named defect, as shown by the empirical check above. The T8 prose-first
     comparison and four mutations are recorded in `test-design-review.md`.
  2. RESOLVED - V4 and the T5 note match the reproduced current and T5-era classifications. The
     T6 entry stays as history, and an appended entry corrects it.
  3. RESOLVED - the README severity list names the errors-everywhere and outside-issue-folder
     rules; see nit N3.
  4. RESOLVED - the agent-memory claim is removed. Improvement 1 is in the Containerfile
     maintenance comment and improvement 3 is in the `run-pre-commit-checks` skill.
  5. RESOLVED - `GIT_CONFIG_PARAMETERS` and `GIT_CONFIG_COUNT` are cleared, and fixture commits
     pass `core.hooksPath=/dev/null`.
  6. PENDING - symlinked files now keep their own path, with a regression test. The
     literal-directory change introduced regression N1.
  7. RESOLVED - an appended entry clarifies the wording without rewriting history.
  8. N/A - pre-existing and out of scope; unchanged.
- New findings:
  - N1. Major (blocking, regression from the finding-6 fix) - `git.rs` `tracked_files`
    (line 29) prefixes the directory with `:(literal)` pathspec magic. When
    `GIT_LITERAL_PATHSPECS=1` is inherited, git treats that prefix as literal text and matches
    nothing. `frontmatter-validator <dir>` then exits `0` with no records: a silent false pass.
    `git --literal-pathspecs` exports that variable to hooks and subprocesses, and front ends
    such as Magit use that option by default. The same environment fails two `tests/cli.rs`
    tests, which would break pre-push. Pre-commit (`--staged`) is unaffected.
  - N2. Nit - `tests/cli.rs` `relocated_fixture` (lines 31-35) uses `str::replace`, which
    silently does nothing if a fixture's `spec-path` or `status` text changes. The closed case
    would then regain hidden state that the structural error masks. Assert that each
    replacement matched.
  - N3. Nit - `README.md` line 46 says strict v1 records outside `docs/issues/` get "no lifecycle
    checks". In fact `repository_findings` skips every D7 check there (`spec-path`, artifacts,
    skills), so "no repository-aware checks" is accurate.
  - N4. Nit - the `ISSUE.md` progress entry at line 546 is stamped `15:10 UTC`, but the report it
    cites is stamped `14:23 UTC`.
  - N5. Info - an in-repository symlink to a file outside the repository is now validated. It
    was previously rejected with "outside the repository" (exit `2`). Symlinked directories still
    report target paths. Both are consistent with how `--all` reads tracked symlinks. Document
    the behavior if it is intended.
- Completion review: `implementation-retrospective.md` is updated with the review lessons and
  applied improvements. The folder-style spec, test-design, and manual-evidence records are
  consistent with the reproduced output. Accepted.
- Issue-spec updates: none. No checkbox or AC row was changed, because the verdict is failed and
  this report is the only authorized write.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - N1: in `git.rs`, drop the `:(literal)` prefix and pass the global `--literal-pathspecs`
    option, or remove the `GIT_*_PATHSPECS` variables from the git child environment. Add a
    `tests/cli.rs` regression test that sets `GIT_LITERAL_PATHSPECS=1` for the binary and
    expects the directory's records. Re-run the crate tests and request a re-review.
  - N2-N4: optional hardening and wording fixes. N5: document the behavior or leave it as is.

### 2026-09-28 07:00 UTC - GitHub Copilot Task Reviewer

- Invocation scope: Final independent pre-PR re-review of issue #2281 on branch
  `2281-frontmatter-validator-command` (31 commits over `torrust/develop`, nothing pushed).
  It verifies the N1 remediation in `fix(frontmatter): [#2281] expand directories under
  GIT_LITERAL_PATHSPECS`, all AC1-AC11 acceptance criteria, prior-review remediation,
  repository conventions, and completion evidence.
- Inputs: `ISSUE.md`; this review log; `manual-verification-evidence.md`;
  `test-design-review.md`; `implementation-retrospective.md`; the validator crate sources and
  CLI tests; the pre-commit hook; and the branch diff from `torrust/develop`.
- Evidence:
  - `GIT_LITERAL_PATHSPECS=1 cargo test --package frontmatter-validator --test cli`: exit `0`;
    34 tests passed. This directly exercises the N1 condition with the inherited environment.
  - `cargo test --package frontmatter-validator`: exit `0`; 179 tests passed.
  - `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`: exit `0`;
    all nine checks passed.
  - Offline `frontmatter-validator --all`: exit `1`, zero stdout, and 48 expected diagnostics:
    44 `legacy-shape` errors, the accepted #2324 `invalid-allowed-value` error, and three
    warnings.
  - `git diff --check torrust/develop...HEAD`: clean. The worktree is clean and all branch
    commits are GPG-signed.
- N1 remediation: PASS. `git.rs` uses Git's global `--literal-pathspecs` option rather than
  inline pathspec magic, so explicit directories expand correctly when
  `GIT_LITERAL_PATHSPECS=1` is inherited. The focused regression and full CLI suite passed.
- Acceptance criteria: AC1-AC11 PASS. Mode selection, NDJSON-only stderr, exit codes,
  repository-aware validation, ownership/exclusions, staged-hook rollout, failure-family tests,
  manual evidence, baseline behavior, and documentation/migration work are all present and
  verified.
- Findings: None blocking. No non-blocking concerns were raised.
- Completion review: PASS. The folder-style issue includes test-design, manual-verification,
  and retrospective evidence, and records how the two previous failed reviews were remediated.
- Issue-spec updates: None; the completed acceptance-criteria state remains unchanged.
- Verdict: REVIEW PASSED

### 2026-09-28 09:21 UTC - GitHub Copilot Rebase Correction

- Invocation scope: Correct the current-branch interpretation of the 2026-09-26 21:47 UTC and
  2026-09-28 07:00 UTC review entries after the implementation branch rebased onto newer
  `develop`.
- Inputs: the rebased branch, current `develop`, `ISSUE.md`, and
  `manual-verification-evidence.md`.
- Evidence:
  - The earlier reviews ran before the rebase from the then-current `develop`; their recorded
    results remain historical observations and do not describe the rebased branch.
  - A fresh offline `frontmatter-validator --all` run after repairing the rebased open specs exits
    `1`, writes zero bytes to stdout, and reports 35 `legacy-shape` errors plus four advisory
    closed-spec warnings (one `invalid-field-value`, one `invalid-reference-syntax`, and two
    `wrong-scalar-type`).
  - The current branch has no non-`legacy-shape` errors. #2324 is now archived, so it is no longer
    an AC10 exception.
- Correction: This entry supersedes the older report counts only for the rebased branch. It does
  not alter their evidence about the pre-rebase branch.
- Findings: None blocking after the current-baseline repairs.
- Verdict: REVIEW PASSED
