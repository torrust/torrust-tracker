---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2618-2482-decide-test-support-rules/ISSUE.md
last-updated-utc: "2026-10-22 18:08"
---

# Manual Verification Evidence

## Purpose

Record the Cargo behaviour that the options of the [#2482 EPIC](../2482-1669-reorganize-shared-test-support/EPIC.md)
rely on (scenarios MV1 to MV4 of the [spec](ISSUE.md)), plus a packaging baseline on this
repository. The scenarios test Cargo, not this repository's code, so they ran in scratch Cargo
workspaces outside the repository. Every scratch manifest and source file is embedded below,
because the scratch workspaces were removed afterwards.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-21, 17:21 to 19:02 (V1 to V4b), and 2026-10-22 at 18:07 (V4c, second run).
- Artifact under test: Cargo itself, on two toolchains:
  - stable: `cargo 1.99.1 (5f94df478 2026-08-27)`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`;
  - the workspace MSRV: `cargo 1.88.0 (873a06493 2025-05-10)`, `rustc 1.88.0 (6b00bc388
    2025-06-23)` (the root `Cargo.toml` sets `rust-version = "1.88"`).
  - The packaging baseline ran on `develop` at `7963007ee` (the merge of PR #2619).
- Operating system: Linux (the developer workstation), crates.io index reachable.
- Setup: scratch workspaces under `/tmp/2482-mv/` (`mv1` to `mv4`), each with `resolver = "3"`
  and `edition = "2024"` (whose default resolver is 3), like this workspace's root package. One `CARGO_TARGET_DIR` per scenario and toolchain.
- Every command ran through a small driver loop (kept only in `/tmp`, not in the repository): the
  `cargo` command lines below are exactly the ones it ran, in the order shown. The driver filtered
  the lines `Compiling`, `Fresh`, `Blocking`, `Locking`, `Updating`, `Downloaded` and
  `Downloading` out of the output, and, for packaging, the `warning: manifest has no
  documentation, homepage or repository` notice. Timestamps are wall-clock
  (`date -u +%T.%3N`); sub-second values are not always monotonic (V2 shows `05.702` followed by
  `05.151`), so use the order of the steps, not the sub-second digits, to read them.
- Re-runs: MV1, MV3 and MV4 each ran twice. The second run added commands (MV1: the
  `normal,features` and `no-dev,features` tree views and the `cargo metadata` listing; MV3: the
  `[lib]`-to-end manifest print; MV4: `cargo test -p <crate> --all-targets`). The exit codes of the
  commands both runs shared were identical, and this document reports the second run of each.
- Dependent steps ran one at a time, in one shell.
- Scratch workspaces and logs under `/tmp/2482-mv/` and `/tmp/2482-mv/extract-*` were removed
  after the evidence was written (see [Cleanup](#cleanup)).

## Verification Processes

### V1 - Dev-dependency features stay out of normal builds (MV1)

- Goal: show that a feature enabled only from a consumer's `[dev-dependencies]` is off in normal
  builds and on when building tests, and that an optional dependency behind the feature is not
  compiled in normal builds.
- Initial state: crate `a` has `testing = ["dep:c"]` with `c` optional; crate `b` depends on `a`
  normally and enables `a/testing` only under `[dev-dependencies]`.
- Status: `DONE`

Files (`/tmp/2482-mv/mv1`):

```toml
# Cargo.toml
[workspace]
members = ["a", "b", "c"]
resolver = "3"

[workspace.package]
edition = "2024"
version = "0.1.0"
publish = false
```

```toml
# a/Cargo.toml
[package]
name = "a"
edition.workspace = true
version.workspace = true
publish.workspace = true

[features]
testing = ["dep:c"]

[dependencies]
c = { path = "../c", optional = true }
```

```rust
// a/src/lib.rs
pub const TESTING_ON: bool = cfg!(feature = "testing");

#[cfg(feature = "testing")]
pub mod testing {
    pub fn helper() -> u32 {
        c::c_value() + 1
    }
}
```

```toml
# b/Cargo.toml
[package]
name = "b"
edition.workspace = true
version.workspace = true
publish.workspace = true

[dependencies]
a = { path = "../a" }

[dev-dependencies]
a = { path = "../a", features = ["testing"] }
```

```rust
// b/src/main.rs
fn main() {
    println!("a::TESTING_ON = {}", a::TESTING_ON);
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_test_sees_the_testing_feature() {
        assert!(a::TESTING_ON);
        assert_eq!(a::testing::helper(), 42);
    }
}
```

```rust
// b/tests/it.rs
use std::process::Command;

#[test]
fn integration_test_sees_the_testing_feature() {
    assert!(a::TESTING_ON);
    assert_eq!(a::testing::helper(), 42);
}

// The binary of `b` is built in the same `cargo test` invocation as the test, so it shares
// the feature set of that invocation.
#[test]
fn binary_built_for_the_test_run_prints_what() {
    let out = Command::new(env!("CARGO_BIN_EXE_b")).output().unwrap();
    println!("binary says: {}", String::from_utf8_lossy(&out.stdout).trim());
}
```

Crate `c` is `pub fn c_value() -> u32 { 41 }` with the same workspace-inherited package fields.

#### Steps Performed

Both toolchains, in this order (`TC` is `+stable` or `+1.88.0`; `bin` runs the binary the
previous build left in `target/debug` or `target/release`, to show what that build contained):

1. `cargo $TC build -p b`, then `bin`
2. `cargo $TC tree -p b -e normal`
3. `cargo $TC tree -p b -e features`, `-e dev,features`, `-e normal,features`, `-e no-dev,features`
4. `cargo $TC metadata --no-deps --format-version 1` (dependencies of `a` and `b`)
5. `cargo $TC build --workspace`, then `bin`
6. `cargo $TC build --workspace --all-targets`, then `bin`
7. `cargo $TC test -p b -- --nocapture`, then `bin`
8. `cargo $TC build --release -p b`, then `bin` (release)
9. `cargo $TC build -p b`, then `bin`

#### Observed Result

Identical on both toolchains (stable shown; the `1.88.0` run printed the same results):

```text
--- [17:23:49.779] cargo +stable build -p b
exit=0
--- [17:23:49.863] .../debug/b (artifact left by the previous build): a::TESTING_ON = false

--- cargo +stable tree -p b -e normal
b v0.1.0 (/tmp/2482-mv/mv1/b)
└── a v0.1.0 (/tmp/2482-mv/mv1/a)

--- cargo +stable tree -p b -e normal,features
b v0.1.0 (/tmp/2482-mv/mv1/b)
└── a feature "default"
    └── a v0.1.0 (/tmp/2482-mv/mv1/a)

--- cargo +stable tree -p b -e dev,features
b v0.1.0 (/tmp/2482-mv/mv1/b)
[dev-dependencies]
├── a feature "default"
│   └── a v0.1.0 (/tmp/2482-mv/mv1/a)
└── a feature "testing"
    └── a v0.1.0 (/tmp/2482-mv/mv1/a)

--- cargo +stable tree -p b -e features      (normal, build and dev edges together)
b v0.1.0 (/tmp/2482-mv/mv1/b)
└── a feature "default"
    └── a v0.1.0 (/tmp/2482-mv/mv1/a)
        └── c feature "default"
            └── c v0.1.0 (/tmp/2482-mv/mv1/c)
[dev-dependencies]
├── a feature "default" (*)
└── a feature "testing"
    └── a v0.1.0 (/tmp/2482-mv/mv1/a) (*)

--- cargo +stable metadata --no-deps --format-version 1 (dependencies of a and b)
a -> c kind=None optional=True features=[]
b -> a kind=None optional=False features=[]
b -> a kind=dev optional=False features=['testing']

--- cargo +stable build --workspace
--- .../debug/b: a::TESTING_ON = false

--- cargo +stable build --workspace --all-targets
--- .../debug/b: a::TESTING_ON = true

--- cargo +stable test -p b -- --nocapture
test tests::unit_test_sees_the_testing_feature ... ok            (src/main.rs unit test)
test integration_test_sees_the_testing_feature ... ok            (tests/it.rs)
binary says: a::TESTING_ON = true
test binary_built_for_the_test_run_prints_what ... ok
--- .../debug/b: a::TESTING_ON = true

--- cargo +stable build --release -p b
--- .../release/b: a::TESTING_ON = false

--- cargo +stable build -p b            (after the test run)
--- .../debug/b: a::TESTING_ON = false
```

#### Conclusion

The EPIC's expected result holds: with `resolver = "3"`, the feature enabled only from `b`'s
`[dev-dependencies]` is off in `cargo build -p b`, `cargo build --workspace` and
`cargo build --release -p b`, and on in `cargo test -p b` (unit and integration tests). `c`, the
optional dependency behind the feature, does not appear in `cargo tree -p b -e normal,features`.

Three details the options must account for:

1. **`--all-targets` turns the feature on for every target of that invocation.**
   `cargo build --workspace --all-targets` built `b`'s binary with `a/testing` on (`true`), as
   did `cargo test -p b` (the binary the test launched printed `true`). A plain `cargo build`
   rebuilt it with the feature off (`false`). A production build must therefore not use
   `--all-targets` (or `--tests`, `--examples`, `--benches`).
2. **`cargo tree -e features` without `normal,`** includes the dev edges and so shows `c`; the view
   that shows what a production build contains is `-e normal,features` (or `no-dev,features`).
3. **`cargo metadata` lists the optional dependency as a normal edge** (`a -> c kind=None
   optional=True`) and lists the dev-dependency with its features (`b -> a kind=dev
   features=['testing']`). A tool that reads edges without checking `optional` and `kind` reports
   `a -> c` as a runtime edge. This is the reason for row 8 of #2482.

### V2 - Dev-dependency cycle and type identity (MV2)

- Goal: record whether a crate's unit tests and integration tests see the same type as a support
  crate that depends on the crate under test.
- Initial state: `support` depends on `a`; `a` dev-depends on `support` (a path-only cycle).
- Status: `DONE`

Files (`/tmp/2482-mv/mv2`): the workspace manifest is the same as in V1 with members `a` and
`support`.

```toml
# a/Cargo.toml
[package]
name = "a"
edition.workspace = true
version.workspace = true
publish.workspace = true

[dev-dependencies]
support = { path = "../support" }
```

```rust
// a/src/lib.rs
pub struct Config {
    pub n: u32,
}

pub fn use_config(c: &Config) -> u32 {
    c.n
}

#[cfg(test)]
mod tests {
    // Unit test: `support` depends on `a`, so `support::config()` returns the type of the
    // *dependency* copy of `a`, while `crate::use_config` takes the type of this test copy.
    #[test]
    fn unit_test_uses_a_support_type() {
        let c = support::config();
        assert_eq!(crate::use_config(&c), 7);
    }
}
```

```rust
// a/tests/it.rs
// Integration test: the test links the dependency copy of `a`, the same one `support` uses.
#[test]
fn integration_test_uses_a_support_type() {
    let c = support::config();
    assert_eq!(a::use_config(&c), 7);
}
```

```toml
# support/Cargo.toml
[package]
name = "support"
edition.workspace = true
version.workspace = true
publish.workspace = true

[dependencies]
a = { path = "../a" }
```

`support/src/lib.rs` is `pub fn config() -> a::Config { a::Config { n: 7 } }`.

#### Steps Performed

Both toolchains: `cargo $TC build --workspace`, `cargo $TC test -p a --lib`,
`cargo $TC test -p a --test it`, `cargo $TC test -p a`, `cargo $TC test --workspace`.

#### Observed Result

Exit codes were identical on stable and `1.88.0`:

| Command | Exit | Result |
| --- | --- | --- |
| `cargo build --workspace` | 0 | The dev-dependency cycle builds |
| `cargo test -p a --lib` | 101 | `error[E0308]: mismatched types` in the unit test |
| `cargo test -p a --test it` | 0 | The integration test passes |
| `cargo test -p a` | 101 | The same E0308; nothing runs |
| `cargo test --workspace` | 101 | The same E0308 |

```text
--- [17:24:04.765] cargo +stable test -p a --lib
error[E0308]: mismatched types
  --> a/src/lib.rs:16:38
   |
16 |         assert_eq!(crate::use_config(&c), 7);
   |                    ----------------- ^^ expected `Config`, found `a::Config`
   |                    |
   |                    arguments to this function are incorrect
   |
note: there are multiple different versions of crate `a` in the dependency graph
...
error: could not compile `a` (lib test) due to 1 previous error
exit=101
```

#### Conclusion

Confirmed as the EPIC states for O2 and O8: when the support crate depends on the crate under
test, the crate's own `#[cfg(test)]` code and the support crate see two different copies of the
crate, so their types do not match, and the whole `cargo test -p a` fails to compile. The crate's
own `tests/` use the dependency copy and see the same types as the support crate. A support crate
therefore cannot serve unit tests that call into the crate with the support crate's values. It can
serve `tests/`, `examples/` and other packages.

### V3 - Publishing with optional and dev dependencies (MV3)

- Goal: record which dependency forms `cargo package --no-verify` and `cargo publish --dry-run`
  accept, which they strip, and which need a registry.
- Initial state: a scratch workspace of 12 crates named `zz2482-*` (names absent from crates.io),
  each `pub fn f() {}`. `publish = false` is set only on `zz2482-priv`. Every `publish` command
  carried `--dry-run`.
- Status: `DONE`

Files (`/tmp/2482-mv/mv3`): workspace `members` list the 12 directories, with `resolver = "3"`
and workspace-inherited `edition`, `version = "0.1.0"`, `description` and `license = "MIT"`. The
only differences between the member manifests are their dependency tables:

| Crate (suffix of `zz2482-`) | Dependency table in its `Cargo.toml` |
| --- | --- |
| `leaf` | none |
| `priv` | none; `publish = false` |
| `opt-versioned` | `[features] testing = ["dep:zz2482-leaf"]`; `[dependencies] zz2482-leaf = { version = "0.1.0", path = "../leaf", optional = true }` |
| `opt-pathonly` | the same, without `version` |
| `devpriv-pathonly` | `[dev-dependencies] zz2482-priv = { path = "../priv" }` |
| `devpriv-versioned` | `[dev-dependencies] zz2482-priv = { version = "0.1.0", path = "../priv" }` |
| `self-pathonly` | `[features] testing = []`; `[dev-dependencies] zz2482-self-pathonly = { path = ".", features = ["testing"] }` |
| `self-versioned` | `[features] testing = []`; `[dev-dependencies] zz2482-self-versioned = { version = "0.1.0", path = ".", features = ["testing"] }` |
| `cyc-a` | `[dev-dependencies] zz2482-cyc-support = { version = "0.1.0", path = "../cyc-support" }` |
| `cyc-support` | `[dependencies] zz2482-cyc-a = { version = "0.1.0", path = "../cyc-a" }` |
| `cycp-a` | `[dev-dependencies] zz2482-cycp-support = { path = "../cycp-support" }` |
| `cycp-support` | `[dependencies] zz2482-cycp-a = { version = "0.1.0", path = "../cycp-a" }` |

#### Steps Performed

Both toolchains, with `P=zz2482`. After each successful `package` of the crates below, the
`.crate`'s normalized `Cargo.toml` was read with `tar -xzOf target/package/<crate>-0.1.0.crate
<crate>-0.1.0/Cargo.toml`.

1. `cargo $TC package -p $P-leaf --no-verify`
2. `cargo $TC package -p $P-opt-versioned --no-verify`, then `... -p $P-opt-versioned -p $P-leaf
   --no-verify`
3. `cargo $TC package -p $P-opt-pathonly -p $P-leaf --no-verify`
4. `cargo $TC package -p $P-devpriv-pathonly --no-verify`, then `-p $P-devpriv-versioned`
5. `cargo $TC package -p $P-self-pathonly --no-verify`, then `-p $P-self-versioned`
6. `cargo $TC package -p $P-cyc-a --no-verify`, then `-p $P-cyc-a -p $P-cyc-support`
7. `cargo $TC package -p $P-cycp-a --no-verify`, then `-p $P-cycp-a -p $P-cycp-support`
8. `cargo $TC package --workspace --no-verify`
9. `cargo $TC publish --dry-run -p <crate>` for `leaf`, `opt-versioned`, `opt-pathonly`,
   `devpriv-pathonly`, `devpriv-versioned`, `self-pathonly`, `self-versioned`, `cyc-a`, `cycp-a`,
   then `cargo $TC publish --dry-run --workspace`

#### Observed Result

Exit codes (`0` accepted, `101` rejected); `stable` is cargo 1.99.1, `1.88` is cargo 1.88.0:

| Command | stable | 1.88 | Reason on rejection |
| --- | --- | --- | --- |
| `package -p leaf` | 0 | 0 | |
| `package -p opt-versioned` | 101 | 101 | `no matching package named zz2482-leaf found; location searched: crates.io index` |
| `package -p opt-versioned -p leaf` | 0 | 101 | on 1.88: the same registry lookup for `leaf`, although `leaf` was packaged in the same run |
| `package -p opt-pathonly -p leaf` | 101 | 101 | `all dependencies must have a version requirement specified when packaging. dependency zz2482-leaf does not specify a version` |
| `package -p devpriv-pathonly` | 0 | 0 | the dev-dependency is stripped |
| `package -p devpriv-versioned` | 101 | 101 | `no matching package named zz2482-priv found` |
| `package -p self-pathonly` | 0 | 0 | the self dev-dependency is stripped |
| `package -p self-versioned` | 101 | 101 | `no matching package named zz2482-self-versioned found` (the #2582 failure) |
| `package -p cyc-a` | 101 | 101 | `no matching package named zz2482-cyc-support found` |
| `package -p cyc-a -p cyc-support` | 101 | 101 | `no matching package named zz2482-cyc-a found` (each needs the other first) |
| `package -p cycp-a` | 0 | 0 | the path-only dev-dependency is stripped |
| `package -p cycp-a -p cycp-support` | 0 | 101 | on 1.88: `no matching package named zz2482-cycp-a found` |
| `package --workspace` | 101 | 101 | the first rejected member (`cyc-support`) stops it |
| `publish --dry-run -p leaf` | 0 | 0 | |
| `publish --dry-run -p opt-versioned` | 101 | 101 | `leaf` is not on the registry |
| `publish --dry-run -p opt-pathonly` | 101 | 101 | the version requirement error above |
| `publish --dry-run -p devpriv-pathonly` | 0 | 0 | verifies the build, then stops at the dry run |
| `publish --dry-run -p devpriv-versioned` | 101 | 101 | `priv` is not on the registry |
| `publish --dry-run -p self-pathonly` | 0 | 0 | |
| `publish --dry-run -p self-versioned` | 101 | 101 | |
| `publish --dry-run -p cyc-a` | 101 | 101 | |
| `publish --dry-run -p cycp-a` | 0 | 0 | |
| `publish --dry-run --workspace` | 101 | 101 | stable: stopped at `opt-pathonly`; 1.88: `the --workspace flag is unstable, and only available on the nightly channel` |

The normalized `Cargo.toml` of each accepted crate, from `[lib]` to the end:

```text
zz2482-devpriv-pathonly      zz2482-self-pathonly          zz2482-cycp-a
[lib]                        [lib]                         [lib]
name = "zz2482_devpriv_..."  name = "zz2482_self_pathonly"  name = "zz2482_cycp_a"
path = "src/lib.rs"          path = "src/lib.rs"           path = "src/lib.rs"

[dev-dependencies]           [dev-dependencies]            [dev-dependencies]
```

The packaged `self-pathonly` manifest still has `testing = []` under `[features]`. The packaged
`opt-versioned` manifest keeps the optional dependency and the feature:

```toml
[features]
testing = ["dep:zz2482-leaf"]

[dependencies.zz2482-leaf]
version = "0.1.0"
optional = true
```

Baseline on this repository (read-only; `git status --short` was empty, `HEAD` was `7963007ee`),
each with its own `CARGO_TARGET_DIR`:

```text
--- [17:33:41.303] cargo +stable -V: cargo 1.99.1 (5f94df478 2026-08-27)
--- cargo +stable package --workspace --no-verify
exit=0 [17:34:21.183]        (32 "Packaged" lines: one per workspace member)

--- [17:34:21.189] cargo +1.88.0 -V: cargo 1.88.0 (873a06493 2025-05-10)
--- cargo +1.88.0 package --workspace --no-verify
exit=101 [17:34:25.492]
error: failed to prepare local package for uploading
Caused by:
  no matching package named `torrust-tracker-axum-server` found
  location searched: crates.io index
  required by package `torrust-tracker-axum-health-check-api-server v0.1.0 (...)`
```

#### Conclusion

- **Dev-dependencies without a `version` are stripped** by `cargo package`, whether they point to
  a `publish = false` crate (`devpriv-pathonly`), to the crate itself (`self-pathonly`, O1c) or to
  a sibling (`cycp-a`, O2). One **with a version** is kept and must exist on a registry, so it
  fails for a `publish = false` crate (`devpriv-versioned`), for the crate itself
  (`self-versioned`, the #2582 failure) and for a cycle between two publishable crates
  (`cyc-a`/`cyc-support`: neither can be packaged first). The packaged manifest keeps an empty
  `[dev-dependencies]` table. This confirms the EPIC's O1c and O2 statements, and adds: **for
  O2, `a`'s dev-dependency on its support crate must be path-only**, even when the support
  crate is publishable, or neither crate can be packaged.
- **An optional dependency is a normal dependency for packaging.** It needs a `version`
  (`opt-pathonly` fails) and the dependency must be on the registry or packaged in the same run
  (`opt-versioned` alone fails; with `-p leaf` it passes on stable). So under O1 every optional
  dependency of a published crate must itself be publishable, and a `publish = false` crate
  cannot be an optional dependency of a published one.
- **Cargo 1.88.0 cannot package workspace members that depend on each other by version**:
  `package -p a -p leaf` fails there, and so does `cargo package --workspace --no-verify` on this
  repository (the last exit above). The CI job and the pre-push step run it with
  `cargo +stable`, which passes (32 members). The check is not available at the MSRV.

### V4 - A crate's own targets using its own `testing` feature (MV4)

- Goal: record, for O1a, O1b and O1c, which of a crate's own targets (unit tests in `src/`,
  `tests/`, `examples/`, doc tests) compile, run or are skipped, with and without
  `--all-features`, and what `cargo package --no-verify` and the packaged `.crate` do.
- Initial state: three scratch crates, each with a `testing` feature gating
  `#[cfg(any(test, feature = "testing"))] pub mod testing`, and a `consumer` crate that
  dev-depends on all three with the feature.
- Status: `DONE`

Files (`/tmp/2482-mv/mv4`): workspace manifest as in V1 with members `o1a`, `o1b`, `o1c` and
`consumer`.

`o1a/Cargo.toml` (O1a: command line only), `o1b/Cargo.toml` (O1b) and `o1c/Cargo.toml` (O1c)
share `edition.workspace`, `version.workspace`, `description` and `license = "MIT"`; they differ
only as follows:

```toml
# o1a/Cargo.toml
[features]
testing = []
```

```toml
# o1b/Cargo.toml
[features]
testing = []

[[test]]
name = "it"
required-features = ["testing"]

[[example]]
name = "ex"
required-features = ["testing"]
```

```toml
# o1c/Cargo.toml
[features]
testing = []

[dev-dependencies]
o1c = { path = ".", features = ["testing"] }
```

The three `src/lib.rs` are the same apart from the crate name (shown for `o1a`):

```rust
#[cfg(any(test, feature = "testing"))]
pub mod testing {
    pub fn helper() -> u32 {
        7
    }
}

/// ```
/// assert_eq!(o1a::testing::helper(), 7);
/// ```
pub fn production() -> u32 {
    1
}

#[cfg(test)]
mod unit {
    #[test]
    fn unit_test_uses_the_testing_module() {
        assert_eq!(crate::testing::helper(), 7);
    }
}
```

Each crate's `tests/it.rs` asserts `<crate>::testing::helper() == 7` in one test
(`integration_test_uses_the_testing_module`), and its `examples/ex.rs` prints
`<crate> helper = {}` from the same call.

```toml
# consumer/Cargo.toml
[package]
name = "consumer"
edition.workspace = true
version.workspace = true
publish.workspace = true

[dependencies]
o1a = { path = "../o1a" }
o1b = { path = "../o1b" }
o1c = { path = "../o1c" }

[dev-dependencies]
o1a = { path = "../o1a", features = ["testing"] }
o1b = { path = "../o1b", features = ["testing"] }
o1c = { path = "../o1c", features = ["testing"] }
```

`consumer/src/lib.rs` returns `o1a::production() + o1b::production() + o1c::production()`, and
`consumer/tests/it.rs` asserts the three `testing::helper()` values sum to 21 and `sum() == 3`.

#### Steps Performed

Both toolchains. For each `X` in `o1a`, `o1b`, `o1c`, in this order:

1. `cargo $TC build -p X`
2. `cargo $TC test -p X`, then `--lib`, `--test it`, `--example ex`, `--doc`, `--all-targets`
3. `cargo $TC run -q -p X --example ex`
4. `cargo $TC test -p X --all-features`
5. `cargo $TC run -q -p X --example ex --features testing`
6. `cargo $TC package -p X --no-verify`, extract the `.crate`, print its `Cargo.toml` from `[lib]`,
   then in the extracted directory `cargo $TC test` and `cargo $TC test --all-features`

Then in the workspace: `cargo $TC test --workspace`, `test --doc --workspace`,
`test --workspace --all-targets`, `test --workspace --all-features`,
`build --workspace --all-targets`. After both toolchains: `cargo +stable metadata --no-deps
--format-version 1` and `cargo machete --with-metadata /tmp/2482-mv/mv4`.

#### Observed Result

Exit codes were identical on stable and `1.88.0`. For the `--all-targets` row, the text after the
exit code lists what ran.

| Command | O1a | O1b | O1c |
| --- | --- | --- | --- |
| `build -p X` (production build) | 0 | 0 | 0 |
| `test -p X` | 101: `tests/it.rs` and `examples/ex.rs` do not compile (`cannot find testing in X`) | 101: the unit test runs, then the doc test fails; `tests/it.rs` and `examples/ex.rs` never run | 0: unit, `tests/it.rs`, `examples/ex.rs` and the doc test run |
| `test -p X --lib` (unit tests) | 0 | 0 | 0 |
| `test -p X --test it` | 101 | 101: `target it in package o1b requires the features: testing` | 0 |
| `test -p X --example ex` | 101 | 101: `requires the features: testing` | 0 |
| `test -p X --doc` | 101 | 101 | 0 |
| `test -p X --all-targets` | 101 | **0, silently skipping**: only `unittests src/lib.rs` ran; `it` and `ex` did not, with no notice | 0: `src/lib.rs`, `tests/it.rs`, `examples/ex.rs` |
| `run -q -p X --example ex` | 101 | 101: `requires the features: testing` | 0 (`o1c helper = 7`) |
| `test -p X --all-features` | 0 | 0 | 0 |
| `run -q -p X --example ex --features testing` | 0 | 0 | 0 |
| `package -p X --no-verify` | 0 | 0 | 0 |
| in the extracted `.crate`: `cargo test` | 101: `tests/it.rs`, `examples/ex.rs` do not compile | 101: the doc test fails | 101: `tests/it.rs` and `examples/ex.rs` do not compile |
| in the extracted `.crate`: `cargo test --all-features` | 0 | 0 | 0 |

Workspace commands (the `consumer` crate enables `testing` on all three), all exit 0 on both
toolchains:

| Command | Result |
| --- | --- |
| `cargo test --workspace` | 0; the unit tests, `tests/it.rs` and doc tests of all three crates ran (examples were built) |
| `cargo test --doc --workspace` (the pre-commit command) | 0, all three doc tests passed |
| `cargo test --workspace --all-targets` | 0 |
| `cargo test --workspace --all-features` | 0 |
| `cargo build --workspace --all-targets` | 0 |

Excerpts (stable):

```text
--- cargo +stable test -p o1b
     Running unittests src/lib.rs
test unit::unit_test_uses_the_testing_module ... ok
   Doc-tests o1b
test o1b/src/lib.rs - production (line 8) ... FAILED
error[E0433]: cannot find `testing` in `o1b`
note: found an item that was configured out
error: doctest failed, to rerun pass `-p o1b --doc`
exit=101

--- cargo +stable test -p o1b --test it
error: target `it` in package `o1b` requires the features: `testing`
exit=101

--- cargo +stable package -p o1b --no-verify       (packaged manifest keeps required-features)
[[example]]  name = "ex"  required-features = ["testing"]
[[test]]     name = "it"  required-features = ["testing"]

--- cargo +stable package -p o1c --no-verify       (packaged manifest of o1c, from [lib])
[lib] name = "o1c" path = "src/lib.rs"
[[example]] name = "ex" path = "examples/ex.rs"
[[test]] name = "it" path = "tests/it.rs"
[dev-dependencies]                                 (empty: the self dev-dependency was stripped)

--- cargo +stable metadata --no-deps --format-version 1
o1c -> o1c kind=dev features=['testing']
consumer -> o1a kind=None features=[]      consumer -> o1a kind=dev features=['testing']
(the same for o1b and o1c)

--- cargo machete --with-metadata /tmp/2482-mv/mv4
cargo-machete didn't find any unused dependencies in /tmp/2482-mv/mv4. Good job!
```

#### Conclusion

- **O1a** leaves `tests/`, `examples/` and doc tests failing to compile under a plain `cargo test -p X`;
  only unit tests (`#[cfg(any(test, feature = "testing"))]`) work. `--all-features` fixes all of
  them, including the doc test, so CI, the pre-push script, the nightly checks and the publish
  workflow (`cargo test -p <crate> --all-targets --all-features`) pass. The pre-commit
  `cargo test --doc --workspace` and a contributor's `cargo test -p X` do not, **unless another
  workspace member dev-depends on `X` with the feature**: in the last table every workspace
  command passed because `consumer` enabled the feature for the whole invocation (feature
  unification), which hides the failures that the per-package commands show.
- **O1b** builds without the feature, but it does not run `tests/it.rs` or `examples/ex.rs`
  without it. `cargo test -p X --all-targets` (the form CI and pre-push use, without
  `--all-features`) exited 0 and ran only the unit test, with no notice: the silent test loss the
  EPIC warns about. A doc test cannot carry `required-features`, so a doc test that uses the
  module still fails `cargo test -p X` (loudly), and O1b does not remove that failure. Naming the
  target explicitly (`--test it`) is an error, not a skip.
- **O1c** builds and runs all four target kinds with plain `cargo test -p X`, with and without
  `--all-features`, on both toolchains, and `cargo package --no-verify` strips the self
  dev-dependency (see V3). Its costs: the packaged `.crate`'s `tests/` and `examples/` need
  `--features testing` to compile (the `.crate` row), and `cargo metadata` lists a self edge
  (`o1c -> o1c kind=dev`) that the coupling tool and the diagram must skip. `cargo machete
  --with-metadata` does not flag it.
- The production build is not affected by any mechanism: V1 shows the feature off for a consumer's
  dev-dependency, and V4c shows it off for a self dev-dependency, in `cargo build` and
  `cargo build --release`.

#### V4b - Does O1c need `any(test, ...)` in the gate? (added after the maintainer's decision)

A follow-up probe, run at 19:02 UTC in a standalone package `q` (its own `[workspace]` table,
removed afterwards), because the ADR's rule for O1c depends on it. The module is gated by the
feature alone:

```toml
# Cargo.toml
[package]
name = "q"
version = "0.1.0"
edition = "2024"

[features]
testing = []

[dev-dependencies]
q = { path = ".", features = ["testing"] }

[workspace]
```

```rust
// src/lib.rs
#[cfg(feature = "testing")]
pub mod testing {
    pub fn helper() -> u32 {
        7
    }
}

#[cfg(test)]
mod unit {
    #[test]
    fn unit_test_uses_the_module_gated_only_by_the_feature() {
        assert_eq!(crate::testing::helper(), 7);
    }
}
```

`tests/it.rs` asserts `q::testing::helper() == 7`. Commands, on `+stable` and `+1.88.0`, in this
order: `cargo $TC test`, `cargo $TC build`. Both toolchains: `cargo test` exited 0 and ran the
unit test (`unit::unit_test_uses_the_module_gated_only_by_the_feature ... ok`) and `tests/it.rs`
(`it ... ok`); `cargo build` finished. So with the self dev-dependency the feature is on for the
crate's own unit tests too, and `#[cfg(feature = "testing")]` alone is enough; the
`any(test, feature = "testing")` form is needed only without it (O1a).

#### V4c - A self dev-dependency in production builds versus test builds (added after the pre-PR review, rerun after PR review)

The review of this work found that V4 did not show, for a crate with a self dev-dependency, that a
production build leaves the feature off (V1 shows it for a consumer's dev-dependency, and V4's gate
was `any(test, feature)`, so `build -p X` exiting 0 proved nothing). A first V4c run (2026-10-22 at
17:15 UTC) had two stale reads, found in the PR review (finding F2): its package had no integration
test, so `cargo build --tests` and `cargo test --no-run` did not build the binary, and the value
they showed was the one the previous row left. It was replaced by the run below, on 2026-10-22 at
18:07 UTC, in a standalone package `q` (its own `[workspace]` table, removed afterwards) that has
an integration test, with the binaries deleted before each row.

`Cargo.toml` is the one of V4b. The other files:

```rust
// src/lib.rs
pub const TESTING_ON: bool = cfg!(feature = "testing");
```

```rust
// src/main.rs
fn main() {
    println!("q::TESTING_ON = {}", q::TESTING_ON);
}
```

```rust
// tests/it.rs
// The integration test makes Cargo build the package's binary for `CARGO_BIN_EXE_q`.
#[test]
fn integration_test_sees_the_feature_and_the_binary_path() {
    assert!(q::TESTING_ON);
    assert!(!env!("CARGO_BIN_EXE_q").is_empty());
}
```

On `+stable` and on `+1.88.0`, in this order, `target/debug/q` and `target/release/q` were deleted
before each command (`rm -f`), the command ran, and the binary it left, if any, was run. Every
command exited 0 on both toolchains:

| Command | Binary left, and what it printed |
| --- | --- |
| `cargo build` | `debug/q`: `q::TESTING_ON = false` |
| `cargo build --release` | `release/q`: `q::TESTING_ON = false` |
| `cargo build --all-targets` | `debug/q`: `q::TESTING_ON = true` |
| `cargo build --tests` | `debug/q`: `q::TESTING_ON = true` |
| `cargo test --no-run` | `debug/q`: `q::TESTING_ON = true` |
| `cargo test` | `debug/q`: `q::TESTING_ON = true` |

A control run with the `tests/` directory moved away, `debug/q` deleted before each command:
`cargo build --tests` and `cargo test --no-run` exited 0 and built no binary, on both toolchains.
That is what made the first run's rows stale: Cargo builds a package's binary as an executable for
`cargo build --tests` and `cargo test` only when an integration test (or bench) needs it, for
`CARGO_BIN_EXE_<name>`.

So with a self dev-dependency, `cargo build` and `cargo build --release` leave the feature off.
`cargo build --all-targets` always builds the package's binary with it on, and so do `--tests` and
`cargo test` when the package has an integration test (the root crate does), without
`--all-features`. The shipped binary must therefore be built by an invocation of its own, without
those flags.

## Evidence versus the EPIC

| EPIC statement | Result |
| --- | --- |
| MV1: the feature is off in the normal build and on in `cargo test -p B` | Confirmed (V1) |
| MV2: unit tests see mismatched types, integration tests do not | Confirmed (V2) |
| O1c: plain `cargo test -p` builds unit tests, `tests/` and examples with the feature; `cargo package --no-verify` strips a path-only self dev-dependency | Confirmed (V4, V3), and doc tests also get the feature |
| O1c: the published `.crate`'s tests need `--features testing` | Confirmed (V4) |
| O1b: Cargo skips the targets without failing when the feature is off | Confirmed for `--all-targets`; with a doc test that uses the module `cargo test -p` fails on the doc test, and explicit `--test it` is an error (V4) |
| O2: a `publish = false` support crate must be a path-only dev-dependency | Confirmed (V3). Also: a publishable support crate that depends on the crate under test must also be path-only in the crate's dev-dependencies (V3, `cyc-*`) |

No verification result contradicts the EPIC. Separately, while preparing the ADR the inventory's
claim that only `tracker-core`'s own source uses its mocks was checked against the other packages
and found wrong (`http-core` and `udp-core` import `MockTorrentMetricsStore`); it was corrected in
the inventory. Four findings from the verification are new and are not in the EPIC:

1. `cargo test --workspace` hides O1a and O1b gaps whenever another member dev-depends on the
   crate with the feature (V4).
2. `--all-targets` (and `--tests`, `--examples`) enable dev-dependency features for every target
   of the invocation, including binaries (for `--tests`, when the package has an integration test)
   and without `--all-features`; a production build must be `cargo build` without them (V1, V4c).
3. Cargo 1.88.0 (the MSRV) cannot run `cargo package --workspace --no-verify` on this repository;
   stable can (V3).
4. A versioned dev-dependency cycle between two publishable crates cannot be packaged at all, so
   an O2 support crate that depends on the crate it supports forces a path-only dev-dependency
   (V3).

## Failures and Follow-up

No scenario failed. Commands that exited `101` are expected results recorded above, not failures
of the verification. The first MV3 and MV4 runs were repeated only to add commands, as described
under Environment. The first V4c run had two stale reads (its `--tests` and `--no-run` rows); V4c
was rerun and replaced, as its text says.

## Cleanup

After the evidence was written, the scratch workspaces, target directories, extracted `.crate`
directories, driver scripts and logs under `/tmp/2482-mv/` were deleted
(`rm -rf /tmp/2482-mv`), and the `CARGO_TARGET_DIR`s of the baseline runs
(`/tmp/2482-mv/target-real-stable`, `/tmp/2482-mv/target-real-1.88.0`) went with them. The
repository's `target/` directory and working tree were not touched by the scenarios (the baseline
used the external target directories above; `git status --short` was empty before the runs).
