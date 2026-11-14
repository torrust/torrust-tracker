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
both replied to and resolved before this file existed (F8's thread and the `related-issues.md`
pipe/timezone threads were answered with "Not changed" rather than the exact `Superseded by`
form). F1 to F6 are da2ce7's own finding IDs from review 5878819832, with no collision, so no
reassignment. F7 to F9 are Copilot's review 5878898401, source order; Copilot gives no finding ID.
F10 to F15 are da2ce7's review 5887683678; his own IDs F7, F8, F2, F4, F9 and F10 all collide with
an existing audit ID (F7, F8, F2, F4, F9, and the just-assigned F10 itself), so each is
reassigned the next ordinal and the original ID is recorded as the reviewer finding ID. His F2 and
F4 are re-raises of the audit's own F2 and F4. Copilot's review 5887735321 found nothing new
(approval recommended, three prior findings resolved) and adds no row. F16 is da2ce7's review
5889290894; his own ID F11 collides with the audit's F11, so it is reassigned. F17 to F19 are
Copilot's review 5890114272, source order; Copilot gives no finding ID. F17 and F18 are re-raises
of F7 (the same rendering-artefact claim, now on two different lines after the table moved and
the audit record itself grew a table). All twenty threads are inline.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------- | ------------- | -------- | -------------- | ------------------ | ------------ | ------------- |
| F1 | review-finding:pr-2849-f1 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2849-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2849-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2849-f4 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2849-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2849-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2849-f7 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | RESOLVED |
| F8 | review-finding:pr-2849-f8 | Copilot | Minor (inferred) | metadata | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F9 | review-finding:pr-2849-f9 | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | RESOLVED |
| F10 | review-finding:pr-2849-f10 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2849-f11 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2849-f12 | Human | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F13 | review-finding:pr-2849-f13 | Human | Nit | metadata | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F14 | review-finding:pr-2849-f14 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2849-f15 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2849-f16 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2849-f17 | Copilot | Medium (inferred) | formatting | RE_RAISE_OF:F7 | NO_ACTION | RESOLVED |
| F18 | review-finding:pr-2849-f18 | Copilot | Medium (inferred) | formatting | RE_RAISE_OF:F7 | NO_ACTION | RESOLVED |
| F19 | review-finding:pr-2849-f19 | Copilot | Medium (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

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
- Solution: The entry now reads `2026-11-12 16:24 UTC` (the commit time of `0d3331c00`), and both
  files' `last-updated-utc` are bumped. (The remaining part, the bumped value postdating the
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

### F11 - Six thread replies cited the PR-branch commit id `0aab68fdf` instead of its commit subject

- PR number: 2849
- Source review ID: 5887683678
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4560841010
- Concern: Replies on the F1, F2, F4, F5 and F6 threads and on F8's thread read "Fixed in
  0aab68fdf", a branch SHA that can change after a rebase (and did, during this round); the two
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
  20:30`, but the commit that wrote them, `0aab68fdf`, had commit time `20:20:18Z`; `process-pr-review`
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
- Concern: Commit `0aab68fdf` fixed F1, F2, F4, F5 and F6 together with no body, so no audit row
  could cite its own commit subject, and the commit named none of the findings. `process-pr-review`
  (step 6) asks for one independent fix per Conventional Commit.
- Solution: `0aab68fdf` was split, before merge, into five commits, one per finding (F1, F6, F2,
  F5, F4 in that order), and the three later fix commits were rebased on top unchanged; the
  resulting tree is byte-identical to `0aab68fdf`'s tree (`git diff 0aab68fdf <new-head> --
  docs/issues/closed/2776-2411-axum-server-0-9-timeouts/` is empty).
- Current-tree verification: `git log --oneline 0d3331c00..HEAD` shows eight commits, each with a
  Conventional subject naming one concern; the empty diff above.
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
  seventeen threads now have a reply and are resolved.
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
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2849#discussion_r4572421737

## Processing Log

- 2026-11-12 17:12 UTC - Review 5878819832 (da2ce7) submitted: F1 to F6.
- 2026-11-12 17:18 UTC - Review 5878898401 (Copilot) submitted: F7, F8 (re-raise of F4), F9.
- 2026-11-12 20:28 UTC - Replied to and resolved the F1 to F9 threads, before this record existed;
  late start logged now per the workflow's late-start rule. Two replies used "Not changed:" rather
  than the exact `Superseded by` form (F11), and six cited the branch SHA `0aab68fdf` (F11).
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
