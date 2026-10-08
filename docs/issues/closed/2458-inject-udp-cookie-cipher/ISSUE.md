---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p2
epic: null
github-issue: 2458
spec-path: docs/issues/closed/2458-inject-udp-cookie-cipher/ISSUE.md
branch: "2458-inject-udp-cookie-cipher"
related-pr: 2470
last-updated-utc: "2026-10-08 05:38"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - handle-secrets
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md
    - packages/udp-core/src/crypto/cookie_cipher.rs
    - docs/issues/closed/2458-inject-udp-cookie-cipher/performance-evidence.md
    - docs/issues/closed/2458-inject-udp-cookie-cipher/implementation-retrospective.md
    - docs/adrs/20261007085634_inject_the_udp_connection_cookie_cipher.md
    - docs/benchmarking.md
    - packages/udp-core/src/connection_cookie.rs
    - packages/udp-core/src/container.rs
    - packages/udp-core/src/lib.rs
    - src/bootstrap/app.rs
---

<!-- skill-link: create-issue -->

# Issue #2458 - Inject the UDP Connection-Cookie Cipher Instead of Global Static Keys

## Goal

Replace the process-global, `#[cfg(test)]`-switched cookie cipher with one key value created at
startup and injected into everything that issues or validates UDP connection IDs. Remove the
all-zero test key from production builds, and replace the startup seed check, which cannot detect
a zeroed cookie key, with guarantees that hold by construction and are pinned by tests.

## Background

### How connection IDs are protected today

BEP 15 connection IDs ("cookies") stop a client from announcing or scraping for an address it does
not control. `packages/udp-core/src/connection_cookie.rs` builds a cookie by adding a hash of the
client socket address to the issue time and encrypting the 8 bytes with Blowfish. `check` decrypts,
subtracts the fingerprint, and accepts the cookie if the recovered time is in the valid range. The
secrecy of the Blowfish key is the whole protection: anyone who knows the key can forge a valid
cookie for any address and time.

The key comes from global statics (`packages/udp-core/src/crypto/`):

- `ephemeral_instance_keys.rs` defines three `LazyLock` statics: `RANDOM_SEED` (32 random bytes),
  `RANDOM_CIPHER_BLOWFISH` (Blowfish keyed with **its own, separately drawn** random bytes), and
  `ZEROED_TEST_CIPHER_BLOWFISH` (Blowfish keyed with 32 zero bytes). None is behind `#[cfg(test)]`.
- `keys.rs` exposes a `Keeper` trait with two facades. `Instance` always returns the random seed
  and cipher. `Current` returns whatever the `detail_seed` and `detail_cipher` modules alias with
  `#[cfg(test)]` / `#[cfg(not(test))]` `pub use` lines: the zeroed ones in this crate's unit-test
  build, the random ones otherwise.
- `connection_cookie.rs` encrypts and decrypts with `Current::get_cipher_blowfish()`.
- `lib.rs` `initialize_static()` forces all three statics, **including the zeroed test cipher**, in
  production.

### The bug, in plain terms

1. Production safety rests on one `#[cfg(not(test))]` alias in `keys.rs::detail_cipher` pointing
   at the random cipher.
2. To catch a mistake there, `src/bootstrap/app.rs` runs `check_seed()` at startup in non-test
   builds. It asserts `Current::get_seed() == Instance::get_seed()` and otherwise panics with
   "maybe using zeroed seed in production!?".
3. But no cookie uses the seed. Before commit `e3562f069` ("udp: symmetric encrypted cookie",
   2024-11-18), connection IDs were a hash that included the seed, and nothing checked the seed at
   startup. That commit replaced the hash with the Blowfish cipher and, in the same patch, added
   `check_seed()`. The cipher is keyed with its own, separately drawn random bytes, despite its doc
   comment "The random cipher from the seed". So the new safeguard checked the old, now unused,
   seed instead of the new cipher.
4. Therefore, if the cipher alias were wrong (for example, a refactor that points
   `#[cfg(not(test))] CURRENT_CIPHER` at `ZEROED_TEST_CIPHER_BLOWFISH`), production would encrypt
   cookies with the public all-zero key, anyone could forge connection IDs, and `check_seed()`
   would still pass.

**Contract violated:** the startup check and its panic message promise to detect a zeroed key in
production; they cannot detect it for the key that matters. The doc comment "The random cipher
from the seed" is also false.

**Impact today:** nothing observable is wrong in a correctly built production binary; the alias
currently points at the random cipher. The defect is a safeguard that guards the wrong value, and a
public test key that is compiled into, and initialized in, every production binary.

### Root cause and chosen design

The root cause is design, not one line. The key is a hidden global dependency: `make`/`check` cannot
receive a key, so the only way to get deterministic tests was a compile-time swap, and the only
way to protect production from that swap was a runtime check of a proxy value. The maintainer
chose dependency injection (decision recorded in Issue #1348's discussion, 2026-10-06):

- one cipher value is created at startup from a cryptographically secure RNG;
- it is owned by the composition root and shared (for example `Arc`) with every component that
  issues or validates cookies;
- tests construct a cipher from an explicit fixed key, which exists only under `#[cfg(test)]`;
- there is no process-global key and no `cfg`-selected key facade.

Security assessment (also from the #1348 discussion): a `static` gives no protection that a
process-local immutable value lacks. Both live in the same address space, and safe Rust can mutate
neither. Injection removes the real risk (a public key reachable from production). It adds two
duties this spec makes explicit: never log the key, and guarantee one shared key per process.

## Scope

### In Scope

- A key type for the cookie cipher (name chosen during implementation, for example `CookieCipher`
  or `ConnectionIdKey`): built from a cryptographically secure RNG for production, from an
  explicit key for tests;
  implements a redacted `Debug` that prints no key material.
- `connection_cookie::make` and `check` (and the private `encode`/`decode`) take the key as a
  parameter instead of reading a global.
- Wire one shared key from the composition root through `UdpTrackerCoreContainer` (or the nearest
  composition point) to `ConnectService` (issues cookies), `AnnounceService` and `ScrapeService`
  (validate cookies), and to the `udp-server` announce/scrape handlers, which call `check` directly
  for the disabled-validation observability path
  (`packages/udp-server/src/handlers/announce.rs`, `handlers/scrape.rs`).
- Exclude the key from every `#[instrument]` span (`skip(...)`) on functions that receive it.
- Remove the `Keeper` trait, `Instance`/`Current` facades, `detail_seed`/`detail_cipher` aliases,
  and the `ZEROED_TEST_*` and `RANDOM_*` statics, or reduce them to the minimum still needed.
  Remove the vestigial `RANDOM_SEED` and `check_seed()` unless a remaining consumer is found. If
  one is found, record it and ask the maintainer.
- Update `initialize_static()` and every caller (`src/bootstrap/app.rs`,
  `packages/axum-rest-api-server/src/testing/environment.rs`, and others found by search).
- Update all affected tests, benches, and test environments in `udp-core` and `udp-server`.
- Correct the module documentation in `connection_cookie.rs` and `crypto/` to match the new design.
- Measure UDP performance before and after the change (see Performance Considerations), including
  a new Criterion benchmark for `connection_cookie::make` and `check`, and a repair of the existing
  `udp-core` `connect_once` benchmark, which does not await the code it claims to measure.
- Update the benchmarking documentation with what this issue learns, and add a benchmarking skill
  under `.github/skills/dev/benchmarking/` that points to it (task D1).

### Out of Scope

- Changing the cookie format, the cipher algorithm (Blowfish), or the fingerprint/time arithmetic.
- Sharing one key across several tracker processes (for example, behind a load balancer). The
  injected design makes it possible later; record it as a follow-up idea only.
- Key rotation. Whether the key is zeroed on drop is decided in the new ADR (see Architectural
  Decisions).
- The other `udp-core` findings in Issue #1348's `lessons.md` (error-message wording, scrape
  whitelist variants, `is_finite` doc drift).
- Test-coverage work owned by Issue #1348 beyond keeping existing tests passing and adding this
  issue's regression and contract tests.

## Architectural Decisions

- Related ADRs: none found for the cookie key design.
  [Adopt `secrecy` for sensitive values](../../../adrs/20260822094338_adopt_secrecy_for_sensitive_values.md)
  governs how the key is kept out of diagnostics:
  - The ADR asks to use `secrecy` directly. But `secrecy::SecretBox<T>` requires `T: Zeroize`, and
    the `blowfish` 0.10 cipher implements only `ZeroizeOnDrop`, behind its `zeroize` feature.
  - Wrapping just the raw key bytes in `SecretBox` would rerun the Blowfish key schedule on every
    `make`/`check`.
  - So the plan is a hand-written redacted `Debug` that prints `[REDACTED]`, the representation
    the ADR uses, with no accessor that exposes key material. R4 asserts that exact output and that
    a unique test key's bytes are absent, as the ADR asks of tests.
  - The new ADR records this deviation and decides whether to enable `blowfish`'s `zeroize`
    feature.
- ADRs to create: **Inject the UDP connection-cookie cipher instead of using global statics**.
  The issue chooses among meaningful alternatives (keep statics with a `cfg(test)`-gated test key;
  derive the cipher from the seed; injection), with consequences for every cookie consumer. It also
  records the secrecy deviation and the zeroize decision above.

## Design and Ownership Review

- **Interface:** `make(&key, fingerprint, issue_at)` and `check(&key, &cookie, fingerprint, range)`
  (parameter order chosen during implementation). The key type is cheap to share (`Arc`) and has no
  mutation API.
- **Ownership:** the application composition root creates exactly one key per process at startup
  and passes clones of one `Arc` to all consumers. No component creates its own key.
- **Lifetime:** the key lives for the process lifetime; there is no drop-path requirement beyond
  normal `Arc` release.
- **Readiness/deadlines:** not applicable (no asynchronous readiness involved).
- **Design-review checkpoint:** after the first vertical slice (key type + `make`/`check` +
  `ConnectService`/`AnnounceService` wired and tests green), stop for maintainer review before
  updating `udp-server`, benches, and the remaining callers.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. **Analysis:** done in this spec (Background). Re-verify with a search that no production code
   uses `RANDOM_SEED` for cryptography.
2. **Reproduction (done before maintainer review; outcome: Reproduced).** The wrong outcome is
   internal, so a temporary change was made at the nearest seam. The non-test `CURRENT_CIPHER`
   alias in `detail_cipher` was pointed at `ZEROED_TEST_CIPHER_BLOWFISH`, and the tracker started
   normally (`cargo run`): `check_seed()` did not panic. A disposable example then forged a
   connection ID with its own all-zero-key Blowfish and passed it to the production `check`
   in-process, which accepted it. A network announce via `UdpTrackerClient::send` was not sent
   before the fix; it is part of the post-fix recheck (M2b). Both changes were reverted. The code
   and output are in `manual-verification-evidence.md` section V1 (scenario M2a).
3. **Regression-test boundary:** see Regression Test Strategy. One maintained `compile_fail`
   doctest (R2) can be red against the current code; the other regression tests (R1, R3, R4) pin
   guarantees of the new design.
4. **Red evidence:** run R2 against the current code before any production change and record the
   red output (T3). R1, R3 and R4 cannot be red before the fix: R1 and R4 test a key type that does
   not exist yet, and R3 passes today because every service reads the same global cipher. For each
   of them, the substitute is mutate-then-restore on the new code, recorded in
   `manual-verification-evidence.md`. The Regression Test Strategy names each mutation.
5. **Fix:** the injection refactor (T4 to T7).
6. **Green and recheck:** rerun the tests and repeat the reproduction (T8): the forged connection
   ID must now be rejected, and the fixed test key must not be reachable from a production build,
   recorded in `manual-verification-evidence.md` (scenario M2b).

## Regression Test Strategy

The defect is a missing guarantee, so the regression tests pin the guarantees that replace the
seed check. Only R2 can be red against the current code. Each entry says how the test is shown to
guard the bug.

- **R2 (maintained `compile_fail` doctest, `udp-core`; written first):** a doctest that tries to
  reach the fixed test key from outside the crate. A doctest builds the library without
  `cfg(test)`, so it sees what a production build sees.
  - Before the fix it references `crypto::ephemeral_instance_keys::ZEROED_TEST_CIPHER_BLOWFISH`,
    which is public, so the doctest compiles and fails. A probe on 2026-10-06 confirmed this:
    rustdoc reported "Test compiled successfully, but it's marked `compile_fail`".
  - When the fix replaces that static with the new key type's test-only constructor, the doctest
    references the constructor instead and passes, because the constructor exists only under
    `#[cfg(test)]`. Mutate-then-restore confirms it: removing the `#[cfg(test)]` gate makes it fail.
  - A `compile_fail` doctest also passes on unrelated errors, such as a typo. Pin the expected
    error code (for example `compile_fail,E0599`), and keep a companion doctest that compiles with
    the production constructor.
- **R1 (unit, `udp-core`):** the production key constructor yields a key that does not encrypt a
  known block the way the all-zero key does, and two production keys differ (probabilistic,
  negligible collision chance). No pre-fix red run is possible because the constructor does not
  exist yet. Substitute: temporarily make the production constructor return the fixed key and
  record the failure.
- **R3 (container collaboration, `udp-core`):** a cookie issued by the container's
  `ConnectService` validates in the same container's `AnnounceService` and `ScrapeService`, proving
  one shared key. It passes today, because every service reads the same global cipher, so it
  guards the new wiring rather than the old bug. Substitute: temporarily give one service its own
  key and record the failure.
- **R4 (unit):** the key's `Debug` output is exactly the redacted representation and contains no
  bytes of a unique test key (see the secrecy ADR in Architectural Decisions). No pre-fix red run
  is possible
  because the key type does not exist yet. Substitute: temporarily derive `Debug` and record the
  failure.
- Existing cookie tests (`connection_cookie.rs`, including the make-then-check round trip
  `it_should_validate_a_valid_cookie` and the pinned-encoding test, which will now pin the encoding
  for the explicit fixed test key) keep passing with the key passed explicitly. Correction: this
  line first cited a round-trip `quickcheck` property; that property exists only on the #1348
  branch, not on `develop`.

## Performance Considerations

`make` runs for every UDP connect request, and `check` for every announce and scrape, so this
change touches the UDP hot path. The maintainer asked for the cost to be measured, not reasoned
about.

Expected cost: none measurable. Today each call reads the key through a `LazyLock` static, which
costs one atomic load per call. After the change it receives a reference to an already-built
cipher. The Blowfish key schedule still runs once per key, at startup, and the `zeroize` wipe runs
only when a key is dropped. A change that rebuilt the key schedule per call would be a large
regression; the benchmarks below would expose it.

Two instruments, following the issue #2314 and #2342 precedent and
[`docs/benchmarking.md`](../../../benchmarking.md):

- **Microbenchmark (precise):** a new Criterion benchmark in `udp-core` for
  `connection_cookie::make` and `connection_cookie::check`, plus the existing `connect_once`
  benchmark once it is repaired. Today `connect_once` passes the `async fn sync::connect_once` to
  `b.iter` without awaiting it, so Criterion times only the creation of a future. B1 makes it
  measure `ConnectService::handle_connect`, and records a before-and-after Criterion run that
  shows the change.
- **End-to-end load test (user-visible):** `aquatic_udp_load_test` against a release build started
  with `share/default/config/tracker.udp.benchmarking.toml`, with the #2342 load-test settings
  (connect and announce weights 50/50, scrape weight 1, 30-second runs, last 20 seconds
  summarized), five runs per measurement, a fresh tracker per run.

Method:

- P1 (baseline) runs on the code before any production change (B2). P2 (after) runs after T8 (B3),
  on the same machine with the same build profile, configuration, and load-test settings. The
  benchmark code may change between P1 and P2 only as far as the new `make`/`check` signatures
  require.
- Record the machine, toolchain, aquatic commit, commands, and results in issue-local
  `performance-evidence.md`.
- Pass rule (one-sided, as the #2342 retrospective recommends): the P2 mean responses per second is
  not below the lowest P1 run, and each P2 Criterion median is not above the highest of three P1
  Criterion medians. State the observed P1 noise, so readers know which regressions the
  measurement can detect. A failure is investigated, for example with
  [`docs/profiling.md`](../../../profiling.md), before the issue closes.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce and record | `manual-verification-evidence.md` records a reproduced production-mode safeguard bypass and independently forged accepted cookie. |
| T2 | DONE | ADR | [`20261007085634_inject_the_udp_connection_cookie_cipher.md`](../../../adrs/20261007085634_inject_the_udp_connection_cookie_cipher.md), indexed; includes the secrecy deviation and the zeroize decision. |
| T3 | DONE | Regression test R2, red before the fix | Write the R2 `compile_fail` doctest against the current code and record its red run in `manual-verification-evidence.md`. No production change in this task. |
| B1 | DONE | Add the cookie benchmark and repair `connect_once` | New Criterion benchmark for `make` and `check` in `udp-core`; `connect_once` awaits the connect it measures, with a Criterion run before and after the repair recorded. No production change. |
| B2 | DONE | Record the performance baseline (P1) | Three Criterion runs and five `aquatic_udp_load_test` runs on the code before any production change, in `performance-evidence.md`. |
| T4 | DONE | Fix: key type and explicit-key `make`/`check` | Key type with redacted `Debug`; fixed test key only under `#[cfg(test)]`, which turns R2 green; R1 and R4 added, with their mutate-then-restore red runs recorded; existing cookie tests pass with an explicit test key. |
| T5 | DONE | Fix: wire `udp-core` services and container | One shared key; R3 added, with its mutate-then-restore red run recorded. **Design-review checkpoint** with the maintainer. |
| T6 | DONE | Fix: wire `udp-server` and remaining callers | No change left: the constructor changes in T5 forced the `udp-server` handlers, tests, and both benchmarks to change there (see the progress log). The `axum-rest-api-server` test environment and `src/bootstrap/app.rs` changed in T7 with `initialize_static()`. |
| T7 | DONE | Fix: remove the global keys | `Keeper`, facades, aliases, statics, `check_seed()`, and stale `initialize_static()` steps removed; docs corrected. |
| T8 | DONE | Green and recheck | R1 to R4 and the existing tests green, automatic checks, manual recheck (M1, M2b, M3), and acceptance review. |
| B3 | DONE | Record the performance after the fix (P2) | Same measurements as B2 on the finished code; comparison against the one-sided pass rule in `performance-evidence.md`. |
| D1 | DONE | Update the benchmarking docs and add a benchmarking skill | `docs/benchmarking.md` and related docs updated with what this issue learned (for example the benchmark levels and tools, criteria for choosing one, how to record evidence, stale package names); a new skill under `.github/skills/dev/benchmarking/` that points to them. Done last. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Reproduction evidence and ADR | One `docs(...)` commit after maintainer review. |
| T3 | R2 red-run evidence | One `docs(issues)` commit with the recorded red output. The red doctest itself is not committed here, because pre-commit runs `cargo test --doc` and would reject it; it is committed in T4, which makes it green. |
| B1 | Cookie benchmark and `connect_once` repair | One `perf(udp-core)` commit, before any production change. |
| B2 | Performance baseline | One `docs(issues)` commit with `performance-evidence.md`, before any production change. |
| T4 | Key type, explicit-key cookie API, R2, and R1/R4 | Commit after focused validation; note that callers are updated in the same commit only as far as needed to compile. |
| T5 | `udp-core` wiring and container test | Commit after the design-review checkpoint. |
| T6 | `udp-server` and remaining callers | Commit after focused validation. |
| T7 | Removal of the global keys and doc corrections | Commit after focused validation. |
| T8 | Verification evidence | Commit after maintainer review. |
| B3 | Performance after the fix | One `docs(issues)` commit. |
| D1 | Benchmarking docs and skill | Separate commits from the issue's own changes, made at the end: one `docs(...)` commit for the documentation and one for the skill. |

All commits use Conventional Commits with the narrow scope (`udp-core`, `udp-server`, ...) and are
GPG signed. Every test increment follows the `write-unit-test` skill's prose-first
Arrange-Act-Assert review.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted (hand-off draft in `.tmp/inject-udp-cookie-cipher/ISSUE.md`)
- [x] Draft moved to `docs/issues/drafts/inject-udp-cookie-cipher/ISSUE.md`
- [x] Reproduction attempted and classified before review
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation (PR #2461)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Manual verification scenarios executed and recorded in `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-10-06 13:38 UTC - GitHub Copilot (Issue #1348 session) - Drafted from the `crypto/keys.rs`
  file test plan finding in Issue #1348 and the maintainer's decision to use injection. Hand-off
  to a separate agent: `.tmp/inject-udp-cookie-cipher/HANDOFF.md`.
- 2026-10-06 16:14 UTC - Copilot - Independently reproduced the safeguard bypass and accepted a cookie forged with the public all-zero key after a reversible production-mode alias mutation. Evidence: `manual-verification-evidence.md` V1.
- 2026-10-06 16:17 UTC - Copilot - Created GitHub issue #2458 and moved the approved draft and reproduction evidence to `docs/issues/open/2458-inject-udp-cookie-cipher/`.
- 2026-10-06 16:26 UTC - Copilot - Renamed the specification branch to `2458-inject-udp-cookie-cipher-spec`, reserving `2458-inject-udp-cookie-cipher` for implementation.
- 2026-10-06 18:13 UTC - Copilot - Review feedback (`review-finding:pr-2461-f13`): added T3, a regression test that is red before the fix, and split the fix from green-plus-recheck. A temporary `compile_fail` doctest probe that referenced `ZEROED_TEST_CIPHER_BLOWFISH` from outside the crate compiled on today's code, so rustdoc reported it as failing. The probe was reverted; T3 records the red run officially.
- 2026-10-06 18:26 UTC - Copilot - Logged all PR #2460/#2461 review fixes, as `review-finding:pr-2461-f19` asked. The audit is `docs/pr-reviews/pr-2461-review/PR-REVIEW.md`; findings are cited by audit ID.
  - Spec changes:
    - Corrected the `check_seed()` history (F8).
    - Split M2 into M2a and M2b (F2, F3).
    - Rewrote Bug-Fix step 2 as completed (F15).
    - Planned the R2 red run and split the fix and recheck tasks (F13, logged above).
    - Added the disposable-script record (F12).
    - Gave pending scenarios planned evidence sections (F6).
    - Linked the secrecy ADR and planned the redacted `Debug` (F17).
    - Restored template checkpoints (F9).
  - Evidence changes:
    - Recorded the reproducer verbatim with a rerun and a control run (F1).
    - Removed a duplicate section and quoted the timestamp (F4, F5).
    - Named the `develop` commit under test (F11).
    - Corrected the alias diff hunk (F21).
- 2026-10-07 08:56 UTC - Copilot - The spec-only PR #2461 merged as `develop` `7836471b3`. Implementation started on branch `2458-inject-udp-cookie-cipher`. Maintainer decision for the ADR: enable the `blowfish` `zeroize` feature, so the key schedule is wiped on drop. Design finding while reading the composition code: the production `AppContainer` builds `UdpTrackerCoreServices` once and shares it with every UDP tracker instance (the REST API takes only its ban service and statistics repository), so the shared key belongs there. The standalone constructors (`UdpTrackerCoreContainer::initialize`, `initialize_from_tracker_core`, and the REST API runtime adapter's `initialize`) are their own composition roots and each create one key; the guarantee is one key per composition root, which is one key per process in the tracker binary.
- 2026-10-07 09:02 UTC - Copilot - T2: wrote the ADR (`docs/adrs/20261007085634_inject_the_udp_connection_cookie_cipher.md`), with the key type `CookieCipher`, ownership in `UdpTrackerCoreServices`, the secrecy deviation, and the `zeroize` decision. T3: ran the R2 `compile_fail` doctest (pinned `E0432`) and its compiling companion against `develop` `7836471b3`; R2 is red ("Test compiled successfully, but it's marked `compile_fail`"). The temporary doctests were reverted. Evidence: `manual-verification-evidence.md`, "R2 - Red Before the Fix (T3)".
- 2026-10-07 09:12 UTC - Copilot - Maintainer review: the ADR is approved, but the change touches the UDP hot path and the spec had no performance task. Added Performance Considerations, tasks B1-B3, AC7, and the issue #2314/#2342 method (a Criterion benchmark of the changed functions plus an `aquatic_udp_load_test` baseline before any production change, and a one-sided pass rule). While surveying the benchmarks, found that `udp-core`'s `connect_once` benchmark never awaits the connect it claims to measure; B1 repairs it. The maintainer also asked to fold a benchmarking documentation update and a new skill under `.github/skills/dev/benchmarking/` into this issue (D1, AC8), committed separately at the end. Survey findings for D1: `docs/benchmarking.md` lists four benchmark types but omits the `udp-core`, `udp-server`, and `http-core` Criterion benchmarks, the persistence benchmark, and the workflow benchmarks, has no criteria for choosing one, and, like `contrib/dev-tools/benches/run-benches.sh`, uses the old package name `torrust-tracker-torrent-repository`.
- 2026-10-07 09:27 UTC - Copilot - B1: added `connection_cookie_benchmark` (`make`, `check`) and repaired `connect_once`, which reported 3.49 ns before and 57.6 ns after awaiting the connect. B2: recorded the P1 baseline on the code before any production change, in `performance-evidence.md`: load-test mean 152197.78 responses/s (lowest run 142155.29, spread about 8.8%) and highest Criterion medians `make` 49.93 ns, `check` 48.30 ns, `connect_once` 59.10 ns, on a shared desktop whose load rose during the runs.
- 2026-10-07 09:37 UTC - Copilot - T4: added `CookieCipher` (redacted `Debug`, no key accessor, `zeroize` on drop), passed it explicitly to `make`/`check`, and removed the public zero-key static and the cfg-switched cipher alias. Callers pass a transitional `RANDOM_CIPHER_BLOWFISH` static until T5-T7; `udp-server` handler tests issue connection IDs through one named helper, `connection_id_issued_to`, so T6 changes one place. R1, R2, and R4 are green, and mutate-then-restore made each red (evidence: "R1, R2, and R4 - Green After the Key Type"). Finding: rustdoc checks a `compile_fail` doctest's error code only on nightly, so R2's pinned `E0599` is enforced by the nightly CI job, not by the stable pre-commit gate; noted for D1.
- 2026-10-07 09:44 UTC - Copilot - T5: `UdpTrackerCoreServices` creates the one `Arc<CookieCipher>` and the container passes it to the connect, announce, and scrape services as a required constructor argument. R3 (two container collaboration tests) is green, and giving the announce or the scrape service its own key made the matching test red (evidence: "R3 - Shared Key Across the Container's Services"). Design change for the checkpoint: instead of exposing the key from the container to `udp-server`, `AnnounceService::authenticate` and `ScrapeService::authenticate` became public and the handlers' disabled-validation path calls them, so `udp-server` production code never holds the key; the ADR still describes the container exposing it. Because the constructors changed, the compiler required the `udp-server` test helpers and both benchmarks to change in this step: `udp-server` tests share one test-only key through `test_cookie_cipher`. That leaves T6 with no remaining wiring; only `initialize_static` still uses the transitional static, which T7 removes. **Design-review checkpoint reached; waiting for the maintainer.**
- 2026-10-07 09:50 UTC - Copilot - The maintainer reviewed the code and approved the T4/T5 design, including the public `authenticate` methods and the shared test-only key in `udp-server` tests. The ADR was updated to match.
- 2026-10-07 10:07 UTC - Copilot - T7: deleted `crypto::keys` and `crypto::ephemeral_instance_keys`, `check_seed()`, and `torrust_tracker_udp_core::initialize_static()` with its callers; corrected the `udp-server` crate docs (connection IDs described as a hash with a secret seed), the bootstrap docs, and `src/AGENTS.md`. A search found no consumer of the seed besides `check_seed()`. The `udp-core`, `udp-server`, `axum-rest-api-server`, and root-crate tests passed (503 tests). T8 so far: M1, M2b, and M3 done (evidence V2-V4); the forged connection ID is rejected by the running tracker and the test key cannot be named from a production build. B3: P2 measured after waiting for other sessions' compilations to finish; all four measurements meet the one-sided rule (load-test mean 154260.85 against a limit of 142155.29; `make`, `check`, and `connect_once` medians 44.77, 46.99, and 59.05 ns against 49.93, 48.30, and 59.10 ns). `linter all` passes. AC1-AC7 reviewed against the evidence and marked done; AC8 waits for D1. Remaining: D1, pre-push checks, and the completion review.
- 2026-10-07 10:17 UTC - Copilot - D1: updated `docs/benchmarking.md` (benchmark levels and tools, choosing a benchmark, before-and-after evidence, checking that a benchmark measures something, and the `http-core` announce benchmark's defect (a future that is never awaited) as a known defect), added the `run-benchmarks` skill under `.github/skills/dev/benchmarking/`, fixed the stale package name in `run-benches.sh`, and added the nightly-only `compile_fail` error-code check to the `write-unit-test` skill, each in its own commit. AC8 done. Completion review: created `implementation-retrospective.md`. Remaining: the pre-push checks, which run on push, and the maintainer's review of the T8 evidence before the pull request.
- 2026-10-07 10:25 UTC - Copilot - Independent Task Reviewer: REVIEW PASSED, AC1-AC8 pass, no blockers (report: `agent-review-reports.md`). Applied should-fix S1 (AC6 and the Regression Test Strategy cited a round-trip `quickcheck` property that exists only on the #1348 branch; they now cite `it_should_validate_a_valid_cookie`) and nit N3 (removed the template instruction left in Implementation Completion Review). Nit N1 (wiping the raw 32-byte key array in `random()`) is left for the maintainer, because it needs a direct `zeroize` dependency; N2, N4, and N5 need no action. The maintainer chose to review on GitHub.
- 2026-10-07 10:53 UTC - Copilot - Rebased onto `develop` twice (#2438 changed the #2411 EPIC, whose citations of the deleted key file are now updated; #2465 changed only docs), passed the pre-push checks, and opened PR #2470.
- 2026-10-07 13:19 UTC - Copilot - The maintainer accepted the Task Reviewer's nit N1: `CookieCipher::random()` now fills a `zeroize::Zeroizing` buffer in place, so the raw key bytes are wiped once the cipher is built. `zeroize` 1.9.1 (latest stable) became a direct `udp-core` dependency with default features off; it was already in the graph through `secrecy` and `blowfish`, and the lock file moves from 1.9.0. The wipe is not tested, because observing freed memory would be undefined behavior. A new mutation, filling a throwaway buffer instead of the key, left the all-zero key and failed both R1 tests (evidence: "R1 After Wiping the Raw Key Bytes"). Performance is not re-measured: `random()` runs once per composition root, not per request. ADR updated.
- 2026-10-07 15:03 UTC - Copilot - PR #2470 review rounds 1 and 2 (Copilot and da2ce7): rebased onto `develop` (the #2411 EPIC conflicted only in its `last-updated-utc` stamp); made `UdpTrackerCoreServices::cookie_cipher` `pub(crate)` with a `compile_fail,E0616` doctest, so code outside `udp-core` cannot read the key (red when the field is made `pub` again; evidence: "Key Kept Inside `udp-core`"); corrected the `src/AGENTS.md` bootstrap diagram, which still said the global-services step set up a crypto seed; and reworded the `write-unit-test` note on stable rustdoc. The generated 2026-10-06 workspace coupling report on `develop` still lists `crypto::keys` and `initialize_static`; it is a dated snapshot and stays unchanged. Audit: `docs/pr-reviews/pr-2470-review/PR-REVIEW.md`.
- 2026-10-08 05:38 UTC - Copilot - PR #2470 merged on 2026-10-08 05:34 UTC as `develop` `44a280c0a` after da2ce7 approved (rounds 1 to 4 are audited in `docs/pr-reviews/pr-2470-review/PR-REVIEW.md`), which closed issue #2458. T8 done. Archived this specification to `docs/issues/closed/` and pointed the live references (the ADR, `docs/benchmarking.md`, the `run-benchmarks` skill, and the PR #2461 and #2470 audit records) at the new path; progress-log entries that record the old path stay as history. Issue #1348 can resume and adapt its `udp-core` tests to the new `make`/`check` and service constructors.

## Acceptance Criteria

- [x] AC1: No production code path can obtain a cookie key other than the one created at startup
  from a cryptographically secure RNG; the fixed test key is unreachable from a production build
  (R1, R2).
- [x] AC2: Every component that issues or validates connection IDs uses the same key instance (R3).
- [x] AC3: The key never appears in logs or `Debug` output (R4 and `skip` on every instrumented
  function that receives it).
- [x] AC4: The global `Keeper`/`Current`/`Instance` facades, the `cfg`-switched aliases, the
  `ZEROED_TEST_*`/`RANDOM_*` statics, and `check_seed()` are removed, or each survivor is justified
  in the progress log.
- [x] AC5: The reproduction from Bug-Fix Process step 2 is repeated after the fix and recorded.
- [x] AC6: Cookie format and behavior are unchanged: existing cookie tests, including the
  make-then-check round trip, pass with an explicit key.
- [x] AC7: UDP performance is measured before (P1) and after (P2) the change on the same machine,
  and P2 meets the one-sided pass rule in Performance Considerations, with the observed noise stated.
- [x] AC8: The benchmarking documentation reflects what this issue learned, and a benchmarking skill
  under `.github/skills/dev/benchmarking/` points to it.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation (module docs, ADR) is updated

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server`
- `cargo test --doc --workspace`
- `linter all`
- `./contrib/dev-tools/git/hooks/pre-commit.sh` before every commit; pre-push checks before push

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Normal client flow | Start the tracker (`cargo run`); `cargo run -p torrust-tracker-client --bin tracker_client -- udp announce udp://127.0.0.1:6969/announce <info-hash>` | Connect then announce succeed | DONE | `manual-verification-evidence.md` section V2 |
| M2a | Forged connection ID, before the fix | With the non-test cipher alias temporarily pointed at the all-zero cipher, start the tracker (`cargo run`) and run a disposable example that forges a connection ID with its own all-zero-key Blowfish and passes it to the production `check` | The tracker starts (`check_seed()` passes) and the forged connection ID is accepted | DONE | `manual-verification-evidence.md` section V1 |
| M2b | Forged connection ID, after the fix | Repeat M2a against the fixed code: try to wire the fixed test key into a production build, and send an announce with an all-zero-key connection ID via `UdpTrackerClient::send` to the running tracker | The test key cannot be referenced in a production build, and the forged connection ID is rejected | DONE | `manual-verification-evidence.md` section V3 |
| M3 | Key not logged | Run the tracker with debug/trace logging, exercise connect and announce, and search the logs for key material | No key bytes in logs | DONE | `manual-verification-evidence.md` section V4 |

Record the toolchain for every validation command result (for example, `nightly Rust toolchain`
for `cargo +nightly fmt --all -- --check`).

### Disposable Verification Scripts

- **`forge_zeroed_cookie` (scenario M2a, evidence V1).** A temporary Rust example at
  `packages/udp-core/examples/forge_zeroed_cookie.rs`. It forges a connection ID with its own
  all-zero-key Blowfish and passes it to the production `check`. It ran only together with a
  temporary edit of the non-test cipher alias in `packages/udp-core/src/crypto/keys.rs`.
  - **Why not a maintained test:** it proves the bug only in a production build whose cipher alias
    has been deliberately broken. A maintained test cannot keep production code in that state. The
    maintained regression tests R1 to R4 protect the guarantees instead.
  - **Location and retention:** the file was removed after each run. Its source and the alias diff
    are kept verbatim in `manual-verification-evidence.md` section V1, inside this issue folder, so
    reviewers can inspect and repeat the run. The implementer of this issue may recreate it from
    that record for the M2b recheck and must remove it again before committing.
  - **Language:** Rust, so no Python rationale is needed.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | DONE | R1, R2 (stable and nightly), M2b compile probe |
| AC2 | DONE | R3, M1 |
| AC3 | DONE | R4, M3; `make`, `check`, `encode`, and `decode` skip the key |
| AC4 | DONE | T7 commit: `crypto::keys` and `crypto::ephemeral_instance_keys` deleted, `check_seed()` and `initialize_static()` removed; no survivors |
| AC5 | DONE | M2a (V1, before), M2b (V3, after) |
| AC6 | DONE | `connection_cookie` tests, including the pinned encoding (`it_should_make_a_connection_cookie`) and the round trip (`it_should_validate_a_valid_cookie`), pass with the explicit fixed key |
| AC7 | DONE | `performance-evidence.md` Comparison: all four measurements pass |
| AC8 | DONE | `docs/benchmarking.md` (levels and tools, choosing a benchmark, before-and-after evidence, checking that a benchmark measures something, known defects) and `.github/skills/dev/benchmarking/run-benchmarks/SKILL.md` |

## Risks and Trade-offs

- **Wide signature change.** Every cookie consumer changes. Mitigation: the design-review
  checkpoint after T5; keep the key a single cheap `Arc` parameter.
- **Two keys by accident.** A component that builds its own key breaks every announce. Mitigation:
  R3 and a single construction site.
- **Logging leak.** Mitigation: redacted `Debug`, `skip` in spans, R4, M3.
- **Conflicts with Issue #1348.** That issue's branch adds tests in `udp-core` that call
  `make`/`check` and construct the services. Mitigation: #1348 is paused until this merges, and the
  #1348 author adapts those tests during their rebase. Do not edit #1348's issue folder.

## Implementation Completion Review

- Retrospective: created, [`implementation-retrospective.md`](implementation-retrospective.md), because the implementation changed the planned design (the key does not reach `udp-server`), left T6 empty, and produced reusable lessons (performance planning, `compile_fail` toolchains, broken benchmarks, evidence times).

## References

- Origin: Issue #1348 (`udp-core` package tests), `crypto/keys.rs` file test plan.
- Commit `e3562f069` "udp: symmetric encrypted cookie" (2024-11-18) introduced the cipher and
  `check_seed()` in the same patch. The seed itself (`RANDOM_SEED`) dates from commit `873293a6f`
  "crypto: ephemeral instance seeds with keepers" (2022-09-21).
- BEP 15: <https://www.bittorrent.org/beps/bep_0015.html>
