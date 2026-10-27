---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md
last-updated-utc: "2026-10-27 12:43"
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
