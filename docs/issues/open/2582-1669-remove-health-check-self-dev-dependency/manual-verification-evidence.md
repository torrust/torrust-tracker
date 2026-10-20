---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md
last-updated-utc: 2026-10-20 17:35
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

Relevant output is embedded verbatim below; absolute local paths are shortened to `<repo>`.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-20 17:34 to 17:35
- Artifact under test: `develop` at merge commit `5b54bb0f4` (Merge torrust/torrust-tracker#2577)
- Operating system / environment: Linux; stable Rust toolchain, `rustc 1.99.0 (b940084d7 2026-09-28)`,
  `cargo 1.99.0 (5f94df478 2026-08-27)`; crates.io index reachable.
- Prerequisites and setup performed: clean worktree (`git status --short` listed only the
  untracked draft folder of the publish tracking issue, outside every package).

## Verification Processes

### V1 - Reproduce: the workspace cannot be packaged

- Goal: observe the failure a maintainer meets when preparing the first publish.
- Initial state: unmodified tree at `5b54bb0f4`.
- Status: `DONE` (outcome: **Reproduced**)

#### Steps Performed

1. 17:34:24 `cargo package --workspace --no-verify` (stable Rust toolchain); ended 17:34:46.

#### Observed Result

The command exited `101` after packaging 25 members in dependency order (25 `Packaged` lines):

```text
   Packaging torrust-tracker-axum-rest-api-server v0.1.0 (<repo>/packages/axum-rest-api-server)
    Updating crates.io index
    Packaged 53 files, 339.1KiB (74.0KiB compressed)
   Packaging torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)
    Updating crates.io index
error: failed to prepare local package for uploading

Caused by:
  no matching package named `torrust-tracker-axum-health-check-api-server` found
  location searched: crates.io index
  required by package `torrust-tracker-axum-health-check-api-server v0.1.0 (<repo>/packages/axum-health-check-api-server)`
```

#### Conclusion

Reproduced. `packages/axum-health-check-api-server/Cargo.toml` lists the crate itself in
`[dev-dependencies]` with `version = "0.1.0"`. `cargo package` resolves versioned
dev-dependencies against crates.io plus the crates it has already packaged in this run, and a crate
is never packaged before itself, so the requirement cannot be met. `cargo publish` of the crate
would fail the same way.

### V2 - Feasibility probe: the crate without the self dev-dependency

- Goal: check, before planning the fix, that removing the line is enough and breaks no test.
- Initial state: same tree; the line is removed temporarily and restored after the probe.
- Status: `DONE`

#### Steps Performed

1. 17:34:46 `sed -i '/^torrust-tracker-axum-health-check-api-server = /d' packages/axum-health-check-api-server/Cargo.toml`
   (`git diff --stat`: 1 file changed, 1 deletion).
2. `cargo test -p torrust-tracker-axum-health-check-api-server` (stable Rust toolchain); exited `0`
   at 17:34:50.
3. `cargo package --workspace --no-verify` exited `101` at 17:35:03 only because the tree was
   dirty (`error: 1 files in the working directory contain changes that were not yet committed into git`);
   the edit was restored and the step was repeated as step 5.
4. 17:35:03 `git checkout -- packages/axum-health-check-api-server/Cargo.toml Cargo.lock`, then
   `git diff --quiet HEAD -- packages/axum-health-check-api-server/Cargo.toml Cargo.lock` exited `0`.
5. 17:35:11 the same `sed` edit, then `cargo package --workspace --no-verify --allow-dirty`
   exited `0` at 17:35:28 and packaged all 32 workspace members
   (32 `Packaging` lines).
6. 17:35:28 the same `git checkout`, then the same `git diff --quiet HEAD -- ...` exited `0`.

#### Observed Result

```text
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_axum_health_check_api_server-9fb5f44dbd97c5bb)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/integration.rs (target/debug/deps/integration-66a0ce008e79654d)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.04s
   Doc-tests torrust_tracker_axum_health_check_api_server
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Conclusion

Removing the line is sufficient: the crate's tests still pass (integration tests import the
package under test without declaring it) and the whole workspace packages. This probe is not the
final recheck; the implementation repeats V1 after the fix.

## Failures and Follow-up

V2 step 3 failed for an unrelated reason (dirty tree without `--allow-dirty`) and was rerun as
step 5.
