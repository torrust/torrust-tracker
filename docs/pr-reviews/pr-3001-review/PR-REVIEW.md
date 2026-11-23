---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3001 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3001.

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

F1 and F2 come from Copilot's review 6021313664 (COMMENTED on the first head, "docs(issues): [#2917] record the SI-5 verification evidence and the configuration 4.0.0 breaking change"), in source order. Copilot noted that its full agentic review did not start before its timeout; the two findings are its inline threads. F3 to F6 come from Copilot's review 6022021456 (COMMENTED on the round-1 head, "docs(pr-reviews): [#2917] add the review audit of PR #3001", with ACK comments of the maintainer on the first head and on that head), in source order; its overview lists F1 and F2 as resolved. F7 to F13 come from da2ce7's review 6022459738 (APPROVED on the round-2 head, "docs(pr-reviews): [#2917] record round 2 of the review audit of PR #3001"), in source order; the reviewer numbered them F1 to F7 and each entry's "Reviewer finding ID" keeps that number. F14 and F15 come from Copilot's review 6022662894 (COMMENTED on the round-2 head, with no ACK of its own), in source order; Copilot's overview there lists the earlier findings as resolved. F16 comes from CodeRabbit's review 6022628130 (COMMENTED on the head "docs(issues): [#2917] repeat the recheck after the last production change and align the spec rows and metadata"), which the poller does not watch; it is placed after F14 and F15 because I found it late.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3001-f1 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3001-f2 | Copilot | Nit | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-3001-f3 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3001-f4 | Copilot | Minor | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F5 | review-finding:pr-3001-f5 | Copilot | Minor | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-3001-f6 | Copilot | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3001-f7 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3001-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3001-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3001-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3001-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3001-f12 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3001-f13 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F14 | review-finding:pr-3001-f14 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3001-f15 | Copilot | Minor | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F16 | review-finding:pr-3001-f16 | Unknown | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3001-f17 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3001-f18 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-3001-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-3001-f20 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-3001-f21 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-3001-f22 | Unknown | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-3001-f23 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-3001-f24 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-3001-f25 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-3001-f26 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-3001-f27 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-3001-f28 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The authentication classification test repeats the error's display string as a literal

- PR number: 3001
- Source review ID: 6021313664
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665512621
- Concern: Copilot (rated Medium, recorded as Minor) noted that `it_should_classify_an_authentication_error` in `packages/udp-server/src/event.rs` repeats the `AuthRequired` display string as a literal, although the conversion now uses `error.to_string()`; it suggested comparing against `Error::AuthRequired.to_string()` so a wording change does not need edits at several sites.
- Solution: The assertion now compares with `ErrorKind::TrackerAuthentication(Error::AuthRequired.to_string())`. The wording stays pinned once, in `it_should_not_hold_a_source_path_when_an_authentication_required_error_is_displayed` in `error.rs`.
- Current-tree verification: `grep -n "AuthRequired" packages/udp-server/src/event.rs` shows the comparison with `Error::AuthRequired.to_string()`; `cargo test -p torrust-tracker-udp-server --lib` (stable Rust toolchain) passes 226 tests.
- Resolution reference: test(udp-server): [#2917] compare the authentication classification with the error's own message
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665567589

### F2 - The issue status is `in-progress` while the task table is `DONE`

- PR number: 3001
- Source review ID: 6021313664
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665512674
- Concern: Copilot (rated Low, recorded as Nit) read the front-matter `status: in-progress` of the spec as inconsistent with the `DONE` tasks and acceptance criteria and suggested `done`.
- Solution: No change. The tasks and acceptance criteria describe the implementation, which is finished; the workflow checkpoints for review validation, the committer check and the archive stay unticked until the PR merges. The repository's close-out step (as in SI-2 to SI-4) moves the spec to `docs/issues/closed/` and sets `status: done` then; setting it earlier would claim a closed issue while the PR is open.
- Current-tree verification: `grep -n "^status:" docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` prints `status: in-progress`; `grep -n "Reviewer validated" docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` shows that checkpoint unticked.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665567937
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665567937

### F3 - The severity values of the findings table are not the documented ones

- PR number: 3001
- Source review ID: 6022021456
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666073875
- Concern: Copilot (rated Medium, recorded as Minor) noted that the findings table of this audit used `Medium` and `Low`, which are not in the documented set (`Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`).
- Solution: The table now uses the documented values, mapping Copilot's Medium to Minor and Low to Nit as the audit of PR #2991 did, and the detail entries say so. A sweep of the same mistake found no other non-documented value in this audit.
- Current-tree verification: `grep -n "^| F" docs/pr-reviews/pr-3001-review/PR-REVIEW.md` lists only `Minor` and `Nit`; `validate-audit-record.py --pr-number 3001` (run with the bot identity) reports 0 failures.
- Resolution reference: docs(pr-reviews): [#2917] record round 2 of the review audit of PR #3001
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666135737

### F4 - The `axum-server` path check lacks backslash variants for the registry and standard-library paths

- PR number: 3001
- Source review ID: 6022021456
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666073945
- Concern: Copilot (rated Medium, recorded as Minor) noted that the `assert_names_no_source_path` copy in `packages/axum-server/src/tls.rs` forbids `.cargo/registry` and `rustlib/src` but not their Windows spellings `.cargo\\registry` and `rustlib\\src`.
- Solution: No change. A Windows registry or standard-library source path always continues with a `src\\` component (`...\\.cargo\\registry\\src\\<index>\\<crate>\\src\\...`, `...\\rustlib\\src\\rust\\library\\...`), and the helper already forbids `src\\`, so both spellings are caught. The list is the one the SI-5 specification fixes (Regression Test Strategy, "Portable path check") and the same as the shared helper that SI-8 (#2996) moves into `torrust-tracker-test-helpers`.
- Current-tree verification: `grep -n 'src' packages/axum-server/src/tls.rs` shows `"src\\"` in the helper's list.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666136252
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666136252

### F5 - The `configuration` path check lacks the same backslash variants

- PR number: 3001
- Source review ID: 6022021456
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666074007
- Concern: The same request as F4 for the copy of `assert_names_no_source_path` in `packages/configuration/src/lib.rs`, where it is the first of two copies reported by Copilot.
- Solution: No change, for the reason given on F4: `src\\` is in the list and catches the Windows registry and standard-library paths.
- Current-tree verification: `grep -n 'src' packages/configuration/src/lib.rs` shows `"src\\"` in the helper's list.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666136697
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666136697

### F6 - `error @ Error::AuthRequired` is an unnecessary binding

- PR number: 3001
- Source review ID: 6022021456
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666074088
- Concern: Copilot (rated Low, recorded as Nit) noted that the `ErrorKind` conversion in `packages/udp-server/src/event.rs` binds the unit variant with `error @` only to call `to_string()` on it, and asked for the direct pattern.
- Solution: The arm now matches `Error::AuthRequired` and builds the message with `Error::AuthRequired.to_string()`.
- Current-tree verification: `grep -n "AuthRequired" packages/udp-server/src/event.rs` shows the direct pattern; `cargo test -p torrust-tracker-udp-server --lib` (stable Rust toolchain) passes 226 tests and `cargo clippy -p torrust-tracker-udp-server --all-targets -- -D warnings` is clean.
- Resolution reference: refactor(udp-server): [#2917] match the unit variant AuthRequired directly
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666137078

### F7 - SI-8 as specified will not share the two helper copies of this PR

- PR number: 3001
- Source review ID: 6022459738
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666416864
- Concern: da2ce7 (Minor) noted that the progress log and the PR body said each crate keeps its copy of `assert_names_no_source_path` "until SI-8 (#2996) shares it", although the SI-8 spec lists seven copies (none in `configuration` or `axum-server`), adds no dependency edge, and `torrust-tracker-test-helpers` depends on `torrust-tracker-configuration` (a cycle for the `configuration` copy) while `axum-server` has no `test-helpers` dev-dependency. There are nine copies at this head, so SI-8's counts, cross-link plan and EPIC row 8 were stale. He suggested recording copies 8 and 9 in the SI-8 spec and the EPIC row (as cross-linked copies) and rewording the line and the PR body.
- Solution: The SI-8 spec lists copies 8 (`configuration`) and 9 (`axum-server`) as staying and cross-linked, and its goal, scope, acceptance criteria and risks read nine copies to five definitions; EPIC row 8 says the same and names SI-5 among the sub-issues that add copies; the progress-log line of the SI-5 spec and the PR body no longer promise that SI-8 shares them.
- Current-tree verification: `grep -n "| 8 \|| 9 " docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` shows both rows; `grep -n "nine copies" docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` shows row 8; `gh pr view 3001` body carries the new sentence.
- Resolution reference: docs(issues): [#2917] record the SI-5 copies of the path helper in the SI-8 spec and the EPIC
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666514765

### F8 - The 4.0.0 list for `configuration` leaves out the SI-20 break

- PR number: 3001
- Source review ID: 6022459738
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666416878
- Concern: da2ce7 (Minor) noted that the lead-in listed the breaking changes of `configuration` for 4.0.0 besides the #2454 marks and the `Error::Infallible` removal, while the same file records that SI-20 (#2668) added the public field `Configuration::application`, a breaking change for the 4.0.0 notes; a reader compiling the notes would miss it.
- Solution: The lead-in now also names the SI-20 field and links the section that records it. The publish plan also gained the progress-log line it lacked, and its `last-updated-utc` moved (the same commit).
- Current-tree verification: `grep -n "Configuration::application" docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` shows the lead-in and the original record.
- Resolution reference: docs(issues): [#2917] name the SI-20 break in the configuration 4.0.0 list of the publish plan
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666515277

### F9 - The recheck and the green run predate the last production change, and no build is named

- PR number: 3001
- Source review ID: 6022459738
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666416886
- Concern: da2ce7 (Minor) noted that the "after" captures and `tests-green.txt` were taken before two later commits that changed code (the test comparison and the `AuthRequired` arm), and that the `fix-bug` skill asks for both runs to be repeated and for the evidence to name the build.
- Solution: V1, V2 and the tests of `configuration`, `axum-server` and `udp-server` were run again after the last production change (`evidence/startup-after-final.txt`, `evidence/tests-green-final.txt`), and `manual-verification-evidence.md` now has an "Artifact under test" paragraph naming the builds by commit subject. The messages and test results are unchanged.
- Current-tree verification: `grep -c "\.rs:" docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/evidence/startup-after-final.txt` prints 0, and `evidence/tests-green-final.txt` has only `test result: ok` lines.
- Resolution reference: docs(issues): [#2917] repeat the recheck after the last production change and align the spec rows and metadata
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666515752

### F10 - The `axum-server` README still lists `torrust-located-error` as a dependency of `tls.rs`

- PR number: 3001
- Source review ID: 6022459738
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666416897
- Concern: da2ce7 (Minor; outside the diff) noted that `packages/axum-server/README.md` lines 39-40 said the TLS helper depends on `LocatedError` / `DynError` from `torrust-located-error`, which neither `tls.rs` nor the manifest uses any more.
- Solution: The bullet is removed. A sweep for the same statement found no other mention of the crate in the READMEs of `configuration` and `axum-server`.
- Current-tree verification: `grep -rn "located" packages/axum-server/README.md packages/configuration/README.md` finds nothing.
- Resolution reference: docs(axum-server): [#2917] drop the README bullet on torrust-located-error
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666516247

### F11 - Two DONE rows describe tests that were not written

- PR number: 3001
- Source review ID: 6022459738
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666416903
- Concern: da2ce7 (Nit) noted that the acceptance-verification row for AC1 cited tests `it_should_not_hold_a_source_path_*` in `tls.rs` and `event.rs`, which do not exist there, and that T2 promised tests for the four configuration variants while two were written.
- Solution: T2 now lists the tests that were written (two `configuration` tests, the path check in `it_should_error_on_bad_tls_config`, two UDP `Display` tests, the pinned message in `it_should_classify_an_authentication_error`) and says that `UnableToLoadFromEnvironmentVariable` and `UnableToLoadFromConfigFile` are never built and have none; the AC1 row names the actual tests.
- Current-tree verification: `grep -n "^| T2 \|^| AC1 " docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` shows the new text.
- Resolution reference: docs(issues): [#2917] repeat the recheck after the last production change and align the spec rows and metadata
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666516699

### F12 - The spec metadata and the publish plan's log were not updated

- PR number: 3001
- Source review ID: 6022459738
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666416906
- Concern: da2ce7 (Nit) noted that `related-pr` was `null` and `last-updated-utc` still read 2026-11-18, and that the publish plan had gained its 4.0.0 entry with no progress-log line and no date change.
- Solution: `related-pr` is `3001` and `last-updated-utc` is current in the SI-5 spec; the publish plan has its progress-log line and a new `last-updated-utc` (in the commit cited under F8).
- Current-tree verification: `sed -n 11,12p docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` shows both; `grep -n "Issue #2917" docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md` shows the log line.
- Resolution reference: docs(issues): [#2917] repeat the recheck after the last production change and align the spec rows and metadata
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666517224

### F13 - Two commits do not match their scope

- PR number: 3001
- Source review ID: 6022459738
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666416914
- Concern: da2ce7 (Nit) noted that the commit "fix(configuration): [#2917] remove source locations from configuration errors" drops the dependency from the manifest but leaves its `Cargo.lock` entry (removed only in "fix(axum-server): [#2917] remove source locations from TLS errors"), so a `--locked` build of that commit stops on the stale lockfile, and that the red-test commit also adds the `axum-server` and `udp-server` red tests. The final tree is right; only bisecting is affected.
- Solution: No change. The repository keeps intermediate commits and does not rewrite a pull request's history (`open-pull-request`, "Keep intermediate commits"), and the final tree is correct. The two commits stay as they are.
- Current-tree verification: `git diff --stat torrust/develop...HEAD -- Cargo.lock` shows the lockfile change that the final tree carries.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666517629
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666517629

### F14 - The test of the missing TLS files does not check that the message holds no source path

- PR number: 3001
- Source review ID: 6022662894
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666579091
- Concern: Copilot (rated Medium, recorded as Minor) noted that `it_should_error_on_missing_cert_or_key_paths` does not assert that `MissingTlsConfig`'s message holds no source path, although the variant previously carried a caller location, and asked for the same check as in `it_should_error_on_bad_tls_config`.
- Solution: The test now also calls `assert_names_no_source_path(&err.to_string())`. The message had no location before either (its format never printed the field), so the assertion guards against a regression and was not a red test.
- Current-tree verification: `grep -n "assert_names_no_source_path(&err.to_string())" packages/axum-server/src/tls.rs` shows both uses; `cargo test -p torrust-tracker-axum-server --lib` (stable Rust toolchain) passes 9 tests.
- Resolution reference: test(axum-server): [#2917] check that the missing TLS files message holds no source path
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666636036

### F15 - `ErrorKind::TrackerAuthentication` is tied to the `Display` text

- PR number: 3001
- Source review ID: 6022662894
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666579162
- Concern: Copilot (rated Medium, recorded as Minor) noted that `ErrorKind::TrackerAuthentication` is now built from `Error`'s `Display` text and suggested a stable message constant, or a stable code plus a human message, so that wording changes cannot break a client-visible surface.
- Solution: No change. `AuthRequired` is built only in `#[cfg(test)]` code, so no production event carries it today. The payload was `location.to_string()` before this PR, a source path that is less stable than the message; the other `ErrorKind` arms (`BadRequest`, `Whitelist`, `Database` and the rest) carry their error's `Display` text in the same way. A stable code and message for error events is the work of the general error-events EPIC that the module's own doc comment names, not of this sub-issue; the two test-only variants are the subject of the follow-up the spec lists as out of scope.
- Current-tree verification: `grep -n "TrackerAuthentication\|BadRequest(\|Whitelist(" packages/udp-server/src/event.rs` shows the sibling arms built from `to_string()`; `grep -rn "AuthRequired" packages --include=*.rs` finds no use outside `error.rs` and `event.rs` tests and conversion.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666636634
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666636634

### F16 - Tasks T1 and T5 of the SI-8 spec name only copies 1 and 2

- PR number: 3001
- Source review ID: 6022628130
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666550639
- Concern: CodeRabbit (rated Minor) noted that the SI-8 spec's scope now requires each of the five definitions to name the other four, but tasks T1 and T5 still named only copies 1 and 2, so following them would leave copies 8 and 9 without cross-links and fail AC3.
- Solution: Resolved by mistake in a batch resolve before this reply and reopened, then handled; GitHub shows the thread resolved after the reply (not by a resolve command of this session). T1 and T5 now cover the other four definitions (copies 1, 2, 8 and 9) and say to change all five together. A sweep of the spec for other leftovers of the old counts (`grep -n "seven\|three\|two other\|two other definitions" docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md`) found the sentence of the In Scope bullet that said the shared function's doc comment names "the two other definitions", which now says the four other definitions by path; only the seven fragments of the helper remain.
- Current-tree verification: `grep -n "^| T1 \|^| T5 " docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` shows the new rows.
- Resolution reference: docs(issues): [#2917] extend tasks T1 and T5 of the SI-8 spec to all five definitions
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666716838

### F17 - The SI-8 option row and rationale still say two copies

- PR number: 3001
- Source review ID: 6023246827
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4667040607
- Concern: Copilot (rated Low, recorded as Nit) noted that the chosen option row and its rationale still describe two copies, while the scope and AC3 require four.
- Solution: The row now says "cross-link the four that cannot" and "the four copies"; the rationale says "the last four copies". Same text as F19.
- Current-tree verification: `grep -n "two copies\|last two\|the two that" docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` finds nothing.
- Resolution reference: docs(issues): [#2917] fix the SI-8 option rationale, dates and provenance wording after review round 5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668280193

### F18 - F14 and F15 record the wrong source severity

- PR number: 3001
- Source review ID: 6023021299
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666865659
- Concern: da2ce7 (Minor) noted that F14 and F15 were recorded as Nit with "rated Low", while the overview of Copilot review 6022662894 marks both Medium.
- Solution: Both rows are now Minor and both Concern lines say "rated Medium, recorded as Minor".
- Current-tree verification: `grep -n "^| F14 \|^| F15 " docs/pr-reviews/pr-3001-review/PR-REVIEW.md` shows Minor.
- Resolution reference: docs(pr-reviews): [#2917] record round 6 of the review audit of PR #3001
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668280696

### F19 - The option-E rationale still says two, and "18 files" is stale

- PR number: 3001
- Source review ID: 6023021299
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666865668
- Concern: da2ce7 (Nit) noted that the option-E row and the sentence after the table still say two, and that line 69 says 18 files while 21 files call or define the helper.
- Solution: The row and the sentence say four, and line 69 says 21 files (`grep -rln assert_names_no_source_path packages | wc -l` gives 21). The earlier sweep missed these phrases; the new sweep greps "two" in the whole spec.
- Current-tree verification: `grep -n "two copies\|last two\|18 files" docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` finds nothing.
- Resolution reference: docs(issues): [#2917] fix the SI-8 option rationale, dates and provenance wording after review round 5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668281256

### F20 - The F16 sentence is duplicated, and one log time is off

- PR number: 3001
- Source review ID: 6023021299
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666865681
- Concern: da2ce7 (Nit) noted that the origin of F16 is stated twice in the intro paragraph and placed before the F14 and F15 sentence although it says it is last, and that the 13:51 correction gives 13:44:52 for the batch resolve while the 13:45 entry gives 13:45:33.
- Solution: The duplicate sentence is removed, the remaining one is moved after the F14 and F15 sentence and says it is placed after them; the correction now says the resolve ran at 13:45:33 and 13:44:52 was the time of the replies.
- Current-tree verification: `grep -o "F16 comes fro[m]" docs/pr-reviews/pr-3001-review/PR-REVIEW.md | wc -l` prints 1.
- Resolution reference: docs(pr-reviews): [#2917] record round 6 of the review audit of PR #3001
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668281799

### F21 - A few dates and provenance lines do not match the record

- PR number: 3001
- Source review ID: 6023021299
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666865693
- Concern: da2ce7 (Nit) noted that the evidence says the first after captures were taken at a commit build that did not yet exist, that three `last-updated-utc` values were stale, that the 11:21 progress-log entry was rewritten instead of appended to, and that the 13:26 audit entry ends in a double period.
- Solution: The evidence says "a working tree matching" the commit; the evidence file, the SI-5 and SI-8 specs and the EPIC carry the current `last-updated-utc`; the 11:21 entry is restored to its first text and the SI-8 record is a new 15:53 line; the double period is removed.
- Current-tree verification: `grep -n "last-updated-utc" docs/issues/open/2996*/ISSUE.md docs/issues/open/2912*/EPIC.md` shows 2026-11-23 15:53.
- Resolution reference: docs(issues): [#2917] fix the SI-8 option rationale, dates and provenance wording after review round 5; docs(issues): [#2917] use the real time in the updated metadata; docs(pr-reviews): [#2917] record round 6 of the review audit of PR #3001
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668282221

### F22 - Row F21 and the audit say 15:55 where the files read 15:53

- PR number: 3001
- Source review ID: 6024928625
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668369792
- Concern: Greptile (P2, recorded as Minor) noted that row F21 and the audit say 15:55 where the files read 15:53.
- Solution: The Solution and the verification of row F21 now say 15:53.
- Current-tree verification: `grep -n "15:55" docs/pr-reviews/pr-3001-review/PR-REVIEW.md` finds only text that quotes the old value.
- Resolution reference: docs(pr-reviews): [#2917] cite the commits that fixed rows F18, F20 and F21 and correct the times in row F21
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668814828

### F23 - Rows F17 to F21 cite a resolution commit that did not touch this record

- PR number: 3001
- Source review ID: 6024983512
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668414012
- Concern: da2ce7 (Minor) noted that rows F17 to F21 cite as their resolution a commit that did not touch this record.
- Solution: Rows F18 and F20 cite the round 6 audit commit; row F21 cites the three commits, separated by semicolons; rows F17 and F19 cite the SI-8 spec commit, which is right.
- Current-tree verification: The five `Resolution reference` lines of rows F17 to F21 show the subjects.
- Resolution reference: docs(pr-reviews): [#2917] cite the commits that fixed rows F18, F20 and F21 and correct the times in row F21
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668815383

### F24 - Three replies still cite branch commits by id

- PR number: 3001
- Source review ID: 6024983512
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668414018
- Concern: da2ce7 (Nit) noted that three replies cite branch commits by id.
- Solution: The three replies were edited (GitHub allows the author to edit a reply) and now name the commits by subject.
- Current-tree verification: `gh-bot api repos/torrust/torrust-tracker/pulls/comments/4668282221 --jq .body` shows no id.
- Resolution reference: docs(pr-reviews): [#2917] record round 7 of the review audit of PR #3001
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668815930

### F25 - Row F21 and the newest log entry carry times the head does not match

- PR number: 3001
- Source review ID: 6024983512
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668414021
- Concern: da2ce7 (Nit) noted that row F21 says 15:55 where the head has 15:53, that the 15:58 log entry is stamped after the commit that adds it, and that the 13:26 and 13:51 entries were edited in place instead of corrected by an appended entry.
- Solution: Row F21 says 15:53. The appended entry of this round names the 15:58 stamp (the commit is 15:57:04) and the in-place edits of the 13:26 and 13:51 entries, which are left as they are.
- Current-tree verification: The last entry of the Processing Log.
- Resolution reference: docs(pr-reviews): [#2917] cite the commits that fixed rows F18, F20 and F21 and correct the times in row F21; docs(pr-reviews): [#2917] record round 7 of the review audit of PR #3001
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668816541

### F26 - Say that the 15:53 line corrects the 11:21 claim

- PR number: 3001
- Source review ID: 6024983512
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668414029
- Concern: da2ce7 (Suggestion, recorded as Nit) asked that the 15:53 progress-log line say that it corrects the 11:21 claim and name the commit that recorded the SI-8 copies.
- Solution: A new 16:40 progress-log line says so, naming that commit by subject.
- Current-tree verification: `grep -n "supersedes the claim" docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` shows the line.
- Resolution reference: docs(issues): [#2917] note that the 15:53 line corrects the 11:21 one and name SI-A for secret values
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668817068

### F27 - The five threads of the last round are unanswered

- PR number: 3001
- Source review ID: 6025316233
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668686764
- Concern: da2ce7 (Minor) noted that the five threads of the last round had no reply, row or resolution, which holds his verdict at COMMENT.
- Solution: All five threads and the two of review 6025316233 now have rows F22 to F28, replies and resolutions.
- Current-tree verification: This table lists rows F22 to F28; the validator reports every thread covered.
- Resolution reference: docs(pr-reviews): [#2917] record round 7 of the review audit of PR #3001
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668817676

### F28 - Record the EPIC #2993 interaction with AC2

- PR number: 3001
- Source review ID: 6025316233
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668686785
- Concern: da2ce7 (Suggestion, recorded as Nit) asked that the spec name SI-A as the owner of the secret-value case of AC2, and noted that line 86 of EPIC #2993 becomes stale.
- Solution: A Risks bullet names SI-A (#2999). EPIC #2993 line 86 belongs to that EPIC's owner, who refreshes the inventory after this merge; it is not edited here.
- Current-tree verification: `grep -n "AC2 and secret values" docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` shows the bullet.
- Resolution reference: docs(issues): [#2917] note that the 15:53 line corrects the 11:21 one and name SI-A for secret values
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4668818262

## Processing Log

- 2026-11-23 12:00 UTC - Copilot's review 6021313664 (COMMENTED, submitted 12:00:15Z on the first head) arrived with two inline findings, F1 (Medium, recorded as Minor) and F2 (Low, recorded as Nit). The audit was started before the first reply. F1 fixed in "test(udp-server): [#2917] compare the authentication classification with the error's own message", pushed at 12:04 UTC (pre-push hook passed). F2 declined with a reason (the spec `status` becomes `done` at close-out).
- 2026-11-23 12:06 UTC - Replied on both threads (replies posted 12:05:34 UTC by `date -u`) and resolved both (`resolveReviewThread` returned `true` for each; `date -u` after the resolve commands: 12:06:00). GraphQL refresh: zero unresolved threads on PR #3001.
- 2026-11-23 12:53 UTC - Copilot's review 6022021456 (COMMENTED, submitted 12:53:25Z on the round-1 head) arrived with four inline findings, recorded as F3 to F6; its overview lists F1 and F2 as resolved. All CI checks of that head passed. F6 fixed in "refactor(udp-server): [#2917] match the unit variant AuthRequired directly", pushed at 12:58 UTC (pre-push hook passed). F3 fixed in this audit's round-2 commit; F4 and F5 declined with a reason.
- 2026-11-23 13:00 UTC - Replied on the four threads (replies posted 12:59:28 UTC by `date -u`). The threads are resolved after the audit commit is pushed.
- 2026-11-23 13:03 UTC - Resolved the four round-2 threads after the audit commit was pushed (`resolveReviewThread` returned `true` for each; `date -u` after the resolve commands: 13:03:21). GraphQL refresh: zero unresolved threads. This entry completes the round-2 entry above, which said the threads were resolved after the push.
- 2026-11-23 13:26 UTC - da2ce7's review 6022459738 (APPROVED, submitted on the round-2 head) arrived with seven findings, recorded as F7 to F13 with the reviewer's F1 to F7. F7 to F12 fixed in "docs(axum-server): [#2917] drop the README bullet on torrust-located-error", "docs(issues): [#2917] record the SI-5 copies of the path helper in the SI-8 spec and the EPIC", "docs(issues): [#2917] name the SI-20 break in the configuration 4.0.0 list of the publish plan" and "docs(issues): [#2917] repeat the recheck after the last production change and align the spec rows and metadata" (pushed at 13:31 UTC, pre-push hook passed); the PR body was edited for F7. F13 declined with a reason (intermediate commits are kept).
- 2026-11-23 13:35 UTC - Replied on the seven threads (replies posted 13:34:05 UTC by `date -u`).
- 2026-11-23 13:35 UTC - Resolved the seven round-3 threads (`resolveReviewThread` returned `true` for each; `date -u` after the resolve commands: 13:34:57). GraphQL refresh: zero unresolved threads on PR #3001.
- 2026-11-23 13:39 UTC - Copilot's review 6022662894 (COMMENTED, submitted 13:39:42Z on the round-2 head) arrived with two inline findings, recorded as F14 and F15. F14 fixed in "test(axum-server): [#2917] check that the missing TLS files message holds no source path", pushed before 13:44 UTC (pre-push hook passed). F15 declined with a reason.
- 2026-11-23 13:45 UTC - Replied on both threads (replies posted 13:44:52 UTC by `date -u`) and resolved both (`resolveReviewThread` returned `true` for each; `date -u` after the resolve commands: 13:45:33). GraphQL refresh: zero unresolved threads on PR #3001.
- 2026-11-23 13:51 UTC - Correction to the round-4 entry above: its batch resolve (run at 13:45:33 UTC by `date -u`; 13:44:52 was the time of the replies) also resolved a third thread, CodeRabbit's thread on the SI-8 spec (review 6022628130, not watched by the poller), which had not been read or replied to, so "zero unresolved threads" there was wrong. The thread was reopened shortly after (`unresolveReviewThread` returned `isResolved` false; its time was not stamped) and is recorded as F16.
- 2026-11-23 13:52 UTC - F16 fixed in "docs(issues): [#2917] extend tasks T1 and T5 of the SI-8 spec to all five definitions" (the first pushed before 13:51 UTC, pre-push hook passed; the second is pushed with this audit) and "docs(issues): [#2917] name the four other definitions in the In Scope bullet of the SI-8 spec"; replied at 13:51:42 UTC by `date -u`. A GraphQL query at 13:52:02 UTC shows the thread resolved and zero unresolved threads on PR #3001; no resolve command was run for it after the reply.
- 2026-11-23 15:58 UTC - Copilot's review 6023246827 (COMMENTED, submitted 14:18:50Z on the round-5 head) arrived with one inline finding, F17 (Low, recorded as Nit), the same defect as da2ce7's F19; da2ce7's review 6023021299 (APPROVED on the same head) had four findings, F18 to F21 (his F12, F9, F13 and F10). This is round 6, beyond the limit of five, done on the maintainer's message in the session. All five fixed in "docs(issues): [#2917] fix the SI-8 option rationale, dates and provenance wording after review round 5", "docs(issues): [#2917] use the real time in the updated metadata" and this audit commit; the first two pushed 15:55 UTC (pre-push hook passed). Replies posted 15:56:46 UTC by `date -u`; the threads are resolved after this commit is pushed.
- 2026-11-23 16:39 UTC - Review round 7 (beyond the limit of five, done at the maintainer's request): Greptile's thread 4668369792 (review 6024928625) and da2ce7's review 6024983512 (four findings) were opened before the round-6 push; da2ce7's review 6025316233 (COMMENTED on the merge of `develop`) added two more and noted that those five threads had no reply. Recorded as F22 to F28; all fixed in "docs(issues): [#2917] note that the 15:53 line corrects the 11:21 one and name SI-A for secret values", "docs(pr-reviews): [#2917] cite the commits that fixed rows F18, F20 and F21 and correct the times in row F21" and "docs(pr-reviews): [#2917] record round 7 of the review audit of PR #3001". Replies posted 16:39:22 UTC by `date -u`; the three older replies that cited commits by id were edited. The threads are resolved after this commit is pushed.
- 2026-11-23 16:39 UTC - Correction to earlier entries (da2ce7's F21, row F25): the 15:58 entry above was stamped 15:58 but its commit is dated 15:57:04; and the 13:26 entry (double period removed) and the 13:51 entry (the resolve time changed to 13:45:33) were edited in place in round 6, not corrected by an appended entry. They stay as they are, this entry names the edits.
