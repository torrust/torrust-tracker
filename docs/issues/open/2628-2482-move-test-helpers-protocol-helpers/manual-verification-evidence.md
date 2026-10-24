---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2628-2482-move-test-helpers-protocol-helpers/ISSUE.md
last-updated-utc: "2026-10-24 19:37"
---

# Manual Verification Evidence

## Purpose

Record the commands run against the real workspace to verify scenarios M1 and M2 of the
[spec](ISSUE.md), plus a check that the new `deny.toml` bans fire (the evidence for AC3). Every
output below is the real output of the command shown, with the repository path replaced by
`<repo>` and the `warning[duplicate]` lines of `cargo deny` (external crates, unrelated to this
issue) removed.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-24, 19:17 (baseline) to 19:36 (final runs); each step of V3 carries
  its own stamp.
- Artifact under test: the implementation branch of this issue, after the commits whose subjects
  are `refactor(test-helpers): [#2628] move the http helpers to the root tests`,
  `refactor(test-helpers): [#2628] move the udp helpers to the root tests` and
  `chore(deny): [#2628] ban the crates test-helpers must not depend on`; the baseline is the tip of
  `develop` after the merge of PR #2629 (merge commit `0599fbd05`).
- Operating system / environment: Linux developer workstation; `cargo 1.99.1 (5f94df478 2026-08-27)`,
  `rustc 1.99.1 (58cf8f9c3 2026-10-13)`.
- Prerequisites and setup performed: none beyond a checkout of the branch. No scratch workspace was
  needed; V3 edits one manifest in the working tree and restores it with `git checkout`, and the
  restore is checked in each step.
- Dependent steps ran one at a time, in one shell. The baseline `cargo tree` (V1) ran while the
  baseline test run (V2) was compiling; `cargo tree` only reads the manifests, so it does not
  depend on that run.

## Verification Processes

### V1 - `test-helpers` has no protocol dependency (M1)

- Goal: show that `torrust-tracker-test-helpers` no longer depends directly on `client-lib`,
  `http-protocol`, `udp-protocol`, `primitives`, `torrust-info-hash`, `torrust-peer-id` or `url`.
- Initial state: `develop` at the merge of PR #2629.
- Status: `DONE`

#### Steps Performed

1. Before the change: `cargo tree -p torrust-tracker-test-helpers -e normal --depth 1` (stamped
   2026-10-24 19:17 UTC, taken with the baseline test run).
2. After the change, at 19:34:42 UTC: the same command, then the same command with
   `-e normal,dev,build`, which has no further dependency.
3. At 19:34:47 UTC: `cargo machete --with-metadata`.

#### Observed Result

Before (the 11 direct dependencies):

```text
torrust-tracker-test-helpers v3.0.0 (<repo>/packages/test-helpers)
├── rand v0.10.3
├── torrust-info-hash v0.2.0
├── torrust-peer-id v0.1.0
├── torrust-tracker-client-lib v0.1.0 (<repo>/packages/tracker-client)
├── torrust-tracker-configuration v3.0.0 (<repo>/packages/configuration)
├── torrust-tracker-http-protocol v0.1.0 (<repo>/packages/http-protocol)
├── torrust-tracker-primitives v3.0.0 (<repo>/packages/primitives)
├── torrust-tracker-udp-protocol v0.1.0 (<repo>/packages/udp-protocol)
├── tracing v0.1.44
├── tracing-subscriber v0.3.23
└── url v2.5.8
```

After (exit 0; the `-e normal,dev,build` run prints the same four):

```text
torrust-tracker-test-helpers v3.0.0 (<repo>/packages/test-helpers)
├── rand v0.10.3
├── torrust-tracker-configuration v3.0.0 (<repo>/packages/configuration)
├── tracing v0.1.44
└── tracing-subscriber v0.3.23
```

`cargo machete --with-metadata` (exit 0):

```text
Analyzing dependencies of crates in this directory...
cargo-machete didn't find any unused dependencies in this directory. Good job!
Done!
```

Who depends on the protocol crates after the change, from `cargo metadata --format-version 1
--locked` (workspace crates only; `torrust-tracker` is the root crate):

```text
torrust-tracker-client-lib     <- axum-http-server:dev, client:normal, udp-server:normal, torrust-tracker:dev
torrust-tracker-http-protocol  <- axum-http-server, client-lib, client, http-core (normal), torrust-tracker:dev
torrust-tracker-udp-protocol   <- client-lib, client, udp-core, udp-server (normal), torrust-tracker:dev
```

`test-helpers` is in none of these lists.

#### Conclusion

The direct dependencies fell from 11 to the 4 the spec expects (`rand`, `configuration`,
`tracing`, `tracing-subscriber`), so M1 and AC1 hold for direct dependencies. `primitives` is
still reachable transitively through `configuration`, which depends on it; that edge was there
before and is allowed by ADR rule B.1 (`configuration` is part of the generic leaf), so it is not
a finding.

### V2 - The root tests keep running (M2)

- Goal: show that the root crate's tests give the same results before and after the move.
- Initial state: baseline on `develop` at the merge of PR #2629 (run started 19:17 UTC); final run
  on the branch after all four implementation commits (19:35 UTC).
- Status: `DONE`

#### Steps Performed

1. Baseline: `cargo test -p torrust-tracker --tests`, exit 0.
2. After the `http` move, after the `udp` move, and on the final branch (exit 0 each time):
   the same command. The per-binary result lines of the three runs are identical.
3. The lines below are the `Running` and `test result` lines of the baseline and of the final run
   (`diff` of the two extracts is empty), shortened by removing the `target/debug/deps/...` suffix
   and the `0 measured; 0 filtered out` tail.

#### Observed Result

Both runs (15 test binaries, 194 tests passed, 0 failed, 0 ignored):

```text
 Running unittests src/lib.rs                        ok. 146 passed
 Running unittests src/bin/http_health_check.rs      ok. 0 passed
 Running unittests src/main.rs                       ok. 7 passed
 Running tests/banning/udp_metrics_disabled_port_zero.rs                              ok. 1 passed
 Running tests/banning/udp_shared_connection_id_error_limit.rs                        ok. 1 passed
 Running tests/banning/udp_shared_connection_id_error_limit_reverse_order.rs          ok. 1 passed
 Running tests/configuration/cli_configuration.rs    ok. 18 passed
 Running tests/lifecycle/signals.rs                  ok. 13 passed
 Running tests/metrics/fixed_ports.rs                ok. 1 passed
 Running tests/metrics/port_zero.rs                  ok. 1 passed
 Running tests/metrics/udp_error_disabled_port_zero.rs    ok. 1 passed
 Running tests/metrics/udp_error_enabled_port_zero.rs     ok. 1 passed
 Running tests/persistence/completed_downloads_on_shutdown.rs    ok. 1 passed
 Running tests/persistence/scrape_after_restart.rs   ok. 1 passed
 Running tests/scaffold.rs                           ok. 1 passed
```

The pre-push checks (`./contrib/dev-tools/git/hooks/pre-push.sh`) also passed, including the full
stable workspace test suite (`PASS (1m 0s)`) and the packaging check, so the six other
dev-dependents of `test-helpers` still build and pass.

#### Conclusion

The same 15 binaries ran the same 194 tests with the same per-binary counts, so M2 and AC2 hold.

### V3 - The new `deny.toml` bans fire (AC3)

- Goal: show that `cargo deny check bans` rejects a dependency of `test-helpers` on each of the
  four newly banned crates, and passes on the real tree.
- Initial state: the branch with the `deny.toml` entries for `client-lib`, `tracker-core`,
  `swarm-coordination-registry` and `events`.
- Status: `DONE`

#### Steps Performed

For each crate in turn, one step at a time: append a `path` dependency on it to
`packages/test-helpers/Cargo.toml`, run `cargo deny check bans`, restore with
`git checkout -- packages/test-helpers/Cargo.toml Cargo.lock`, then check the restore with
`git diff --quiet HEAD -- packages/test-helpers/Cargo.toml Cargo.lock`. Finally run
`cargo deny check bans` on the restored tree.

#### Observed Result

```text
[19:35:23] append to packages/test-helpers/Cargo.toml: torrust-tracker-client-lib = { version = "0.1.0", path = "../tracker-client" }
[19:35:23] $ cargo deny check bans
error[banned]: crate 'torrust-tracker-client-lib = 0.1.0' is explicitly banned
bans FAILED
[19:35:24] $ git checkout -- packages/test-helpers/Cargo.toml Cargo.lock
[19:35:24] git diff --quiet HEAD -- packages/test-helpers/Cargo.toml Cargo.lock exited 0
[19:35:24] append to packages/test-helpers/Cargo.toml: torrust-tracker-core = { version = "0.1.0", path = "../tracker-core" }
[19:35:24] $ cargo deny check bans
error[banned]: crate 'torrust-tracker-core = 0.1.0' is explicitly banned
bans FAILED
[19:35:25] $ git checkout -- packages/test-helpers/Cargo.toml Cargo.lock
[19:35:25] git diff --quiet HEAD -- packages/test-helpers/Cargo.toml Cargo.lock exited 0
[19:35:25] append to packages/test-helpers/Cargo.toml: torrust-tracker-swarm-coordination-registry = { version = "0.1.0", path = "../swarm-coordination-registry" }
[19:35:25] $ cargo deny check bans
error[banned]: crate 'torrust-tracker-swarm-coordination-registry = 0.1.0' is explicitly banned
bans FAILED
[19:35:27] $ git checkout -- packages/test-helpers/Cargo.toml Cargo.lock
[19:35:27] git diff --quiet HEAD -- packages/test-helpers/Cargo.toml Cargo.lock exited 0
[19:35:27] append to packages/test-helpers/Cargo.toml: torrust-tracker-events = { version = "0.1.0", path = "../events" }
[19:35:27] $ cargo deny check bans
error[banned]: crate 'torrust-tracker-events = 0.1.0' is explicitly banned
bans FAILED
[19:35:28] $ git checkout -- packages/test-helpers/Cargo.toml Cargo.lock
[19:35:28] git diff --quiet HEAD -- packages/test-helpers/Cargo.toml Cargo.lock exited 0
[19:35:28] $ cargo deny check bans (restored tree)
bans ok
```

On the real tree the only warning other than the external `duplicate` ones is the one that was
already there before this issue (an unmatched `torrust-tracker-axum-health-check-api-server`
wrapper at `deny.toml:35`):

```text
warning[unused-wrapper]: wrapper for banned crate was not encountered
   ┌─ <repo>/deny.toml:35:6
   │
35 │     "torrust-tracker-axum-health-check-api-server",
   │      ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ unmatched wrapper

bans ok
```

The baseline printed two such warnings, this one and one for the `torrust-tracker` wrapper of
`http-protocol` (`deny.toml:63`), because the root crate did not yet depend on `http-protocol`.
The move makes that wrapper real, so only the first remains.

#### Conclusion

Each of the four bans rejects the dependency and the restored tree passes, so the entries work as
rule B.1 needs and AC3 holds. The wrappers of the new entries list only dependents inside the graph
that `cargo deny check bans` checks, which is rooted at the root crate: the console
`torrust-tracker-client` and `torrust-tracker-persistence-benchmark` depend on some of these
crates, but the check never reaches them, and listing them printed an unmatched-wrapper warning.

## Failures and Follow-up

No process failed. Two findings came out of the run, both recorded in the
[implementation retrospective](implementation-retrospective.md): the commit points of the spec
could not hold (the pre-commit checks forced the dependency pruning and the wrapper change into the
move commits), and the spec's wrapper list for `client-lib` named a crate the check never reaches.
