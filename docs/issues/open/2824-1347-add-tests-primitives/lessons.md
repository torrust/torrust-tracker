---
doc-type: issue-lessons
issue: 2824
package: torrust-tracker-primitives
---

# Primitives Testing Lessons

Candidate lessons collected while working on Issue #2824. Promoting any of them into `docs/testing/`,
`.github/skills/`, or a production change is a separate decision after this issue, per the
[subissue guidelines](../1347-overhaul-packages-testing/subissue-spec-guidelines.md).

## Follow-Up Candidates

| Source | Candidate | Evidence |
| --- | --- | --- |
| [Pagination plan](test-refactor-plans/pagination.md) (review of #2852, F8) | Correct the doc comment of `QueryParams::offset` in the REST API torrent handlers, and check the REST API documentation for the same wording. | `packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs` says "The offset of the first page to return. Starts at 0." but `Pagination::new_with_options` passes the value into `Pagination::offset`, which every consumer uses as the number of items to skip (`.skip(pagination.offset as usize)`). The domain comment was corrected in S3; the REST one is outside this issue's scope. |
| [Coverage evidence](coverage-evidence.md) (S3 reconciliation) | In the S4 `readiness.rs` record, find out why its unit-only line coverage varied between runs (50 / 52 in one run, 52 / 52 in every other) and make the test deterministic if it is. | One S3 run measured `readiness.rs` at 50 / 52 with no change to the file, which moved the package totals by two lines in that step. |
| [Issue spec](ISSUE.md#out-of-scope) (Out of Scope) | Audit the implicit lowercase and snake_case convention of enum serialization (the F2 `PeerRole` defect came from a `rename_all_fields` that does not apply to unit variants) in a separate issue, under `create-adr`. | `PeerRole` serialized as `"Seeder"` while `Display` and the metric label use `seeder`; fixed in S1 (#2834). |

## Testing Lessons

- **A property needs a generator that reaches the interesting side.** `quickcheck` draws a full-range `u64` almost always
  above `2_147_483_647`, so the first form of the `AnnounceIntervalInSecs` property tested one side only; drawing a `u32`
  gave a balanced split. Check the share of generated values on each side of the boundary before keeping a property.
- **Name the mutants in advance, and say when the deciding one was not among them.** The property of
  `announce_interval.rs` was kept because a band mutant that no example catches was killed by it; that mutant was added
  after the three that the plan named. The record now says so.
- **Measure again after the last code change.** A first measure taken before a clippy rewrite left a stale per-file
  figure that did not reconcile with the package total; the review found it. Re-run coverage after the final edit of a
  file, not before it.
