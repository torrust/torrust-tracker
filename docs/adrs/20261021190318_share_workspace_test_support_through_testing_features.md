---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/issues/open/2618-2482-decide-test-support-rules/ISSUE.md
    - docs/issues/open/2618-2482-decide-test-support-rules/manual-verification-evidence.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - deny.toml
    - "issue #2482"
    - "issue #2582"
    - "issue #2618"
---

<!-- skill-link: create-adr -->

# Share Workspace Test Support Through `testing` Features and a Generic Helper Crate

## Scope

Root ADR. It governs how workspace packages provide test support code (test environments,
fixtures, helpers and mocks) to their own tests and to the tests of other packages. The decision
binds every workspace package and the dev-dependency edges between them, and no package owns it,
so it lives in `docs/adrs/`. It amends DEC-13 of the EPIC #1669 decision log (see
[Relationship to DEC-13](#relationship-to-dec-13)).

## Description

The [test support inventory](../issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md)
found five patterns in the workspace (32 members on 2026-10-21):

- **A. Public `testing`/`environment` modules** in `axum-http-server`, `axum-rest-api-server`,
  `udp-server` and `axum-health-check-api-server`. Each starts a server for tests and is compiled
  into every build. They force nine normal dependency edges that production code does not use
  (six in `axum-rest-api-server`, one each in the other three).
- **B. The shared `test-helpers` crate**, a dev-dependency of seven packages. Its `http` and `udp`
  modules have one consumer (the root crate's `tests/common/`) but pull three unpublished protocol
  and client crates into every dependent.
- **C. Crate-private helpers in `src/`** (`tracker-core`), declared `pub mod` although their
  content is `#[cfg(test)]`.
- **D. Unconditional mocks**: `tracker-core`'s four `#[automock]` database traits make `mockall`
  a normal dependency and put `Mock*` types in the public API. One of them,
  `MockTorrentMetricsStore`, is imported by `#[cfg(test)]` modules of `http-core` and `udp-core`.
- **E. Per-package `tests/common/` folders**, the right place for single-package helpers.

DEC-13 (2026-06-11) put the three server environments in public `src/testing/` modules, "importable
by external test packages", on the premise that `tracker-core/src/test_helpers.rs` already did the
same. The modules' own documentation adds that each is "exported unconditionally" and "compiled in
all build profiles". That module is `#[cfg(test)]` and private
to its crate, so the premise was wrong: pattern A is the only pattern that ships test code and
test-only dependencies in production builds.

Every workspace member without `publish = false` is planned to be published to crates.io (the
[independent versioning ADR](20260629000000_adopt_independent_package_versioning.md)). Publishability
is not the driver here. The drivers are coupling (`test-helpers` has become a hub) and test code in
production builds.

### Criteria

1. A package's normal build contains no test-only code or dependencies unless the consumer opts in.
2. No crate depends on unrelated packages just to share helpers.
3. Unit tests (`#[cfg(test)]` in `src/`), integration tests (`tests/`), examples, benches and doc
   tests can all use the support code they need, with the same types as the code under test.
4. A package's own test targets build and run with a plain `cargo test -p <package>`, and none is
   skipped without notice when a feature is off.
5. Every package without `publish = false` stays publishable, and
   `cargo package --workspace --no-verify` (the CI job and pre-push step added by #2582) passes.
6. Few extra workspace members and simple rules for contributors.

### Verified Cargo behaviour

The options below rely on Cargo behaviour that was checked, not assumed, in scratch workspaces on
stable (cargo 1.99.1) and on the workspace MSRV (cargo 1.88.0). The commands, manifests and output
are in the
[manual verification evidence](../issues/open/2618-2482-decide-test-support-rules/manual-verification-evidence.md)
(V1 to V4c).

- A feature enabled only from a consumer's `[dev-dependencies]` is off in `cargo build`,
  `cargo build --workspace` and `cargo build --release`, and on in `cargo test`. `--all-targets`
  turns dev-dependency features on for every target of that invocation, binaries included. (V1)
- A support crate that depends on the crate under test gives that crate's `#[cfg(test)]` code a
  second copy of the crate: `error[E0308]: mismatched types`. The crate's `tests/` see the same
  types as the support crate. (V2)
- `cargo package` strips a dev-dependency that has no `version`, and keeps one that has: the kept
  one must exist on a registry, which fails for a `publish = false` crate, for the crate itself
  (#2582) and for a publishable crate that has a versioned dev-dependency on a publishable crate
  that depends back on it (neither can be packaged first). An optional dependency is
  a normal dependency for packaging: it needs a `version`, and its target must be publishable. (V3)
- Cargo 1.88.0 cannot run `cargo package --workspace --no-verify` on this repository; stable can,
  and CI and pre-push use stable. (V3)
- With a path-only self dev-dependency, a plain `cargo test -p <crate>` builds and runs the crate's
  unit tests, `tests/`, `examples/` and doc tests with the feature on, and a module gated by
  `#[cfg(feature = "testing")]` alone is visible to the unit tests. Without it, only the unit
  tests run. `cargo test --workspace` passes either way when another member enables the feature,
  which hides the gap. `required-features` makes `cargo test -p <crate> --all-targets` skip
  `tests/` and `examples/` silently and cannot be put on doc tests. (V4, V4b)
- With the self dev-dependency, `cargo build` and `cargo build --release` leave the feature off,
  and `cargo build --all-targets` or `--tests` builds the package's binary with it on, without
  `--all-features`. (V4c)

## Agreement

Test support code is shared through a Cargo feature named `testing` on the package that owns it,
and through one generic helper crate for protocol-agnostic helpers. Everything else stays in the
package that needs it. The maintainer chose this combination on 2026-10-21, after the evidence
above.

Test support that lives outside a test module is production code: it is compiled into the crate
and is merely code that other projects' tests use. It must not reach a shipped artifact unless a
build asks for it. So it sits behind a feature that is off by default, and every build command that
produces a shipped artifact names its features explicitly instead of enabling all of them. The
maintainer added the last part on 2026-10-22: he prefers a CI error for a missing required feature
to a risk of shipping a feature that was not meant for production.

### Where does a helper go?

| The helper is... | It goes in... | Rule |
| --- | --- | --- |
| A server environment other packages' tests start (pattern A) | The `src/testing/` (or `src/environment.rs`) module of the package whose server it starts, behind the `testing` feature | [Rule A](#rule-a-server-environments-behind-a-testing-feature) |
| A protocol-agnostic utility many packages use (configuration, logging, randomness) | `torrust-tracker-test-helpers` | [Rule B](#rule-b-test-helpers-stays-generic) |
| A protocol-specific helper with one consumer | The consumer's `tests/common/` | [Rule B](#rule-b-test-helpers-stays-generic) |
| A helper or mock used only inside one crate's unit tests | A private `#[cfg(test)]` module in that crate | [Rule C](#rule-c-crate-private-test-helpers-are-private) |
| A mock that other packages' tests also use | The owning crate, behind the `testing` feature | [Rule D](#rule-d-mocks-other-packages-use-sit-behind-the-testing-feature) |
| A helper used only by one package's integration tests | That package's `tests/common/` | [Rule E](#rule-e-per-package-tests-folders-stay) |

### Rule A: server environments behind a `testing` feature

This rule covers `axum-http-server`, `axum-rest-api-server`, `udp-server` and
`axum-health-check-api-server`, and any future server package.

1. The environment module is compiled only when the package's `testing` feature is on, with
   `#[cfg(feature = "testing")]` on the module declaration. `testing` is never in `default`.
2. A dependency that only the module uses is `optional = true` and is enabled by
   `testing = ["dep:<name>", ...]`, so no implicit feature is created.
3. **The package's own targets use the path-only self dev-dependency (O1c):**

   ```toml
   [dev-dependencies]
   torrust-tracker-udp-server = { path = ".", features = ["testing"] }
   ```

   It has no `version`: a self dev-dependency with a `version` makes `cargo package` fail (#2582).
   It makes plain `cargo test -p <package>` build the unit tests, `tests/`, `examples/` and doc
   tests with the feature on. `required-features` on a test, example or bench target stays
   forbidden unless CI checks that the target runs, and nothing under this rule needs it.
4. Another package uses the environment only through `[dev-dependencies]`, with
   `features = ["testing"]`. A `[dependencies]` entry never enables `testing`, and production
   code never enables it.
5. **A shipped artifact is built by an invocation of its own**, with `cargo build` (or
   `cargo install`) and without `--all-targets`, `--tests`, `--examples`, `--benches` and
   `--all-features`, naming the features it needs. Those flags enable dev-dependency features or
   every feature, and V1 and V4c show they turn `testing` on in the binary. A check fails the build
   if a `testing` feature is enabled in a shipped binary's dependency tree. Today the container
   image's release binary comes from `cargo nextest archive --tests --all-features ... --release`
   (`Containerfile`, "Build Archive (release)"), which would carry every `testing` feature after
   rows 3 to 7; subissue 9 of #2482 changes that before the first move. Test builds keep
   `--tests`, and name the features they need instead of `--all-features` where practical.
6. The feature is test support, outside the package's semver guarantees. The package says so in
   its `Cargo.toml` (a comment on the feature) and in the module's documentation, and does not list
   `testing` in `[package.metadata.docs.rs]`. Every optional dependency behind `testing` is itself
   publishable and carries a `version`.

### Rule B: `test-helpers` stays generic

1. `torrust-tracker-test-helpers` keeps only protocol-agnostic helpers: `configuration`, `logging`
   and `random`. It depends on no protocol, core, server or client crate. `configuration` (the
   ephemeral configurations it builds) is the one workspace crate it may depend on.
2. A protocol-specific helper used by one package goes in that package's `tests/common/`. The
   `http` and `udp` modules, whose only consumer is the root crate's `tests/common/`, move there.
3. The move also removes `torrust-tracker-test-helpers` from the `wrappers` lists of the HTTP and
   UDP protocol crates in `deny.toml`, so `cargo deny check bans` rejects the dependency if it
   returns.

### Rule C: crate-private test helpers are private

A helper module used only by the crate's own unit tests is declared `#[cfg(test)] mod`, not
`pub mod`, and its content is `#[cfg(test)]`. It adds nothing to the public API. A crate that
needs the same helpers in its `tests/` keeps separate fixtures there (rule E).

### Rule D: mocks other packages use sit behind the `testing` feature

1. A `mockall` mock that another package's tests use belongs to the owning crate and is generated
   behind its `testing` feature: `#[cfg_attr(feature = "testing", automock)]`, with `mockall` an
   optional dependency enabled by `testing = ["dep:mockall"]`. The consumer enables `testing` from
   its `[dev-dependencies]` (rule A, point 4). `tracker-core` applies this to all four database
   traits, so the rule is the same for each of them.
2. A mock used only inside its own crate uses `#[cfg_attr(test, automock)]`, as `events` does, and
   `mockall` is a dev-dependency only.

### Rule E: per-package `tests/` folders stay

A helper with one consuming package lives in that package's `tests/common/` (or `tests/fixtures/`).
Nothing changes for the packages that already do this.

### Ownership map

The package whose server an environment starts owns that environment. Other packages use only its
`testing` feature, or write their own `tests/` code. No package, and no shared crate, hosts another
package's environment.

| Test support | Owner | Used today by | Others reach it through |
| --- | --- | --- | --- |
| UDP tracker environment (`Environment<Stopped/Running>`) | `udp-server` | Own `tests/`, own `src/` unit tests, `examples/udp_only_public_tracker.rs`, `axum-health-check-api-server` tests | `udp-server` `testing` feature |
| HTTP tracker environment | `axum-http-server` | Own `tests/`, `examples/http_only_public_tracker.rs`, `axum-health-check-api-server` tests | `axum-http-server` `testing` feature |
| REST API environment | `axum-rest-api-server` | Own `tests/`, `axum-health-check-api-server` tests | `axum-rest-api-server` `testing` feature |
| Health check API environment | `axum-health-check-api-server` | Own `tests/` | `axum-health-check-api-server` `testing` feature |
| `MockTorrentMetricsStore` and the other `tracker-core` database mocks | `tracker-core` | Own `src/`; `http-core` and `udp-core` unit tests (`MockTorrentMetricsStore`) | `tracker-core` `testing` feature |
| Ephemeral configurations, test logging, random strings | `test-helpers` (generic) | Seven packages | `[dev-dependencies]` entry |
| Protocol client helpers (`http`, `udp` modules today) | The root crate's `tests/common/` | The root crate's integration tests | Not shared |

### Alternatives Considered

| ID | Option | Strengths | Weaknesses | Outcome |
| --- | --- | --- | --- | --- |
| O1 | `testing` feature per package, optional dependencies, enabled from consumers' `[dev-dependencies]` | Code stays next to what it tests; nothing ships unless enabled; one compiled copy of the crate, so unit tests and other packages share types; no new member | Feature is public surface; optional dependencies look like normal edges to `cargo metadata`; a mechanism is needed for the crate's own targets | **Chosen** for patterns A and D, with O1c |
| O1a | O1 enabled by `--features testing` / `--all-features` only | No manifest changes; CI, pre-push, nightly and the publish workflow already pass `--all-features` | Plain `cargo test -p <package>` fails to compile `tests/`, `examples/` and doc tests; `cargo test --workspace` and `cargo test --doc --workspace` pass whenever another member enables the feature, so the gap shows only on per-package runs (V4) | **Rejected**: fails criterion 4 |
| O1b | O1 with `required-features = ["testing"]` on tests and examples | Builds without the feature | `cargo test -p <package> --all-targets` exits 0 and silently skips `tests/` and `examples/`; doc tests cannot carry `required-features` and still fail (V4); the maintainer bans it unless CI checks the targets run | **Rejected**: silent test loss, and fails criterion 4 for doc tests |
| O1c | O1 with a path-only self dev-dependency enabling the feature | Plain `cargo test -p <package>` runs unit tests, `tests/`, `examples/` and doc tests; `cargo package --no-verify` strips it on stable and 1.88.0 (V3, V4); `cargo machete --with-metadata` does not flag it | The packaged `.crate`'s `tests/` and `examples/` need `--features testing`; `cargo metadata` lists a self edge (`X -> X`, dev); a `version` on it breaks packaging (#2582) | **Chosen**: the only mechanism that meets criterion 4 |
| O2 | A support crate per package, used as a dev-dependency | Zero test code in the production crate; explicit ownership; works for `tests/`, `examples/` and other packages with plain `cargo test` | Four more members; unit tests that use it get a second copy of the crate and fail with E0308 (V2), and `udp-server`'s `src/` unit tests (`handlers/mod.rs`, `server/processor.rs`) use its environment; the crate's dev-dependency on it must be path-only even when the support crate is publishable, or neither can be packaged (V3); the published `.crate`'s tests cannot compile standalone | **Rejected**: fails criterion 3 for `udp-server`, and criterion 6 |
| O3 | `tests/common/` in the consuming package | Simplest; nothing leaves the consumer | Not shareable across packages | **Chosen** for single-consumer helpers (rules B and E) |
| O4 | A generic leaf `test-helpers` with protocol-agnostic helpers only | One home for cross-cutting utilities; no protocol coupling | Needs a rule against protocol helpers returning | **Chosen** for `configuration`, `logging`, `random` (rule B), with the `deny.toml` guard |
| O5 | `#[cfg(test)]` and `cfg_attr(test, ...)` for helpers and mocks used inside one crate | No API or dependency cost; `events` already does it | Crate-private: unusable by the crate's own `tests/` and by other packages | **Chosen** for patterns C and for mocks used only in their crate (rules C and D point 2) |
| O6 | Keep DEC-13: a public module compiled unconditionally | No work | Keeps nine test-only edges and test code in every build; its premise was wrong | **Rejected**: fails criterion 1 |
| O7 | Turn the environments into a supported start API | Their dependencies become legitimately normal; the examples get a real startup path; no feature or extra crate | Ephemeral ports, readiness waits, teardown and `add_torrent_peer` become public API to maintain and version; production startup lives in `src/bootstrap/jobs/` and has diverged, so it adds a second startup path; keeps `axum-rest-api-server`'s six edges as runtime coupling | **Rejected**: fails criterion 1 and adds a second startup path |
| O8 | One workspace-wide test-support crate for all environments | One member instead of four | Recreates the coupling hub: every consumer compiles every server; dev-dependency cycles with all four servers and O2's type mismatch (V2); no single owner | **Rejected**: fails criteria 2 and 3 |

### Relationship to DEC-13

DEC-13 chose public unconditional `src/testing/` modules. This ADR keeps the location (the module
stays in `src/testing/` of the owning package, so other packages can still import it) and replaces
the unconditional compilation with the `testing` feature. DEC-13's "existing pattern" premise,
`tracker-core/src/test_helpers.rs`, was a private `#[cfg(test)]` module and not an example of
public test infrastructure. DEC-18 in the
[decision log](../issues/open/1669-overhaul-packages/DECISIONS.md) records the amendment.

### Consequences

**Positive:**

- The nine test-only dependency edges leave the production builds of the four server packages,
  and `mockall` leaves `tracker-core`'s; the `Mock*` types and the environments leave the public
  API of a default build, and (once subissue 9 lands) the container image's binary.
- `test-helpers` loses its protocol and client dependencies, and a guard keeps them out.
- Each package's targets run with a plain `cargo test -p <package>`, so a contributor does not
  need `--all-features`.
- No new workspace member.

**Negative:**

- Every `testing` feature is public surface. It is declared outside semver, but a consumer outside
  the workspace can still enable it.
- `cargo metadata` reports an optional dependency as a normal edge and a self dev-dependency as a
  `X -> X` dev edge. Without a change, the coupling tool and the dependency diagram keep showing
  test-only edges as runtime edges, and #2482's AC8 cannot be met. Subissue 8 of #2482 (label
  optional and self edges in the coupling tool and diagram) is therefore required and runs before
  the first environment move merges.
- The packaged `.crate` of a package that uses O1c needs `--features testing` to run its `tests/`
  and `examples/`.
- A dev-dependency feature unifies across one Cargo invocation: `cargo test --workspace` builds
  `udp-server` with `testing` on because `axum-health-check-api-server` enables it. That is
  harmless for tests, and rule A point 5 keeps it out of production builds.
- The packaging check needs `cargo +stable`; Cargo 1.88.0 cannot run it (V3).
- The container build must change (subissue 9 of #2482): its test archive and its shipped binary
  are one `cargo nextest archive --tests --all-features --release` invocation today, and must
  become two. The cached third-party layers may need rebuilding once.

### What this ADR does not decide

- The exact Containerfile and CI changes that keep a `testing` feature out of shipped artifacts
  (subissue 9 of #2482 specifies them).
- Whether the two examples that start a tracker through an environment keep doing so, are labelled
  test-support demonstrations, or get a production-style startup. The subissues that move
  `udp-server` and `axum-http-server` decide it.
- The exact module paths after each move, and the order of the moves (the migration drafts of
  #2482 set them).
- Any version bump (#2583 compares the final API with the published one).
- Whether to adopt O7 later. It is rejected for now, not forbidden.

## Date

2026-10-21

## References

- [#2482 sub-EPIC](../issues/open/2482-1669-reorganize-shared-test-support/EPIC.md) and its
  [inventory](../issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md)
- [#2618 spec](../issues/open/2618-2482-decide-test-support-rules/ISSUE.md) and
  [manual verification evidence](../issues/open/2618-2482-decide-test-support-rules/manual-verification-evidence.md)
- DEC-13 and DEC-18 in the [decision log](../issues/open/1669-overhaul-packages/DECISIONS.md)
- #2582, the packaging failure caused by a self dev-dependency with a `version`
- [Independent package versioning](20260629000000_adopt_independent_package_versioning.md)
- [`deny.toml`](../../deny.toml), the layer boundary rules referenced by rule B
