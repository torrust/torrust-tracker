---
semantic-links:
  skill-links:
    - write-markdown-docs
    - fix-bug
    - run-benchmarks
  related-artifacts:
    - docs/issues/closed/2458-inject-udp-cookie-cipher/ISSUE.md
    - docs/issues/closed/2458-inject-udp-cookie-cipher/manual-verification-evidence.md
    - docs/issues/closed/2458-inject-udp-cookie-cipher/performance-evidence.md
    - docs/adrs/20261007085634_inject_the_udp_connection_cookie_cipher.md
---

# Implementation Retrospective — Inject the UDP Connection-Cookie Cipher

## Purpose

Record evidence-based process improvements discovered while implementing issue #2458. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

Connection IDs are encrypted with one `CookieCipher` per composition root, created from a
cryptographically secure random number generator by `UdpTrackerCoreServices` and injected into the
connect, announce, and scrape services. The all-zero test key exists only under `#[cfg(test)]`, the
key has a redacted `Debug` and is wiped on drop, and the global statics, the `Keeper` facades, and
`check_seed()` are gone. Regression tests R1-R4 pass and were each seen to fail against a
reintroduced bug; the running tracker rejects a connection ID forged with the all-zero key; and UDP
performance shows no regression (load-test mean 154260.85 against a baseline of 152197.78
responses/s). Evidence: `manual-verification-evidence.md` and `performance-evidence.md`.

## What Went Well

1. **Reproducing before review.** The V1 reproduction gave a like-for-like recheck (V3) that showed
   the fix at the same two seams plus the network.
2. **The design-review checkpoint after the first vertical slice.** The maintainer reviewed real
   code, not a plan, and approved a design change that kept the key out of `udp-server`.
3. **Measuring the baseline before any production change.** Writing the benchmark first exposed a
   broken benchmark (`connect_once` measured 3.49 ns for nothing) before it could produce a
   misleading comparison.
4. **Mutate-then-restore found a gap in a test that was believed to guard more than it did.** R2's
   pinned error code is not enforced on stable (M-R2c).

## What Changed During Implementation

- **The plan had no performance task.** The maintainer asked for one after the ADR review. Tasks
  B1-B3, AC7, and the benchmarking documentation and skill (D1, AC8) were added before any
  production change.
- **The key does not reach `udp-server`.** The ADR planned to expose the key from the container to
  the handlers that call `check` directly. Those calls are only the disabled-validation
  observability path, which repeats the services' check, so the services' `authenticate` methods
  became public and the handlers call them. The ADR was updated after the maintainer approved it.
- **T6 had nothing left to do.** Adding the key as a required constructor argument in T5 forced
  every caller to change in the same commit, so the planned split between `udp-core` wiring (T5)
  and `udp-server` wiring (T6) did not hold.
- **T4 needed a transitional static** (`RANDOM_CIPHER_BLOWFISH` as a `CookieCipher`) so each commit
  compiled before the wiring existed; T7 removed it.
- **Two benchmarks measure nothing.** `udp-core`'s `connect_once` was repaired (B1);
  `http-core`'s announce benchmark has the same defect and is recorded as a known defect in
  `docs/benchmarking.md`.
- **`compile_fail` error codes are checked only on nightly.** Recorded in the evidence, the ADR, and
  the `write-unit-test` skill.

## Root Cause

- The specification's task split followed package boundaries (`udp-core` in T5, `udp-server` in
  T6) rather than compile boundaries. A required constructor argument cannot be added to one
  package without changing its callers.
- The specification template has no prompt for performance on hot paths, so the risk was found by
  the maintainer during review rather than at planning time.
- The regression-test plan relied on a pinned doctest error code without checking how rustdoc
  enforces it on each toolchain.
- The existing benchmarks had never been checked against a known cost, so the broken ones looked
  healthy.

## Improvements for Future Work

1. When a specification plans to add a required parameter or constructor argument, split tasks at
   compile boundaries, or plan explicitly for a transitional shim and its removal.
2. Ask, when drafting a specification, whether the change touches a hot path, and if so plan a
   baseline before any production change (now covered by the `run-benchmarks` skill and
   `docs/benchmarking.md`).
3. For each regression test, record on which toolchain and gate it is enforced (now in the
   `write-unit-test` skill for `compile_fail` doctests).
4. Take evidence times from output files, logs, or `git log`, never from memory. Two evidence times
   in this issue were first typed from memory and had to be corrected.
5. After a failed commit, unstage what it staged before committing something else: the pre-commit
   frontmatter check validates every staged file, and a stale staged file made an unrelated commit
   fail and then rode along in the next one.

## Avoiding Overcorrection

- No new rule is needed against transitional statics: one existed for three commits, was named and
  documented as transitional, and was removed in the same pull request.
- A shared test-only key in `udp-server` tests is not a precedent for production globals; it
  matches one composition root sharing one key, and the maintainer approved it.
- Not every issue needs end-to-end load tests; the `run-benchmarks` skill asks for them only when a
  change touches the request path.

## Evidence

- Issue specification: [`ISSUE.md`](ISSUE.md)
- Manual and regression evidence: [`manual-verification-evidence.md`](manual-verification-evidence.md)
- Performance evidence: [`performance-evidence.md`](performance-evidence.md)
- ADR: [Inject the UDP connection-cookie cipher](../../../adrs/20261007085634_inject_the_udp_connection_cookie_cipher.md)
- Benchmarking guide and skill: [`docs/benchmarking.md`](../../../benchmarking.md),
  [`run-benchmarks`](../../../../.github/skills/dev/benchmarking/run-benchmarks/SKILL.md)
