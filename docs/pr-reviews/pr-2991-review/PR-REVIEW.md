---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore keye expirede UHWK -->

# PR #2991 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2991.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

F1 and F2 come from Copilot's review 6006219064 (COMMENTED on the first head, "docs(issues): [#2916] record the reproduction, red and green runs and the manual verification"), in source order. F3 to F8 come from da2ce7's review 6006330368 (APPROVED on the round-1 head, "docs(pr-reviews): [#2916] record the replies and resolved threads of PR #2991 round 1", with an ACK comment); the reviewer numbered them F1 to F6, kept in each entry's "Reviewer finding ID".

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2991-f1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2991-f2 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2991-f3 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-2991-f4 | Human | Minor | security | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-2991-f5 | Human | Minor | testing | ORIGINAL | FIXED | OPEN |
| F6 | review-finding:pr-2991-f6 | Human | Nit | formatting | ORIGINAL | FIXED | OPEN |
| F7 | review-finding:pr-2991-f7 | Human | Suggestion | maintainability | ORIGINAL | NO_ACTION | OPEN |
| F8 | review-finding:pr-2991-f8 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - The doc comment of `verify_auth_key` reads as if a found key returns an error

- PR number: 2991
- Source review ID: 6006219064
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653326514
- Concern: Copilot (rated Medium) read the reworded doc comment of `AuthenticationService::verify_auth_key` as saying that a found key "returns an error that names the key", while the found case goes on to the expiration check and can succeed. It asked for wording that follows the control flow.
- Solution: The sentence split across a line break made the two cases easy to misread. The comment now says that a key that is not found yields `Error::UnableToReadKey`, and that a found key is checked for expiration, the function returning `Ok(())` when the check passes or is disabled.
- Current-tree verification: `sed -n 74,82p packages/tracker-core/src/authentication/service.rs` shows the new wording; `cargo doc -p torrust-tracker-core --no-deps` (stable Rust toolchain) resolves the `Error::UnableToReadKey` link without a warning.
- Resolution reference: docs(tracker-core): [#2916] describe the key verification outcomes and the bencode terminator in samples
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653353310

### F2 - The trailing bencode `e` in the extractor samples reads like a typo

- PR number: 2991
- Source review ID: 6006219064
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653326542
- Concern: Copilot (rated Low) noted that the module-doc samples of `authentication_key.rs` end with the bencode dictionary terminator right after the message (`keye`, `expirede`), which can read as a typo, and suggested decoding the samples or saying what the `e` is.
- Solution: The samples stay raw, as they are what a client receives and they match the recorded replies byte for byte; a sentence before them now says that each sample is the raw bencoded body and that the final `e` closes the dictionary and is not part of the `failure reason`.
- Current-tree verification: `sed -n 15,20p packages/axum-http-server/src/v1/extractors/authentication_key.rs` shows the note before the samples.
- Resolution reference: docs(tracker-core): [#2916] describe the key verification outcomes and the bencode terminator in samples
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653353557

### F3 - The REST delete-key doc example still shows a source path

- PR number: 2991
- Source review ID: 6006330368
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653421925
- Concern: The module doc of `packages/axum-rest-api-server/src/v1/context/auth_key/mod.rs` (outside the diff) showed the reply to deleting a missing key with an old source path (`src/tracker/databases/sqlite.rs:267:27`), a shape the code no longer produces; the spec's documentation box covered only the HTTP examples.
- Solution: The example now shows the recorded V3 K3 reply, `failed to delete key: Failed to remove record from Sqlite3 database, error-code: 0`, with no path.
- Current-tree verification: `grep -rn "\.rs:[0-9]" packages/axum-rest-api-server/src packages/axum-http-server/src packages/tracker-core/src packages/http-core/src packages/udp-core/src packages/udp-server/src` finds no doc example with a source path.
- Resolution reference: docs(axum-rest-api-server): [#2916] show the delete-key error reply without a source path
- Follow-up PR URL: N/A
- Reply URL: N/A

### F4 - A changed message keeps and prints the peer key

- PR number: 2991
- Source review ID: 6006330368
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653421932
- Concern: `authentication::key::Error::UnableToReadKey` changes in this PR but keeps `key: Box<Key>` and prints it ("Failed to read key: {key}"), against ADR rule 3; the new unit test pinned the key in the text. The reviewer suggested a follow-up that names the parameter and drops the field and, until then, a test that checks the prefix only.
- Solution: The test `it_should_reject_an_unregistered_key_with_a_message_that_holds_no_source_path` now checks that the message starts with "Failed to read key" and holds no source path, so it no longer pins the key. Dropping the key from the variant stays outside this spec's scope; the question went to the EPIC #2912 coordinator before the PR was opened and is open with the maintainer, who decides whether it becomes a follow-up issue.
- Current-tree verification: `sed -n 212,222p packages/tracker-core/src/authentication/service.rs` shows the prefix check; `cargo test -p torrust-tracker-core --lib authentication::service` (stable Rust toolchain) passes 8 tests.
- Resolution reference: test(tracker-core): [#2916] check only the prefix of the unregistered-key message
- Follow-up PR URL: N/A
- Reply URL: N/A

### F5 - The red and green commands are not recorded, and three driver steps were never seen red

- PR number: 2991
- Source review ID: 6006330368
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653421934
- Concern: The evidence stored output only, without the `cargo test` commands, and the shared SQLite driver test stopped at its first failing step in the red run, so the missing-key, duplicate-info-hash and missing-info-hash steps had no red run.
- Solution: `manual-verification-evidence.md` now lists the five red commands and the five green commands with their evidence files. The three steps were run red: with the red-test bytes of `packages/tracker-core` restored, each step was moved to run first and the driver test run; all three fail on a path (`evidence/tests-red-driver-steps.txt`). The branch was restored afterwards and `git diff --quiet HEAD` exited 0.
- Current-tree verification: `grep -c "^test result: FAILED" docs/issues/open/2916-2912-si-4-tracker-core/evidence/tests-red-driver-steps.txt` counts three failed runs, one per step, each with a `.rs:` path in the panic message.
- Resolution reference: docs(issues): [#2916] record the red-run commands and the red runs of the three later driver steps
- Follow-up PR URL: N/A
- Reply URL: N/A

### F6 - A blank line at the end of `tests-green.txt`

- PR number: 2991
- Source review ID: 6006330368
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653421938
- Concern: `git diff --check` reported a new blank line at the end of `evidence/tests-green.txt`; the other evidence files end with a single newline.
- Solution: The file now ends with a single newline.
- Current-tree verification: `git diff --check torrust/develop...HEAD` prints nothing.
- Resolution reference: docs(issues): [#2916] record the red-run commands and the red runs of the three later driver steps
- Follow-up PR URL: N/A
- Reply URL: N/A

### F7 - `assert_names_no_source_path` now exists in seven copies

- PR number: 2991
- Source review ID: 6006330368
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653421941
- Concern: This PR adds the tracker-core, REST and UDP copies of the path assertion to those in `http-protocol`, `primitives` and `axum-http-server`. Each placement fits rules C and E of the test-support ADR, but rule B puts a protocol-agnostic utility used by many packages in `torrust-tracker-test-helpers`. The reviewer suggested SI-6, which adds the enforcement check, as the place to merge them.
- Solution: No change in this PR: merging the copies touches packages outside SI-4's scope (`http-protocol`, `primitives`, `axum-http-server`). The suggestion is passed to the EPIC #2912 coordinator for SI-6.
- Current-tree verification: `grep -rln "fn assert_names_no_source_path" packages` lists the seven copies.
- Resolution reference: N/A
- Follow-up PR URL: N/A
- Reply URL: N/A

### F8 - The evidence counts eleven requests; the captures hold twelve

- PR number: 2991
- Source review ID: 6006330368
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2991#discussion_r4653421946
- Concern: `responses-before.txt` and `responses-after.txt` each record twelve requests (U1, H1, W1 to W3, H2, H3, K1 to K4, H4), but the evidence said eleven.
- Solution: The evidence now says twelve.
- Current-tree verification: `grep -c "^[UHWK][0-9]" docs/issues/open/2916-2912-si-4-tracker-core/evidence/responses-after.txt` counts 12.
- Resolution reference: docs(issues): [#2916] record the red-run commands and the red runs of the three later driver steps
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-11-21 15:15 UTC - Started audit for Copilot's review 6006219064 (COMMENTED, submitted 2026-11-21 15:12:35Z on the first head). Copilot noted that its full agentic review did not start before its timeout; the review has two inline findings, F1 (Medium, recorded as Minor) and F2 (Low, recorded as Nit). Both fixed in "docs(tracker-core): [#2916] describe the key verification outcomes and the bencode terminator in samples". Not pushed yet; replies and thread resolution follow the push. da2ce7 is a requested reviewer and has not reviewed yet.
- 2026-11-21 15:19 UTC - Pushed "docs(pr-reviews): [#2916] add the review audit of PR #2991" as the new head after the commit-id check printed nothing (pre-push hook passed). Replied on F1 and F2, recorded the reply URLs, and resolved both threads (`resolveReviewThread` returned `true` for both; `date -u` after the resolve commands: 15:19:50). GraphQL refresh: zero unresolved threads on PR #2991.
- 2026-11-21 15:43 UTC - Started round 2 for da2ce7's review 6006330368 (APPROVED, submitted 15:38:15Z on the round-1 head, with an ACK comment at 15:38:19Z): six non-blocking findings, recorded as F3 to F8 with the reviewer's F1 to F6. F3 to F6 and F8 fixed in "docs(axum-rest-api-server): [#2916] show the delete-key error reply without a source path", "test(tracker-core): [#2916] check only the prefix of the unregistered-key message" and "docs(issues): [#2916] record the red-run commands and the red runs of the three later driver steps"; F7 answered without change. Not pushed yet; replies and thread resolution follow the push. The ACK names the round-1 head, which this push replaces.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
