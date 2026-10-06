---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 2458
spec-path: docs/issues/open/2458-inject-udp-cookie-cipher/ISSUE.md
branch: "2458-inject-udp-cookie-cipher-spec"
related-pr: null
last-updated-utc: "2026-10-06 16:26"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - packages/udp-core/src/crypto/keys.rs
    - packages/udp-core/src/crypto/ephemeral_instance_keys.rs
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

### Out of Scope

- Changing the cookie format, the cipher algorithm (Blowfish), or the fingerprint/time arithmetic.
- Sharing one key across several tracker processes (for example, behind a load balancer). The
  injected design makes it possible later; record it as a follow-up idea only.
- Key rotation and zeroing the key on drop.
- The other `udp-core` findings in Issue #1348's `lessons.md` (error-message wording, scrape
  whitelist variants, `is_finite` doc drift).
- Test-coverage work owned by Issue #1348 beyond keeping existing tests passing and adding this
  issue's regression and contract tests.

## Architectural Decisions

- Related ADRs: none found for the cookie key design. Search `docs/adrs/` before starting.
- ADRs to create: **Inject the UDP connection-cookie cipher instead of using global statics**.
  The issue chooses among meaningful alternatives (keep statics with a `cfg(test)`-gated test key;
  derive the cipher from the seed; injection), with consequences for every cookie consumer.

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
2. **Reproduction (not yet attempted; must be done before maintainer review of this spec).** The
   wrong outcome is internal, so use a temporary change at the nearest seam: in a non-test build,
   point `detail_cipher`'s `#[cfg(not(test))] CURRENT_CIPHER` at `ZEROED_TEST_CIPHER_BLOWFISH`,
   build and start the tracker (`cargo run`), and observe that `check_seed()` does not panic. Then,
   from a separate Rust snippet or test that encrypts with the all-zero Blowfish key, forge a
   connection ID for the client address and send an announce with `UdpTrackerClient::send`
   (`packages/tracker-client`), skipping connect. Classify the outcome (Reproduced / Trigger only /
   Infeasible), record it verbatim in `manual-verification-evidence.md`, and revert the change.
3. **Regression-test boundary:** unit tests in `udp-core` (see Regression Test Strategy).
4. **Red evidence:** write the regression tests first where practical; for tests that cannot
   compile against the old API, use mutate-then-restore on the new code (for example, temporarily
   wire a fixed key into production composition) and record the failure.
5. **Fix:** the injection refactor.
6. **Green and recheck:** rerun the tests and repeat the reproduction: the forged connection ID
   must now be rejected (or the forging path must no longer be expressible in a production build),
   recorded in `manual-verification-evidence.md`.

## Regression Test Strategy

The defect is a missing guarantee, so the regression tests pin the guarantees that replace the
seed check:

- **R1 (unit, `udp-core`):** the production key constructor yields a key that does not encrypt a
  known block the way the all-zero key does, and two production keys differ (probabilistic,
  negligible collision chance).
- **R2 (compile-time):** the fixed test-key constructor and constant are under `#[cfg(test)]` (or a
  test-only feature not enabled in production). Record the evidence that a production build cannot
  reference them, for example a failed `cargo build` after a temporary reference.
- **R3 (container collaboration, `udp-core`):** a cookie issued by the container's
  `ConnectService` validates in the same container's `AnnounceService` and `ScrapeService`, proving
  one shared key.
- **R4 (unit):** the key's `Debug` output contains no key bytes.
- Existing cookie tests (`connection_cookie.rs`, including the round-trip `quickcheck` property and
  the pinned-encoding test, which will now pin the encoding for the explicit fixed test key) keep
  passing with the key passed explicitly.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce and record | `manual-verification-evidence.md` records a reproduced production-mode safeguard bypass and independently forged accepted cookie. |
| T2 | TODO | ADR | ADR for injection versus the alternatives, in `docs/adrs/`, indexed. |
| T3 | TODO | Key type and explicit-key `make`/`check` | Key type with redacted `Debug`; R1, R2, R4 recorded red then green; existing cookie tests pass with an explicit test key. |
| T4 | TODO | Wire `udp-core` services and container | One shared key; R3 red then green. **Design-review checkpoint** with the maintainer. |
| T5 | TODO | Wire `udp-server` and remaining callers | Handlers, launcher/environment, benches, `axum-rest-api-server` test environment, `src/bootstrap/app.rs`. |
| T6 | TODO | Remove the global keys | `Keeper`, facades, aliases, statics, `check_seed()`, and stale `initialize_static()` steps removed; docs corrected. |
| T7 | TODO | Verification and recheck | Automatic checks, manual recheck, acceptance review. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Reproduction evidence and ADR | One `docs(...)` commit after maintainer review. |
| T3 | Key type, explicit-key cookie API, and its tests | Commit after focused validation; note that callers are updated in the same commit only as far as needed to compile. |
| T4 | `udp-core` wiring and container test | Commit after the design-review checkpoint. |
| T5 | `udp-server` and remaining callers | Commit after focused validation. |
| T6 | Removal of the global keys and doc corrections | Commit after focused validation. |
| T7 | Verification evidence | Commit after maintainer review. |

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
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-10-06 13:38 UTC - GitHub Copilot (Issue #1348 session) - Drafted from the `crypto/keys.rs`
  file test plan finding in Issue #1348 and the maintainer's decision to use injection. Hand-off
  to a separate agent: `.tmp/inject-udp-cookie-cipher/HANDOFF.md`.
- 2026-10-06 16:14 UTC - Copilot - Independently reproduced the safeguard bypass and accepted a cookie forged with the public all-zero key after a reversible production-mode alias mutation. Evidence: `manual-verification-evidence.md` V1.
- 2026-10-06 16:17 UTC - Copilot - Created GitHub issue #2458 and moved the approved draft and reproduction evidence to `docs/issues/open/2458-inject-udp-cookie-cipher/`.
- 2026-10-06 16:26 UTC - Copilot - Renamed the specification branch to `2458-inject-udp-cookie-cipher-spec`, reserving `2458-inject-udp-cookie-cipher` for implementation.

## Acceptance Criteria

- [ ] AC1: No production code path can obtain a cookie key other than the one created at startup
  from a cryptographically secure RNG; the fixed test key is unreachable from a production build
  (R1, R2).
- [ ] AC2: Every component that issues or validates connection IDs uses the same key instance (R3).
- [ ] AC3: The key never appears in logs or `Debug` output (R4 and `skip` on every instrumented
  function that receives it).
- [ ] AC4: The global `Keeper`/`Current`/`Instance` facades, the `cfg`-switched aliases, the
  `ZEROED_TEST_*`/`RANDOM_*` statics, and `check_seed()` are removed, or each survivor is justified
  in the progress log.
- [ ] AC5: The reproduction from Bug-Fix Process step 2 is repeated after the fix and recorded.
- [ ] AC6: Cookie format and behavior are unchanged: existing cookie tests, including the
  round-trip property, pass with an explicit key.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation (module docs, ADR) is updated

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server`
- `cargo test --doc --workspace`
- `linter all`
- `./contrib/dev-tools/git/hooks/pre-commit.sh` before every commit; pre-push checks before push

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Normal client flow | Start the tracker (`cargo run`); `cargo run -p torrust-tracker-client --bin tracker_client -- udp announce udp://127.0.0.1:6969/announce <info-hash>` | Connect then announce succeed | TODO | Not yet recorded |
| M2a | Forged connection ID, before the fix | With the non-test cipher alias temporarily pointed at the all-zero cipher, start the tracker (`cargo run`) and run a disposable example that forges a connection ID with its own all-zero-key Blowfish and passes it to the production `check` | The tracker starts (`check_seed()` passes) and the forged connection ID is accepted | DONE | `manual-verification-evidence.md` section V1 |
| M2b | Forged connection ID, after the fix | Repeat M2a against the fixed code: try to wire the fixed test key into a production build, and send an announce with an all-zero-key connection ID via `UdpTrackerClient::send` to the running tracker | The test key cannot be referenced in a production build, and the forged connection ID is rejected | TODO | Not yet recorded |
| M3 | Key not logged | Run the tracker with debug/trace logging, exercise connect and announce, and search the logs for key material | No key bytes in logs | TODO | Not yet recorded |

Record the toolchain for every validation command result (for example, `nightly Rust toolchain`
for `cargo +nightly fmt --all -- --check`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | R1, R2 |
| AC2 | TODO | R3 |
| AC3 | TODO | R4, M3 |
| AC4 | TODO | Diff and progress log |
| AC5 | TODO | M2a (before), M2b (after) |
| AC6 | TODO | Cookie test run |

## Risks and Trade-offs

- **Wide signature change.** Every cookie consumer changes. Mitigation: the design-review
  checkpoint after T4; keep the key a single cheap `Arc` parameter.
- **Two keys by accident.** A component that builds its own key breaks every announce. Mitigation:
  R3 and a single construction site.
- **Logging leak.** Mitigation: redacted `Debug`, `skip` in spans, R4, M3.
- **Conflicts with Issue #1348.** That issue's branch adds tests in `udp-core` that call
  `make`/`check` and construct the services. Mitigation: #1348 is paused until this merges, and the
  #1348 author adapts those tests during their rebase. Do not edit #1348's issue folder.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`
  if the implementation invalidates assumptions here; otherwise add a progress-log entry saying why
  not.

## References

- Origin: Issue #1348 (`udp-core` package tests), `crypto/keys.rs` file test plan.
- Commit `e3562f069` "udp: symmetric encrypted cookie" (2024-11-18) introduced the cipher and
  `check_seed()` in the same patch. The seed itself (`RANDOM_SEED`) dates from commit `873293a6f`
  "crypto: ephemeral instance seeds with keepers" (2022-09-21).
- BEP 15: <https://www.bittorrent.org/beps/bep_0015.html>
