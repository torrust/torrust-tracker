---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2454-1669-mark-public-error-enums-non-exhaustive/ISSUE.md
last-updated-utc: 2026-10-19 19:23
---

# Manual Verification Evidence

## Purpose

Real verification of scenario M1 from the [issue spec](ISSUE.md): `#[non_exhaustive]` has no effect
inside the defining crate, so only a dependent crate shows the change.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-19; M1 at 17:18, before the rebase onto `develop`; pre-push checks at
  19:19 to 19:20, after it. The rebase brought in documentation-only changes:
  `git diff 55e08830f..torrust/develop --stat -- packages console src Cargo.toml Cargo.lock` printed
  nothing, where `55e08830f` was the branch base and `torrust/develop` was `1eae3c33e`, so M1's
  crates are unchanged.
- Artifact under test: the branch after its last implementation commit, whose subject is
  `refactor(configuration)!: [#2454] remove the never-constructed Infallible error variant`. The
  branch's per-package patches are titled `feat(<package>)!: [#2454] mark public error enums
  non_exhaustive` (one per package, listed in the
  [inventory](error-enum-inventory.md#per-crate-completion)), plus `docs(events): [#2454] document
  why RecvError stays exhaustive` and `docs(udp-protocol): [#2454] document why RequestParseError
  stays exhaustive`.
- Operating system / environment: Linux 7.0.0-38-generic, local development workspace.
- Toolchain: `rustc 1.99.0 (b940084d7 2026-09-28)` and `cargo 1.99.0 (5f94df478 2026-08-27)`
  (stable Rust toolchain) for M1 and for the tests; `rustc 1.101.0-nightly (7df81381d 2026-10-14)`
  (nightly Rust toolchain) for the format, check, and documentation steps of the pre-push script.
- Prerequisites and setup performed: a scratch crate outside the workspace, with its own
  `[workspace]` table and target directory, depending by path on three crates of this branch:
  `torrust-tracker-primitives`, `torrust-tracker-configuration`, and `torrust-tracker-events`
  (manifest: [Cargo.toml.txt](evidence/m1/Cargo.toml.txt), sources: [evidence/m1/src/](evidence/m1/src),
  stored with a `.txt` suffix on the manifest so that Cargo does not treat it as a package). The
  scratch crate lived in `/tmp/m1`; the stored outputs replace the worktree path with `<worktree>`
  and the scratch path with `<scratch-crate>`.

## Verification Processes

### V1 - A downstream crate must handle new variants (M1)

- Goal: show that a dependent crate cannot exhaustively match a marked enum, that a wildcard arm
  fixes it, and that an enum kept exhaustive on purpose still matches exhaustively.
- Initial state: the branch tree as described above; the scratch crate had one binary per case.
- Status: `DONE`

#### Steps Performed

The steps ran one at a time, in this order, each with `cargo +stable check --bin <name>` in the
scratch crate. Each output file records its own start and end time. They do not depend on each
other's state, because each binary is checked on its own.

| Step | Binary | Matches | Started (UTC) | Ended (UTC) | Exit code |
| ---- | ------ | ------- | ------------- | ----------- | --------- |
| 1 | `exhaustive_primitives` | `torrust_tracker_primitives::peer::IdConversionError` with its two variants and no wildcard | 17:18:05.113 | 17:18:05.211 | 101 |
| 2 | `wildcard_primitives` | the same match with a `_` arm | 17:18:05.216 | 17:18:05.309 | 0 |
| 3 | `exhaustive_configuration` | `torrust_tracker_configuration::Error` with its seven variants and no wildcard | 17:18:05.314 | 17:18:05.422 | 101 |
| 4 | `wildcard_configuration` | the same match with a `_` arm | 17:18:05.427 | 17:18:05.522 | 0 |
| 5 | `exhaustive_recv_error` | `torrust_tracker_events::receiver::RecvError` with `Closed` and `Lagged` and no wildcard | 17:18:05.527 | 17:18:05.617 | 0 |

The first compile of the scratch crate, which built the three path dependencies, ran earlier in
the session and is not part of the table; the table shows the rechecks with a warm build.

#### Observed Result

Step 1 ([full output](evidence/m1/exhaustive_primitives.txt)) failed with `E0004`. Its relevant lines:

```text
error[E0004]: non-exhaustive patterns: `&_` not covered
note: `IdConversionError` defined here
    = note: `IdConversionError` is marked as non-exhaustive, so a wildcard `_` is necessary to match exhaustively
```

Step 3 ([full output](evidence/m1/exhaustive_configuration.txt)) failed with the same `E0004`
diagnostic for `Error`. Steps 2, 4, and 5 exited 0 ([wildcard_primitives](evidence/m1/wildcard_primitives.txt),
[wildcard_configuration](evidence/m1/wildcard_configuration.txt),
[exhaustive_recv_error](evidence/m1/exhaustive_recv_error.txt)).

#### Conclusion

M1 met its expected result: the exhaustive `match` on a marked enum fails with `E0004` in a
dependent crate, and the same `match` with a `_` arm compiles. The control (step 5) shows that
`RecvError`, which stays exhaustive by decision, still matches exhaustively.

## Automated Verification Recorded

- `cargo check` and `cargo clippy` over the whole workspace (`--workspace --all-targets
  --all-features`, stable Rust toolchain) ran while editing the packages that force downstream
  arms, and `linter all` (which includes clippy and rustfmt) ran on every commit through
  `./contrib/dev-tools/git/hooks/pre-commit.sh`, together with `cargo test --doc --workspace`.
  Each commit was made only after the script printed `SUCCESS`; the script's output is not stored.
- `./contrib/dev-tools/git/hooks/pre-push.sh` passed on the rebased branch on 2026-10-19 (19:19:16 to
  19:20:12 UTC; output in [evidence/pre-push-output.txt](evidence/pre-push-output.txt)):
  `cargo +nightly fmt --check`, `cargo +nightly check --tests --benches --examples --workspace
  --all-targets --all-features`, the two `cargo +nightly doc` runs, and
  `cargo +stable test --tests --benches --examples --workspace --all-targets --all-features`.
- Count of added attributes, run on 2026-10-19 after the last implementation commit:
  `git diff torrust/develop..HEAD -- packages console src | grep -c '^+ *#\[non_exhaustive\]'`
  printed `58`, matching the inventory's 58 enums to mark. The same diff removes the single
  `Infallible` variant (`grep -c '^-.*Infallible'` printed `1`).

## Failures and Follow-up

None. No process failed or was blocked.
