---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/ (module declaration files)
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/mod.rs
    - packages/axum-rest-api-server/src/v1/context/mod.rs
    - packages/axum-rest-api-server/src/v1/context/torrent/resources/mod.rs
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
---

# Wiring Modules File Test Plan

Follow the shared [guidance](README.md). One plan for the declaration-only group, ledger rows 22-31, as the spec allows:
`v1/mod.rs`, `v1/context/mod.rs`, `v1/middlewares/mod.rs`, the `mod.rs` of `auth_key`, `health_check`, `stats`, `torrent` and
`whitelist`, `torrent/resources/mod.rs` and `torrent/resources/torrent.rs`. Status: done; the maintainer approved the plan on
2026-11-06 (no test is added).

## Current State

Each file holds only `pub mod` declarations and documentation (some of the `mod.rs` files carry long endpoint
documentation). None has an instrumented line: they do not appear in the coverage report. No file in the group
decides anything.

## Coverage And Decision Analysis

| Rows | Files | Classification | Decision |
| --- | --- | --- | --- |
| 22-28, 31 | `v1/mod.rs`, `v1/context/mod.rs`, `v1/middlewares/mod.rs`, `auth_key/mod.rs`, `health_check/mod.rs`, `stats/mod.rs`, `torrent/mod.rs`, `whitelist/mod.rs` | Wiring only | No test. Removing any `pub mod` of these files breaks the build (see the evidence below), so the compiler is the guard. |
| 29 | `torrent/resources/mod.rs` | Wiring only; dead module | No test. The module only declares `pub mod torrent;`, and the file below is an empty documentation module. |
| 30 | `torrent/resources/torrent.rs` | Empty module (three documentation lines) | No test. See the observation. |

### Observation for the record, not a finding of #2637

`torrent/resources` and `torrent/resources/torrent.rs` contain no code: since the DTOs moved to
`torrust-tracker-rest-api-protocol`, they are empty public modules kept for documentation. Removing `pub mod resources;`
from `torrent/mod.rs`, or `pub mod torrent;` from `resources/mod.rs`, still compiles, and no package or the tracker binary
imports `torrust_tracker_axum_rest_api_server::v1::context::torrent::resources`. Removing them is a production (public API)
change, so it is not part of this issue; it is recorded in `lessons.md` as a clean-up candidate for the maintainer.

## Challenge Of The No-Change Conclusion

Mutation evidence (already run on the S6 baseline, `cargo check -p torrust-tracker-axum-rest-api-server --all-targets`
after deleting one `pub mod` line at a time, each file restored afterwards): 23 of 25 declarations break the build when
removed (`v1/mod.rs` 4, `v1/context/mod.rs` 5, `v1/middlewares/mod.rs` 1, `auth_key` 3, `health_check` 1, `stats` 3,
`torrent` 3 of 4, `whitelist` 3); the two survivors are the two empty modules above (`torrent/mod.rs`: `pub mod resources;`
and `resources/mod.rs`: `pub mod torrent;`). The mutation is applied again after approval to confirm the numbers before they
enter the evidence.

## Property-Test Decision

None.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No tests to refactor. | None. |
| R2 | SKIPPED | No tests added. | None. |
| R3 | SKIPPED | Integration increment: `none`. | None. |
| R4 | DONE | Re-run the declaration mutations; record the rows `DONE` with the decision and the clean-up candidate in `lessons.md`. | `coverage-evidence.md`, `lessons.md`. |

## Prose-First AAA Review

No test is added or changed.

## Results

The maintainer approved the plan on 2026-11-06 (no test). The declaration mutations were run again on the S6 branch (`cargo check -p torrust-tracker-axum-rest-api-server --all-targets` after deleting one `pub mod` line at a time, each file restored afterwards): 23 of 25 removals break the build, and the two survivors are `pub mod resources;` in `torrent/mod.rs` and `pub mod torrent;` in `resources/mod.rs`, the empty documentation modules. `torrent/resources/torrent.rs` has no declaration to remove. The clean-up candidate is in `lessons.md`.

## Progress Log

- 2026-11-06 10:39 UTC - Claude Code - Plan drafted; waiting for the maintainer's approval.
- 2026-11-06 11:17 UTC - Claude Code - Plan approved by the maintainer; decision recorded, rows `DONE`.
