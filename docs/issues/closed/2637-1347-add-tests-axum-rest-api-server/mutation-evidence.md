---
doc-type: mutation-evidence
issue: 2637
package: torrust-tracker-axum-rest-api-server
status: completed
measured-utc: 2026-11-06T16:52:26Z
---

# Axum REST API Server Mutation Evidence

This is a bounded mutation-testing assessment for Issue #2637. The package has only 118 mutants, so every source
file was run. It is not a package score, a coverage target or a CI gate.

## Configuration

```text
RUSTFLAGS="--cap-lints=warn" cargo mutants \
  -p torrust-tracker-axum-rest-api-server --all-features \
  --output .tmp/mutants-2637 --timeout-multiplier 5 --jobs 2
```

`cargo-mutants 27.0.0`, stable Rust 1.99.1. The command ran twice on the S6 branch. The first run (output
`.tmp/mutants-2637`) found the survivor in `authenticate`; the second (`.tmp/mutants-2637b`) ran after the test that
kills it and after the rebase onto `develop` with the `testing` feature gate of #2745, and is the final run reported
below. The baseline without mutations passed (unit and integration
tests of the package; 32 s build, 3 s test). `--cap-lints=warn` follows the #1348 lesson: the workspace denies
`unused`, which would turn replaced bodies into unviable mutants. The unit tests of earlier slices therefore
carried this run together with the 58 integration tests.

## Results

118 mutants tested in the final run: **71 caught, 3 missed, 0 timeouts, 44 unviable.** The first run, before the
`authenticate` test, gave 70 caught, 4 missed and the same 44 unviable.

| File | Caught | Missed | Unviable |
| --- | ---: | ---: | ---: |
| `routes.rs` | 2 | 0 | 0 |
| `server.rs` | 5 | 0 | 16 |
| `testing/environment.rs` | 3 | 2 | 12 |
| `v1/context/auth_key/handlers.rs` | 9 | 1 | 0 |
| `v1/context/auth_key/responses.rs` | 7 | 0 | 0 |
| `v1/context/auth_key/routes.rs` | 1 | 0 | 0 |
| `v1/context/health_check/handlers.rs` | 0 | 0 | 4 |
| `v1/context/stats/handlers.rs` | 2 | 0 | 0 |
| `v1/context/stats/responses.rs` | 4 | 0 | 0 |
| `v1/context/stats/routes.rs` | 1 | 0 | 0 |
| `v1/context/torrent/handlers.rs` | 5 | 0 | 1 |
| `v1/context/torrent/responses.rs` | 1 | 0 | 11 |
| `v1/context/torrent/routes.rs` | 1 | 0 | 0 |
| `v1/context/whitelist/handlers.rs` | 4 | 0 | 0 |
| `v1/context/whitelist/responses.rs` | 3 | 0 | 0 |
| `v1/context/whitelist/routes.rs` | 1 | 0 | 0 |
| `v1/middlewares/auth.rs` | 15 | 0 | 0 |
| `v1/responses.rs` | 6 | 0 | 0 |
| `v1/routes.rs` | 1 | 0 | 0 |

### Missed

Fixed after the first run: `v1/middlewares/auth.rs:162`, replace `|` with `^` in `authenticate`. The fold combines one
constant-time comparison result per configured token. With distinct token values at most one comparison matches, so
`|` and `^` give the same answer; they differ only when two labels hold the **same** token value (two matches XOR to
"no match" and a valid token is rejected). No test configured duplicate values. This was a real, cheap gap, not an
equivalent mutant. A test was added after the maintainer approved its plan (`v1-middlewares-auth-tests.md`, update
2026-11-06) and the final run kills the mutant. The three survivors of the final run:

1. `v1/context/auth_key/handlers.rs:154`: replace `<AuthKeyErrorDisplay as Debug>::fmt` with `Ok(Default::default())`.
   The `Debug` impl is the three lines decided as deliberately untested in the row 6 plan. No behavior changes.
   Selected action: none.
2. `testing/environment.rs:278` `initialize_global_services` and 3. `:283` `initialize_static`: replaced with `()`.
   This is the shared test support owned by #2482 and #2449 (out of scope, rows 19-20); logging setup and the
   clock initialisation are process-global side effects that the tests do not observe. Selected action: none.

### Unviable

All 44 failed to build; none is a lint failure (see the configuration). The first compiler error of each per-mutant log
gives these kinds, which add up to 44:

- **No `Default` for the returned type** (16): `ApiServer`, `Environment`, `Launcher`, `ServiceHealthCheckJob`,
  `ConnectionInfo`, `SocketAddr`, `EnvContainer`, `CancellationRunning` and the `join_owned_tasks` result.
- **No such associated function** (16): the `Json::new`, `Json::from_iter` and `Json::from(...)` replacements of
  `health_check_handler` (4), `torrent_list_response` (7) and `torrent_info_response` (4), and one more of the same kind.
- **Ambiguous `new`** (4, `E0034`): `Environment::new` exists for two states (the `Environment` start and stop).
- **Other type errors** (8): a wrong argument count for `new()` (2), `new` on `Pin<Box<dyn Future>>` (2 plus 1), missing
  type annotations (2) and a non-const call (1). One of them is `empty_string_as_none` returning
  `Ok(Some(Default::default()))`, because the generic `T` has no `Default` bound.
