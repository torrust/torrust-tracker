---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2525-ci-gate-required-check/ISSUE.md
last-updated-utc: 2026-10-20 12:28
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
  files (11 paths).
- Status: `DONE`

```sh
{ git diff --name-only torrust/develop...HEAD; git ls-files --others --exclude-standard; git diff --name-only; } \
  | sort -u | target/debug/ci-gate classify --base-branch develop
```

Result (the JSON object, formatted): 11 changed paths. `docs-lint`, `testing`, `os-compatibility`,
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
- Initial state: the 60 most recently merged pull requests into `develop` (merged 2026-10-08 to
  2026-10-20). For each, the changed files from the GitHub API, and the workflow runs of event
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
printf '%s\n' <files> | target/debug/ci-gate classify --base-branch <base>
```

The workflow names `Docs Lint`, `Testing`, `OS Compatibility`, `Generate Coverage Reports (PR)`,
`Container`, `Database Compatibility`, `Database Benchmarking`, and `Security Scan` map to the
groups `docs-lint`, `testing`, `os-compatibility`, `coverage`, `container`, `db-compatibility`,
`db-benchmarking`, and `security-scan`. The per-pull-request results are in
[`evidence/ci-gate-differential-check.txt`](evidence/ci-gate-differential-check.txt).

Result: **60 of 60 pull requests match**, 0 differ, 0 skipped.

| Groups run                                                          | Pull requests |
| ------------------------------------------------------------------- | ------------: |
| `docs-lint` only                                                    |            32 |
| `docs-lint`, `testing`, `os-compatibility`, `coverage`, `container` |            23 |
| the five above plus `db-compatibility` and `db-benchmarking`        |             4 |
| the five above plus `security-scan`                                 |             1 |

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

## Failures and Follow-up

None.
