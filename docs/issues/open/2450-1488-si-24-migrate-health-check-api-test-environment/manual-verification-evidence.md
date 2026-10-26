---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md
last-updated-utc: "2026-10-26 07:30"
---

# Manual Verification Evidence

## Purpose

Evidence for #2450 (SI-24): the health-check API test `Environment` moves to the
token lifecycle. Every number below was derived from the tree on 2026-10-26 UTC.

## V0: Baseline before the change

Command: `cargo test -p torrust-tracker-axum-health-check-api-server` on the
`torrust/develop` tree (rustc/cargo 1.99.1). Result: 3 lib tests and 8
integration tests passed, 0 doc-tests; integration tests 3.04 s; total 34.255 s
including a 31 s cold build.

Counts in `tests/server/contract.rs`:

- `grep -c "state.binding"` gave 8.
- `grep -c "env.stop()"` gave 8.

## V1: T1 tests

After T1 (with the 8 reads already moved to `bind_address()` so the tree
compiles): 11 lib tests (3 existing in `server::tests`, 8 new in
`environment::tests`, 1.01 s) and 8 integration tests (3.04 s) pass.

## V2: Mutation proofs for the T1 tests

Each mutation was applied to `src/environment.rs`, the filtered lib tests were
run (`cargo test -p torrust-tracker-axum-health-check-api-server --lib environment`),
then the file was restored from a saved copy and `cmp` printed no difference.
Times are UTC on 2026-10-26.

| #   | Time         | Mutation                                                              | Result                                                                                                                         |
| --- | ------------ | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| M1  | 07:14:40.635 | `stop()` disarms the guard but never cancels (`mem::forget(disarm())`) | 3 failed: stop without failure, binding released on stop, stop-then-start serves (10 s bounded wait)                          |
| M2  | 07:14 | Drop guard bound to an unrelated token                                | 4 failed: drop releases binding, stop without failure, binding released on stop, stop-then-start serves                        |
| M4  | 07:14 | Drain `TimedOut` no longer reported                                   | 3 failed: server error plus drain timeout together, error naming each failing task, drain timeout when both tasks join        |
| M5  | 07:14 | Sequential join with early return on server failure                   | 2 failed: joins the drain controller before reporting a server failure, error naming each failing task                         |

Restore proof: after each run `cmp src/environment.rs /tmp/env.orig` printed
nothing (the run printed `restored-ok` / `restored-cmp-ok`), and the original
file passes all tests (V1).

Two attempted mutations did not compile (`let _ = guard.disarm()` is rejected
for a type with a destructor; a wrong `bind_to` left a variable unused). They
proved nothing and are not counted.

## V3: No legacy stop in the environment (spec M1, read from source, not run)

Read the final `src/environment.rs`, as committed in
`refactor(axum-health-check-api-server): [#2450] migrate the health-check API test environment to the token lifecycle`.
The only `server` function it calls is `server::start_with_cancellation`
(plus the type import `CancellationRunning`). A search for `Halted` and for
`server::start` finds no legacy call, so no `signals::Halted` or
`signals::Started` channel is used.

Restart address (read from source, not run): the legacy `stop()` also returned
the configured `bind_to`, so a restart on a `:0` configuration binds a fresh
ephemeral port both before and after this change. The behaviour is unchanged.

## V4: Suite timing (spec M2)

Toolchain: rustc 1.99.1 (58cf8f9c3 2026-10-13), warm build.

| Run                   | Lib tests        | Integration tests | Total wall time      |
| --------------------- | ---------------- | ----------------- | -------------------- |
| Baseline (V0, before) | 3 passed         | 8 passed, 3.04 s  | 34.255 s with build  |
| After this change     | 11 passed, 1.0 s | 8 passed, 3.04 s  | 4.272 s, warm build  |

The integration suite time is identical. The baseline total included a 31 s
cold build, so it is not comparable with the warm total. The lib tests rose from
3 to 11 because of the added `environment::tests`; the existing tests are not
slower.
