---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2298-rust-dev-tool-container-integration/ISSUE.md
last-updated-utc: 2026-09-30 12:26
---

<!-- markdownlint-disable MD003 -->

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-29 16:15 to 2026-09-30 11:18
- Artifact under test: branch `2298-rust-dev-tool-container-integration`, patches
  `docs(issues): [#2298] record root causes and selected container design` plus the working-tree
  changes to `Containerfile`, `.dockerignore`, and `Cargo.toml` (D1, D3, D4). Baseline runs used
  the `develop` `Containerfile` at merge commit `bc90cde1b`.
- Operating system / environment: Linux workstation, Docker 28.3.3 (BuildKit, default `docker`
  driver, local layer cache). Not the self-hosted runner.
- Prerequisites and setup performed: each scenario started from a warm cache produced by a
  preceding full `docker build --target test_debug --file Containerfile .` on the same
  `Containerfile`. Raw BuildKit logs are kept locally in `.tmp/2298/` (git-ignored) and are the
  source of every number below. Stage status is the BuildKit marker (`CACHED` or `DONE <s>`) on
  the `RUN`/`COPY` step of the named stage.

## Verification Processes

### V1 - M5/M6 cache measurement, baseline `Containerfile` (manifest-only recipe stage, #1852)

- Goal: establish the cache behaviour the new design must match (spec scenarios M5, M6 on the
  baseline).
- Initial state: `develop` `Containerfile`; warm cache from `baseline-run0`
  (`dependencies_thirdparty_debug` cook 45.7 s, `dependencies_debug` cook 23.6 s, `build_debug`
  archive 34.6 s).
- Status: `DONE`

#### Steps Performed

1. M5: appended a comment line to one tracker `.rs` file; ran
   `docker build --target test_debug --file Containerfile .` (`baseline-m5-source-change.log`).
2. Reverted step 1. M6a: appended a comment line to `packages/primitives/Cargo.toml`; rebuilt
   (`baseline-m6-manifest-change.log`).
3. Reverted step 2. M6b: toggled a feature in a `packages/*/Cargo.toml` (a change that alters
   external feature resolution); rebuilt (`baseline-m6b-feature-change.log`).

#### Observed Result

```text
Scenario | recipe stage                | thirdparty cook | full cook   | build_debug archive
M5       | CACHED (all COPY + prepare) | CACHED          | CACHED      | DONE 32.0s
M6a      | DONE (prepare 0.3s + 0.2s)  | CACHED          | CACHED      | DONE 30.1s
M6b      | DONE (prepare 0.3s + 0.4s)  | DONE 46.9s      | DONE 19.0s  | DONE 26.4s
```

#### Conclusion

Baseline recorded. The recipe layer is protected from source edits as #1852 intended; the
third-party cook survives a manifest comment change thanks to #1869's `--external-only` recipe;
a feature toggle rebuilds both cooks because it changes external feature resolution. M6b is the
expected invalidation, not a defect.

### V2 - M5/M6 cache measurement, D1 `Containerfile` (canonical `COPY . .` recipe stage)

- Goal: gate decision D1. The third-party cook must stay `CACHED` for source-only and
  manifest-only changes; otherwise fall back to D2.
- Initial state: D1 applied (manifest-copy and stub lists deleted, `COPY . /build/src` restored);
  warm cache from `d1-run0` (`thirdparty` cook 42.5 s, full cook 26.7 s, archive 30.5 s; the cooks
  rebuilt once because the recipe produced from real sources differs from the stub-based one).
- Status: `DONE`

#### Steps Performed

1. M5: same `.rs` comment edit as V1; rebuilt `test_debug` (`d1-m5-source-change.log`).
2. Reverted. M6a: same `packages/primitives/Cargo.toml` comment edit; rebuilt
   (`d1-m6-manifest-change.log`).
3. Reverted. M6b: same feature toggle; rebuilt (`d1-m6b-feature-change.log`).

#### Observed Result

```text
Scenario | recipe stage                       | thirdparty cook | full cook   | build_debug archive
M5       | DONE (COPY 0.1s, prepare 0.2+0.2s) | CACHED          | CACHED      | DONE 33.2s
M6a      | DONE (COPY 0.1s, prepare 0.3+0.3s) | CACHED          | CACHED      | DONE 36.8s
M6b      | DONE (COPY 0.1s, prepare 0.2+0.2s) | DONE 43.4s      | DONE 20.7s  | DONE 27.3s
```

The `COPY --from=recipe` steps in both cook stages reported `CACHED` in M5 and M6a even though
the producing `recipe` stage had re-run.

#### Conclusion

D1 passes the gate. Cache behaviour is identical to the baseline in all three scenarios: BuildKit
keys `COPY --from` on the content checksum of the copied recipe files, so a re-run recipe stage
that emits byte-identical recipes keeps the cook layers cached. The recipe stage re-run costs
about 0.5 s. D2 (generated stubs) is not needed and was not implemented.

### V3 - `test_debug` on D1 + D3 + D4

- Goal: the debug test archive builds and runs with the allow-list context and the
  `default-members` positive list; test scope is unchanged from the baseline.
- Initial state: D1, D3 (`.dockerignore` allow-list), D4 (`default-members`; `cargo nextest archive`
  without `--workspace`/`--exclude`) applied. First attempt failed in `dependencies_debug` because
  `default-members` named `packages/*` crates that were only auto-discovered through path
  dependencies; the `--external-only` skeleton strips those, so Cargo could not resolve them. Fixed
  by listing every in-repo crate explicitly in `[workspace].members`.
- Status: `DONE`

#### Steps Performed

1. `docker build --target test_debug --file Containerfile .` (`d4-test-debug.log`).
2. Compared the nextest summary line with `baseline-run0.log`.

#### Observed Result

```text
baseline: Starting 1121 tests across 38 binaries ... 1121 tests run: 1121 passed, 0 skipped
D1+D3+D4: Starting 1121 tests across 38 binaries ... 1121 tests run: 1121 passed, 0 skipped
```

Stage durations: `thirdparty` cook 46.5 s (rebuilt: `Cargo.toml` membership changed), full cook
9.4 s, archive 34.1 s. The warm-up archive in `dependencies_debug` and the final archive in
`build_debug` compiled no `contrib/dev-tools` crate and no verification tool.

#### Conclusion

Met. Identical test scope (1121 tests / 38 binaries) with no `--exclude` list; the positive list
selects exactly what the negative lists used to leave in.

### V4 - Release `test` target and final `release` image

- Goal: the release archive builds and runs; the final `release` image builds and contains only
  tracker binaries.
- Initial state: same as V3.
- Status: `DONE`

#### Steps Performed

1. `docker build --target test --file Containerfile .` — first attempt was interrupted by a host
  restart during the release compile. Reran outside the IDE on 2026-09-30; output is in
  `d4-test-release.log`.
2. `docker build --target runtime --tag torrust-tracker:2298-local --file Containerfile .` —
  completed on 2026-09-30; output is in `d4-runtime.log`.
3. `docker run --rm --entrypoint /bin/ls torrust-tracker:2298-local -la /app/bin` — failed:
  `/app/bin` does not exist in the `runtime` target.
4. `docker build --target release --tag torrust-tracker:2298-release-local --file Containerfile .`
  — completed on 2026-09-30; output is in `d4-release.log`. This is the final production image:
  `release` copies `/app/` from `test` to `/usr/`.
5. `docker run --rm --entrypoint /bin/ls torrust-tracker:2298-release-local -la /usr/bin`.

#### Observed Result

```text
#39 exporting to image
#39 writing image sha256:3be0a637e044d401c8bed97f04802552e21bcd54a65272c9535119402c24b06d done
#39 DONE 0.0s

runtime image configuration:
Entrypoint: ["/usr/local/bin/entry.sh"]
User: "0"

ls: /app/bin: No such file or directory

#50 writing image sha256:f2a579437bd6f6507eb8139a055211c96d479bf8f48662dce92fe44d9a5e0c4f done
#50 naming to docker.io/library/torrust-tracker:2298-release-local done

/usr/bin:
http_health_check (28463408 bytes)
torrust-tracker (138496632 bytes)

release image configuration:
Entrypoint: ["/usr/local/bin/entry.sh"]
Cmd: ["/usr/bin/torrust-tracker"]
```

#### Conclusion

The release `test` target completed successfully. On the resumed build, every stage was `CACHED`,
including the release `cargo nextest run` stage; this reused the successful release compilation
from the interrupted first attempt. The `runtime` target also builds successfully, but it is a
deliberate base stage and does not contain tracker binaries. The final-image assertion must target
`release`, not `runtime`. The final `release` image built successfully and contains exactly the
expected tracker binaries plus the intentionally included BusyBox links and `su-exec` helper.

### V5 - M1, M2, M3, M7 (disposable package and context probe)

- Goal: prove the old failure mode, prove a new package needs no container edit, prove the positive
  list controls archive scope, and prove the build context contains only allow-listed paths.
- Initial state: a disposable mixed-target crate `packages/container-build-probe` (no
  dependencies; a library, default binary, additional binary, example, and benchmark). The three
  testable targets each have one unit test. M1 ran in a temporary detached `develop` worktree
  (`bc90cde1b`); M2, M3, and M7 ran on this branch. The probe was removed after each run and left
  no tracked change.
- Status: `DONE` (M2 closed together with V6; see the conclusion below)

#### Steps Performed

1. M1 (baseline): in a detached `develop` worktree, added the probe to `[workspace].members`
   without touching `Containerfile` or `.dockerignore`;
   `docker build --target recipe --file Containerfile .` (`m1-baseline-missing-member.log`).
2. M2: on this branch, added the probe to `members` and `default-members` with no `Containerfile`
  or `.dockerignore` edit; `docker build --target test_debug --file Containerfile .`
  (`m2-default-member-probe.log`). The original run used a library-only probe. In the corrective
  rerun, the lib+bin probe above was used with the same command
  (`m2-mixed-target-probe.log`).
3. M3: removed the probe from `default-members` only (still a member); rebuilt `test_debug`
   (`m3-non-default-member-probe.log`).
4. Removed the probe; `git status` clean for `Cargo.toml` and `Cargo.lock`.
5. M7: built a throwaway BusyBox image that copies the context and lists it
   (`.tmp/2298/context-probe.Dockerfile`, `m7-context-probe.log`). Found nested `AGENTS.md` and
   package `docs/` trees in the context; replaced the three top-level `AGENTS.md` re-exclusions with
   `**/AGENTS.md` and `**/docs/`, reran the probe (`m7-context-probe-v2.log`), and rebuilt
   `--target recipe` (`m7-recipe-after-docs-exclusion.log`).

#### Observed Result

```text
M1: ERROR: process "/bin/sh -c cargo chef prepare --recipe-path /build/recipe.json" did not complete successfully
    `cargo metadata` exited with an error: error: failed to load manifest for workspace member
    `/build/src/packages/container-build-probe`
    failed to read `/build/src/packages/container-build-probe/Cargo.toml`
    No such file or directory (os error 2)

M2 original library-only probe: Starting 1122 tests across 39 binaries
    Summary [   6.691s] 1122 tests run: 1122 passed, 0 skipped

M2 corrected lib+bin probe: Starting 1141 tests across 41 binaries
    container-build-probe tests::probe_library_target_is_tested: PASS
    container-build-probe::bin/container-build-probe
      tests::probe_default_binary_target_is_tested: PASS
    container-build-probe::bin/probe_extra
      tests::probe_extra_binary_target_is_tested: PASS
    Summary [   3.309s] 211/1141 tests run: 210 passed, 1 failed, 0 skipped
    Unrelated failure: torrust-tracker-axum-http-server
      server::tests::it_should_release_the_listener_and_preserve_the_duplicate_binding_error_when_registration_fails

M3: Starting 1121 tests across 38 binaries
    Summary [   6.800s] 1121 tests run: 1121 passed, 0 skipped

M7 first probe, forbidden paths found: ./packages/configuration/AGENTS.md (depth 3; package docs/
   trees deeper)
M7 second probe, top level: .cargo Cargo.lock Cargo.toml console contrib packages share src tests
   forbidden paths (docs, .github, .tmp, storage, target, .git, AGENTS.md, **/docs): none
   recipe stage after the change: cargo chef prepare DONE 0.2s (both invocations)
```

#### Conclusion

M1, M3, and M7 met. M1 reproduces the original bug on the baseline with Cargo's own diagnostic.
The corrected M2 run demonstrates that the archive includes the disposable library and both binary
targets without a `Containerfile` or `.dockerignore` change. Its three probe tests passed before
an unrelated `axum-http-server` listener test failed. On 2026-09-30 the maintainer accepted M2 as
done without another local run: this run proves target inclusion, and V6 proves the full
container suite passes. M3 shows `default-members` alone controls archive scope. M7
shows the context is limited to the allow-list; the probe caught nested agent and documentation
files that the first allow-list version admitted, now re-excluded recursively. No Rust source
lives under any `docs/` directory, no manifest references one, and no `.rs` file embeds or reads a
`docs/` path or `AGENTS.md` in a default member, so the exclusion cannot break a build stage.

### V6 - M4 hosted Container workflow

- Goal: the hosted Container workflow builds and tests the image on the self-hosted runner.
- Initial state: PR #2385 head `3ae381d5d` (rebased on `develop`), then the merge commit
  `0b130ec0d` on `develop`.
- Status: `DONE`

#### Steps Performed

1. Pushed the branch to the fork PR; GitHub ran the `Container` workflow.
2. Merged PR #2385; GitHub ran the `Container` workflow on `develop`.
3. Read the job results with `gh run view <run-id> --json jobs` and the runner names from the
   Actions jobs API.

#### Observed Result

```text
run 36710654492 (pull_request, 3ae381d5d): success
  Test (Docker) (release): success 11:51:11Z -> 12:07:59Z  runner torrust-runner-01 (self-hosted)
  Context: success; Publish jobs skipped (PR event)
run 36714211896 (push, develop, 0b130ec0d):
  Test (Docker) (release): success 12:21:46Z -> 12:24:01Z
```

#### Conclusion

M4 met. The `release` target build runs the release `test` stage (archive, extract, and the full
`cargo nextest run`), and it passed on the self-hosted runner for the PR and again on `develop`.
This is also the clean full container test stage that the corrected M2 run did not reach.

## Failures and Follow-up

- V3 first attempt: `cargo chef cook` in `dependencies_debug` failed to resolve `default-members`
  entries for auto-discovered `packages/*` crates because the `--external-only` skeleton drops path
  dependencies. Remediation: list all in-repo crates explicitly in `[workspace].members` (recorded
  as ADR agreement 5). Rerun passed.
- V4: interrupted by a host restart (the Docker build saturated the workstation while the IDE was
  running). The release `test` target was rerun outside the IDE and passed.
- V4: the original runtime-image inspection expected `/app/bin`, but `runtime` is intentionally a
  base stage. The final `release` stage copies `/app/` from `test` to `/usr/`; verification is
  corrected to target `release`.
- V5 M7 first probe: the allow-list re-excluded only top-level `AGENTS.md` files, so
  `packages/configuration/AGENTS.md` and package `docs/` trees (ADRs, benchmark reports, licenses)
  reached the build context. Remediation: `**/AGENTS.md` and `**/docs/` re-exclusions. Rerun
  showed no forbidden paths and the `recipe` stage still passes.
- V5 corrected M2: `torrust-tracker-axum-http-server`
  `server::tests::it_should_release_the_listener_and_preserve_the_duplicate_binding_error_when_registration_fails`
  failed once in the local `test_debug` stage with `Address already in use (os error 98)`. It
  passed in the pre-push suite and in CI, so it looks like a flaky port-binding test unrelated to
  this issue. Left for separate triage.
