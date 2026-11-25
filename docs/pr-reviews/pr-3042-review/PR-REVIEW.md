---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #2999"
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore nnounce oken coderabbitai rposition -->

# PR #3042 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/3042>.

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

Review 6044736873 (Greptile, 2026-11-24 18:06 UTC) posted two inline comments, F1 and F2, and a summary comment (<https://github.com/torrust/torrust-tracker/pull/3042#issuecomment-7140177515>) that lists the same two findings and adds none. The CodeRabbit comment on the pull request only says its free review limit was reached; it holds no finding.

Review 6054012280 (da2ce7, approved, 2026-11-25 08:18 UTC) posted seven inline comments with the reviewer ids F1 to F7. F1 and F2 are taken in this record, so they are the audit-local F3 to F9; each detail entry records the reviewer's id. The review body lists the same seven findings and adds none.

Review 6054287756 (CodeRabbit, 2026-11-25 08:40 UTC) posted three inline comments, F10 to F12. Its body only repeats them, so it adds no row.

Review 6054388295 (da2ce7, changes requested, 2026-11-25 08:49 UTC) read the head before the round 3 fix and posted three inline comments with the reviewer ids F10, F11 and F9. F10 and F11 are taken in this record by CodeRabbit's findings and F9 by an earlier one, so they are the audit-local F13 to F15. F13 and F14 ask for the changes F12 and F10 already made. The review body lists the same three findings and adds none; it also says the reviewer's earlier F8 is fixed.

Review 6054457576 (CodeRabbit, 2026-11-25 08:54 UTC) holds one comment, a reply on the F10 thread that accepts the answer; it adds no row. Review 6054462408 (Greptile, 08:55 UTC) posted two inline comments, F16 and F17; as in the first Greptile round, its `P1` badge is recorded as `Blocker` and its `P2` badge as `Nit`. Review 6054537873 (da2ce7, commented, 09:00 UTC) posted one inline comment with the reviewer id F12, the audit-local F18. Review 6054646814 (da2ce7, changes requested, 09:08 UTC) read the head before the round 5 fix and posted two inline comments with the reviewer ids F13 and F14, the audit-local F19 and F20; they ask for the changes F16 and F17 already made. The review bodies list the same findings and add none.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3042-f1 | Unknown | Blocker | security | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3042-f2 | Unknown | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3042-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3042-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3042-f5 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3042-f6 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3042-f7 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3042-f8 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3042-f9 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3042-f10 | Unknown | Major | security | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3042-f11 | Unknown | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3042-f12 | Unknown | Major | security | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3042-f13 | Human | Major | security | RE_RAISE_OF:F12 | NO_ACTION | SUPERSEDED |
| F14 | review-finding:pr-3042-f14 | Human | Major | security | RE_RAISE_OF:F10 | NO_ACTION | SUPERSEDED |
| F15 | review-finding:pr-3042-f15 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3042-f16 | Unknown | Blocker | correctness | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3042-f17 | Unknown | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3042-f18 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-3042-f19 | Human | Major | correctness | RE_RAISE_OF:F16 | NO_ACTION | SUPERSEDED |
| F20 | review-finding:pr-3042-f20 | Human | Minor | documentation | RE_RAISE_OF:F17 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Encoded tokens reach logs

- PR number: 3042
- Source review ID: 6044736873
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4684413746>
- Concern: `redact_query` compared query parameter names before decoding them, but the server decodes them, so `/api/v1/stats?%74oken=<admin-token>` authenticates and was logged unredacted in the request event and the span.
- Solution: names are percent-decoded before they are compared, for query parameters and for the `announce`, `scrape` and `key` path segments; the logged text keeps the original spelling. Four cases were added to the redaction table (`%74oken`, `%74%6F%6B%65%6E`, `to%6ben`, `/%61nnounce/<key>`).
- Current-tree verification: `cargo test -p torrust-tracker-axum-server --lib request_log` failed on the four new cases before the change (21 passed, 4 failed) and passes after it (25 passed); on a running private tracker `GET /api/v1/stats?%74oken=TestAdminToken0001` returned the statistics and the log held `uri=/api/v1/stats?%74oken=***` and 0 lines with the token (section R1 of `docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/manual-verification-evidence.md`).
- Resolution reference: fix(axum-server): [#2999] decode query and path names before redacting the request URI
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4684525263>

### F2 - Imports break repository style

- PR number: 3042
- Source review ID: 6044736873
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4684413764>
- Concern: the code this branch added imported `Arc` inside a function and used full paths such as `std::fmt::Debug`, against the repository style (imports at the top, short names).
- Solution: the imports moved to the file headers with short names in `handler.rs`, `peer_key.rs`, `add_key_form.rs`, `databases/error.rs` and `test-helpers/src/logging.rs`. Behaviour is unchanged.
- Current-tree verification: `grep -n "std::fmt::\|std::borrow::Cow"` on those five files prints no full path in the added code (only the header imports of `databases/error.rs`); `cargo build --workspace --tests`, `cargo test --workspace --no-fail-fast` and the pre-commit gate pass.
- Resolution reference: fix(axum-server): [#2999] decode query and path names before redacting the request URI
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4684525681>

### F3 - `percent_decoded` decodes `%+1`

- PR number: 3042
- Source review ID: 6054012280
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692119143>
- Concern: `u8::from_str_radix` accepts a leading sign, so `percent_decoded` turned `%+1` into the byte 0x01 although its comment called such an escape malformed, and the evidence claimed a malformed-escape table case that did not exist. Nothing leaked: such an escape yields only 0x00 to 0x0F.
- Solution: the decoder takes an escape only when both characters after the `%` are ASCII hexadecimal digits; a test of the decoder and a table row (`%7=1&%+4=2`) were added; the evidence sentence is corrected (sections R1 and R2).
- Current-tree verification: `cargo test -p torrust-tracker-axum-server --lib request_log` failed on the sign case before the change (30 passed, 1 failed; `left: "\u{4}oken"`, `right: "%+4oken"`) and passes after it (32 passed).
- Resolution reference: fix(axum-server): [#2999] decode only a percent followed by two hex digits when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692248924>

### F4 - The query-parameter section shows the header command

- PR number: 3042
- Source review ID: 6054012280
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692119154>
- Concern: the `Query Parameter Method` section of the `use-rest-api` skill showed the same `Authorization: Bearer` command as the header section, so a reader was told the query method is the header.
- Solution: the section is headed "Query Parameter Method (accepted, not recommended)" and holds one paragraph (the server still accepts `?token=`; why the header is the safe form), with no command.
- Current-tree verification: `grep -n "Query Parameter Method" -A5 .claude/skills/use-rest-api/SKILL.md` prints that paragraph and no command.
- Resolution reference: fix(axum-server): [#2999] decode only a percent followed by two hex digits when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692249265>

### F5 - The configuration red run holds six tests, not seven

- PR number: 3042
- Source review ID: 6054012280
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692119162>
- Concern: `evidence/t7-red.txt` records six tests (1 passed, 5 failed), while the evidence said "five of seven"; the seventh test (the environment override) was never seen red and no mutation touched the rule it guards.
- Solution: section T7 of the evidence says five of the six tests the module had then, and that the seventh was added afterwards; a mutation for it is recorded (the `core.database.password` entry removed from `SECRET_OPTIONS`: it and the numeric-password case fail). The other red-run counts in the evidence were compared with their output files and match.
- Current-tree verification: the mutation run printed `5 passed; 2 failed` with `the error text holds the secret value` twice, and the restored file `7 passed` (`evidence/review-round-2.txt`); `grep -H "test result"` over the red-run files matches the counts the evidence states.
- Resolution reference: fix(axum-server): [#2999] decode only a percent followed by two hex digits when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692249686>

### F6 - Two word fragments in the project dictionary

- PR number: 3042
- Source review ID: 6054012280
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692119170>
- Concern: `nnounce` and `oken` are fragments of `%61nnounce` and `%74oken`; in the project dictionary they are accepted in every file, so a dropped letter passes the spelling check anywhere.
- Solution: both are removed from `project-words.txt` and ignored inline with `cspell:ignore` in `request_log.rs`, the evidence file and this record; `unredacted` stays.
- Current-tree verification: `grep -c '^nnounce$\|^oken$' project-words.txt` prints 0 and the pre-commit gate, which runs cspell, passes.
- Resolution reference: fix(axum-server): [#2999] decode only a percent followed by two hex digits when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692250021>

### F7 - Blank line at the end of two evidence files

- PR number: 3042
- Source review ID: 6054012280
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692119182>
- Concern: `git diff --check` reported a new blank line at the end of `evidence/t6-mutations.txt` and `evidence/t7-mutations.txt`.
- Solution: the trailing blank line of both files is removed.
- Current-tree verification: `git diff --check torrust/develop...HEAD` prints nothing and exits 0.
- Resolution reference: fix(axum-server): [#2999] decode only a percent followed by two hex digits when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692250349>

### F8 - Stale evidence date and unset `related-pr`

- PR number: 3042
- Source review ID: 6054012280
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692119186>
- Concern: `last-updated-utc` of the evidence file predated its last change, `ISSUE.md` kept `related-pr: null`, and its progress log had no entry for the pull request or the review round.
- Solution: both `last-updated-utc` values are current, `related-pr` is 3042, and the progress log has an entry for the pull request and the first round (marked as recorded late) and one for the second round.
- Current-tree verification: `grep -n '^related-pr\|^last-updated'` on both files prints `related-pr: 3042` and the dates of 2026-11-25.
- Resolution reference: fix(axum-server): [#2999] decode only a percent followed by two hex digits when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692250732>

### F9 - A test name promises a length it does not assert

- PR number: 3042
- Source review ID: 6054012280
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692119192>
- Concern: `it_should_count_characters_not_bytes_when_naming_the_received_length` asserted the characters message of a 32-character, 33-byte key, not a `got N` length.
- Solution: the test uses a 31-character, 32-byte key and asserts `expected 32 characters, got 31`; the former case keeps its assertion under the name `it_should_reject_a_32_character_key_with_a_non_ascii_character_for_its_characters`.
- Current-tree verification: with the length counting bytes (mutation, restored) both tests fail; with the production code `cargo test -p torrust-tracker-core --lib peer_key` passes (50 passed).
- Resolution reference: fix(axum-server): [#2999] decode only a percent followed by two hex digits when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692251259>

### F10 - Keep secret values out of TOML syntax errors

- PR number: 3042
- Source review ID: 6054287756
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692330999>
- Concern: `redact_one` redacts only typed value errors whose path matches a secret option; a TOML syntax error has no option path, so for a malformed password line the printed parser text can hold the password. The reviewer asked for the parse-error path to be covered and tested before `docs/containers.md` keeps its guarantee.
- Solution: confirmed in part. The `figment` error object for a syntax error did quote the source line and was left unchanged; it did not reach a user, because both loaders replace any parse error with a missing-mandatory-option error before extracting. The redaction now keeps the location of a syntax error and drops the quoted line. Two tests were added: one on `redact_secret_values` with a real syntax error, one through `Configuration::load`. The documentation sentence stays.
- Current-tree verification: `cargo test -p torrust-tracker-configuration --lib secret_options` failed on the new unit test before the change (8 passed, 1 failed; `the error text holds the secret value`) and the package passes after it (170 passed); starting the tracker with a broken password line prints 0 occurrences of the password.
- Resolution reference: fix(secrets): [#2999] redact a key after an empty path segment and the source line of a TOML syntax error
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692447625>

### F11 - Do not describe the header as safe without requiring TLS

- PR number: 3042
- Source review ID: 6054287756
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692331016>
- Concern: `docs/containers.md` called the `Authorization` header "the safe form", although it only keeps the token out of URLs and does not protect it in transit; the Azure example sends a bearer token over `http://`.
- Solution: the sentence states what the header does and that HTTPS is needed in transit; the Azure example, whose container serves no TLS, says so and warns not to send a real token over an untrusted network. The same wording was changed in `src/lib.rs`, `axum-rest-api-server` (`lib.rs`, `v1/middlewares/auth.rs`), the skills `handle-secrets` and `use-rest-api`, and the secrecy ADR amendment.
- Current-tree verification: `grep -rn "safe form"` over the Markdown and Rust files outside `docs/issues/` and `docs/pr-reviews/` prints nothing; the pre-commit gate passes.
- Resolution reference: fix(secrets): [#2999] redact a key after an empty path segment and the source line of a TOML syntax error
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692448018>

### F12 - Redact secret suffixes after empty path segments

- PR number: 3042
- Source review ID: 6054287756
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692331029>
- Concern: for `/announce//<key>` the segment right after `announce` is empty, so `redact_path` skipped the prefix and returned the whole key; the same for `/scrape//<key>` and `/api/v1/key//<key>`.
- Solution: `redact_path` redacts when anything that is not empty follows the prefix; `/announce`, `/announce/` and `/announce//` are unchanged. Four rows were added to the redaction table.
- Current-tree verification: `cargo test -p torrust-tracker-axum-server --lib request_log` failed on three new rows before the change (33 passed, 3 failed) and passes after it (36 passed); on a running tracker `GET /announce//<key>` and `DELETE /api/v1/key//<key>` were logged as `uri=/announce/***?port=1` and `uri=/api/v1/key/***` with 0 lines holding the key.
- Resolution reference: fix(secrets): [#2999] redact a key after an empty path segment and the source line of a TOML syntax error
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692448364>

### F13 - A key after an empty segment is logged in full

- PR number: 3042
- Source review ID: 6054388295
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692413532>
- Concern: `/announce//<key>`, `/scrape//<key>` and `/api/v1/key//<key>` are logged with the key, because `has_a_secret_after_it` looks only at the segment right after the prefix.
- Solution: no further change. It is the change F12 made, in a commit pushed after the head this review read.
- Current-tree verification: `redact_path` uses `segments[index + 1..].iter().any(|later| !later.is_empty())`, and the redaction table holds the rows for the three double-slash paths; `cargo test -p torrust-tracker-axum-server --lib request_log` passed with them (36 passed) in round 3.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692540318>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692540318>

### F14 - The containers guide overstates the configuration-error guarantee

- PR number: 3042
- Source review ID: 6054388295
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692413539>
- Concern: "A configuration error names the option and never prints the password" is not true for a TOML syntax error on the password line, whose parser text quotes the source line (read from the locked crates, not run).
- Solution: no further change. It is the change F10 made, in a commit pushed after the head this review read: the redaction drops the quoted line of a syntax error. The reply also records that the text does not reach stderr at either head, because the loaders replace a parse error with a missing-mandatory-option error.
- Current-tree verification: the tracker binary built after the round 3 fix, started with a configuration whose last line is an unterminated `password = "TestDbPasswordUnterminated01`, ended with `Missing mandatory configuration option. Option path: metadata.schema_version` and its output held 0 occurrences of the password.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692540743>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692540743>

### F15 - The progress log is not in stamp order

- PR number: 3042
- Source review ID: 6054388295
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692413546>
- Concern: an entry of the spec's progress log stamped 08:28 came before one stamped 08:27.
- Solution: the entry recorded late (08:28) was moved after the 08:27 entry.
- Current-tree verification: `grep -n '^- 2026-11-2[45]' docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md` prints the stamps 18:00, 08:27, 08:28, 08:47 and 08:56 in that order.
- Resolution reference: docs(issues): [#2999] put the progress log entries in stamp order
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692541219>

### F16 - Slash-heavy requests waste CPU

- PR number: 3042
- Source review ID: 6054462408
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692474095>
- Concern: `redact_path` scanned every remaining segment for each segment, so a path with thousands of trailing slashes made the loop do quadratic work in the request logging callback, also for a 404 and without authentication.
- Solution: `redact_path` finds the last segment that is not empty once and looks for a prefix only before it; the empty-segment fix of F12 is kept. Two tests with a run of 60000 slashes were added.
- Current-tree verification: with a temporary timing print in a test build, a slash-only path of 60000 bytes took 657.9 ms before the change and 1.0 ms after it (`evidence/review-round-5.txt`); `cargo test -p torrust-tracker-axum-server --lib request_log` passes (38 passed); on a running tracker a `GET` of 30000 slashes answered 404 in 0.0014 s.
- Resolution reference: fix(axum-server): [#2999] find the end of the secret path once when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692673032>

### F17 - Header commands remain in shell history

- PR number: 3042
- Source review ID: 6054462408
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692474106>
- Concern: the `use-rest-api` skill said the header keeps the token out of logs that included the shell history, but the shown `curl -H "Authorization: Bearer ..."` command puts the token in the shell history when typed.
- Solution: the sentence names proxy, browser and server logs only; a new paragraph says neither form keeps a typed token out of the shell history and shows reading it from a file inside the command.
- Current-tree verification: `grep -rn "shell-history"` over the Markdown and Rust files outside `docs/issues/` and `docs/pr-reviews/` prints nothing; `grep -n shell .claude/skills/use-rest-api/SKILL.md` prints only the new paragraph.
- Resolution reference: fix(axum-server): [#2999] find the end of the secret path once when redacting
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692673354>

### F18 - Six review threads have no audit row

- PR number: 3042
- Source review ID: 6054537873
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692534562>
- Concern: at the head the review read, the three threads of review 6054287756 were answered with replies naming F10 to F12 but had no row, and the three threads of review 6054388295 had neither a reply nor a row.
- Solution: the rows F10 to F12 and F13 to F15 were committed in the third and fourth audit commits, and the three threads of review 6054388295 were answered and resolved. The cause is recorded in the reply: each round's replies were posted after its fix was pushed and before its audit commit was pushed.
- Current-tree verification: `grep -c '^| F1[0-5] ' docs/pr-reviews/pr-3042-review/PR-REVIEW.md` prints 6 at the pushed head.
- Resolution reference: docs(pr-reviews): [#2999] record the fourth review round in the PR 3042 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692673761>

### F19 - `redact_path` is quadratic in the number of empty segments

- PR number: 3042
- Source review ID: 6054646814
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692618613>
- Concern: for each segment, `segments[index + 1..].iter().any(..)` walks the rest of the path, which is quadratic on a run of empty segments, for every request, before routing and without a credential.
- Solution: no further change. It is the change F16 made, in a commit pushed after the head this review read.
- Current-tree verification: `grep -n "rposition" packages/axum-server/src/request_log.rs` prints the single search for the last segment that is not empty, and the loop runs over `segments[..last_not_empty]`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692674221>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692674221>

### F20 - The header does not keep a typed token out of shell history

- PR number: 3042
- Source review ID: 6054646814
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692618621>
- Concern: "it keeps the token out of those logs" included the shell history, where a typed header lands just as a URL does.
- Solution: no further change. It is the change F17 made, in a commit pushed after the head this review read.
- Current-tree verification: `grep -n shell .claude/skills/use-rest-api/SKILL.md` prints only the paragraph that says neither form keeps a typed token out of the shell history.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692674670>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3042#discussion_r4692674670>

## Processing Log

- 2026-11-24 18:06 UTC - Greptile review 6044736873 arrived with two inline findings.
- 2026-11-24 18:16 UTC - Replied on both threads after the fix commit was pushed (replies 4684525263 and 4684525681).
- 2026-11-24 18:20 UTC - `validate-audit-record.py --pr-number 3042` run with the bot token set for that one command (identity `josecelano-bot`) printed `{"status": "ok", "rows": 2, "log_entries": 2, "failures": 0}`. A refreshed GraphQL listing showed the F1 thread already resolved by `greptile-apps[bot]`; the F2 thread was then resolved by `josecelano-bot`. No unresolved thread remains.
- 2026-11-25 08:18 UTC - Review 6054012280 by da2ce7 (approved) arrived with seven inline findings, recorded as F3 to F9.
- 2026-11-25 08:32 UTC - Replied on the seven threads after the fix commit was pushed (replies 4692248924, 4692249265, 4692249686, 4692250021, 4692250349, 4692250732 and 4692251259).
- 2026-11-25 08:33 UTC - `validate-audit-record.py --pr-number 3042`, run with the bot token set for that one command (identity `josecelano-bot`), printed `{"status": "ok", "rows": 9, "log_entries": 5, "failures": 0}`. The seven threads were then resolved by `josecelano-bot`; a refreshed GraphQL listing shows the nine threads of the pull request resolved.
- 2026-11-25 08:40 UTC - Review 6054287756 by CodeRabbit arrived with three inline findings, recorded as F10 to F12.
- 2026-11-25 08:52 UTC - Replied on the three threads after the fix commit was pushed (replies 4692447625, 4692448018 and 4692448364).
- 2026-11-25 08:53 UTC - `validate-audit-record.py --pr-number 3042`, run with the bot token set for that one command (identity `josecelano-bot`), printed `{"status": "ok", "rows": 12, "log_entries": 8, "failures": 0}`. A GraphQL listing showed the F11 thread already resolved by `coderabbitai[bot]`; the F10 and F12 threads were then resolved by `josecelano-bot`. The same listing showed three new unresolved threads of a later review by da2ce7 (6054388295), which are not processed in this entry.
- 2026-11-25 09:00 UTC - Review 6054388295 by da2ce7 (changes requested, submitted at 08:49 UTC) with three inline findings was recorded as F13 to F15, and the three threads got their replies after the log-order commit was pushed (replies 4692540318, 4692540743 and 4692541219).
- 2026-11-25 09:02 UTC - `validate-audit-record.py --pr-number 3042`, run with the bot token set for that one command (identity `josecelano-bot`), first failed with `Processing Log entries are not in chronological order`: this record had two new entries, one stamped 08:49 for the arrival of the review and one stamped 09:00 for the replies, after the 08:53 entry. They were merged into the single 09:00 entry above before any commit, and the second run printed `{"status": "ok", "rows": 15, "log_entries": 10, "failures": 0}`. The F13, F14 and F15 threads were then resolved by `josecelano-bot`. A GraphQL listing taken before resolving showed three more unresolved threads, two by Greptile and one by da2ce7, which are not processed in this entry.
- 2026-11-25 09:13 UTC - Reviews 6054462408 (Greptile, submitted at 08:55 UTC), 6054537873 (da2ce7, 09:00 UTC) and 6054646814 (da2ce7, 09:08 UTC) were recorded as F16 to F20, and their five threads got their replies after the round 5 fix commit was pushed (replies 4692673032, 4692673354, 4692673761, 4692674221 and 4692674670).
- 2026-11-25 09:14 UTC - `validate-audit-record.py --pr-number 3042`, run with the bot token set for that one command (identity `josecelano-bot`), printed `{"status": "ok", "rows": 20, "log_entries": 12, "failures": 0}`. The five threads were then resolved by `josecelano-bot`; a refreshed GraphQL listing shows 20 threads and 0 unresolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
