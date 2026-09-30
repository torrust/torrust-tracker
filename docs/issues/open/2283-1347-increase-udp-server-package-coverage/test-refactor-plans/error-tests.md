---
doc-type: test-refactor-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/error.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/error.rs
    - packages/udp-server/src/handlers/announce.rs
    - packages/udp-server/src/handlers/scrape.rs
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/error-tests.md
---

# UDP Server Service-Error Adapter File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-server/src/error.rs`.
It predates the section layout defined in the issue's Per-File Workflow; its `R1`-`R3` map to
R1 refactor, R2 unit tests, and R4 record coverage. No integration increment (R3) was selected.

## Current State

- **Unit-only coverage:** 86 / 86 lines (100.00%), 112 / 112 regions (100.00%), and 10 / 10
  functions (100.00%) from the R3 report at
  `.tmp/2283-coverage/error-r3-unit.json`.
- **Current contracts:** five direct tests cover sendable and unsendable `RequestParseError`
  adaptation, `Error::InvalidRequest` wrapping, and each service-error conversion.
- **Package-owned decisions:** `From<UdpAnnounceError>` and `From<UdpScrapeError>` preserve
  UDP-core service errors in `Error::AnnounceFailed` and `Error::ScrapeFailed`, respectively.
- **Property-test decision:** Not selected. The conversion has two discrete typed branches, for
  which example-based tests are clearer than a generated input space.

## Ownership Boundary

`udp-core` owns the contents and business semantics of `UdpAnnounceError` and `UdpScrapeError`.
This module owns only preservation of each error in the correct UDP-server error variant. The
handlers own their service calls; `handlers/error.rs` owns wire responses and event publication;
`event.rs` owns stable error classification. No socket, event bus, handler, or integration fixture
belongs in this test.

## Approved-For-Review Work

| ID | Status | Decision | Evidence required before completion |
| --- | --- | --- | --- |
| R1 | DONE | Reviewed and refactored the parse-error adapter tests. Each now compares its complete independently constructed `SendableRequestParseError` result with one semantic assertion. | Focused `error::tests`, nightly formatting, and prose-first review result. |
| R2 | DONE | Added one direct typed conversion contract for each service-error variant after R1 review. | Fixed `ConnectionCookieError` source, direct `Error::from` Act, and expected `Error` variant remain visible in both tests. |
| R3 | DONE | The fresh unit-only report attributes all 86 executable lines, 10 functions, and 112 regions in `error.rs` to package-local unit tests. | Updated unit-only coverage evidence; no percentage-only additions. |

## Guardrails And Review Questions

- Construct fixed valid UDP-core service errors directly; do not invoke handlers or parse packets.
- Keep both variants in the same `error.rs` file because they are sibling conversions in the same
  error-adapter subsystem.
- Do not change production errors, their display text, `ConnectionCookie`, or the completed
  parse-error tests unless R1 identifies an approved test-only improvement.
- Before moving R1 or R2 to `DONE`, confirm the Arrange names the source-error variant, the Act is
  the direct `Error::from` conversion, and the Assert compares the independently selected variant.

## Validation

After each approved increment run:

```text
cargo test -p torrust-tracker-udp-server error::tests
cargo +nightly fmt --all -- --check
git diff --check
```

Run `linter markdown` and `linter cspell` after this record changes. The completed-file review must
record the command results with their Rust toolchain before any other source file becomes
`IN_PROGRESS`.

## Progress Log

- 2026-09-22 07:10 UTC - User/maintainer - Approved T3 and the `error.rs` decision record as the
  only active file in the top-level error-adapter subsystem.
- 2026-09-22 07:10 UTC - GitHub Copilot - Completed R1 with no test-code change. Prose-first
  review found the current parse-error tests already state their distinct causal source error,
  direct conversion Act, and independently specified expected routing fields or typed variant.
  There is no concrete readability, duplication, fixture, assertion, or AAA defect to refactor.
  R2 remains pending approval.
- 2026-09-22 07:10 UTC - User/maintainer - Approved R2.
- 2026-09-22 07:10 UTC - GitHub Copilot - Completed R2. Added direct contracts that convert
  `UdpAnnounceError::ConnectionCookieError` to `Error::AnnounceFailed` and
  `UdpScrapeError::ConnectionCookieError` to `Error::ScrapeFailed`, preserving the fixed source
  error in each case. The focused command `cargo test -p torrust-tracker-udp-server error::tests`
  passed 12 tests; `cargo +nightly fmt --all -- --check` and `git diff --check` also passed.
- 2026-09-22 07:10 UTC - User/maintainer - Identified the multiple-assert smell in the two
  parse-error adapter tests and approved their review and improvement.
- 2026-09-22 07:10 UTC - GitHub Copilot - Refactored each parse-error adapter test to construct
  and compare one complete `SendableRequestParseError` result. This retains the independently
  specified message and routing identifiers while giving each test one semantic assertion and one
  complete observable result. The focused command `cargo test -p torrust-tracker-udp-server
  error::tests` passed 12 tests; `cargo +nightly fmt --all -- --check` and `git diff --check`
  passed with the nightly Rust toolchain.
- 2026-09-22 07:10 UTC - GitHub Copilot - Completed R3. The fresh unit-only
  `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` report recorded
  `error.rs` at 86 / 86 lines, 112 / 112 regions, and 10 / 10 functions (100.00% each). The file
  remains in progress pending its completed-file maintainer review gate.
- 2026-09-22 07:10 UTC - User/maintainer - Approved the completed-file review for `error.rs`.
- 2026-09-22 07:10 UTC - GitHub Copilot - Marked this decision record complete. No other source
  file has entered `IN_PROGRESS`.
