# PR #2641 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2641>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/pull/2641#issuecomment-6420315256>

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

Review 5640012066 (maintainer da2ce7, submitted 2026-10-24 21:01 UTC) was submitted before the
pull request merged on 2026-10-25 06:25 UTC and was still unprocessed at merge, so its findings
follow the post-merge workflow. The other review, Copilot's review 5639975679, has no actionable
finding and no row. Every disposition stays `FOLLOW_UP` and every thread `OPEN` until the follow-up
pull request merges.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2641-f1 | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F2 | review-finding:pr-2641-f2 | Human | Major | maintainability | ORIGINAL | FOLLOW_UP | OPEN |
| F3 | review-finding:pr-2641-f3 | Human | Minor | testing | ORIGINAL | FOLLOW_UP | OPEN |
| F4 | review-finding:pr-2641-f4 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F5 | review-finding:pr-2641-f5 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F6 | review-finding:pr-2641-f6 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - A missing `gh` ends in a traceback and exit 1

- PR number: 2641
- Source review ID: 5640012066
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4368505605>
- Concern: Every run now calls `gh` for commit lists, and only `CalledProcessError` was caught, so a missing `gh` raised `FileNotFoundError` (exit 1, the audit-failure code) and a `--comments-file` run was no longer offline.
- Solution: `valid_subjects` also catches `OSError` and exits 2 naming the endpoint; the `--comments-file` help text, the skill and the spec's Risks line say `gh` is needed for commits. The optional captured commit list was not added.
- Current-tree verification: `python3 -m unittest discover -s .github/skills/dev/pr-reviews/process-pr-review/scripts` passes 9 tests on the follow-up branch; `test_it_should_exit_with_a_usage_error_when_gh_is_not_installed` was red before the change.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379352574>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2657>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379352574>

### F2 - Python bytecode is tracked

- PR number: 2641
- Source review ID: 5640012066
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4368505606>
- Concern: `scripts/__pycache__/*.pyc` were committed and no `.gitignore` rule covered them.
- Solution: Removed both files from the index and added `__pycache__/` to `.gitignore`. The blobs remain in the history of `develop`.
- Current-tree verification: On the follow-up branch `git ls-files | grep -c '\.pyc$'` prints 0, `git check-ignore -v` matches `.gitignore:24:__pycache__/`, and running the unit tests leaves `git status --short` empty.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379353028>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2657>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379353028>

### F3 - A test that cannot fail

- PR number: 2641
- Source review ID: 5640012066
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4368505612>
- Concern: The "no source" test asserted that a literal the test never supplied was absent from the set, so it could not fail, and no test called `check_row`, where rejection happens.
- Solution: Replaced it with `CheckRowTest`: an exact match passes and three near-misses (other case, prefix, suffix) each yield one `not a commit subject` failure.
- Current-tree verification: The 9 tests pass on the follow-up branch. I did not mutate `check_row` to watch them fail.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379353391>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2657>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379353391>

### F4 - Failure text still says "on branch"

- PR number: 2641
- Source review ID: 5640012066
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4368505615>
- Concern: The `check_row` failure text did not match the three accepted sources named by the skill.
- Solution: The text now reads `resolution reference not a commit subject of the audited pull request, its follow-ups, or the local range: <ref>`, keeping "not a commit subject".
- Current-tree verification: `grep -n "not a commit subject" validate-audit-record.py` shows the new text on the follow-up branch; `CheckRowTest` asserts on "not a commit subject".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379353990>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2657>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379353990>

### F5 - Stale records left in the #2606 spec and evidence

- PR number: 2641
- Source review ID: 5640012066
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4368505616>
- Concern: The completion review, the recheck status, the "fails on the current code" sentence, the evidence file's `last-updated-utc` and `related-pr` still described the state before completion.
- Solution: Corrected each in the archived spec and evidence file; `related-pr` and the first `last-updated-utc` were already set by the archive pull request.
- Current-tree verification: Read the changed lines under `docs/issues/closed/2606-2278-validate-audit-record-merged-fix-subjects/` on the follow-up branch.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379354473>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2657>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379354473>

### F6 - "11 of 157 audits with a FIXED row" is not the spec's count

- PR number: 2641
- Source review ID: 5640012066
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4368505618>
- Concern: The spec and evidence called the 157 files that contain the word `FIXED` "audits with a `FIXED` row".
- Solution: Reworded both to say what was counted and cited the reviewer's 148 files with a `FIXED` row. I did not recount at the reviewer's head; a rough recount on today's `develop` gave different numbers (151 and 163), so no figure of mine was substituted.
- Current-tree verification: Read the changed sentences in the archived spec and evidence file on the follow-up branch.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379354878>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2657>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2641#discussion_r4379354878>

## Processing Log

- 2026-10-26 15:44 UTC - Read-only triage: the review was submitted before the merge and unprocessed at merge. The maintainer approved the follow-up in chat at 15:43 UTC; the durable approval comment was created at 15:44 UTC.
- 2026-10-26 15:51 UTC - Opened the follow-up pull request and replied on the six threads with disposition `FOLLOW_UP`.

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
