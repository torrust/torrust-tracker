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

### V8 - The runner time that the gate adds to a documentation-only pull request (task T4, experiment E1)

- Goal: measure what a gate job costs in runner time on a documentation-only change, in four ways of
  getting the `ci-gate` binary, next to the real `Docs Lint` job.
- Initial state: the fork `josecelano/torrust-tracker` (public, Actions enabled, `ubuntu-latest` runners)
  with a throwaway branch of `develop` at `6fd4bc597` carrying the sandbox workflows in
  `evidence/t4-sandbox/`, and a branch with one Markdown file added. The change classified as
  `docs-lint` only, in every variant.
- Status: `DONE`

One workflow run was started by a push and then rerun with `gh run rerun` until it had 12 attempts. The
five jobs of an attempt start together, so they see the same runner conditions, and each job is one
variant, plus a copy of the steps of the real `Docs Lint` job as the reference. A variant includes
everything it needs, checkout and toolchain setup included, and ends by classifying the diff between the
base and the branch. Runner time is the job's `completed_at` minus `started_at`, as in
[`pr-ci-cost-baseline.md`](pr-ci-cost-baseline.md). All 60 jobs succeeded, and the log of each variant
shows `changed_path_count` 1 and `docs-lint` as the only needed group. The per-job data is in
[`evidence/t4-gate-cost-runs.csv`](evidence/t4-gate-cost-runs.csv). The workflow files saved in `evidence/t4-sandbox/` are the sandbox files; only the comment on their first line, which said
they were never merged upstream, was reworded when they were stored here as evidence.

| Variant                                                                                         | Attempt 1 | Median of attempts 2 to 12 | Mean of attempts 2 to 12 | Range of attempts 2 to 12 |
| ----------------------------------------------------------------------------------------------- | --------- | -------------------------- | ------------------------ | ------------------------- |
| `docs-lint-reference` (the steps of the real `Docs Lint` job)                                   | 3.57      | 1.02                       | 1.47                     | 0.67 to 4.25              |
| `gate-cold`: toolchain setup, `cargo build --release -p ci-gate`                                | 0.28      | 0.40                       | 0.41                     | 0.27 to 0.67              |
| `gate-rust-cache`: the same with the Rust cache                                                 | 0.72      | 0.37                       | 0.37                     | 0.23 to 0.60              |
| `gate-binary-cache`: binary restored from a cache keyed on a hash of the crate and `Cargo.lock` | 0.33      | 0.13                       | 0.15                     | 0.08 to 0.40              |
| `gate-release-download`: binary downloaded from a release asset                                 | 0.13      | 0.12                       | 0.12                     | 0.10 to 0.17              |

Runner minutes per job. In `gate-binary-cache`, attempt 1 is a cache miss that builds and saves the
binary, and attempts 2 to 12 are cache hits.

What it shows:

- Building the tool from scratch is cheap: a median of 0.40 minutes, which is the toolchain setup, the
  compile of `serde`, `serde_json`, and `ci-gate`, and the run. The Rust cache does not help, because
  there is almost nothing to cache; its first run is slower (0.72) because it saves the cache.
- A prebuilt binary costs about 0.12 minutes, whether it comes from a cache hit or from a release asset.
- The reference `Docs Lint` job shows the same slow and fast runs as the baseline: three slow attempts
  (3.57, 3.27, 4.25) and then about one minute.
- The gate will need two tool runs, `classify` before the heavy jobs and `verdict` after them, so the cost
  to a documentation-only head is about twice the figure of one job: roughly 0.8 minutes with a cold build
  each time, and roughly 0.25 minutes with a prebuilt binary, against a `Docs Lint` baseline of 0.85 minutes
  (median) per documentation-only head. Passing the binary between the two jobs would save one build,
  but it was not measured.

Limits: one commit rerun 12 times in a fork, so caches behave as they do for a branch that is pushed
repeatedly. Across different pull requests a cache hit depends on cache scoping and on when caches are
evicted, which was not measured. The `gate-binary-cache` key includes `Cargo.lock`, so it misses after
every dependency update. Only job minutes were measured, not wall-clock time: `classify` would sit on
the critical path of every pull request, for about 0.4 minutes with a cold build.

### V9 - Whether a later check run named `CI gate` supersedes an earlier one (task T4, experiment E2)

- Goal: find out, with a ruleset that requires the status check `CI gate` and a real merge push, how
  GitHub treats several check runs of that name for the same head commit.
- Initial state: two throwaway base branches in the fork, each with a ruleset that requires the status
  check `CI gate` (Actions app, not strict), the pull request rule of the upstream rulesets, and signed
  commits. The rulesets and the workflows are in `evidence/t4-sandbox/`. The job named `CI gate` decides
  from a label: it fails while the label `exp-fail` is set, and passes otherwise.
- Status: `DONE`

The full output of every observation is in
[`evidence/t4-check-run-superseding.txt`](evidence/t4-check-run-superseding.txt). Summary:

| Case                                                                      | Check runs for the head commit                                        | Merge state | Merge push                                              |
| ------------------------------------------------------------------------- | --------------------------------------------------------------------- | ----------- | ------------------------------------------------------- |
| (b) a pass, then a later failure from a new run (label added)             | success at 09:25, then failure at 09:26                               | `BLOCKED`   | rejected: `Required status check "CI gate" is failing.` |
| (a) after (b), a pass from a later run (label removed)                    | failure at 09:26, then success at 09:27, with the first success above | `UNSTABLE`  | accepted                                                |
| (c) two workflows each define `CI gate`; one fails, the other passes      | two failures and two successes, in four check runs                    | `BLOCKED`   | rejected: `Required status check "CI gate" is failing.` |
| (c, then a) the failing one is removed and runs again, so both pass later | the two earlier failures, and four passes (two new, two earlier)      | `UNSTABLE`  | accepted                                                |

`UNSTABLE` is GitHub's state for required checks that pass while a non-required check, here `Docs Lint`,
was still running, so it does not block a merge. The `statusCheckRollup` state stays `FAILURE` after the
later passes, because it still lists the earlier failures, so it cannot be used to decide whether a pull
request may merge; the ruleset decides, and in the single-workflow cases it followed the later run.

What it shows:

- **The later run wins, in both directions, for the same job of the same workflow.** A later passing run
  overrides an earlier failing one for the same commit, and a later failing run overrides an earlier
  passing one. This is what the override needs: a run
  started by a `labeled` event can turn a failing `CI gate` into a passing one, and the run started by the
  `unlabeled` event turns it back, with no ruleset change.
- **Two workflows with the same check name do not combine into a pass.** While one of them was failing,
  the other's passing run did not help, and the merge was rejected. The gate should therefore be one
  workflow with one job named `CI gate`, whose runs for label events are in the same workflow as the
  runs for pushes, so that a later run of that job supersedes an earlier one. Not tested: a later passing run
  from a different workflow than the failing one; the design avoids needing it.
- Creating a pull request with a label starts two runs, `opened` and `labeled`, for the same commit.
- In the fork the default workflow permission is `write`; in `torrust/torrust-tracker` it is `read`
  (`gh api repos/<owner>/<repo>/actions/permissions/workflow`). The sandbox therefore says nothing about
  what the default token of upstream can do, which T5 has to take from the documentation of GitHub and
  verify on real pull requests when T6 lands.

Cleanup, 2026-10-21 at 09:41 to 09:42 UTC: both sandbox rulesets, the six `exp-2525-*` branches, the
`exp-2525-binary` release and its tag, and the labels `exp-fail` and `exp-second-fail` were deleted from
the fork, and no pull request is open. Verified afterwards: the fork has only its two tag rulesets, no
`exp-` branch, no `exp-2525` tag, and no open pull request. Fork pull requests 20 and 21 remain, merged, as
history of the experiment; the workflow runs remain too.

### V10 - Cross-run state of the override (task T5)

- Goal: show, with hosted runs, how a `CI gate` job started by a label event can learn the results of the
  groups, who applied the override label, whether that actor may, and whether the override is still valid
  after a push, using only a read-only token.
- Initial state: a throwaway base branch in the fork `josecelano/torrust-tracker` (from `develop` at
  `4ad183d18`) with one orchestrator workflow, and a ruleset that requires the status check `CI gate`. The
  workflow and the ruleset are in `evidence/t5-sandbox/`. It declares `permissions` of `contents`, `checks`,
  `issues`, and `pull-requests`, all `read`, so its token has no write scope. A matrix job named after the
  group runs a stub for `docs-lint` and `db-benchmarking`, and fails when a marker file exists in the pull
  request; label events skip these jobs. The `CI gate` job builds `ci-gate` from the base branch, classifies
  the pull request's files, reads the results and the override from the GitHub API, and decides with
  `ci-gate verdict`. Fork pull request 22 changes a file under `packages/persistence-benchmark/` (so
  `docs-lint` and `db-benchmarking` are needed) and carries the marker that makes `db-benchmarking` fail.
- Status: `DONE`

The log of the `CI gate` job of every run is in
[`evidence/t5-gate-logs.txt`](evidence/t5-gate-logs.txt). Times are UTC, 2026-10-21.

| Run | Event, at                      | Head        | What the gate read and decided                                                                                                                                                                          | Result                             |
| --- | ------------------------------ | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------- |
| 1   | `opened`, 10:38                | `270a89e08` | `docs-lint` success, `db-benchmarking` failure. No label. (My script passed an empty override actor, so the tool printed a usage error instead of a failing verdict.)                                   | `CI gate` failed                   |
| 2   | `labeled` `ci-override`, 10:40 | `270a89e08` | Group jobs skipped. Results read from run 1's check runs. Events API: actor `josecelano`, the same as the event's sender. Role `admin`.                                                                 | pass, `overridden_by` `josecelano` |
| 3   | `unlabeled`, 10:43             | `270a89e08` | Ran the first revision of the workflow (see below), with the same empty-actor error as run 1.                                                                                                           | `CI gate` failed                   |
| 4   | `labeled`, 10:44               | `270a89e08` | As run 2, still on the first revision of the workflow.                                                                                                                                                  | pass                               |
| 5   | `synchronize`, 10:46           | `ab8b42e90` | Group jobs ran again and `db-benchmarking` failed. The label is still on the pull request, applied at 10:44:46, but the new head's first check suite was created at 10:46:06, so the override is stale. | fail, `BLOCKED`                    |
| 6   | `unlabeled`, 10:47             | `ab8b42e90` | No label.                                                                                                                                                                                               | fail                               |
| 7   | `labeled`, 10:49               | `ab8b42e90` | Applied at 10:49:13, after the first check suite at 10:46:06. Role `admin`.                                                                                                                             | pass                               |

After run 7 the head had three `CI gate` check runs (failure at 10:46, failure at 10:47, success at
10:49), `db-benchmarking` was still a failing check run, and the merge push at 10:50:47 was accepted
(merge commit `a4b78b909`), so the later success satisfied the required check as in V9.

The check runs behind those observations, listed on 2026-10-22 from the real API for the head of runs 5 to 7
(`gh api --paginate "repos/josecelano/torrust-tracker/commits/ab8b42e90ea02164e1da7df6c83e072022a2205c/check-runs?per_page=100"`,
reduced to the columns id, name, status, conclusion, start, and app):

```text
119179518396  matrix.group                      completed  skipped  2026-10-21T10:49:16Z  github-actions
119179516571  CI gate                           completed  success  2026-10-21T10:49:18Z  github-actions
119178973907  matrix.group                      completed  skipped  2026-10-21T10:47:55Z  github-actions
119178972434  CI gate                           completed  failure  2026-10-21T10:47:57Z  github-actions
119178335798  CI gate                           completed  failure  2026-10-21T10:46:21Z  github-actions
119178273455  docs-lint                         completed  success  2026-10-21T10:46:13Z  github-actions
119178273388  db-benchmarking                   completed  failure  2026-10-21T10:46:12Z  github-actions
119178273128  Persistence Benchmark MySQL       completed  success  2026-10-21T10:46:12Z  github-actions
119178273001  Docs Lint                         completed  failure  2026-10-21T10:46:12Z  github-actions
119178272825  Persistence Benchmark PostgreSQL  completed  success  2026-10-21T10:46:12Z  github-actions
119178272612  Persistence Benchmark SQLite3     completed  success  2026-10-21T10:46:12Z  github-actions
119178250074  Persistence Benchmark PostgreSQL  completed  success  2026-10-21T10:46:09Z  github-actions
119178250061  Docs Lint                         completed  failure  2026-10-21T10:46:10Z  github-actions
119178249956  Persistence Benchmark SQLite3     completed  success  2026-10-21T10:46:09Z  github-actions
119178249586  Persistence Benchmark MySQL       completed  success  2026-10-21T10:46:09Z  github-actions
```

It lists 15 runs, with three `CI gate` runs and two skipped `matrix.group` runs. The workflow's own request sets
no `filter`, as this one, so it used GitHub's default. The same request with `filter=all` returned the same 15
runs, so for this head the default hid nothing. A rerun of a job inside one check suite was not tested. The
listings in the logs of the runs are in [`evidence/t5-gate-logs.txt`](evidence/t5-gate-logs.txt), restored in
full on 2026-10-22, because an earlier copy had kept only some of their lines.

What it shows:

- **Reading the results from the checks API works with a read-only token, in every event.** A label run,
  whose own group jobs are skipped, found the earlier failure of `db-benchmarking` in the check runs of the
  head commit. A skipped group job leaves a check run with the conclusion `skipped`, and a skipped matrix job
  is named literally `matrix.group`, so the gate must ignore skipped runs and names that are not groups, or
  a label run would hide the earlier result. A group job that has not completed counts as missing.
- **Who applied the label comes from the issue events API.** The latest `labeled` event for the label gives
  the actor and the time. In label runs it equals the event's sender, but a `synchronize` run has no such
  field, so the events API is the one source that works for every event.
- **The collaborator permission API works with the read-only token.** It returned `admin` for the
  maintainer, and `read` for two users without write access (`da2ce7` on the fork, and `octocat`), so the
  Maintain or Admin check can tell them apart. Not tested: a label applied by a real user without that role,
  because the sandbox had only one account.
  **Correction (review of the T6a pull request, finding F13).** Only `admin` and `read` were observed, so this did
  not show that Maintain can be told apart: the endpoint's `permission` field is the legacy base role and reports
  Maintain as `write`, and the Triage role as `read`. The role is in `role_name`. The real responses of
  `gh api repos/torrust/torrust-tracker/collaborators/<login>/permission` on 2026-10-21 were `admin` and `admin`
  for two administrators, and `read` and `triage` for a Triage collaborator (`permission`, then `role_name`). No
  collaborator of the repository has the Maintain role, so its mapping to `write` is GitHub's documented
  behavior and was not observed.
- **An override does not survive a push, and this works without write access.** Comparing the label's time
  with the creation time of the head commit's first check suite made the override stale after the new push
  although the label stayed on the pull request, and a fresh label after the push counted. The first check
  suite appeared 4 seconds after the push, so a label applied in that window would count as stale, which
  fails safe.
- **A workflow change on the base branch does not reach the label runs of an open pull request until its
  head changes.** After the base branch got a fix at 10:42, runs 3 and 4, at 10:43 and 10:44, still used the
  first revision of the workflow, and only run 5, after the push that changed the head, used the fixed one: a pull
  request keeps the merge ref, and the workflow file in it, that it was last given. The logs show it, because
  only the fixed revision prints `probe:` lines. This matters when T6 changes the
  orchestrator while pull requests are open.
- The gate built the tool from a checkout of the base branch, so the pull request's own changes do not
  alter the tool that decides.

Limits: the sandbox pull request is in the same repository as its base, not a fork, so it cannot show how
the token of a pull request from a fork behaves. By the maintainer's decision this is verified on real
pull requests when T6 lands. The jobs in the sandbox are stubs, and the jobs of the real workflows were
not exercised. Not tested: reading the results from artifacts, or rerunning the jobs.
The saved workflow reads the check suites of the head commit with one request and does not paginate it, so
with more than 100 check suites on a head commit its first-seen time, and with it the stale-override
decision, could be wrong. It is saved as it ran and was not changed. The adapter of T6a, which replaces it,
reads every page of every response through `gh api --paginate`, and a test puts the earliest check suite on
the second page (pull request #2610).

Two revisions of the sandbox workflow ran, and both are saved in `evidence/t5-sandbox/`: the first revision
(`exp-2525-orchestrator-first-revision.yaml`, which ran in runs 1 to 4) and the fixed one
(`exp-2525-orchestrator.yaml`, runs 5 to 7). They differ in the probe step and in the test of the override actor
file, which the first revision made with `-s`, true for a file that holds only a newline, so it passed an empty
`--override-actor` when there was no label; the header of
[`evidence/t5-gate-logs.txt`](evidence/t5-gate-logs.txt) explains how to tell the revisions apart in the logs.
In both files only the comment on the first line was reworded, as for the files of V8.

Setup note: the fix of the sandbox workflow, the second revision, was pushed to the base branch at about 10:42, after setting the
sandbox ruleset to `disabled` for the push and back to `active` right after, because the ruleset (like the
real one) refuses a direct push.

Cleanup, 2026-10-21 at 10:52 UTC: the sandbox ruleset, both `exp-2525-t5-*` branches, and the `ci-override`
label were deleted from the fork, and verified: only the two tag rulesets remain, no `exp-` branch, no open
pull request, and none of those labels. Fork pull request 22 remains, merged, as history.

## Failures and Follow-up

None.
