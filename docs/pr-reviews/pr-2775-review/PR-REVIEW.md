---
pr-number: 2775
pr-url: https://github.com/torrust/torrust-tracker/pull/2775
last-updated-utc: "2026-11-07 07:15"
---

# PR #2775 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2775.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: Copilot review 5806236557 on the head `e3cb61b77` (six inline findings). F1 to F5 are valid and fixed; F6 concerns a spec from PR #2771 and is a follow-up there.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2775-f1 | Copilot | Medium | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2775-f2 | Copilot | Medium | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2775-f3 | Copilot | Low | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2775-f4 | Copilot | Low | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2775-f5 | Copilot | Low | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2775-f6 | Copilot | Low | link-integrity | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - Test malformed JSON response handling

- PR number: 2775
- Source review ID: 5806236557
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497016172
- Concern: `tests/cli.rs` tested an unreadable threads file but not a readable, malformed one, although AC3 includes both failure cases.
- Solution: Added a `reply-status` test that writes `{ not json` to a file and asserts exit 1, empty stdout and a `reply_status_error` diagnostic.
- Current-tree verification: `cargo test -p github-review-threads --test cli` passed 10 tests, including `it_should_fail_with_exit_one_and_a_diagnostic_when_the_threads_file_is_not_valid_json`.
- Resolution reference: test(github-review-threads): [#2761] cover malformed threads JSON and isolate the missing --login usage error
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497577205

### F2 - Test missing --login with a valid threads file

- PR number: 2775
- Source review ID: 5806236557
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497016247
- Concern: The usage-error test omitted `--threads-file` as well as `--login`, so it would pass if `--login` stopped being required.
- Solution: The test now passes the valid fixture, omits only `--login`, and asserts the message lists `--login <LOGIN>` as the missing argument.
- Current-tree verification: Same `cargo test` run; running `reply-status --threads-file <fixture>` alone printed a `usage_error` naming only `--login`.
- Resolution reference: test(github-review-threads): [#2761] cover malformed threads JSON and isolate the missing --login usage error
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497577487

### F3 - Enable pipefail and reject empty resolution input

- PR number: 2775
- Source review ID: 5806236557
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497016298
- Concern: The skill example mentioned `pipefail` but did not enable it, so a `reply-status` failure could be hidden behind `jq`.
- Solution: The example now starts with `set -o pipefail`, and the text explains that `jq -e` exits 4 on the empty stdout of a failed `reply-status`.
- Current-tree verification: `printf "" | jq -e .` exited 4; the edited section of `resolve-review-threads/SKILL.md` shows `set -o pipefail`.
- Resolution reference: docs(skills): [#2761] make the reply-status gate fail on could-not-answer with pipefail
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497577822

### F4 - Record the required prose-first test design review

- PR number: 2775
- Source review ID: 5806236557
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497016354
- Concern: T1 was `DONE` but no prose-first Arrange-Act-Assert design review was recorded, as `write-unit-test` requires.
- Solution: Added a "Test Design Review (Prose-First AAA)" section to the spec with the comparison for each new or changed test, stating that it was done in review round 1 and not before the first commit.
- Current-tree verification: The section exists in `docs/issues/open/2761-2758-reply-status-result/ISSUE.md` and lists the five tests of `tests/cli.rs`.
- Resolution reference: docs(issues): [#2761] record the prose-first test design review and a real PR capture run
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497578166

### F5 - Verify reply-status with a real PR capture

- PR number: 2775
- Source review ID: 5806236557
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497016408
- Concern: T4 was `DONE` but V1 recorded only the synthetic fixture, no fetched PR response.
- Solution: Ran `reply-status` on a capture fetched from this PR (6 unresolved threads) and recorded it as V3: `missing_reply` for `josecelano-bot` and `all_replied` for `copilot-pull-request-reviewer`, both exit 0.
- Current-tree verification: V3 in `manual-verification-evidence.md` holds the commands and output; both runs were at 21:02:17 UTC on 2026-11-06.
- Resolution reference: docs(issues): [#2761] record the prose-first test design review and a real PR capture run
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497578548

### F6 - Repair live references to the moved EPIC

- PR number: 2775
- Source review ID: 5806236557
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497016475
- Concern: When the EPIC spec moves to `docs/issues/closed/`, the open child specs still list its former path in `related-artifacts`, which the frontmatter validator reports as missing.
- Solution: Valid, but `2770-2758-close-out-adr-and-epic/ISSUE.md` comes from spec PR #2771 and is outside the files issue #2761 may touch. The EPIC coordinator added the repair to the #2770 scope in PR #2771.
- Current-tree verification: Coordinator message: commit `docs(issues): [#2758] make the close-out issue repair the references to the moved EPIC spec` on PR #2771.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497605463
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2775#discussion_r4497578977

## Processing Log

- 2026-11-06 21:05 UTC - Round 1 (Copilot review 5806236557): fixes pushed, then replies posted on all six threads (GitHub reply times 21:05:22 to 21:05:36); a second reply on F6 at 21:08:50.
- 2026-11-07 07:10 UTC - Threads F1 to F5 resolved with `gh-bot`; F6 left open as FOLLOW_UP. This audit record written after those events.

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
