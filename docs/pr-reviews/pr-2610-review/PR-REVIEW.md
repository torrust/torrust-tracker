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

Eight reviews were submitted so far, six by Copilot and two by the reviewer `da2ce7`. Each body repeats its
inline threads, so the bodies add no rows.

- Review 5599696060 read the head whose subject is "feat(ci-gate): [#2525] add the GitHub adapter of the merge
  verdict". It carries four inline threads, F1 to F4.
- Review 5602014129 read the head whose subject is "docs(pr-reviews): add the PR #2610 review audit". It marks
  F1 to F4 as resolved and carries two inline threads, F5 and F6.
- Review 5602354694 read the head whose subject is "docs(pr-reviews): record the second review round in the PR
  #2610 audit". It marks F5 and F6 as resolved and carries three inline threads, F7 to F9, which make one point
  at three lines of the same file, so F8 and F9 are recorded as re-raises of F7.
- Review 5603061561 read the head whose subject is "docs(pr-reviews): add the PR #2610 review audit", the head
  of the third round. It marks F5 and F6 as resolved and carries two inline threads, F10 and F11, which make one
  point at two lines, so F11 is recorded as a re-raise of F10.
- Review 5603718492 read the head whose subject is "docs(pr-reviews): record the fourth review round in the PR #2610
  audit". It marks F10 and F11 as resolved and carries one inline thread, F12, a different property of the same
  test helper, so it is an original finding.
- Review 5604832273, by `da2ce7`, state `CHANGES_REQUESTED`, read the head whose subject is "docs(pr-reviews):
  record the fifth review round in the PR #2610 audit". Its body says that a round computed at the earlier head
  was never posted, so its findings are numbered from F13 and re-verified at this head. It carries nine inline
  threads, F13 to F21, each with a severity in its own bracket, and says that all twelve threads of the review bot
  are resolved and repaired by the commits the record names.
- Review 5605009491, by Copilot, read the same head, "docs(pr-reviews): record the fifth review round in the PR #2610
  audit", and was submitted at 19:20 UTC, before the previous round was pushed. It marks F12 as resolved and
  carries three inline threads, F22 to F24.
- Review 5605208219, by `da2ce7`, state `DISMISSED` (GitHub dismissed it when the previous round was pushed), read the
  head whose subject is "docs(pr-reviews): record the sixth review round in the PR #2610 audit". It finds all of F13 to F20 repaired and withdraws F21. It carries three inline threads,
  labeled F22 to F24 by the reviewer; they are recorded as F25 to F27 because those labels were already used for the
  review bot's threads. Each bracket is the reviewer's own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2610-f1 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2610-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2610-f3 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2610-f4 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2610-f5 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2610-f6 | Copilot | Nit (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2610-f7 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2610-f8 | Copilot | Suggestion (inferred) | testing | RE_RAISE_OF:F7 | NO_ACTION | SUPERSEDED |
| F9 | review-finding:pr-2610-f9 | Copilot | Suggestion (inferred) | testing | RE_RAISE_OF:F7 | NO_ACTION | SUPERSEDED |
| F10 | review-finding:pr-2610-f10 | Copilot | Nit (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2610-f11 | Copilot | Nit (inferred) | testing | RE_RAISE_OF:F10 | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-2610-f12 | Copilot | Nit (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2610-f13 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2610-f14 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2610-f15 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2610-f16 | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2610-f17 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2610-f18 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2610-f19 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2610-f20 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2610-f21 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F22 | review-finding:pr-2610-f22 | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F23 | review-finding:pr-2610-f23 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2610-f24 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2610-f25 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2610-f26 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2610-f27 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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
- Solution: `assess_override` returns that no override was asked for as soon as the labels do not include the label, and reads the events, check suites, and permission only after that. The documentation of the field says that it is `None` when the labels were read and the label is not among them. The behavior was in the first version of the adapter, not introduced by a later change: finding F1 of the first review pointed at the same unreadable-events case and offered to fix the implementation or to broaden the documentation, the documentation was broadened, and this finding asked for the implementation fix, so the same defect cost a second round. No test covered a label that is absent while the events request fails.
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

### F7 - The github-verdict command-line tests exist only on Unix

- PR number: 2610
- Source review ID: 5602354694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338863115>
- Concern: all `github-verdict` command-line tests and the `FakeGh` harness are gated behind `#[cfg(unix)]`, so the subcommand has no end-to-end coverage on other platforms. If the crate is meant to be portable, or CI runs tests on Windows, rework `FakeGh` to be cross-platform, for example with a helper executable or a `.cmd` wrapper.
- Solution: no cross-platform stand-in was built, because the pull-request workflows run tests on Linux only (the Windows and macOS jobs of `os-compatibility.yaml` only run `cargo build`), the sibling tool `frontmatter-validator` gates its command-line tests the same way, and a helper binary would be built by every `cargo build -p ci-gate`, which the cold-build budget approved for AC6 counts. The finding did show that the parsing of the paginated output of `gh` and the failure message were covered only by the Unix tests, so both are now pure functions with unit tests that run on every platform, and the reason for the gating is written next to the stand-in.
- Current-tree verification: the unit tests of `gh.rs` run on every platform (eight new ones), four mutants of the two functions are killed, and `cargo test -p ci-gate` passes with 209 unit and 19 command-line tests. What only the Unix tests still cover is the spawning of the program and the mapping of its exit status.
- Resolution reference: refactor(ci-gate): [#2525] test the parsing of the output of gh on every platform
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338932386>

### F8 - The github-verdict command-line tests exist only on Unix (same concern, another line)

- PR number: 2610
- Source review ID: 5602354694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338863184>
- Concern: all `github-verdict` command-line tests and the `FakeGh` harness are gated behind `#[cfg(unix)]`, so the subcommand has no end-to-end coverage on other platforms. If the crate is meant to be portable, or CI runs tests on Windows, rework `FakeGh` to be cross-platform, for example with a helper executable or a `.cmd` wrapper. It is the concern of F7, raised at another line.
- Solution: no change, for the reason given for F7.
- Current-tree verification: the reply of F7 answers it, and the text next to the stand-in covers every use of it.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338932892>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338932892>

### F9 - The github-verdict command-line tests exist only on Unix (same concern, another line)

- PR number: 2610
- Source review ID: 5602354694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338863231>
- Concern: all `github-verdict` command-line tests and the `FakeGh` harness are gated behind `#[cfg(unix)]`, so the subcommand has no end-to-end coverage on other platforms. If the crate is meant to be portable, or CI runs tests on Windows, rework `FakeGh` to be cross-platform, for example with a helper executable or a `.cmd` wrapper. It is the concern of F7, raised at another line.
- Solution: no change, for the reason given for F7.
- Current-tree verification: the reply of F7 answers it, and the text next to the stand-in covers every use of it.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338933317>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4338933317>

### F10 - The test name goes unchanged into a directory that is removed recursively

- PR number: 2610
- Source review ID: 5603061561
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4339419280>
- Concern: `test_name` is interpolated unchanged into a filesystem path that is later removed recursively, so a future caller could pass a path separator or two dots and make the helper create and delete somewhere unexpected. Sanitize the name to a safe set of characters, or leave it out of the directory name.
- Solution: every character of the test name that is not a letter, a digit, a hyphen, or an underscore is replaced in the directory name, and a test creates a fixture with the name `nested/name` and expects the directory to be a direct child of the temporary directory.
- Current-tree verification: the test failed before the fix, with the directory one level too deep, and a mutant that replaces only the dots is killed. `cargo test -p ci-gate` passes with 209 unit and 20 command-line tests.
- Resolution reference: test(ci-gate): [#2525] keep a fake gh directory inside the temporary directory
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4339476131>

### F11 - The test name goes unchanged into a directory that is removed recursively (same concern, another line)

- PR number: 2610
- Source review ID: 5603061561
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4339419349>
- Concern: `test_name` is interpolated unchanged into a filesystem path that is later removed recursively, so a future caller could pass a path separator or two dots and make the helper create and delete somewhere unexpected. Sanitize the name to a safe set of characters, or leave it out of the directory name. It is the concern of F10, raised at another line.
- Solution: no change, for the reason given for F10.
- Current-tree verification: the reply of F10 answers it, and the sanitizing covers every use of the name.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4339476470>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4339476470>

### F12 - A failed cleanup of the fake gh directory is dropped without a trace

- PR number: 2610
- Source review ID: 5603718492
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4339943065>
- Concern: dropping the result of removing the fixture directory hides a cleanup failure and can leave stray temporary directories that affect later runs. Handle the error, for example by panicking only when the thread is not already panicking, or by writing it on stderr.
- Solution: the removal is a function that returns a message for any failure except a directory that is already gone, and the fixture writes the message on stderr when it is dropped. It does not panic, because a second panic while a test is unwinding aborts the process and hides the first. Three tests cover a successful cleanup, a directory that is already gone, and a failed cleanup. A second commit fixed the formatting and the lint on stderr output of the first.
- Current-tree verification: the tests failed to compile before the function existed; a mutant that swallows failures and one that reports a missing directory are each killed by one test. The call from the `Drop` implementation to the function is not covered by a test. `cargo test -p ci-gate` passes with 209 unit and 23 command-line tests.
- Resolution reference: test(ci-gate): [#2525] report a failed cleanup of a fake gh directory
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340710055>

### F13 - The permission endpoint never says maintain, so a Maintain-role override is always refused

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837004>
- Concern: the adapter reads the `permission` field of the collaborator permission endpoint and accepts only `admin` or `maintain`. GitHub documents that field as the legacy base role (`admin`, `write`, `read`, `none`), with the Maintain role reported as `write`; the role itself is `role_name`. A Maintain-role collaborator is therefore refused, which contradicts the module documentation, the T5 design, and AC3. The test case for `maintain` feeds a value that field does not carry, and the sandbox saw only `admin` and `read`. Read `role_name`, decide how a custom role based on Maintain counts, and use a fixture shaped like the real response.
- Solution: the adapter reads `role_name`. Only `admin` and `maintain` count and a custom role never does, even when it extends one of them, which fails closed and is a decision the reply asks the reviewer to confirm. The fixtures have the shape of the real response, with both fields, derived from the role by a helper that follows GitHub's documented mapping; a response without `role_name` is refused with a reason. The module documentation, the T5 decision (b) in the specification, and V10 (a correction) and V11 say the same.
- Current-tree verification: a new test of a Maintain-role actor, with a `permission` of `write`, failed against the old code. Mutants that accept only `admin`, that accept `write`, and that accept any role that starts or ends with the name are each killed (the third survived until a case with `admin` in the name of a custom role was added). The real responses of three collaborators of the repository are in V11; none has the Maintain role, so the mapping of Maintain to `write` is taken from GitHub's description and not observed.
- Resolution reference: fix(ci-gate): [#2525] read the role of the actor from role_name, not permission
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340990414

### F14 - The 11:23 progress log entry is rewritten each round

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837163>
- Concern: the entry dated 11:23 had its counts replaced in each later round, so it now says `209 unit and 23 command-line tests, 27 of 27 mutants`, while the round entries below it say `Now 200 unit and 18` and so on; at 11:23 neither those tests nor those mutants existed. Restore the 11:23 counts, and let V11 carry the current totals.
- Solution: the entry says again `198 unit and 18 command-line tests, 16 of 16 mutants killed`, as it did when it was written. The later totals stay in the round entries and in V11.
- Current-tree verification: the original text was taken from the commit that wrote it, whose subject is `docs(issues): [#2525] record the T6a design decisions and evidence`.
- Resolution reference: docs(issues): [#2525] restore the counts of the 11:23 progress log entry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340990705

### F15 - The documentation of override_ignored omits four of the reasons the code gives

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F15
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837173>
- Concern: the documentation lists a stale label, an actor who may not override, no `labeled` event, and unreadable events, check suites, or permission. The code also returns that the actor is not known, that the login is not valid, that the head commit has no check suite, and the not-in-the-form-of reason. The body of an earlier commit says to list every case, and the audit's F1 verification says the documentation matches. Add the four reasons or say the list is not exhaustive.
- Solution: the documentation is a list of every reason: the labels cannot be read; no `labeled` event was found or the events cannot be read; the actor is not known or not a valid login; the check suites cannot be read or there is none; a time is not in GitHub's form; the label was applied at or before the first check suite; the role cannot be read or is not `admin` or `maintain`.
- Current-tree verification: each item was matched to a return of `assess_override` and `judge_override`; the documentation build passes with warnings denied.
- Resolution reference: docs(ci-gate): [#2525] list every reason an override can be ignored
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340990979

### F16 - Make the check-run request independent of the default filter

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F16
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837185>
- Concern: the request leaves `filter` at its default, `latest`, but the rules assume that every run of a group job is listed: a later skipped run must not hide an earlier result, and the highest id wins among all runs. If the default omits a superseded run of the same name, that case reads as missing in production while the fixture passes. Add `&filter=all` to the request and the fixture paths.
- Solution: the request is `check-runs?filter=all&per_page=100` and the fixture paths follow.
- Current-tree verification: the fixture tests failed before the change. On the real head commit whose subject is `docs(pr-reviews): record the fifth review round in the PR #2610 audit`, the request with and without the parameter lists the same 22 runs, so it shows only that GitHub accepts the parameter.
- Resolution reference: fix(ci-gate): [#2525] ask for every check run, not only the latest of each name
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340991334

### F17 - is_login and two error paths of assess_override have no test

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F17
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837203>
- Concern: no test calls `is_login` or reaches the branch that refuses a login, and no scenario makes the labels request or the check-suites request fail. Add a rule table for `is_login` and two `.failing(...)` scenarios.
- Solution: a rule table of ten logins, a scenario with an actor that is not a valid login that also asserts that no request for the role is made, and one scenario each for the failing labels request and the failing check-suites request.
- Current-tree verification: five mutants are each killed: `is_login` always true, accepting the empty name, accepting a slash, an unreadable list of labels read as no label, and an unreadable list of check suites read as none. `cargo test -p ci-gate` passes with 226 unit and 23 command-line tests.
- Resolution reference: test(ci-gate): [#2525] cover is_login and the unreadable labels and check suites
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340991707

### F18 - F7 to F9 carry an inferred severity without (inferred)

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F18
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837222>
- Concern: the three comments of the review have no severity bracket, so `Suggestion` was derived from free prose, and the Status Values say to append `(inferred)` in that case, as the other rows do.
- Solution: the rows of F7, F8, and F9 say `Suggestion (inferred)`.
- Current-tree verification: the audit validator passes.
- Resolution reference: docs(pr-reviews): mark the severity of F7 to F9 in the PR #2610 audit as inferred
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340992161

### F19 - The 18:54 log entry has no date, so the validator does not see it

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F19
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837233>
- Concern: the entry starts with a time and not with a date, so the validator's entry pattern does not match it: it reports one entry fewer than there are and never checks the order of that one.
- Solution: the entry starts with its date.
- Current-tree verification: the validator now reports 13 log entries. That it skipped an entry it could not read, instead of reporting it, is a limit of the script, not changed here.
- Resolution reference: docs(pr-reviews): date the 18:54 log entry of the PR #2610 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340992445

### F20 - This test's name promises the cause, but it asserts only the directory

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F20
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837246>
- Concern: the test is named for the directory and the cause, but the assertion checks only that the message contains the path, so a mutant that drops the cause from the message would pass. Assert a known part of the cause, or name the test after what it checks.
- Solution: the test takes the cause from removing the same file as a directory and asserts that the message contains it.
- Current-tree verification: a mutant that leaves the cause out of the message is killed. It was run after the commit: the first mutation script stopped on its own assertion because the formatter had wrapped the text, so it changed nothing, and the second run killed it.
- Resolution reference: test(ci-gate): [#2525] assert the cause in the cleanup failure message
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340992743

### F21 - The lint-fix commit does not pass the pre-commit gate on its own

- PR number: 2610
- Source review ID: 5604832273
- Reviewer finding ID: F21
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340837258>
- Concern: the first commit of the round used `eprintln!`, which `clippy::print_stderr` denies, and was not formatted; the body of the next commit says it was committed without gating. The pre-commit verification is mandatory before every commit, so fold the second commit into the first before merge to keep every commit green for bisecting.
- Solution: no change. The repository's branch rules say not to squash, fix up, or rewrite history to hide steps that were later superseded, and a rewrite would invalidate the commit ids the reviews cite. The mistake and its correction stay as two commits, and the correction's message says what happened.
- Current-tree verification: the first commit fails and the second passes, as the finding says; the tip of the branch passes the pre-commit and pre-push gates.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340992984>
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340992984

### F22 - Option::is_none_or can raise the MSRV

- PR number: 2610
- Source review ID: 5605009491
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340975363>
- Concern: `Option::is_none_or` is relatively new and can inadvertently raise the crate's MSRV, so rewrite the condition with an explicit `match` or `if let`.
- Solution: no change. `Option::is_none_or` is stable since Rust 1.82.0 and the workspace `rust-version` is 1.88, so it cannot raise the MSRV.
- Current-tree verification: the `stable` attribute in the standard library source of the installed toolchain says `since = "1.82.0"`, and `cargo +1.88.0 check -p ci-gate --all-targets` compiles with the 1.88.0 toolchain.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341246453>
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341246453

### F23 - An option of an option is built inside a map

- PR number: 2610
- Source review ID: 5605009491
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340975462>
- Concern: `override_request` builds a `Some(OverrideRequest { ... })` inside a `map`, so the result nests an option inside an option; returning the request from the map and letting `ok_or_else` produce the result would reduce the nesting.
- Solution: the latest `labeled` event is looked up first and a missing one returns the error through a `let ... else`, so the `Some` is built once, at the return.
- Current-tree verification: the existing tests pass, and two mutants (the earliest event used, a missing event read as no request) are each killed. `cargo test -p ci-gate` passes with 226 unit and 23 command-line tests.
- Resolution reference: refactor(ci-gate): [#2525] return the override request without a nested option in a map
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341246862

### F24 - drop(writeln!(...)) hides the intent to ignore the result

- PR number: 2610
- Source review ID: 5605009491
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4340975535>
- Concern: `drop(writeln!(...))` in `Drop` is an unusual pattern and makes it hard to see that the `Result` is ignored on purpose; prefer `let _ = writeln!(...)`.
- Solution: the pattern stays and a comment at the call says why. `let _ = writeln!(...)` does not compile in this workspace, because the lint group `let_underscore` is denied in `Cargo.toml` and `let_underscore_drop` rejects a `Result` whose error has a destructor; the suggestion was tried and the compiler refused it.
- Current-tree verification: `cargo clippy -p ci-gate --all-targets -- -D warnings` fails with `non-binding let on a type that has a destructor` for the suggested form, and passes for the current one.
- Resolution reference: test(ci-gate): [#2525] say why the fixture ignores the write with drop
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341247141

### F25 - The record misses the review bot's sixth review, whose threads were resolved with no reply

- PR number: 2610
- Source review ID: 5605208219
- Reviewer finding ID: F22
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341136871>
- Concern: the review bot's sixth review left three threads that were resolved with no reply and no row, and the paragraph of this record counts six reviews. The Completion Rules say to reply on every resolvable thread before resolving it and to record each finding. None of the three is a defect, so rows with `NO_ACTION`, the replies, and seven reviews in the count would close it.
- Solution: the rows F22 to F24, the replies on the three threads, and the count of reviews were added by the commit "docs(pr-reviews): record the seventh review round in the PR #2610 audit", pushed after the head that was reviewed. The count is now eight with the reviewer's second review. F23 and F24 are `FIXED` and not `NO_ACTION`, because a small change was made for each.
- Current-tree verification: the audit validator reports 27 rows and every thread of the pull request has a reply from the author.
- Resolution reference: docs(pr-reviews): record the seventh review round in the PR #2610 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341294592

### F26 - Three records cite commit ids of the branch

- PR number: 2610
- Source review ID: 5605208219
- Reviewer finding ID: F23
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341136882>
- Concern: the check of the `open-pull-request` skill reports two ids of the branch, cited in the audit and in V11. The skill says that no durable reference may cite a commit of the branch by id, and asks for the check before every push that changes an audit or an evidence file. Cite the subjects instead.
- Solution: the audit (the F14 and F16 verifications) and V11 cite subjects. The reply on F21 cited two ids and was edited to cite subjects. I had not run the check before pushing; it was run for this round. It still prints three ids of commits that exist only on the fork's sandbox branches, in the T5 evidence of the pull request this one is stacked on, which is left for that pull request.
- Current-tree verification: the check, run from the repository root against the upstream remote, prints only those three ids, each in the file `evidence/t5-gate-logs.txt` or in V10 of the evidence.
- Resolution reference: docs(pr-reviews): cite commit subjects, not ids, in the PR #2610 audit and V11
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341295053

### F27 - The step from 198 to 199 tests has no entry

- PR number: 2610
- Source review ID: 5605208219
- Reviewer finding ID: F24
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341136887>
- Concern: the 11:23 progress log entry says 198 unit tests and 16 of 16 mutants, which is right for that time. The next counted state, 199 tests and 17 mutants, has no entry, so the 14:43 entry's "Now 200 ... and one more mutant killed" reads as 198 to 200 and 16 to 17, while the totals were 200 and 18. A short entry, or "from 199 and 17" in the 14:43 entry, would make the chain add up.
- Solution: the 14:43 entry says "from 199 unit tests and 17 mutants after the test that the earliest check suite is taken over all pages". A new entry was not written, because an entry written now for a time in the morning would be a backdated record.
- Current-tree verification: the chain 198 and 16, 199 and 17, 200 and 18 adds up from the commits that changed the counts.
- Resolution reference: docs(issues): [#2525] show the step from 199 tests and 17 mutants in the 14:43 progress log entry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2610#discussion_r4341295317

## Processing Log

- 2026-10-21 12:33 UTC - Copilot's review arrived with four threads, F1 to F4.
- 2026-10-21 14:30 UTC - Checked each against the tree and confirmed all four; F3 was a slip of mine. Wrote the
  tests first, and saw the end-to-end tests of F3 and F4 fail.
- 2026-10-21 14:44 UTC - Fixed each finding in its own signed commit and recorded the round in the specification.
  One commit failed on a GPG passphrase timeout and was rerun after the maintainer was asked.
- 2026-10-21 14:49 UTC - Pushed, replied on the four threads after the push, and started this record.
- 2026-10-21 15:21 UTC - Copilot's second review arrived with two threads, F5 and F6. Both were confirmed,
  and F5 was the implementation fix that F1 of the first round had offered as an alternative to the documentation change that was chosen.
- 2026-10-21 15:27 UTC - Wrote the tests first and saw them fail, fixed each finding in its own signed commit,
  recorded the round in the specification, pushed, replied on both threads, and added F5 and F6 to this record.
- 2026-10-21 15:47 UTC - Copilot's third review arrived with three threads, F7 to F9, one concern at three lines.
  Checked what the Unix-only tests lose in CI: nothing, since tests run on Linux only.
- 2026-10-21 15:55 UTC - Moved the parsing of the output of gh and the failure message into pure functions with
  portable unit tests, wrote the reason for the gating next to the stand-in, recorded the round in the
  specification, pushed, replied on the three threads, and added F7 to F9 to this record.
- 2026-10-21 16:36 UTC - While preparing the review retrospective, found that the origin of F5 was recorded wrongly,
  as a defect of the previous round's change, although the behavior was in the first version of the adapter and
  F1 had offered the implementation fix; corrected the text of F5 here, in the specification, and in the reply on
  the thread.
- 2026-10-21 16:42 UTC - Copilot's fourth review arrived with two threads, F10 and F11, one concern at two lines.
  Both were confirmed. The test failed first and was mutation-checked.
- 2026-10-21 16:48 UTC - Fixed it in one signed commit, recorded the round in the specification, pushed, replied on
  both threads, and added F10 and F11 to this record.
- 2026-10-21 17:34 UTC - Copilot's fifth review arrived with one thread, F12, which I found only when the
  maintainer said that a thread was still open; the watcher had been stopped. Confirmed it against the tree.
- 2026-10-21 18:54 UTC - Fixed it test-first in two signed commits (the first was committed without running the formatter and
  clippy, which rejected it), recorded the round in the specification, pushed, replied, and added F12 to this
  record.
- 2026-10-21 19:08 UTC - The watcher reported nine threads of the reviewer `da2ce7`, review 5604832273 at 19:07
  UTC, a review of the pushed head. Read them and checked F13 against the real API and GitHub's description before
  touching code: it is correct.
- 2026-10-21 19:22 UTC - Fixed F13, F14, F15, F16, F17, F18, F19, and F20 in one signed commit each, test-first where
  there was behavior, with mutants, and gated on the formatter, clippy, and the tests; two further commits fixed a
  spelling slip of mine and recorded the round in the specification. Declined F21 with the reason. Pushed, replied
  on the nine threads, and added F13 to F21 to this record.
- 2026-10-21 19:49 UTC - Resolved all threads of the pull request in one bulk call, which also closed three threads of
  Copilot's sixth review that had arrived at 19:20 UTC and that I had not read: the threads of a review I did not
  know about. I noticed from the count, 24 threads where 21 were expected, read the three, checked each one, and
  replied on them; they stay resolved because the replies and fixes now answer them. Fixed F23 and F24, declined
  F22 with its evidence, recorded the round in the specification, and added F22 to F24 to this record.
- 2026-10-21 19:54 UTC - The reviewer `da2ce7` had posted a second review at 19:37 UTC (three threads), which GitHub then
  marked as dismissed when I pushed. Read the three, checked them, and found two of them to be my own slips: records
  that cite commit ids of the branch, which the check of the `open-pull-request` skill reports and which I had not
  run, and a count chain that skipped a step. Fixed F26 and F27 in one signed commit each, answered F25 with the
  previous round's record, edited my reply on F21 (it cited two ids), pushed, replied on the three threads, and added
  F25 to F27 to this record. The check still prints three ids of commits of the fork's sandbox branches, in the T5
  evidence of the pull request this one is stacked on; they are left for that pull request.

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
