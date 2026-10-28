---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md
last-updated-utc: "2026-10-28 07:07"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify the scenarios of the [spec](ISSUE.md) and the test-design reviews of
its test-producing tasks. Every output below is the real output of the command shown; paths under the
home directory are replaced by `~`.

## Environment and Prerequisites

- Host: Linux, 32 CPUs, 61 GiB of memory, Docker 28.3.3 (BuildKit 0.23.2, `docker` driver).
- Baseline tree: `develop` at the merge commit of PR #2671 (`docs(issues): [#2670] specify building shipped
  artifacts without test features`), in a throwaway worktree, before any change of this subissue.
- A `/tmp` cleanup removed the first baseline attempt's worktree and logs while it was building; the
  attempt exited with code 1 and its numbers are not used. The runs below were redone with the worktree and
  logs under `~/.cache/r9-2670/`.

## V0 - Audit of the commands that build shipped artifacts (task T1)

Re-verified on `develop` at the merge commit of PR #2671. `git diff --stat` of `Containerfile`,
`.dockerignore`, `Cargo.toml`, `docs/containers.md` and `contrib/dev-tools/containers` between the commit the
spec was audited on and this baseline shows no change; the workflow diffs between the two are action version
bumps only. The audit table of the spec therefore holds. Command:

```bash
grep -rnE "all-features|all-targets|--tests|--examples|--benches" Containerfile \
  .github/workflows/{container,deployment,deployment-packages,testing,os-compatibility,security-scan}.yaml \
  contrib/dev-tools/containers/*.sh
```

Output, reduced to file, line and the flags (the `Containerfile` lines are the cook and archive commands of the
spec's table):

```text
Containerfile:85,96,108,116   cargo chef cook --tests --workspace --all-features (third-party and full recipe, debug and release)
Containerfile:100,120         cargo nextest archive --tests --all-features (discarded warm-ups, debug and release)
Containerfile:128             cargo nextest archive --tests --all-features (Build Archive (debug)) - ships torrust-tracker
Containerfile:135             cargo nextest archive --tests --all-features --release (Build Archive (release)) - ships both binaries
.github/workflows/deployment.yaml:40            cargo test --tests --benches --examples --workspace --all-targets --all-features
.github/workflows/deployment-packages.yaml:122  cargo test -p <crate> --all-targets --all-features
.github/workflows/testing.yaml:110              cargo doc --no-deps --lib --workspace --all-features
.github/workflows/testing.yaml:114              cargo test --tests --benches --examples --workspace --all-targets --all-features
```

The two additions to the spec's table are both `testing.yaml` jobs that build documentation or run tests and
publish nothing, so they are outside the shipped-artifact scope. `container.yaml`, `security-scan.yaml` and
`contrib/dev-tools/containers/docker-build.sh` pass no feature flag of their own: they run `docker build`, so
they inherit the `Containerfile`. Only `Containerfile:128` and `:135` produce a shipped binary, as the spec
says.

## V1 - Baseline before the change (scenario M1, task T1)

Commands (run from the baseline worktree):

```bash
docker build --no-cache --progress=plain --target release --tag r9-baseline:release --file Containerfile .
docker build --no-cache --progress=plain --target debug --tag r9-baseline:debug --file Containerfile .
# one line appended to src/lib.rs, then a rebuild with the layer cache kept
docker build --progress=plain --target release --tag r9-baseline:release --file Containerfile .
```

| Build | Started (UTC, 2026-10-27) | Finished (UTC) | Wall time | Tests run by the `test` stage |
| --- | --- | --- | --- | --- |
| `release`, cold (`--no-cache`) | 12:01:49 | 12:06:48 | 4 min 59 s | 1531 passed, 0 skipped, 40 binaries |
| `debug`, cold (`--no-cache`) | 12:07:01 | 12:08:56 | 1 min 55 s | 1531 passed, 0 skipped, 40 binaries |
| `release`, warm (one source line changed) | 12:09:02 | 12:11:56 | 2 min 53 s | 1531 passed, 0 skipped, 40 binaries |

The `release` test list, sorted, is in [`baseline-test-list.txt`](baseline-test-list.txt) (1531 lines, one
`<binary> <test>` per line). The later comparison checks that every line is still present.

## T2 - Test-design review of the `TORRUST_TRACKER_BINARY` tests

Change: `tracker_binary()` in `tests/common/native_tracker/command.rs` delegates to
`resolve_tracker_binary(variable, compiled)`, which reads `TORRUST_TRACKER_BINARY`, then
`NEXTEST_BIN_EXE_torrust-tracker`, then `CARGO_BIN_EXE_torrust-tracker`, then the path compiled into the test.
The three unit tests pass the variables as a slice of pairs, so no test touches the process environment.

Prose-first Arrange-Act-Assert comparison, written before the code was refactored to express it:

| Test | Arrange (the one causal initial-state difference) | Act | Assert (independently specified result) |
| --- | --- | --- | --- |
| `it_should_run_the_binary_named_by_the_shipped_binary_override_before_the_one_nextest_or_cargo_built` | The override variable is set alongside the nextest and Cargo variables | Resolve the binary | The override's path wins |
| `it_should_run_the_binary_nextest_built_when_no_override_is_set` | Only the nextest and Cargo variables are set | Resolve the binary | The nextest path wins (today's behaviour is unchanged) |
| `it_should_run_the_binary_compiled_into_the_test_when_no_variable_is_set` | No variable is set | Resolve the binary | The compiled-in path is returned |

Review result: each test differs from the previous one by exactly one fact (the override present, then
absent with the nextest variable present, then every variable absent). The fixture is the `lookup_in` helper,
which only turns a slice of pairs into a lookup function (incidental mechanics). The production Act
(`resolve_tracker_binary`) and the expected path stay visible in each test. The helper is used by all three
tests and hides no decision. No redundant prose was left in the code.

Red and green runs (Cargo 1.99.1, `cargo test -p torrust-tracker --test lifecycle-signals it_should_run_the_binary`):

- Red: with the override read but its value discarded (`variable(SHIPPED_BINARY_VARIABLE).filter(|_| false)`,
  a scratch change that was reverted), the override test fails with `left: "/nextest/torrust-tracker"`,
  `right: "/shipped/torrust-tracker"` and the other two pass: `2 passed; 1 failed`.
- Green: with the change, `lifecycle-signals` reports `16 passed` and `cli-configuration` reports
  `21 passed`. The three new tests compile into both targets, so the image's test count rises by 6.

## T3 - Test-design review of the `shipped-features-check` tests

Change: the new crate `contrib/dev-tools/checks/shipped-features-check`. `workspace_packages_with_feature(tree, feature)`
parses the output of `cargo tree -p <shipped package> -e normal,build --prefix none -f "{p}|{f}"` and returns
each workspace package (a line with a local path) that has the feature enabled. The binary runs that command per
`--package`, with `--features` and `--forbidden-feature` options, and reports through the global CLI output contract
(`no-stdout-result`: nothing on stdout, NDJSON on stderr, exit 0, 1 or 2).

Two increments, each reviewed after it passed and before the next one started.

### Increment 1 - the `cargo tree` parser (9 unit tests in `src/lib.rs`)

The fixtures are real output: `TRANSITIVE_TREE` is the output on a scratch workspace and `REPOSITORY_TREE` is the
first lines of the same command on this repository (path replaced by `/repo`). The registry, Git, path-with-spaces,
procedural-macro and exact-match cases are lines in the real format with the one varying fact written by hand, which
the test names say.

| Test | Arrange (the one causal initial-state difference) | Act | Assert |
| --- | --- | --- | --- |
| `..._a_dependency_enables_once` | `a` is in the tree twice with `testing` on (a dependency `c` enables it) | Parse for `testing` | One violation for `a` |
| `..._report_nothing_when_no_package_has_the_feature_enabled` | A real repository tree with `(proc-macro)` and `(*)` markers and no `testing` | Parse | No violation |
| `..._shipped_package_itself_...` | The root line of the tree has `testing` | Parse | One violation for the shipped package |
| `..._ignore_a_registry_package_...` | A line with no path and a `testing` feature | Parse | No violation |
| `..._ignore_a_git_package_...` | A line whose location is a URL | Parse | No violation |
| `..._match_the_feature_name_exactly` | Features `testing-utils` and `db-testing` | Parse | No violation |
| `..._path_that_contains_spaces` | The path has spaces | Parse | One violation |
| `..._workspace_procedural_macro_crate_...` | A path plus the `(proc-macro)` marker | Parse | One violation |
| `..._skip_lines_that_are_not_package_lines` | Blank lines and a line without `\|` | Parse | No violation |

Review result: each test changes one fact of the line (location kind, feature name, marker, path), and the Act is the
same public call in all of them. There is no fixture helper beyond `violation()`, which builds the expected value.
The negative tests passed against the first stub, so they were checked by two scratch mutations that were reverted.
Treating every package as local failed the registry and Git tests (`7 passed; 2 failed`); matching the feature by
substring failed the exact-match test (`8 passed; 1 failed`).

Red run before the implementation (stub returning no violation): `5 passed; 4 failed`, the four positive tests.
Green run after it: `9 passed; 0 failed`.

### Increment 2 - the binary against scratch workspaces (7 tests in `tests/cli.rs`)

Real crates cannot get a `testing` feature in this subissue, so every test builds a scratch workspace of path-only
crates in a temporary directory (`a` has the features `testing` and `internal`, `c` depends on `a`, `b` is the shipped
package) and runs the real binary, which runs the real `cargo tree`.

| Test | Arrange | Act | Assert |
| --- | --- | --- | --- |
| `it_should_fail_when_a_normal_dependency_...` | `b` depends on `a` plainly and on `c`, which enables `a`'s `testing` (transitive) | `--package b` | Exit 1, empty stdout, one `validation_error` naming `a`, `b` and `testing` |
| `..._in_a_build_dependency` | `testing` reaches `b` only through a build dependency | `--package b` | Exit 1, one violation |
| `..._pass_when_only_a_dev_dependency_...` | `b` enables `testing` only in `[dev-dependencies]` | `--package b` | Exit 0, empty stdout, no violation, one `checked` record |
| `..._features_that_the_shipped_build_names_explicitly` | `b`'s feature `with-testing` enables `a/testing` | Without and with `--features with-testing` | Exit 0, then exit 1 |
| `..._forbidden_feature_option` | `b` enables `a`'s `internal` feature | Default, then `--forbidden-feature internal` | Exit 0, then exit 1 naming `internal` |
| `..._usage_error_when_no_package_is_named` | An empty argument list | Run | Exit 2, empty stdout, `usage_error` |
| `..._runtime_error_when_cargo_cannot_resolve_the_package` | No package `missing` | `--package missing` | Exit 1, empty stdout, `runtime_error` |

Review result: the `Scratch` helper writes the manifests (incidental mechanics) and takes the two manifest sections
that carry the causal difference of each test, so the section that decides the result stays in the test. The Act is
one command line and the Assert states the exit code, the stdout and the diagnostic the contract requires.

Red run before the CLI existed (an empty `main`): `0 passed; 7 failed`. Green run after it: `7 passed; 0 failed`.
`cargo clippy -p shipped-features-check --all-targets -- -D warnings` reports nothing.

## V4 - The check on the real workspace (scenario M4)

```bash
cargo run --quiet -p shipped-features-check -- --package torrust-tracker > out.txt 2> err.txt; echo $?
```

```text
exit 0
stdout bytes: 0
stderr: {"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `torrust-tracker`","shipped_package":"torrust-tracker","feature":"testing","exit_code":0}
```

The check passes vacuously today: no workspace package has a `testing` feature yet. Its ability to fail is V5.

The plain `cargo tree` view of the same build, counting enabled features named exactly `testing`:

```bash
cargo tree -p torrust-tracker -e normal,build --prefix none -f '{p}|{f}' | awk -F'|' '{n=split($2,f,","); for(i=1;i<=n;i++){g=f[i]; sub(/ \(\*\)$/,"",g); if(g=="testing") c++}} END{print "lines:", NR, "packages with a feature named testing:", c+0}'
```

```text
lines: 1353 packages with a feature named testing: 0
```

## V5 - The check fails when it should (scenario M5)

The scratch workspaces `fx` (members `a`, `b`, `c`) and `fx-dev` (members `a`, `b`) were created under
`~/.cache/r9-2670/` and are not part of the repository. `a` has `[features] testing = []`.

Red, `fx`: `b` depends on `a` and on `c`, and `c` depends on `a` with `features = ["testing"]`.

```text
$ shipped-features-check --package b      (exit 1, stdout empty)
{"kind":"validation_error","message":"workspace package `a` v0.1.0 has the `testing` feature enabled in the dependency tree of shipped package `b`","shipped_package":"b","package":"a","feature":"testing","exit_code":1}
```

Green, `fx-dev`: `b` depends on `a` plainly and enables `testing` only in `[dev-dependencies]`.

```text
$ shipped-features-check --package b      (exit 0, stdout empty)
{"kind":"checked","message":"no workspace package has the `testing` feature enabled in the dependency tree of shipped package `b`","shipped_package":"b","feature":"testing","exit_code":0}

$ cargo tree -p b --prefix none -f '{p}|{f}'                       (dev-dependencies included: what --tests builds)
b v0.1.0 (~/.cache/r9-2670/fx-dev/b)|
a v0.1.0 (~/.cache/r9-2670/fx-dev/a)|testing
a v0.1.0 (~/.cache/r9-2670/fx-dev/a)|testing

$ cargo tree -p b -e normal,build --prefix none -f '{p}|{f}'       (what the check reads: what cargo build -p b builds)
b v0.1.0 (~/.cache/r9-2670/fx-dev/b)|
a v0.1.0 (~/.cache/r9-2670/fx-dev/a)|
```

The same scratch cases run automatically in `tests/cli.rs` on every CI run.

## V2 - The image builds and its tests run after the change (scenario M2, task T5)

Tree: the working tree after the `Containerfile` change of task T5 (the stages "Cook Third-party Shipped",
"Cook Shipped" and "Build Shipped", for the release and debug images), before its commit. Commands, as in V1:

```bash
docker build --no-cache --progress=plain --target release --tag r9-after:release --file Containerfile .
docker build --no-cache --progress=plain --target debug --tag r9-after:debug --file Containerfile .
# one line appended to src/lib.rs (then reverted), a rebuild with the layer cache kept
docker build --progress=plain --target release --tag r9-after:release --file Containerfile .
```

| Build | Started (UTC) | Wall time | Baseline (V1) | Difference | Tests run by the `test` stage |
| --- | --- | --- | --- | --- | --- |
| `release`, cold (`--no-cache`) | 2026-10-27 14:59:31 | 5 min 16 s | 4 min 59 s | +17 s | 1537 passed, 0 skipped, 40 binaries |
| `debug`, cold (`--no-cache`) | 2026-10-27 15:06:54 | 2 min 13 s | 1 min 55 s | +18 s | 1537 passed, 0 skipped, 40 binaries |
| `release`, warm (one source line changed) | 2026-10-27 19:50:47 | 3 min 01 s | 2 min 53 s | +8 s | 1537 passed, 0 skipped, 40 binaries |

The first warm run was interrupted and a rerun with the same change took 2 s because the layers were cached, so
neither is used; the row above is a run with a line that differs from both.

Test comparison with the baseline (the sorted `release` lists, `LC_ALL=C comm`): 1531 baseline tests, 1537 after
the change, **0 baseline tests missing, 6 added**. The six are the three tests of T2 compiled into each of the
`cli-configuration` and `lifecycle-signals` binaries, as the spec predicted:

```text
torrust-tracker::cli-configuration native_tracker::command::tests::it_should_run_the_binary_compiled_into_the_test_when_no_variable_is_set
torrust-tracker::cli-configuration native_tracker::command::tests::it_should_run_the_binary_named_by_the_shipped_binary_override_before_the_one_nextest_or_cargo_built
torrust-tracker::cli-configuration native_tracker::command::tests::it_should_run_the_binary_nextest_built_when_no_override_is_set
torrust-tracker::lifecycle-signals native_tracker::command::tests::it_should_run_the_binary_compiled_into_the_test_when_no_variable_is_set
torrust-tracker::lifecycle-signals native_tracker::command::tests::it_should_run_the_binary_named_by_the_shipped_binary_override_before_the_one_nextest_or_cargo_built
torrust-tracker::lifecycle-signals native_tracker::command::tests::it_should_run_the_binary_nextest_built_when_no_override_is_set
```

The shipped binaries come from their own invocations. The build log of the `release` image shows the stages
`dependencies_shipped_thirdparty` and `dependencies_shipped` running
`cargo chef cook --release --package torrust-tracker --bins`, and `build_shipped` running
`cargo build --release --package torrust-tracker --bins`, none with `--tests`, `--all-features` or a feature
list. The cook layers stack on the third-party test cooks, and the cold-build cost of the extra layers is the
17 s and 18 s above.

## V3 - The override is honoured (scenario M3)

Red: a copy of the `Containerfile` in which the `test` stage sets `TORRUST_TRACKER_BINARY=/app/bin/does-not-exist`
(a scratch change, not committed), built with `docker build --progress=plain --target test --file <copy> .`:

```text
exit 1
#43 0.287     spawn Cargo-built tracker executable: Os { code: 2, kind: NotFound, message: "No such file or directory" }
#43 2.367      Summary [   2.170s] 162/1537 tests run: 155 passed, 7 failed, 0 skipped
```

The native-tracker tests fail because the variable points at a missing file, which shows that they now run the
binary the variable names and not the one Cargo built. The message said "Cargo-built" because the spawn message
predates the override; the commit `test(tracker): [#2670] stop calling the spawned tracker executable Cargo-built`
corrects it.

Green: with the variable set to the shipped binary (the committed `Containerfile`), V2's `test` stage runs
1537 tests and all pass.

## V2b - The final image on the final tree (task T5, review finding 7)

Tree: the branch after the rebase onto `develop` and after the fixes of the pre-PR review below. Commands:

```bash
docker build --progress=plain --target release --tag r9-final:release --file Containerfile .
docker build --target build_shipped --tag r9-final:build-shipped --file Containerfile .
# copy the binaries out of the final image and out of the build_shipped stage with `docker create` and `docker cp`
sha256sum image-* stage-*
docker run -d --name r9-health -e USER_ID=$(id -u) -v <lib>:/var/lib/torrust/tracker \
  -v <log>:/var/log/torrust/tracker -v <etc>:/etc/torrust/tracker r9-final:release
docker inspect --format '{{.State.Health.Status}}' r9-health
```

```text
#48 0.203     Starting 1539 tests across 40 binaries
#48 10.99      Summary [  10.792s] 1539 tests run: 1539 passed, 0 skipped

12a7bb377d719976d141521060f4499fa0940ac9ad2eff352759913cc8218c17  image-http_health_check
68396f9f62c08e59a5e5878fcd04c4d4d301e976199816f393624abda4ce0a11  image-torrust-tracker
12a7bb377d719976d141521060f4499fa0940ac9ad2eff352759913cc8218c17  stage-http_health_check
68396f9f62c08e59a5e5878fcd04c4d4d301e976199816f393624abda4ce0a11  stage-torrust-tracker

t+5s health=starting
t+10s health=healthy
running=true exit=0
```

The test count is the 1531 of V1 plus the 8 tests this subissue adds (the 3 tests of T2 and the empty-override test, each
compiled into two test binaries). The binaries in the final image are byte-identical to the ones the `build_shipped`
stage built, so the image ships the build without test features, and the image's HEALTHCHECK passes with the shipped
`http_health_check`.

## Pre-PR review (independent Task Reviewer)

Verdict: PASS WITH FINDINGS, no blocking defect. The reviewer ran the check crate's tests, clippy, `linter all`, and a
full `docker build --target release` (1537 tests passed, the container healthy). Findings and what was done:

| # | Finding | Severity | Disposition |
| --- | --- | --- | --- |
| 1 | AC3, T4 and M6 need the hosted run; completion review and retrospective missing | Medium (process) | The spec already left them open; the hosted run is recorded after the pull request's first run, and the completion review and retrospective are written in this subissue |
| 2 | The check passed silently when `cargo tree` output stopped parsing as workspace packages | Low | Fixed: `has_workspace_package` and a runtime error unless the shipped package is listed as a workspace package; test red (exit 0) then green (exit 1) |
| 3 | `split_once('\|')` mis-read a checkout path that contains `\|` | Low | Fixed: `rsplit_once`; unit test red then green |
| 4 | No scratch case for a renamed dependency or a feature that implies `testing` | Low | Two CLI tests added; both passed at once, so they pin existing behaviour. A `default = ["testing"]` case was not added: it would put the feature in every scratch test, and the chained feature exercises the same Cargo resolution |
| 5 | An empty `TORRUST_TRACKER_BINARY` spawned an empty path | Low | Fixed: the empty value is ignored; unit test red (`left: ""`) then green |
| 6 | The workflow and the `Containerfile` can drift (D4) | Low | Accepted as decided in D4: comments in both files name each other. A lint comparing them is possible follow-up work |
| 7 | No evidence on the final image | Low | V2b above |
| 8 | Branch behind `develop` | Low | Rebased; none of the changed files had changed on `develop` |

The maintainer (josecelano) reviewed the test increments of findings 2 to 5 and approved them through `ask_user` before
their commits. Results after the fixes: `cargo test -p shipped-features-check` reports 13 unit and 10 CLI tests passing,
`cargo clippy -p shipped-features-check --all-targets -- -D warnings` reports nothing, and the check still passes on the
real workspace.

## V6 - The workflow job runs on a hosted runner (scenario M6)

The `Shipped Features` job of `testing.yaml` ran on the pull request, on `ubuntu-latest` with the stable toolchain,
on two heads of the branch:

| Head (commit subject) | Run | Job result |
| --- | --- | --- |
| `docs(issues): [#2670] record the pre-PR review, the final image and the completion review` | [run 40583603727](https://github.com/torrust/torrust-tracker/actions/runs/40583603727/job/122469179536) | pass, 22 s |
| `fix(tracker): [#2670] anchor a relative TORRUST_TRACKER_BINARY at the test working directory` | [run 40647792189](https://github.com/torrust/torrust-tracker/actions/runs/40647792189/job/122680559895) | pass, 19 s |

The job runs `cargo run --quiet --package shipped-features-check -- --package torrust-tracker`, the command of V4, so
it is green on the real workspace. As in V4 it passes because no `testing` feature exists yet; the cases that make it
fail are the scratch-workspace tests of `tests/cli.rs` (V5), which the `Unit` jobs of the same workflow run.

## V7 - A relative `TORRUST_TRACKER_BINARY` (PR review finding F1)

Copilot's review of the pull request reported that a relative override breaks the configuration test that changes the
child's working directory. Reproduced before the fix, on a release binary built with
`cargo build --release --package torrust-tracker --bin torrust-tracker`:

```bash
TORRUST_TRACKER_BINARY=./target/release/torrust-tracker cargo test -p torrust-tracker --test cli-configuration
TORRUST_TRACKER_BINARY=$PWD/target/release/torrust-tracker cargo test -p torrust-tracker --test cli-configuration
```

```text
relative: test invalid_sources::it_should_not_search_parent_directories_for_a_relative_cli_configuration_file ... FAILED
          spawn tracker executable: Os { code: 2, kind: NotFound, message: "No such file or directory" }
          test result: FAILED. 21 passed; 1 failed
absolute: test result: ok. 22 passed; 0 failed
```

After the fix, `tracker_binary()` anchors a relative path that has a directory part at the test's working directory (the
unit test of that shape was red against a stub returning the path unchanged, `left: "./target/release/torrust-tracker"`,
then green). The same relative invocation now passes: `test result: ok. 25 passed; 0 failed` for `cli-configuration`,
and `lifecycle-signals` reports `20 passed`. The three new tests add 6 to the image's test count.
