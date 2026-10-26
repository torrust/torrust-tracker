---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md
last-updated-utc: "2026-10-26 16:23"
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
| M1  | 07:14 | `stop()` disarms the guard but never cancels (`mem::forget(disarm())`) | 3 failed: stop without failure, binding released on stop, stop-then-start serves (10 s bounded wait)                          |
| M2  | 07:14 | Drop guard bound to an unrelated token                                | 4 failed: drop releases binding, stop without failure, binding released on stop, stop-then-start serves                        |
| M4  | 07:14 | Drain `TimedOut` no longer reported                                   | 3 failed: server error plus drain timeout together, error naming each failing task, drain timeout when both tasks join        |
| M5  | 07:14 | Sequential join with early return on server failure                   | 2 failed: joins the drain controller before reporting a server failure, error naming each failing task                         |

M3 is absent on purpose: the planned mutation (`let _ = guard.disarm();`) does not compile, because `disarm()` returns a `CancellationToken` that must be used, so it was dropped and the numbering kept. The failure counts and the 1 s to 10 s timings in the table were read from the runs of 2026-10-26 and were not re-run for this record.

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

## V5: Prose-first Arrange-Act-Assert comparison for the eight new tests

Each test's `// Arrange`, `// Act` and `// Assert` sections were written as
prose first, then compared with the code. Result per test:

- Stop without a failure: Arrange starts the environment; Act is
  `stop_within_deadline(environment)`; Assert is that the result is `Ok`. The
  deadline wrapper is irreducible, because a hung `stop()` must fail the test
  instead of hanging it. It is named for what it does and is the visible Act.
- Release the binding when stopped, and restart after stop: same wrapper, same
  reasoning. The binding assertion rebinds the released address directly.
- Release the binding when dropped: Act is `drop(environment)`; the assertion
  polls a bounded rebind (spec T1 list, drop path of #2471).
- Join the drain controller before reporting a server failure, report a server
  error and a drain timeout together, the error names each failing task, a
  drain timeout is reported even when both tasks join: Arrange builds the
  failing task handles, Act calls `join_owned_tasks` (or `stop()`), Assert
  reads the returned error text. No assertion is derived through production
  code.

No prose had to be kept beyond the three section markers.

## V6: Automatic checks after the last production change

The last production change is the commit
`fix(axum-health-check-api-server): [#2450] deregister the health-check test environment on stop`
(V7). Earlier last changes were
`test(axum-health-check-api-server): [#2450] document and pin the fixed-port restart limitation`
and, before it, `test(axum-health-check-api-server): [#2450] assert the stop result in the binding-release test`;
the commands below were run again after this one. Toolchain: rustc 1.99.1, with
`torrust-server-lib` 0.4.0 from crates.io (the tree after the commit
`chore(deps): [#2450] bump torrust-server-lib to 0.4.0`).

| Command                                                                                     | Result                                                |
| ------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `cargo test -p torrust-tracker-axum-health-check-api-server`                                | 14 lib passed (1.00 s), 8 integration passed (3.04 s) |
| `cargo clippy -p torrust-tracker-axum-health-check-api-server --all-targets -- -D warnings` | no warnings                                           |
| `cargo check --workspace --all-targets --all-features`                                      | no errors                                             |
| `cargo +nightly fmt --check`                                                                | clean                                                 |
| `linter all`                                                                                | exit 0 (16:23 UTC)                              |

The pre-commit hook passed on each commit.

## V7: Fixed-port restart (`review-finding:pr-2652-f1`, `review-finding:pr-2652-f2`)

Copilot's review of #2652 reported that a stop-then-start on a fixed port
panics, and da2ce7 re-raised it as `review-finding:pr-2652-f2`, asking for a
fix instead of a documented limitation.

Red test, before any change: a stop-then-start on a fixed port, with a port
picked by binding `127.0.0.1:0` and dropping the listener, failed with
`Registration { source: DuplicateBinding(...) }` raised by `start()`.

Cause (read from source, confirmed by the red test): the legacy `server::start`
never registered the service in the `registar`; `server::start_with_cancellation`
does (spec D4), and `stop()` could not remove the registration because
`torrust-server-lib` 0.3.0's `Registar` has no remove operation. Restart on an
ephemeral port (`:0`), which the earlier restart test covers, binds a new
address and is unaffected.

First decision, superseded: the maintainer chose to document the limitation and
pin it with a `should_panic` test, in
`test(axum-health-check-api-server): [#2450] document and pin the fixed-port restart limitation`.
That was made before it was known that the registry could be fixed cheaply,
and da2ce7's re-raise asked for the fix. The final change replaces it.

Final decision (the maintainer chose it): add an additive
`Registar::deregister(&ServiceBinding)` to `torrust-server-lib`
(torrust/torrust-server-lib#2, merged, published as 0.4.0), bump it in the
tracker, and make `stop()` remove the environment's own binding. Registry
keyed by binding only: it removes only the binding the environment registered,
and keeps the services of other owners. A receipt-returning `register` was
rejected as a breaking change to about eight production call sites in four
packages (counted from `grep -rn "\.register("` over `src` and `packages/*/src`).

Tests (in `environment::tests`), red before the change and green after:

- `it_should_serve_requests_after_being_stopped_and_started_again_on_a_fixed_port`
  (replaces the `should_panic` test; green restart on a nonzero port).
- `it_should_remove_only_its_own_registration_from_the_registar_when_stopped`
  (the registry holds the caller's other service and only that one afterwards).
- `it_should_remove_its_registration_even_when_stopped_with_a_failed_task`.

Mutation proofs (each restored from a copy and checked with `cmp`):

| ID  | Mutation in `stop()`                                      | Result                                                  |
| --- | --------------------------------------------------------- | ------------------------------------------------------- |
| M-A | remove the `deregister` call                              | 3 failed: the three tests above                          |
| M-B | deregister only after the join result was checked with `?` | 1 failed: the failed-task test; the other 13 passed      |

In `torrust-server-lib`, the test that keeps the other registrations failed
when the removal cleared the whole map, and passed after the restore (`cmp`).

Not covered: a bare `drop` of a running environment cannot deregister, because it
cannot await the registry lock; a fixed-port restart after a bare drop still
fails (read from source, not run). The spec states it in "Risks and Trade-offs".
