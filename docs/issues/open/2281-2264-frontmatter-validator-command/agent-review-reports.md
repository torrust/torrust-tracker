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
