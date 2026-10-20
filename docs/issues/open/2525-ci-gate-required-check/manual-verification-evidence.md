---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2525-ci-gate-required-check/ISSUE.md
last-updated-utc: 2026-10-20 17:52
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands
or interactions actually performed against the artifact; nothing here is invented.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-20, between about 12:00 and 12:20. Per-step timestamps were not
  captured; the steps ran in the order listed, each as its own command after the previous one
  finished.
- Artifact under test: the `ci-gate` crate (`contrib/dev-tools/checks/ci-gate`), `classify`
  subcommand, as built on branch `2525-ci-gate-change-detection` from `develop` at `5956368b3`
  (`Merge torrust/torrust-tracker#2556`). Patch: `feat(ci-gate): [#2525] add the change-detection tool`
  (committed after maintainer review).
- Operating system / environment: Linux workstation; stable Rust 1.99.0 (`rustc 1.99.0`), nightly
  `cargo 1.101.0-nightly` for formatting; `gh` authenticated as `josecelano`; Docker for V8.
- Prerequisites and setup performed: `cargo build -p ci-gate` (debug), producing `target/debug/ci-gate`.

## Verification Processes

### V1 - TTY refusal (scenario M8)

- Goal: show that the binary refuses to run when stdout is a terminal, as the global CLI output
  contract requires for a `stdout-result-data` command. The test harness always pipes stdout, so
  this cannot be a Rust test.
- Initial state: built binary; a pseudo-terminal from `script`.
- Status: `DONE`

```sh
script -qec "target/debug/ci-gate classify --base-branch develop </dev/null" /dev/null; echo "exit=$?"
```

```text
{"kind":"tty_refusal","message":"stdout is a TTY; pipe the output to consume result data"}
exit=2
```

Result: one JSON diagnostic on stderr, exit code 2, nothing on stdout.

### V2 - Classify this branch's own change set (scenario M9)

- Goal: run the tool the way a workflow would, on a real change set.
- Initial state: this branch's changes against `torrust/develop`: the changed, new, and unstaged
  files (14 paths when re-run on 2026-10-20 13:03 UTC, after the input format changed to
  NUL-separated records; the first run, with newline-separated input, had 11).
- Status: `DONE`

```sh
{ git diff --name-only -z torrust/develop...HEAD; git ls-files --others --exclude-standard -z; git diff --name-only -z; } \
  | sort -zu | target/debug/ci-gate classify --base-branch develop
```

Result (the JSON object, formatted): 14 changed paths. `docs-lint`, `testing`, `os-compatibility`,
`coverage`, and `container` are needed, each with the explanation
``"`Cargo.lock` matches none of the ignored patterns `**/*.md`, `project-words.txt`"``;
`db-compatibility`, `db-benchmarking`, and `security-scan` are skipped, each naming the patterns no
path matched. A change of `docs/adrs/index.md` and `project-words.txt` alone needs only `docs-lint`.
A run without `--base-branch` prints
``{"kind":"usage_error","message":"`--base-branch` is required. usage: ci-gate classify ..."}``
and exits 2.

### V3 - Differential check against what GitHub actually ran (acceptance criterion AC4)

- Goal: show that the tool selects the same job groups that today's workflow filters selected, on
  real pull requests, not only on the unit-test cases.
- Initial state: the 60 most recently merged pull requests into `develop`, re-run on 2026-10-20
  at about 13:05 UTC with the NUL-separated input (merged 2026-10-08 12:32 to 2026-10-20 13:00 UTC;
  the first run, with newline-separated input, covered a slightly older set). For each, the changed files from the GitHub API, and the workflow runs of event
  `pull_request` at the pull request's final head commit, mapped to groups by workflow name.
- Status: `DONE`

A one-off Python script joined the two sources. It is not retained, because it is a single
comparison against GitHub's behavior, not a durable test (the durable tests are the Rust ones); the
commands it ran are:

```sh
gh pr list --repo torrust/torrust-tracker --state merged --base develop --limit 60 --json number,mergedAt
gh api repos/torrust/torrust-tracker/pulls/<n>                       # head sha, base branch
gh api --paginate "repos/torrust/torrust-tracker/pulls/<n>/files?per_page=100"
gh api "repos/torrust/torrust-tracker/actions/runs?head_sha=<sha>&event=pull_request&per_page=100"
printf '%s\0' <files> | target/debug/ci-gate classify --base-branch <base>
```

The workflow names `Docs Lint`, `Testing`, `OS Compatibility`, `Generate Coverage Reports (PR)`,
`Container`, `Database Compatibility`, `Database Benchmarking`, and `Security Scan` map to the
groups `docs-lint`, `testing`, `os-compatibility`, `coverage`, `container`, `db-compatibility`,
`db-benchmarking`, and `security-scan`. The per-pull-request results are in
[`evidence/ci-gate-differential-check.txt`](evidence/ci-gate-differential-check.txt).

Result: **60 of 60 pull requests match**, 0 differ, 0 skipped.

| Groups run                                                          | Pull requests |
| ------------------------------------------------------------------- | ------------: |
| `docs-lint` only                                                    |            30 |
| `docs-lint`, `testing`, `os-compatibility`, `coverage`, `container` |            24 |
| the five above plus `db-compatibility` and `db-benchmarking`        |             4 |
| the five above plus `security-scan`                                 |             2 |

Limits of this check: every pull request in the sample targets `develop`, so the base-branch
filters of `coverage` (`develop` only) and `container` (`develop` or `main`) are covered by unit
tests only; no `paths` rule other than the three that fired (the database workflows and the
security scan) was exercised by a real pull request here.

### V4 - Mutation check of the unit tests

- Goal: show that the tests fail when a rule is wrong, not only that they pass.
- Initial state: all tests passing (87 unit, 3 command-line).
- Status: `DONE`

Ten mutants of `change_detection.rs` were applied one at a time and the library tests run for
each, with the file restored afterwards (`cmp` against a saved copy exited 0). Each mutant compiled
and each was killed:

| Mutant                                                         | Failing tests |
| -------------------------------------------------------------- | ------------: |
| directory prefix without its trailing slash (`tracker-core`)   |             1 |
| file extension matched anywhere in the path                    |             4 |
| exact path matched by suffix                                   |             5 |
| `coverage` no longer limited to `develop`                      |             3 |
| `container` limited to `develop` only                          |             1 |
| `paths-ignore` skipping a change that mixes documentation/code |             5 |
| `persistence-benchmark` dropped from `db-benchmarking`         |             1 |
| `Containerfile` matched below the repository root              |             1 |
| `docs-lint` no longer needed for a change with no paths        |             1 |
| blank lines kept as changed paths                              |             1 |

A first version of the `docs-lint` mutant did not compile (it left a variant unused, which the
workspace's `-D warnings` rejects), so it proved nothing; it was redone as a mutant that compiles.

### V5 - Container build context

- Goal: the new member is admitted to the Docker build context, so `cargo chef prepare` can run.
- Status: `DONE`

```sh
cargo metadata --no-deps --format-version 1 > /dev/null    # exit 0
docker build --target recipe --file Containerfile .         # exit 0
```

### V6 - Review findings: file names with spaces, and git's quoting (PR #2574)

- Goal: show that the two inputs Copilot's review named are classified correctly end to end, with
  real git output, and that the new tests fail on the original bugs.
- Status: `DONE`

Real git output, in a throwaway repository (`/tmp/qp`, removed afterwards), with the files
`packages/tracker-core/src/café.rs`, `README.md` with a trailing space, and `project-words.txt` with a
leading space:

```sh
git diff --cached --name-only        # prints the quoted name "packages/tracker-core/src/caf\303\251.rs"
git diff --cached --name-only -z     # prints the names as they are, NUL-separated
git -C /tmp/qp diff --cached --name-only -z | target/debug/ci-gate classify --base-branch develop
```

Result with `-z`: `testing`, `db-compatibility`, and `db-benchmarking` all needed. The default output
would have given the quoted `café` name, whose leading quote misses the `packages/tracker-core/`
prefix, and `trim()` would have turned the two spaced names into the documentation files the workflows
ignore.

Three mutants that reintroduce the bugs were each killed: trimming each record (the parser test and the
end-to-end space test fail), keeping git's quoted form of the non-ASCII name (the parser test fails),
and splitting records on newlines too (the parser test fails). Cargo stops after the first failing
test target, so the first mutant was re-run with `--no-fail-fast` to see the end-to-end test fail as
well.

Second review round: the same checks were extended for three more findings. Two more mutants were
killed: a parser that drops a path that is not valid UTF-8 instead of decoding it (three unit tests
fail), and a binary that writes a stray line to stderr on success (all four success-path
command-line tests fail). The final run has 104 unit and 5 command-line tests.

Third review round: a real binary run with an argument that is not valid UTF-8 (`--base-branch` given
the bytes `develop` followed by `0xFF`, and a subcommand with the same byte) panicked with exit code 101
and a plain-text message, not the JSON usage diagnostic with exit code 2. After the fix it exits with 2
and one `usage_error` record on stderr. The new Unix-only command-line test was seen to fail against the
original behaviour first. The final run has 104 unit and 6 command-line tests.

### V7 - The verdict (task T3)

- Goal: show that the verdict decides correctly on the real output of the real tools, and that its tests
  fail when a rule is wrong.
- Initial state: branch `2525-ci-gate-verdict` from the merge of PR #2574, with the new work staged.
- Status: `DONE`

The change set was the staged diff against `torrust/develop` (7 paths), piped as NUL-separated records
into `classify`, whose output was then fed unchanged to `verdict` with hand-written results:

```sh
git diff --cached --name-only -z torrust/develop | target/debug/ci-gate classify --base-branch develop
# needed: docs-lint, testing, os-compatibility, coverage, container
```

| Case                                             | Results given                                  | Verdict | Exit | Explanation                                                                                                                                             |
| ------------------------------------------------ | ---------------------------------------------- | ------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| every needed group succeeded                     | the five needed groups, all `success`          | `pass`  |    0 | `every needed group succeeded: docs-lint, testing, os-compatibility, coverage, container`                                                               |
| `testing` failed and three groups never reported | `docs-lint` success, `testing` failure         | `fail`  |    0 | `needed groups without a success: testing (failure), os-compatibility (missing), coverage (missing), container (missing)`                               |
| the same, with `--override-actor josecelano`     | the same                                       | `pass`  |    0 | ``overridden by `josecelano`; needed groups without a success: testing (failure), os-compatibility (missing), coverage (missing), container (missing)`` |
| a group that is not needed failed                | five needed `success`, `security-scan` failure | `pass`  |    0 | `every needed group succeeded: ...` (`security-scan` is ignored)                                                                                        |
| unreadable input (`{"classification": 1}`)       | -                                              | none    |    1 | one `runtime_error` record on stderr, nothing on stdout                                                                                                 |

A first run of the same commands used a branch with no commits, so the change set was empty and only
`docs-lint` was needed; the cases that expect a failing group then passed, which showed the setup was
wrong and not the tool. The table above is the rerun on the staged diff.

Ten mutants of `verdict.rs` were applied one at a time and the whole test suite run for each, with the
file restored afterwards (`diff` against a saved copy). Two of them did not compile the first time,
because the workspace's `-D warnings` rejects an unused value, so they ran no test; they were rewritten
and rerun. All ten compiled and were killed:

| Mutant                                                     | Failing tests |
| ---------------------------------------------------------- | ------------- |
| groups that are not needed count towards the verdict       |            14 |
| a needed group that was skipped counts as a success        |             1 |
| a needed group with no result counts as a success          |             4 |
| a needed group that was cancelled counts as a success      |             2 |
| a needed group that failed counts as a success             |             4 |
| the override is ignored                                    |             2 |
| the override is recorded when there is nothing to override |             1 |
| a failing verdict with no override passes                  |             6 |
| the completeness check of the classification is removed    |             2 |
| unknown input fields are accepted                          |             1 |

Review round on PR #2580: a real run with an extra field (`"unexpected": true`) inside the
classification, and again inside a group decision, still printed `pass` with exit code 0, because
`deny_unknown_fields` on the verdict input does not reach nested types. After the fix both exit with 1,
naming the unknown field, and nothing is printed on stdout. Two more mutants, one per nested type with the
denial removed, were each killed by their own test case, so the verdict has 12 mutants killed in all. The
final run has 134 unit and 10 command-line tests.

Second review round on PR #2580: a classification whose groups arrive in reverse order printed its
problems and its explanation in that reverse order, not in the documented group order. The new test with a
reversed classification failed before the fix, which is the mutant for this rule, and passes after it, so
the verdict has 13 mutants killed in all. The override explanation in the table above was checked against
the real output of the command and quoted verbatim. The final run has 135 unit and 10 command-line tests.

## Failures and Follow-up

None.
