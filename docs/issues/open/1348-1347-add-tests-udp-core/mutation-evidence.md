---
doc-type: mutation-evidence
issue: 1348
package: torrust-tracker-udp-core
status: completed
measured-utc: 2026-10-14T15:17:27Z
---

# UDP Core Mutation Evidence

This is a bounded mutation-testing assessment for Issue #1348. It samples
`packages/udp-core/src/connection_cookie.rs`, the package's highest-risk seam: `make` and `check`
issue and validate the connection IDs that stop IP spoofing (BEP 15). It is not a package score,
coverage target, or CI gate.

## Configuration

```text
RUSTFLAGS="--cap-lints=warn" cargo mutants \
  --package torrust-tracker-udp-core \
  --all-features \
  --file packages/udp-core/src/connection_cookie.rs \
  --timeout 300 \
  --no-times \
  --output .tmp/1348-cookie-mutants-capped-lints.out \
  -- --lib connection_cookie::tests
```

`cargo-mutants 27.0.0`, stable Rust 1.99.0. It generated 20 mutants. The baseline run without mutations ran
the 14 `connection_cookie::tests` tests. Mutants are built in a scratch copy; the working tree is
not modified. The `.tmp` output is a git-ignored working-session artifact.

**Why `--cap-lints=warn`.** A first run with the workspace lint levels reported 12 caught and 8
unviable. Seven of the 8 were real behavior changes (for example, `check` returning `Ok` without
validating, or a constant client fingerprint) that failed to build only because the root
`Cargo.toml` denies unused code in `[workspace.lints.rust]` (`unused = "deny"`), which
`udp-core` inherits through `[lints]`: skipping a function body leaves its helpers and imports
unused. `RUSTFLAGS="--cap-lints=warn"` caps the lint levels Cargo passes from `[lints]`, so the
recorded run demotes those errors to warnings. Without that, the sample would have silently
excluded the most important mutants.

## Results

| Mutation group | Mutants | Outcome |
| --- | --- | --- |
| `check` replaced with `Ok(0.0)`, `Ok(1.0)`, `Ok(-1.0)` (accept any connection ID) | 3 | Caught |
| `!` deleted in `make` and in `check` (the `is_normal` guards) | 2 | Caught |
| `<` in the expiry comparison replaced with `==`, `>`, `<=` | 3 | Caught |
| `>` in the future-dated comparison replaced with `==`, `<`, `>=` | 3 | Caught |
| `gen_remote_fingerprint` replaced with `0` or `1` (one fingerprint for every client) | 2 | Caught |
| `cookie_builder::assemble` replaced with `Default::default()` | 1 | Caught |
| `cookie_builder::disassemble` replaced with `0.0`, `1.0`, `-1.0` | 3 | Caught |
| `cookie_builder::encode` and `decode` replaced with `Default::default()` (no encryption) | 2 | Caught |
| `make` replaced with `Ok(Default::default())` | 1 | Unviable: `ConnectionId` has no `Default` implementation |

19 caught, 1 unviable, 0 missed, 0 timeouts. No follow-up test or production change is selected.

## Scope And Limitations

- The sample covers only `connection_cookie.rs`. The other high-risk seams (the services, the ban
  threshold, the container wiring) were checked during T4 with hand-written mutations recorded in
  each file test plan, not with `cargo-mutants`.
- The run uses only the file's own tests, so a caught mutant is caught by `connection_cookie.rs`
  tests, not by service tests that also exercise cookies.
- `--cap-lints=warn` changes only lint severity; type errors still make a mutant unviable.
