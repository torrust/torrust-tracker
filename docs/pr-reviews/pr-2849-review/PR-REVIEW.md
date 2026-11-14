---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2849 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2849.

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

This record was created late: review 5878819832 (da2ce7) and review 5878898401 (Copilot) were
both replied to and resolved before this file existed (the `related-issues.md` pipe and timezone
threads, F7 and F9, were answered with "Not changed" rather than the exact `Superseded by` form). F1 to F6 are da2ce7's own finding IDs from review 5878819832, with no collision, so no
reassignment. F7 to F9 are Copilot's review 5878898401, source order; Copilot gives no finding ID.
F10 to F15 are da2ce7's review 5887683678; his own IDs F7, F8, F2, F4, F9 and F10 all collide with
an existing audit ID (F7, F8, F2, F4, F9, and the just-assigned F10 itself), so each is
reassigned the next ordinal and the original ID is recorded as the reviewer finding ID. His F2 and
F4 are re-raises of the audit's own F2 and F4. Copilot's review 5887735321 found nothing new
(approval recommended, three prior findings resolved) and adds no row. F16 is da2ce7's review
5889290894; his own ID F11 collides with the audit's F11, so it is reassigned. F17 to F19 are
Copilot's review 5890114272; Copilot gives no finding ID. They were not assigned in source order:
by comment id, the #613 thread (F19) is second and the audit-table thread (F18) third. The IDs are
kept because a review-finding reference never changes once assigned. F17 and F18 are re-raises of
F7 (the same rendering-artefact claim, now on two different lines after the table moved and the
audit record itself grew a table). F20 to F24 are da2ce7's review 5889973387 (submitted
2026-11-13 11:14:32, missed in the previous round), and F25 to F28 are his review 5903116153; his
own IDs F12 to F20 all collide with existing audit IDs, so each is reassigned the next ordinal and
the original ID is recorded as the reviewer finding ID. All twenty-eight threads on the forge are
inline.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------- | ------------- | -------- | -------------- | ------------------ | ------------ | ------------- |
| F1 | review-finding:pr-2849-f1 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2849-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2849-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2849-f4 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2849-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2849-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2849-f7 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F8 | review-finding:pr-2849-f8 | Copilot | Minor (inferred) | metadata | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F9 | review-finding:pr-2849-f9 | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F10 | review-finding:pr-2849-f10 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2849-f11 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2849-f12 | Human | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F13 | review-finding:pr-2849-f13 | Human | Nit | metadata | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F14 | review-finding:pr-2849-f14 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2849-f15 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2849-f16 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2849-f17 | Copilot | Minor (inferred) | formatting | RE_RAISE_OF:F7 | NO_ACTION | SUPERSEDED |
| F18 | review-finding:pr-2849-f18 | Copilot | Minor (inferred) | formatting | RE_RAISE_OF:F7 | NO_ACTION | SUPERSEDED |
| F19 | review-finding:pr-2849-f19 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2849-f20 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2849-f21 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2849-f22 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2849-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2849-f24 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2849-f25 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2849-f26 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2849-f27 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-2849-f28 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The health check API has no TLS listener, so it has no TLS silent-connection test

- PR number: 2849
- Source review ID: 5878819832
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553770720
- Concern: The evidence bullet said the silent-connection tests are plaintext and TLS in the HTTP
  tracker, REST API, and health check API packages; the health check API has no TLS code and only
  a plaintext test, and the listener-level `axum-server` suite was left out entirely.
- Solution: The bullet now splits plaintext/TLS by package and names `packages/axum-server/tests/idle_connections.rs`.
- Current-tree verification: `related-issues.md` line 31-33; `packages/axum-health-check-api-server/src/server.rs` has no TLS code.
- Resolution reference: docs(issues): [#2776] name the health check API's own test and the axum-server suite
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562402199

### F2 - The runtime evidence given for the Tracker API issue was measured on the HTTP tracker

- PR number: 2849
- Source review ID: 5878819832
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553770741
- Concern: The baseline probed the REST API in plaintext only; its TLS figures and the final
  build's TLS figures were all measured on the HTTP tracker port (27171), not the REST API, and
  the REST API's own TLS evidence is its unit test, not a probe.
- Solution: The #612 verdict now names the REST API's plaintext probe, attributes the TLS runtime
  figures to the HTTP tracker port, cites the REST API's own TLS unit test, and states the stalled
  handshake was already closed before the fix. The #613 verdict names the HTTP tracker's own TLS
  test. (The remaining part, an HTTP/1 figure mixed into the TLS list, is F12.)
- Current-tree verification: `related-issues.md` #612 and #613 sections; `evidence/run-probes.sh`
  and `evidence/baseline-probe-results.txt` port assignments; `packages/axum-rest-api-server/src/server.rs`
  and `packages/axum-http-server/src/server.rs` TLS tests.
- Resolution reference: docs(issues): [#2776] attribute the REST API's and HTTP tracker's own TLS evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562402551

### F3 - The pull-request title carries `[#2776]`, an issue this change neither implements nor closes

- PR number: 2849
- Source review ID: 5878819832
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553770751
- Concern: `[#2776]` in the title belongs to the issue implemented or closed by the PR; #2776 is
  closed and merely related, and is already listed under `Related to` in the body.
- Solution: The title was renamed to `docs(issues): record the disposition of #324, #603, #612 and #613`.
- Current-tree verification: `gh pr view 2849 --json title`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4555934749
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4555934749

### F4 - The new progress-log entry has no time, and the spec's `last-updated-utc` was not bumped

- PR number: 2849
- Source review ID: 5878819832
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553770761
- Concern: The template's progress-log entries carry a time; this one read `2026-11-12 UTC`. The
  spec's `last-updated-utc` still read the previous entry's date although the file changed.
- Solution: The entry now reads `2026-11-12 16:24 UTC` (the commit time of
  `docs(issues): [#2776] record the disposition of #324, #603, #612 and #613`), and both files'
  `last-updated-utc` are bumped. (The remaining part, the bumped value postdating the
  commit that writes it, is F13.)
- Current-tree verification: `ISSUE.md` line 234 and its frontmatter; `related-issues.md` frontmatter.
- Resolution reference: docs(issues): [#2776] bump last-updated-utc and add the progress-log entry's time
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562402929

### F5 - "plaintext listeners were already protected" contradicts the baseline for the health check API

- PR number: 2849
- Source review ID: 5878819832
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553770777
- Concern: The health check API is a plaintext listener that the baseline records as still open
  after 25 s, contradicting the blanket "plaintext listeners were already protected" in the #324
  verdict.
- Solution: The sentence now names only the HTTP tracker and REST API as already protected in
  plaintext.
- Current-tree verification: `related-issues.md` #324 section; `evidence/baseline-probe-results.txt`.
- Resolution reference: docs(issues): [#2776] name which listeners the baseline already protected
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562403235

### F6 - The summary table and the section headings give different titles for the Tracker API and HTTP tracker issues

- PR number: 2849
- Source review ID: 5878819832
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553770785
- Concern: The summary table used a shortened title for #612 and #613 while the section headings
  used a different shortened form; the record exists so readers need not open the forge, and the
  two forms leave them to guess which is real.
- Solution: Both the table and the headings now use the issues' own titles.
- Current-tree verification: `related-issues.md` summary table and `## #612`/`## #613` headings.
- Resolution reference: docs(issues): [#2776] use the issues' own titles in the related-issues record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562403557

### F7 - The summary table has an extra leading pipe on each line

- PR number: 2849
- Source review ID: 5878898401
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553833080
- Concern: Copilot's review-overview rendering showed `|| ...` at the start of each summary-table
  row, which GitHub-flavored Markdown would render as an empty first column.
- Solution: No change: the table has one leading pipe per row at every head this PR has had, and
  `linter markdown` passes (which flags malformed tables). This reads as a rendering artefact of
  Copilot's own review-overview panel, not the file.
- Current-tree verification: `related-issues.md` summary table (`| Issue | ... |`); `linter markdown` exit 0.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562403813
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562403813

### F8 - This progress-log entry omits the time component

- PR number: 2849
- Source review ID: 5878898401
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553833184
- Concern: The new progress-log entry read `2026-11-12 UTC` with no time, unlike adjacent entries.
  Same concern as F4.
- Solution: Fixed together with F4: the entry now reads `2026-11-12 16:24 UTC`.
- Current-tree verification: `ISSUE.md` line 234.
- Resolution reference: docs(issues): [#2776] bump last-updated-utc and add the progress-log entry's time
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562404137

### F9 - `last-updated-utc` has no explicit timezone designator

- PR number: 2849
- Source review ID: 5878898401
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4553833239
- Concern: The `*-utc` field's value has no `Z` or `UTC` suffix, which Copilot read as ambiguous
  and not machine-readable.
- Solution: No change: the repository's `-utc` frontmatter keys use `YYYY-MM-DD HH:MM` with no
  suffix throughout (for example the sibling `manual-verification-evidence.md` and the #2776 spec
  itself); an explicit designator here would be inconsistent with the convention, not an
  improvement on it.
- Current-tree verification: `manual-verification-evidence.md` and `ISSUE.md` frontmatter, same `*-utc` format.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562404479
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562404479

### F10 - The review threads were answered and resolved, but the head carried no review audit record

- PR number: 2849
- Source review ID: 5887683678
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4560841004
- Concern: `process-pr-review` requires the PR author to create `docs/pr-reviews/pr-2849-review/PR-REVIEW.md`
  from the template when the first review needing a reply arrives, before the first reply; this PR
  had replied to and resolved nine threads with no such file.
- Solution: This file, created late; the late start is recorded in the Processing Log below.
- Current-tree verification: this file's existence and its Findings/Processing Log sections.
- Resolution reference: docs(pr-reviews): [#2849] create the review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562404739

### F11 - Six thread replies cited a PR-branch commit id instead of its commit subject

- PR number: 2849
- Source review ID: 5887683678
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4560841010
- Concern: Replies on the F1, F2, F4, F5 and F6 threads and on F8's thread read "Fixed in" followed
  by the abbreviated id of `docs(issues): address review findings on the related-issues record`, a
  branch SHA that can change after a rebase (and did, during this round); the two
  no-change replies also used "Not changed:" instead of the exact `Superseded by <FindingId>:`
  form.
- Solution: This audit's detail entries cite each fix by its unique Conventional Commit subject,
  and new replies on every thread use those subjects (or the exact `Superseded by` form for
  no-change findings) in place of the old wording.
- Current-tree verification: the Resolution reference and Reply URL fields of every FIXED/NO_ACTION
  entry in this file; the new replies posted on each thread.
- Resolution reference: docs(pr-reviews): [#2849] create the review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562405034

### F12 - Re-raise: an HTTP/1 figure listed among the runtime TLS figures was measured in plaintext

- PR number: 2849
- Source review ID: 5887683678
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4560841012
- Concern: The remaining part of F2: the #612 verdict's sentence "The runtime TLS figures were
  measured on the HTTP tracker port" went on to list an idle HTTP/1 connection closing after 5 s,
  but that figure (`M4-http-keep-alive`) was probed on the plaintext port 27070, not TLS.
- Solution: The sentence now lists only the TLS (HTTP/2) figure there, states the HTTP/1 figure
  was measured in plaintext, and names `it_should_close_an_idle_http1_connection_after_a_response_over_tls`
  in `packages/axum-server/tests/idle_connections.rs` as the TLS HTTP/1 evidence.
- Current-tree verification: `related-issues.md` #612 section; `evidence/run-probes.sh` port 27070;
  `packages/axum-server/tests/idle_connections.rs:298`.
- Resolution reference: docs(issues): [#2776] separate the TLS and plaintext figures in the #612 verdict
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562405380

### F13 - Re-raise: `last-updated-utc` named a time after the commit that writes it

- PR number: 2849
- Source review ID: 5887683678
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4560841013
- Concern: The remaining part of F4: both `last-updated-utc` values were bumped to `2026-11-12
  20:30`, but the commit that wrote them, `docs(issues): address review findings on the
  related-issues record`, had commit time `20:20:18Z`; `process-pr-review`
  (Statements in Your Own Records) requires a stamp to be taken from the commit's own time.
- Solution: Both values now read `2026-11-13 09:42`, the author time of the commit that sets them.
- Current-tree verification: `git log --format=%cI` for the commit against the frontmatter values it writes.
- Resolution reference: docs(issues): [#2776] set last-updated-utc before the commit time
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562405728

### F14 - The summary table's rows were no longer padded to one width

- PR number: 2849
- Source review ID: 5887683678
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4560841017
- Concern: The F2/F6 fixes widened two of the four data rows without re-padding the header,
  separator and the other two rows to the new width.
- Solution: All six table lines (header, separator, four rows) are re-padded to one width.
- Current-tree verification: `related-issues.md` summary table, all rows the same length.
- Resolution reference: docs(issues): [#2776] pad the related-issues summary table to one width
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562406053

### F15 - Five independent fixes shared one commit

- PR number: 2849
- Source review ID: 5887683678
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4560841023
- Concern: Commit `docs(issues): address review findings on the related-issues record` fixed F1,
  F2, F4, F5 and F6 together with no body, so no audit row could cite its own commit subject, and
  the commit named none of the findings. `process-pr-review` (step 6) asks for one independent fix
  per Conventional Commit.
- Solution: `docs(issues): address review findings on the related-issues record` was split, before
  merge, into five commits, one per finding (F1, F6, F2, F5, F4 in that order), and the three later
  fix commits were rebased on top unchanged. The last of the five split commits,
  `docs(issues): [#2776] bump last-updated-utc and add the progress-log entry's time`, has the same
  tree as the commit it replaced (`git rev-parse <commit>^{tree}` prints the same tree for both, and
  `git diff` between them over `docs/issues/closed/2776-2411-axum-server-0-9-timeouts/` is empty).
  The new head differs from it only by the three later fixes.
- Current-tree verification: `git log --format=%s` from `docs(issues): [#2776] record the
  disposition of #324, #603, #612 and #613` to the last split commit prints the five split
  subjects, each naming one concern; the tree comparison above.
- Resolution reference: docs(pr-reviews): [#2849] create the review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562406380

### F16 - This round's fixes landed without thread replies, and the previous round's threads stayed unanswered

- PR number: 2849
- Source review ID: 5889290894
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562077052
- Concern: Commits fixing F2's and F4's remainders and F9's table padding landed without any
  thread reply, and the six threads opened by review 5887683678 (F2, F4, F7, F8, F9, F10) stayed
  unanswered and unresolved.
- Solution: This audit and the new replies on every open and corrected thread close this; all
  sixteen threads then on the forge had a reply and were resolved.
- Current-tree verification: GraphQL `reviewThreads` for PR #2849, zero unresolved actionable threads.
- Resolution reference: docs(pr-reviews): [#2849] create the review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562406791

### F17 - These table rows start with a double pipe (related-issues.md summary table)

- PR number: 2849
- Source review ID: 5890114272
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562698575
- Concern: Copilot's review again reported `||` at the start of each summary-table row in
  `related-issues.md`, this time at line 48 (the table shifted after the earlier fixes).
- Solution: No change, re-raise of F7: the table has one leading pipe per row at every head this
  PR has had, and `linter markdown` passes (which flags malformed tables). This reads as a
  rendering artefact of Copilot's own review-overview panel, not the file.
- Current-tree verification: `related-issues.md` summary table (`| Issue | ... |`); `linter markdown` exit 0.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572422452
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572422452

### F18 - Same markdown-table issue here: rows start with a double pipe (PR-REVIEW.md findings table)

- PR number: 2849
- Source review ID: 5890114272
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562698682
- Concern: Copilot's review reported the same `||` rendering claim, this time against this audit
  record's own Findings table (line 74 at the reviewed head).
- Solution: No change, re-raise of F7: the table has one leading pipe per row at every head this
  file has had, and `linter markdown` passes. Same rendering artefact as F17, now against this
  file's own table instead of `related-issues.md`'s.
- Current-tree verification: this file's Findings table (`| Finding ID | ... |`); `linter markdown` exit 0.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572423137
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572423137

### F19 - The #613 verdict cites a file path rather than a specific test

- PR number: 2849
- Source review ID: 5890114272
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562698624
- Concern: The #613 verdict said "its TLS test is `packages/axum-http-server/src/server.rs`",
  naming a file rather than a test, unlike the #612 section which names
  `it_should_close_a_tls_connection_that_completes_the_handshake_and_sends_nothing`.
- Solution: Named the same test function, which also exists in `packages/axum-http-server/src/server.rs`,
  in the #613 verdict.
- Current-tree verification: `grep -n "fn it_should_close_a_tls_connection_that_completes_the_handshake_and_sends_nothing"
  packages/axum-http-server/src/server.rs` matches line 734.
- Resolution reference: docs(issues): [#2776] name the HTTP tracker's own TLS test for #613
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572705458

### F20 - The audit cited two PR-branch commit ids

- PR number: 2849
- Source review ID: 5889973387
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562605707
- Concern: The `open-pull-request` commit-id check printed two PR-branch commit ids cited in this
  record, one of them no longer reachable after the history rewrite; `open-pull-request` asks a
  rewrite to replace every old id in durable references with the commit's subject.
- Solution: Every PR-branch commit id in this record is replaced by its Conventional Commit
  subject, including the one in the 2026-11-12 20:28 Processing Log entry; an appended correction
  entry records that substitution.
- Current-tree verification: the `open-pull-request` commit-id check, run against `torrust/develop`
  over every file the branch changes, prints nothing.
- Resolution reference: docs(pr-reviews): [#2849] cite commit subjects instead of PR-branch commit ids
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572706696

### F21 - The description did not say that the history was rewritten

- PR number: 2849
- Source review ID: 5889973387
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562605716
- Concern: `open-pull-request` asks a rewrite of an unmerged pull request to say so in the PR
  body, which the merge tool copies into the merge commit; the body mentioned neither the split nor
  the force-push, and its change list omitted this audit record.
- Solution: The PR body now lists this audit record and says that the commit
  `docs(issues): address review findings on the related-issues record` was split into five, one
  per finding, with the spec files' bytes unchanged, and force-pushed (edited 2026-11-14 08:59:07).
- Current-tree verification: `gh pr view 2849 --json body` shows the audit-record line and the
  rewrite sentence.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572706878
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572706878

### F22 - Processing Log stamps did not match the git and forge times of their events

- PR number: 2849
- Source review ID: 5889973387
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562605720
- Concern: The 09:42 and 09:43 "Pushed" entries gave author times, not commit or push times, and
  the 10:55 entry did not match the forge times of the replies, which also predated the commit
  carrying the entry and the force-push.
- Solution: An appended correction entry gives the times from `git log --format=%cI` and the
  forge for those entries and, from the same sweep, the 09:41 and 10:52 entries. The original
  entries stay as written because the Processing Log is append-only (`process-pr-review` step 5).
- Current-tree verification: `git log --format='%h %aI %cI %s'` for the commits; the forge's
  `created_at` of each reply, the Docs Lint run per head (`actions/runs?head_sha=`), and the
  `HeadRefForcePushedEvent` time.
- Resolution reference: docs(pr-reviews): [#2849] correct the round-2 and round-3 log stamps from git and forge times
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572707054

### F23 - Four statements in the audit did not match the forge or the repository

- PR number: 2849
- Source review ID: 5889973387
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562605728
- Concern: The record counted seventeen threads where the forge had sixteen, attributed a "Not
  changed" reply to F8's thread instead of F7's and F9's, claimed an empty diff to the new head
  where the tree identity holds only up to the last split commit, and counted eight commits where
  the head then had nine.
- Solution: The Findings introduction, F15 and F16 now state the correct facts, F15 lists the split
  subjects instead of a commit count, and an appended correction entry fixes the 10:55 entry.
- Current-tree verification: `git rev-parse <commit>^{tree}` prints the same tree for the replaced
  commit and the last split commit; `git log --format=%s` lists the five split subjects; the F8
  thread's reply reads "Fixed in" followed by an id, not "Not changed".
- Resolution reference: docs(pr-reviews): [#2849] correct the thread count, reply wording and tree claims
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572707274

### F24 - The no-change rows recorded Thread state RESOLVED instead of SUPERSEDED

- PR number: 2849
- Source review ID: 5889973387
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4562605737
- Concern: Rows F7 and F9 (and later F17 and F18) are `NO_ACTION` with `Superseded by` replies,
  but recorded `RESOLVED`; `process-pr-review` step 7 records such a thread as `SUPERSEDED`.
- Solution: Rows F7, F9, F17 and F18 now record `SUPERSEDED`.
- Current-tree verification: every `NO_ACTION` row in the Findings table reads `SUPERSEDED`.
- Resolution reference: docs(pr-reviews): [#2849] record the no-change threads as superseded
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572707449

### F25 - The previous review's five threads had no reply, no audit row and no resolution

- PR number: 2849
- Source review ID: 5903116153
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572548832
- Concern: The five threads opened by review 5889973387 had neither a reply nor a resolution and
  no audit row, while the 08:00 log entry said all threads had a reply and were resolved.
- Solution: Rows F20 to F24 record those five findings; all nine threads of reviews 5889973387 and
  5903116153 were replied to (08:59:44 to 09:00:03) and then resolved; an appended correction
  entry fixes the 08:00 entry.
- Current-tree verification: GraphQL `reviewThreads` for PR #2849 lists twenty-eight threads,
  none unresolved; every discussion URL among the source comments has a row.
- Resolution reference: docs(pr-reviews): [#2849] record the findings of reviews 5889973387 and 5903116153
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572707606

### F26 - A reply cited the PR-branch commit id of the F19 fix

- PR number: 2849
- Source review ID: 5903116153
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572548838
- Concern: The F19 reply cited the fixing commit by its abbreviated PR-branch id; `open-pull-request`
  counts review replies among the durable references that must cite the commit subject.
- Solution: A follow-up reply on the F19 thread cites
  `docs(issues): [#2776] name the HTTP tracker's own TLS test for #613`, and F19's Reply URL now
  points to it. Review comments cannot be edited, so the earlier reply stays.
- Current-tree verification: the follow-up reply's `in_reply_to_id` is F19's source comment; F19's
  Reply URL field.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572705458
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572707780

### F27 - Four statements added in round 4 did not match the forge

- PR number: 2849
- Source review ID: 5903116153
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572548845
- Concern: The log dated review 5890114272 at 2026-11-14 07:59 (forge: 2026-11-13 11:25:28), logged
  a push at 07:59 that came after the 08:00 replies, the introduction claimed source order for
  F17 to F19 although F18 and F19 are swapped by comment id, and it counted twenty threads where
  the forge had twenty-four.
- Solution: An appended correction entry gives the review time and the push order; the
  introduction now says F18 and F19 were not assigned in source order, keeping the IDs because a
  review-finding reference never changes, and counts the forge's twenty-eight threads.
- Current-tree verification: the forge's `submitted_at` for review 5890114272; the replies'
  `created_at` and the Docs Lint run created for the pushed head at 08:05:16; comment ids
  r4562698575, r4562698624, r4562698682 in that order.
- Resolution reference: docs(pr-reviews): [#2849] correct the round-4 review time, push order and counts
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572707957

### F28 - Rows F17 to F19 recorded the severity Medium, which the audit scheme does not have

- PR number: 2849
- Source review ID: 5903116153
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572548851
- Concern: The audit scheme allows `Blocker`, `Major`, `Minor`, `Nit` or `Suggestion`, marked
  inferred when derived from prose; rows F17 to F19 recorded `Medium (inferred)`.
- Solution: Rows F17 to F19 now record `Minor (inferred)`, like the earlier automated rows.
- Current-tree verification: no row in the Findings table records a severity outside the five
  allowed values.
- Resolution reference: docs(pr-reviews): [#2849] use an allowed severity for the automated round-4 rows
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572708142

## Processing Log

- 2026-11-12 17:12 UTC - Review 5878819832 (da2ce7) submitted: F1 to F6.
- 2026-11-12 17:18 UTC - Review 5878898401 (Copilot) submitted: F7, F8 (re-raise of F4), F9.
- 2026-11-12 20:28 UTC - Replied to and resolved the F1 to F9 threads, before this record existed;
  late start logged now per the workflow's late-start rule. Two replies used "Not changed:" rather
  than the exact `Superseded by` form (F11), and six cited the branch SHA of `docs(issues): address
  review findings on the related-issues record` (F11).
- 2026-11-13 07:38 UTC - Review 5887683678 (da2ce7) submitted: F10 (reviewer F7), F11 (reviewer
  F8), F12 (reviewer F2, re-raise of F2), F13 (reviewer F4, re-raise of F4), F14 (reviewer F9), F15
  (reviewer F10).
- 2026-11-13 07:45 UTC - Review 5887735321 (Copilot) submitted: approval recommended, no new findings.
- 2026-11-13 09:41 UTC - Pushed the commit fixing F12's remainder.
- 2026-11-13 09:42 UTC - Pushed the commit fixing F13's remainder.
- 2026-11-13 09:43 UTC - Pushed the commit fixing F14's table padding.
- 2026-11-13 10:13 UTC - Review 5889290894 (da2ce7) submitted: F16 (reviewer F11).
- 2026-11-13 10:48 UTC - Split the commit that fixed F1, F2, F4, F5 and F6 into five independent
  commits and rebased the three later fix commits on top, closing F15; the resulting tree is
  unchanged.
- 2026-11-13 10:52 UTC - Created this audit record, closing F10.
- 2026-11-13 10:55 UTC - Replied to and resolved all seventeen threads, citing each fix's commit
  subject or the exact `Superseded by` form, closing F11 and F16.
- 2026-11-14 07:59 UTC - Review 5890114272 (Copilot) submitted: F17 and F18 (re-raises of F7), F19.
- 2026-11-14 07:59 UTC - Pushed the commit fixing F19.
- 2026-11-14 08:00 UTC - Replied to and resolved the F17, F18 and F19 threads; all twenty threads
  now have a reply and are resolved.
- 2026-11-14 08:55 UTC - Correction (F20): replaced every PR-branch commit id in this record with
  the commit's Conventional Commit subject, including the one in the 2026-11-12 20:28 entry, whose
  event is otherwise unchanged.
- 2026-11-14 08:56 UTC - Correction (F23): the 2026-11-13 10:55 entry counts seventeen threads;
  the forge had sixteen (six, three, six and one from the first four reviews). F7's and F9's
  threads, not F8's, were the ones answered "Not changed". F15's tree identity holds between the
  replaced commit and the last split commit, not the new head, and its commit count is replaced by
  the list of split subjects.
- 2026-11-14 08:57 UTC - Correction (F22): the 2026-11-13 09:41, 09:42 and 09:43 "Pushed" entries
  give the three commits' author times. Their first commit times were 09:41:51, 09:46:19 and
  09:46:19 (`git log --format=%cI`), and they were pushed together: the forge created the Docs Lint
  run for that head at 09:49:23. The 10:52 entry is not the record's commit time, which is 10:54:14.
  The 10:55 entry is not the reply time: the forge dates the fifteen new replies 10:52:48 to
  10:53:20. Those replies came before the record's commit and before the force-push that published
  it (11:01:25, the forge's force-push event); the F10 reply cites that commit's subject, and the
  F11, F15 and F16 replies point to this file, before either existed on the forge.
- 2026-11-14 08:58 UTC - Correction (F27): the forge submitted review 5890114272 at
  2026-11-13 11:25:28, not 2026-11-14 07:59. The commit fixing F19 was committed at 07:59:31, not
  pushed then: the three replies of 08:00:18 to 08:00:30 were posted while the head was still the
  audit-record commit, and the forge created the Docs Lint run for the pushed head at 08:05:16.
  The 08:00 entry's "all twenty threads now have a reply and are resolved" is wrong: the forge then
  had twenty-four threads, and the five of review 5889973387 had no reply and no resolution (F25).
- 2026-11-14 08:59 UTC - Edited the PR body to list this record and say the history was rewritten
  (the command returned at 08:59:07), closing F21.
- 2026-11-14 08:59 UTC - Posted a follow-up reply on the F19 thread citing the fix's commit subject
  (forge time 08:59:28), closing F26.
- 2026-11-14 09:00 UTC - Replied to the nine threads of reviews 5889973387 and 5903116153 (forge
  times 08:59:44 to 09:00:03), each citing its fixing commit subject before that commit was pushed.
- 2026-11-14 09:01 UTC - Recorded review 5889973387 (da2ce7, submitted 2026-11-13 11:14:32, missed
  in the previous round): F20 (reviewer F12) to F24 (reviewer F16); and review 5903116153 (da2ce7,
  submitted 2026-11-14 08:22:36): F25 (reviewer F17) to F28 (reviewer F20).
- 2026-11-14 09:02 UTC - Resolved those nine threads (the command ran from 09:02:27 to 09:02:39),
  closing F25; GraphQL then listed twenty-eight threads, none unresolved.

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
