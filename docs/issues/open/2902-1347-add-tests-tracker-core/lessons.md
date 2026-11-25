---
doc-type: issue-lessons
issue: 2902
package: torrust-tracker-core
---

# Tracker Core Testing Lessons

Candidate lessons collected while working on Issue #2902. Promoting any of them into `docs/testing/`, the
skills or a production change is a separate decision after this issue, per the
[subissue guidelines](../1347-overhaul-packages-testing/subissue-spec-guidelines.md). The
[findings register](ISSUE.md#findings-register) holds each finding's text and owner; this file records the lesson
and the issue that owns the work.

## Follow-Up Candidates

| Source | Candidate | Evidence |
| --- | --- | --- |
| Finding F4, [whitelist plan](test-refactor-plans/whitelist-persisted-and-manager-tests.md) | Make the whitelist add and remove idempotent under concurrent identical calls. | Tracked as [#3052](https://github.com/torrust/torrust-tracker/issues/3052), spec in PR #3053: 199 of 200 concurrent duplicate adds failed on SQLite. A regression test in this package follows the fix. |
| Finding F7, [key module plan](test-refactor-plans/authentication-key-mod-tests.md) | Return an error instead of panicking when a random expiring key's duration overflows the clock. | Tracked as [#3051](https://github.com/torrust/torrust-tracker/issues/3051), spec in PR #3054: `generate_key` unwraps `now_add`, and the REST add-key and generate-key endpoints reach it with `u64::MAX`. A regression test in this package follows the fix. |
| Finding F3, [peer key plan](test-refactor-plans/authentication-peer-key-tests.md) | Decide whether `Key` should keep a `Deserialize` derive that skips `Key::new`. | Recorded without change (maintainer decision, 2026-11-25): no code deserializes a `Key` or `PeerKey` from untrusted input today, so it is a latent bypass for a future caller, not a bug. |
| Finding F1, [server drivers plan](test-refactor-plans/databases-server-drivers-tests.md) | Make the MySQL and PostgreSQL driver suites report a visible skip instead of `ok` when their environment variable is unset, and publish their coverage. | Decided in S3: the README and `packages/AGENTS.md` commands are corrected; the visible skip and the Codecov coverage are a separate issue (Q2) that is not opened yet. |

## Testing Lessons

- **A rebase can change the code a plan was written against.** #3042 changed the key length rule, the key errors
  and the debug output while S4 was in review. Plans and records that cite a base commit, a test count or a coverage
  figure must be re-derived after a rebase, not patched: the first re-measurement of S4 reused the old deltas and
  the review caught it. Measure the head and the base from empty coverage targets in the same session.
- **Put an assertion where the rule lives.** The clearing done by `reset_with` was pinned only by a test of the
  handler that calls it, so the repository's own reset test passed with a `reset_with` that did not clear. A mutant of
  a collaborator is a reason to strengthen the collaborator's own test.
- **Rewrite a mutant that does not compile instead of dropping it.** `unused = "deny"` and `dropping_references` reject the
  obvious edits; `drop(...)` of an owned value or an `if` guard with an impossible condition keep the mutant
  compilable.
- **A quickcheck property must reach its accepted branch.** A property over an arbitrary `String` almost never
  produced a valid key; coverage showed it. Build the input from an alphabet that is mostly valid.
