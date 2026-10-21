---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - contrib/dev-tools/checks/ci-gate/src/github.rs
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2610 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2610>.

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

## Findings

Two reviews were submitted so far, both by Copilot. Each body repeats its inline threads, so the bodies add
no rows.

- Review 5599696060 read the head whose subject is "feat(ci-gate): [#2525] add the GitHub adapter of the merge
  verdict". It carries four inline threads, F1 to F4.
- Review 5602014129 read the head whose subject is "docs(pr-reviews): add the PR #2610 review audit". It marks
  F1 to F4 as resolved and carries two inline threads, F5 and F6.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2610-f1 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2610-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2610-f3 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2610-f4 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2610-f5 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2610-f6 | Copilot | Nit (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The documentation of override_ignored is narrower than the behavior

- PR number: 2610
- Source review ID: 5599696060
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4336817257>
- Concern: the documentation says that `override_ignored` is set only when the label is on the pull request, but it is also set when the labels or the events could not be read, before the presence of the label is known. Fix the implementation, or broaden the documentation.
- Solution: the documentation was broadened: it lists the cases where the label is on the pull request but does not count, the cases where whether it is there could not be told because something could not be read, and says the field is `None` when there is no label.
- Current-tree verification: the documentation of the field on `GithubVerdict` matches the cases of `assess_override`, and the nightly documentation build is clean.
- Resolution reference: docs(ci-gate): [#2525] document every case of the override_ignored field
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338274083>

### F2 - A label with no event of it being applied is silently not an override

- PR number: 2610
- Source review ID: 5599696060
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4336817327>
- Concern: when the `ci-override` label is on the pull request but the events contain no matching `labeled` event, the override is treated as not asked for and `override_ignored` is omitted, which hides a requested override. Return a distinct state that explains it.
- Solution: `override_request` returns an error when the label is on the pull request and no event says that it was applied, and `assess_override` reports it as an ignored override with the reason, so the verdict is computed without the override and the reason is printed.
- Current-tree verification: a new test, with the label on the pull request and only an event of another label, expects a failing verdict and a reason naming the missing event; treating that case as no request makes it fail. `cargo test -p ci-gate` passes with 200 unit and 18 command-line tests.
- Resolution reference: fix(ci-gate): [#2525] explain a label that is on the pull request with no event of it being applied
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338274445>

### F3 - The end-to-end override helper hard-codes the permission path

- PR number: 2610
- Source review ID: 5599696060
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4336817376>
- Concern: `respond_with_an_override` accepts an actor but its permission fixture path names one login, so it is wrong for any other actor. Build the path from the actor.
- Solution: the path is built from the actor, and the end-to-end test of a passing override now uses another login, so the helper is exercised with an actor that is not the one it used to name. The replacement that was meant to make the actor a parameter had silently not applied, and every test used the same login, so none noticed.
- Current-tree verification: before the fix the test with the other login failed, because the fixture path did not match; after it, `cargo test -p ci-gate --test cli` passes with 18 tests.
- Resolution reference: test(ci-gate): [#2525] build the permission fixture path from the actor in the override helper
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338274792>

### F4 - The error of a failed request does not name --paginate

- PR number: 2610
- Source review ID: 5599696060
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4336817443>
- Concern: the error message names `gh api <path>` but the program runs `gh api --paginate <path>`; name the real invocation, to make pagination and permission problems easier to debug.
- Solution: the message names `gh api --paginate <path>`, and the end-to-end test of an unreadable GitHub asserts that it contains the invocation with `--paginate`.
- Current-tree verification: before the fix that assertion failed; after it, `cargo test -p ci-gate --test cli` passes with 18 tests.
- Resolution reference: fix(ci-gate): [#2525] name the real gh invocation in the error of a failed request
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338275066>

### F5 - The events are fetched even when the override label is absent

- PR number: 2610
- Source review ID: 5602014129
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338604801>
- Concern: `assess_override` fetches the events unconditionally, so a failing events request sets `override_ignored` although the labels, which were read, show that there is no override label. Fetch the labels first, return at once when the label is absent, and add a test of no label and a failing events API expecting `override_ignored` to be `None`.
- Solution: `assess_override` returns that no override was asked for as soon as the labels do not include the label, and reads the events, check suites, and permission only after that. The documentation of the field says that it is `None` when the labels were read and the label is not among them. This was a defect in the change of the previous round, which had no test for that case.
- Current-tree verification: the test with no label and an events API that fails expects `override_ignored` `None` and no events request: it failed before the fix, and a mutant that reads the events first fails it again. `cargo test -p ci-gate` passes with 201 unit and 19 command-line tests.
- Resolution reference: fix(ci-gate): [#2525] read the events only when the override label is on the pull request
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338661721>

### F6 - The fake gh directory can collide when a test name is reused

- PR number: 2610
- Source review ID: 5602014129
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338604878>
- Concern: the directory of a fake `gh` is named after the process and the test name, so a reused name, or a concurrent retry, would collide. Add a monotonic counter, the time, or a random suffix.
- Solution: a counter is added to the directory name, and a test creates two fixtures with the same name and expects different directories.
- Current-tree verification: the test failed before the fix, and a mutant that removes the counter fails it again; `cargo test -p ci-gate --test cli` passes with 19 tests.
- Resolution reference: test(ci-gate): [#2525] give each fake gh a directory of its own
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338662055>

## Processing Log

- 2026-10-21 12:33 UTC - Copilot's review arrived with four threads, F1 to F4.
- 2026-10-21 14:30 UTC - Checked each against the tree and confirmed all four; F3 was a slip of mine. Wrote the
  tests first, and saw the end-to-end tests of F3 and F4 fail.
- 2026-10-21 14:44 UTC - Fixed each finding in its own signed commit and recorded the round in the specification.
  One commit failed on a GPG passphrase timeout and was rerun after the maintainer was asked.
- 2026-10-21 14:49 UTC - Pushed, replied on the four threads after the push, and started this record.
- 2026-10-21 15:21 UTC - Copilot's second review arrived with two threads, F5 and F6. Both were confirmed,
  and F5 was a defect of the first round's change.
- 2026-10-21 15:27 UTC - Wrote the tests first and saw them fail, fixed each finding in its own signed commit,
  recorded the round in the specification, pushed, replied on both threads, and added F5 and F6 to this record.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
