---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore Greptile coderabbitai -->

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
| F12 | review-finding:pr-3012-f12 | Unknown | Major (inferred) | security | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3012-f13 | Unknown | Minor | security | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3012-f14 | Unknown | Minor | security | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3012-f15 | Unknown | Minor | security | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3012-f16 | Unknown | Nit | testing | ORIGINAL | FIXED | NON_RESOLVABLE |
| F17 | review-finding:pr-3012-f17 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3012-f18 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-3012-f19 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-3012-f20 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-3012-f21 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-3012-f22 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-3012-f23 | Unknown | Minor | security | ORIGINAL | FIXED | RESOLVED |

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

### F12 - A backslash separator leaves the key in an echo

- PR number: 3012
- Source review ID: 6026959633
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4669969342
- Concern: Greptile (P1, security): `http://tracker.example/announce\<key>` is not masked by the literal search, although HTTP URL parsing treats the backslash as a slash; as an extra argument it reaches stderr through `AppError::from_clap`. Cover separators the parser normalizes and add a usage-error test.
- Solution: The masker takes `\` as `/` and a `\` ends a key segment. The recheck then found the configuration echo still leaking, because `serde_json` writes the backslash as `\\`; the key is now the first non-empty segment after the endpoint (`fix(tracker-client): [#3004] mask the key after an escaped backslash`). Unit and black-box tests for both echoes were red first.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib backslash` passes 2 tests and `cargo test -q -p torrust-tracker-client --test key_redaction backslash` passes 2.
- Resolution reference: `fix(tracker-client): [#3004] mask keys after a backslash, a delimiter or a nested endpoint`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670404999

### F13 - A key after a delimiter that follows the endpoint stays visible

- PR number: 3012
- Source review ID: 6027173167
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670137678
- Concern: CodeRabbit: the masker stops at the first delimiter after `/announce/` or `/scrape/`, so `/announce/abc)<key>` keeps `<key>` and `/announce/(<key>` is unchanged. Mask the key segment whatever delimiter precedes it, and test it.
- Solution: A key segment ends only at a path delimiter, a character that cannot appear in a URL, or a closing quote; `(` and `)` are part of it. The contract states the rule. The test was red first.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib starts_with_or_contains` passes 1 test.
- Resolution reference: `fix(tracker-client): [#3004] mask keys after a backslash, a delimiter or a nested endpoint`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670405555

### F14 - An endpoint that starts at the slash ending another one is skipped

- PR number: 3012
- Source review ID: 6027173167
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670137688
- Concern: CodeRabbit: after masking a key the cursor passes a marker whose leading `/` it shares, so `/announce/scrape/<key>` keeps the key. Find every marker, including overlapping ones.
- Solution: Every marker is found by searching on from the byte after each match; the key ranges are collected and masked, merging overlaps. The test was red first.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib follows_another_endpoint` passes 1 test.
- Resolution reference: `fix(tracker-client): [#3004] mask keys after a backslash, a delimiter or a nested endpoint`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670406053

### F15 - The password of a second URL whose authority follows the first is kept

- PR number: 3012
- Source review ID: 6027173167
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670137696
- Concern: CodeRabbit: the password scan resumes after the whole authority, which can contain the scheme of the next URL, so its password is copied unmasked. Resume at the authority start with a separate copy cursor, and test two URLs.
- Solution: Applied the suggested fix (copy cursor, search from the authority start); `\` also ends an authority. The test was red first.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib password_of_every_url` passes 1 test.
- Resolution reference: `fix(tracker-client): [#3004] mask keys after a backslash, a delimiter or a nested endpoint`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670406539

### F16 - No body-read test for `http scrape`

- PR number: 3012
- Source review ID: 6027173167
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#pullrequestreview-6027173167
- Concern: CodeRabbit (trivial, in the review body): the cut-short tests cover `http announce` and `check`, not the separate body read of `scrape_command`. Add an `http scrape` test with the cut-short fake.
- Solution: Added the test. It passed on the round 1 head because the fix covered the path, so it was proved by mutation: with `scrape_command` reading the body with `response.bytes().await?`, only this test failed (`evidence/r2-scrape-mutation.txt`). The finding has no thread; the reply is a PR comment.
- Current-tree verification: `cargo test -q -p torrust-tracker-client --test key_redaction http_scrape_response_body` passes 1 test.
- Resolution reference: `fix(tracker-client): [#3004] mask keys after a backslash, a delimiter or a nested endpoint`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#issuecomment-7110688881

### F17 - Rebuilding the path could change the percent encoding of other segments

- PR number: 3012
- Source review ID: 6027631676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670516852
- Concern: Copilot: `mask_path_segments` rebuilds the path from `path_segments()`, which it takes to be percent-decoded, so masking could change the encoding of other segments; use `path_segments_mut()` or at least add a test that non-secret percent-encoded segments stay stable.
- Solution: Added the test. It passed when added: `path_segments()` yields the serialized, percent-encoded segments and `set_path` keeps percent sequences, so the code is unchanged and the test guards it.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib percent_encoding` passes 1 test.
- Resolution reference: `fix(tracker-client): [#3004] spell HTTP in upper case in the checker errors`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670622764

### F18 - `Http` in a health-check error message

- PR number: 3012
- Source review ID: 6027631676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670516909
- Concern: Copilot: the health-check message uses `Http` while others use `HTTP`; standardize on `HTTP`.
- Solution: The message reads `HTTP check returned a non-success code: ...`.
- Current-tree verification: `grep -c '"HTTP check returned' console/tracker-client/src/console/clients/checker/checks/health.rs` prints 1.
- Resolution reference: `fix(tracker-client): [#3004] spell HTTP in upper case in the checker errors`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670623296

### F19 - `Http` in the checker timeout message

- PR number: 3012
- Source review ID: 6027631676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670516953
- Concern: Copilot: `Http request did not receive a response within the timeout` uses `Http`; standardize on `HTTP`.
- Solution: The message reads `HTTP request did not receive a response within the timeout: ...`; no test or doc matched the old text.
- Current-tree verification: `grep -c '"HTTP request did not receive' console/tracker-client/src/console/clients/http/mod.rs` prints 1.
- Resolution reference: `fix(tracker-client): [#3004] spell HTTP in upper case in the checker errors`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670623807

### F20 - `Http` in the checker response message

- PR number: 3012
- Source review ID: 6027631676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670516999
- Concern: Copilot: `Http failed to get a response at all` uses `Http`; standardize on `HTTP`.
- Solution: The message reads `HTTP failed to get a response at all: ...`.
- Current-tree verification: `grep -c '"HTTP failed to get a response' console/tracker-client/src/console/clients/http/mod.rs` prints 1.
- Resolution reference: `fix(tracker-client): [#3004] spell HTTP in upper case in the checker errors`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670624265

### F21 - A long single line in the library README

- PR number: 3012
- Source review ID: 6027631676
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670517062
- Concern: Copilot: the README change adds a very long single line; wrap it.
- Solution: The redaction text is its own paragraph, wrapped at 100 columns.
- Current-tree verification: `grep -n "^masks its own key wherever its path puts it" packages/tracker-client/README.md` matches one line of the wrapped paragraph.
- Resolution reference: `fix(tracker-client): [#3004] spell HTTP in upper case in the checker errors`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670624711

### F22 - The tokio dev-dependency does not list the features the seam tests use

- PR number: 3012
- Source review ID: 6028078922
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670866841
- Concern: Copilot: the seam tests use `#[tokio::test]`, `tokio::spawn` and `tokio::net::TcpListener`, but the dev-dependency lists only `io-util` and `time`; list `macros`, a runtime and `net` too.
- Solution: The dev-dependency lists `io-util`, `macros`, `net`, `rt-multi-thread` and `time`. The tests compiled before because Cargo unifies these features with the normal `tokio` dependency of the package; the list now states what the tests use.
- Current-tree verification: `grep -n 'tokio = .*"io-util", "macros", "net", "rt-multi-thread", "time"' packages/tracker-client/Cargo.toml` matches the dev-dependency line, and `cargo test -q -p torrust-tracker-client-lib --lib` passes 41 tests.
- Resolution reference: `test(tracker-client): [#3004] declare the tokio features the seam tests use`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670918979

### F23 - A dot segment moves the key past the masked segment

- PR number: 3012
- Source review ID: 6028220359
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4670983182
- Concern: CodeRabbit: the text masker masks only the first non-empty segment after the endpoint, so `/announce/./<key>` (or `%2e`) masks the `.` and prints the key, although URL parsing removes the dot segment and reaches the same key. Skip `.` and `%2e` segments and add tests.
- Solution: The text masker hides the whole rest of the path after the endpoint, up to the query, the fragment, a character that cannot appear in a URL, or the closing quote; this covers dot segments, `../`, nested endpoints and separators in one rule. The contract and the spec state it. Tests for `./`, `%2E/` and `../x/<key>?...` were red first.
- Current-tree verification: `cargo test -q -p torrust-tracker-client-lib --lib dot_segment` and `cargo test -q -p torrust-tracker-client-lib --lib whole_path_after_the_endpoint` pass 1 test each.
- Resolution reference: `fix(tracker-client): [#3004] mask the whole path after the endpoint in echoed text`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3012#discussion_r4671078323

## Processing Log

- 2026-11-23 18:10 UTC - Round 1 had three reviews of the head `docs(issues): [#3004] record the regression tests, the recheck and the retrospective`: Greptile 6026091763 (COMMENTED, 17:17:59Z, two inline comments, F7 and F8), da2ce7 6026720101 (COMMENTED, 17:54:49Z, six inline comments, F1 to F6) and Copilot 6026728662 (COMMENTED, 17:55:20Z, three inline comments with IDs R1 to R3, recorded as F9 to F11 because the audit IDs F1 to F3 were taken). The new tests were run red at 17:58Z (`evidence/r1-red.txt`); fix commits made at 18:00:56Z, 18:01:42Z and 18:03:44Z (committer times) and pushed before the replies. Replied on the 11 threads as `josecelano-bot` (reply `created_at` 18:07:56Z to 18:08:21Z), edited the PR description (counts and the round 1 changes, F5), and resolved each thread with GraphQL; every `resolveReviewThread` returned `isResolved` true. The audit was written after the replies.
- 2026-11-23 18:49 UTC - Round 2 had two reviews: Greptile 6026959633 (COMMENTED, 18:10:24Z, on the round 1 fix head, one inline comment, F12) and CodeRabbit 6027173167 (COMMENTED, 18:24:53Z, on the round 1 audit head, three inline comments, F13 to F15, and one nitpick in its body, F16). The poller watched only da2ce7 and Copilot, so it did not report them; they were found when the maintainer asked. Red runs at 18:39Z and 18:41Z, the scrape mutation at 18:40Z; fix commits at 18:40:23Z and 18:42:20Z, the evidence commit at 18:44:47Z, after the final recheck at 18:43Z; pushed before the replies. Replied on the four threads (`created_at` 18:48:36Z to 18:48:43Z) and on F16 with a PR comment (18:48:46Z), all as `josecelano-bot`. Resolved F12 and F13 with GraphQL (`isResolved` true); CodeRabbit had already resolved the F14 and F15 threads itself after the push (`resolvedBy` `coderabbitai[bot]`). F16 has no thread.
- 2026-11-23 19:08 UTC - Round 3: Copilot 6027631676 (COMMENTED, 18:58:04Z, on the round 1 audit head; its overview warns the full agentic review did not start before a timeout) with five inline comments, F17 to F21; its overview severities are not shown per comment, so they are inferred. The fix commit at 19:02:00Z changed message text, so the recheck was repeated at 19:03Z and recorded (evidence commit at 19:03Z); pushed before the replies. Replied on the five threads as `josecelano-bot` (`created_at` 19:07:28Z to 19:07:38Z) and resolved each with GraphQL; every `resolveReviewThread` returned `isResolved` true. The two CodeRabbit reviews of 18:50Z (6027521172, 6027522667) only reply on the F13 and F15 threads that the fixes cover their cases; they raise no finding.
- 2026-11-23 19:32 UTC - Round 4: Copilot 6028078922 (COMMENTED, 19:27:45Z, on the round 3 recheck head) with one inline comment, F22. Earlier, at 19:07:27Z, Greptile posted only that the organization ran out of review credits, which is not a finding. Fix commit at 19:28:30Z, test configuration only (no recheck needed: the binary does not change); pushed before the reply. Replied as `josecelano-bot` (19:32:21Z) and resolved the thread with GraphQL (`isResolved` true).
- 2026-11-23 19:46 UTC - Round 5: CodeRabbit 6028220359 (COMMENTED, 19:38:07Z, on the round 4 audit head) with one inline comment, F23. Red run at 19:40Z; fix commit at 19:40:56Z, recheck at 19:41Z and the evidence commit after it; pushed before the reply. Replied as `josecelano-bot` (19:46:17Z) and resolved the thread with GraphQL (`isResolved` true). This is the fifth round of fixes, the limit after which the maintainer is asked before more fixes.

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
