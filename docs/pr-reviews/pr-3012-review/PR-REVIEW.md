---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore Greptile -->

# PR #3012 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3012>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F7 | review-finding:pr-3012-f7 | Unknown | Major (inferred) | security | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3012-f8 | Unknown | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-3012-f1 | Human | Minor | security | RE_RAISE_OF:F7 | FIXED | RESOLVED |
| F2 | review-finding:pr-3012-f2 | Human | Minor | security | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3012-f3 | Human | Nit | maintainability | RE_RAISE_OF:F8 | FIXED | RESOLVED |
| F4 | review-finding:pr-3012-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3012-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3012-f6 | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3012-f9 | Copilot | Major | security | RE_RAISE_OF:F7 | FIXED | RESOLVED |
| F10 | review-finding:pr-3012-f10 | Copilot | Major | security | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3012-f11 | Copilot | Major | security | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F7 - The text masker misses a URL whose scheme is not lowercase

- PR number: 3012
- Source review ID: 6026091763
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669305617
- Concern: Greptile (P1, security): `find_http_url_start` finds only lowercase schemes, so `HTTP://.../announce/<key>` as a wrong-typed `http_trackers` value or an extra argument is echoed with its key. Match the scheme without regard to case and cover both error paths.
- Solution: The masker no longer looks for a scheme: it masks the text after `/announce/` and `/scrape/`, compared in ASCII lowercase. A unit test with `HTTP://` and `Https://` and one black-box test per echo path with `HTTP://`; all were red before the fix (`evidence/r1-red.txt`).
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib not_lowercase` and `cargo test -q -p torrust-tracker-client --test key_redaction uppercase_scheme` pass (1 and 2 tests).
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669938995

### F8 - A test spells the full path of `Announce`

- PR number: 3012
- Source review ID: 6026091763
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669305634
- Concern: Greptile (P2): `announce_query` spells the full path of `Announce`; the Imports rule keeps full paths for name conflicts. Import it next to `AnnounceBuilder`.
- Solution: The `error_text` test module imports `Announce` next to `AnnounceBuilder`; `announce_query` returns `Announce`.
- Current-tree verification: `grep -n "fn announce_query() -> Announce" packages/tracker-client/src/http/client/mod.rs` matches one line.
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669939480

### F1 - Mixed-case keyed URLs in an echo keep their key

- PR number: 3012
- Source review ID: 6026720101
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669785023
- Concern: The text masker finds only lowercase `http://` and `https://`, so `HTTP://` or `Https://` in a configuration or usage echo keeps its key, against the contract, the README and AC1. Scan a lowercased copy; add a unit test and one black-box test per echo path, and tests for the planned "one keyed URL" and "no URL" cases.
- Solution: As F7; the two planned text cases have their own unit tests (`it_should_mask_the_key_of_a_text_that_is_one_keyed_url`, `it_should_not_change_a_text_without_a_url`).
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib redaction` passes 15 tests, among them the two planned cases and the upper-case case.
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669939965

### F2 - An authenticated client can write its key outside `/announce/` and `/scrape/`

- PR number: 3012
- Source review ID: 6026720101
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669785030
- Concern: `Client::authenticated` appends the key after a custom base path, and `health_check()` requests `health_check/<key>`; a failed request on either writes the key. Narrow the claims in the error doc and the README, or mask the client's own key.
- Solution: The client masks its own key: every path segment equal to it is masked (`redaction::mask_key_segments`) in the URL of a stored `reqwest::Error` (`with_url`) and in the new masked `url` field of `UnsuccessfulResponse`. Seam tests for a custom path with no response and with a `404`, and for the health check, were red before the fix. The README says what is masked.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib client_key` passes 3 tests.
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669940423

### F3 - `announce_query` spells the full path of `Announce`

- PR number: 3012
- Source review ID: 6026720101
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669785037
- Concern: The same point as F8: import `Announce` next to `AnnounceBuilder`.
- Solution: As F8.
- Current-tree verification: `grep -n "requests::announce::{Announce, AnnounceBuilder}" packages/tracker-client/src/http/client/mod.rs` matches one line.
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669940845

### F4 - The contract lists only some error kinds

- PR number: 3012
- Source review ID: 6026720101
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669785042
- Concern: The list of kinds reads as complete but `request_error_text` also writes `redirect`, `builder`, `request` and `unknown`. List them all or say "for example".
- Solution: The contract lists all nine kinds and says the chain of causes follows.
- Current-tree verification: `grep -n "the masked URL and the chain of" console/tracker-client/docs/contracts/tracker-cli-io-contract.md` matches one line.
- Resolution reference: `test(tracker-client): [#3004] cover upper-case and malformed URLs in the echo paths`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669941321

### F5 - Test counts and two details of the evidence do not recompute

- PR number: 3012
- Source review ID: 6026720101
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669785050
- Concern: The 11 tests that passed the first library red run are the 8 existing tests and 3 new no-change tests, not 11 existing ones; the progress entry counts 10 of 13 added tests; the quoted port is not in the log; the console red run is stamped 16:56 but its log says 16:57.
- Solution: The evidence and the progress entry give 13 added library tests (10 red); the quoted example uses port 37803 from the log; the console red run is stamped 16:57. The same count in the PR description was corrected with the same edit round.
- Current-tree verification: `grep -n "13 tests were added\|37803\|Console, 2026-11-23 16:57" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/manual-verification-evidence.md` matches three lines.
- Resolution reference: `docs(issues): [#3004] record the review round 1 fix and the repeated recheck`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669941777

### F6 - The kind-only error text drops the transport cause

- PR number: 3012
- Source review ID: 6026720101
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669785055
- Concern: The new texts say only "connection error" or "decode error"; the causes in the red logs (`hyper`, `hyper_util`, `io::Error`) carry no URL. Append the source chain or the `io::ErrorKind`, after confirming no link carries the URL.
- Solution: `request_error_text` appends the `Display` of each cause, each passed through the text masker. A refused connection reads `...: client error (Connect): tcp connect error: Connection refused (os error 111)`. A seam test asserts the cause is kept; it was red before.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib transport_cause` passes 1 test.
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669942210

### F9 - HTTP schemes are not recognized without regard to case

- PR number: 3012
- Source review ID: 6026728662
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669791623
- Concern: `HTTP://.../announce/<key>` in a misplaced argument or a wrong-typed configuration value bypasses the lowercase search. Compare case-insensitively and add upper- and mixed-case tests.
- Solution: As F7.
- Current-tree verification: `cargo test -q -p torrust-tracker-client --test key_redaction uppercase_scheme` passes 2 tests.
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669943606

### F10 - The masker cuts an echoed URL at a `)` or `'` inside it

- PR number: 3012
- Source review ID: 6026728662
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669791476
- Concern: For `http://host/prefix)/announce/<key>` the scan stops at `)` and leaves the key; an apostrophe in the user info stops it the same way. Use formatter-aware boundaries and add tests for both.
- Solution: The masker no longer cuts a token: it finds `/announce/` and `/scrape/` anywhere and masks the segment after them; the password is masked between the first `:` and the last `@` of the authority. Tests for a `)` before the endpoint and an apostrophe in the user info were red before the fix.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib closing_parenthesis` and `cargo test -q -p torrust-tracker-client-lib --lib apostrophe` pass 1 test each.
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669942710

### F11 - A URL that does not parse is echoed with its key

- PR number: 3012
- Source review ID: 6026728662
- Reviewer finding ID: R3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669791540
- Concern: `http://127.0.0.1:bad/announce/<key>` fails to parse and reaches the fallback that writes it unchanged. Mask malformed candidates, update the documented fallback, and add tests for both echo paths.
- Solution: There is no parse step and no unmasked fallback; the function documentation says it does not parse. A unit test and one black-box test per echo path with `http://127.0.0.1:bad/...` were red before the fix.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib does_not_parse` passes 1 test and `cargo test -q -p torrust-tracker-client --test key_redaction does_not_parse` passes 2.
- Resolution reference: `fix(tracker-client): [#3004] mask keys in any echoed URL and the client's own key`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669943169

## Processing Log

- 2026-11-23 18:10 UTC - Round 1 had three reviews of the head `docs(issues): [#3004] record the regression tests, the recheck and the retrospective`: Greptile 6026091763 (COMMENTED, 17:17:59Z, two inline comments, F7 and F8), da2ce7 6026720101 (COMMENTED, 17:54:49Z, six inline comments, F1 to F6) and Copilot 6026728662 (COMMENTED, 17:55:20Z, three inline comments with IDs R1 to R3, recorded as F9 to F11 because the audit IDs F1 to F3 were taken). The new tests were run red at 17:58Z (`evidence/r1-red.txt`); fix commits made at 18:00:56Z, 18:01:42Z and 18:03:44Z (committer times) and pushed before the replies. Replied on the 11 threads as `josecelano-bot` (reply `created_at` 18:07:56Z to 18:08:21Z), edited the PR description (counts and the round 1 changes, F5), and resolved each thread with GraphQL; every `resolveReviewThread` returned `isResolved` true. The audit was written after the replies.

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
