---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3063 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3063.

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
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
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

Findings F1 to F7 come from review 6070914967 and findings F8 to F10 from review 6075692867, both by `da2ce7`; F11 to F13 come from review 6075857869 by `coderabbitai[bot]`; F14 is an inline comment of `da2ce7` (14:32 UTC) in review 6075885313, F15 and F16 come from review 6076348650 by `da2ce7`, and F17 to F19 are follow-up replies of `coderabbitai[bot]` (14:46 UTC, reviews 6076107248, 6076112434 and 6076112623) to the replies on the threads of F12, F13 and F11, which the poller did not report. The
`claude[bot]` review (6062721178, automated review is disabled for fork pull requests) and the
`coderabbitai[bot]` rate-limit notice hold no actionable assertion and have no row, and neither does the `coderabbitai[bot]` reply of 15:23 UTC (4710387901, review 6076691282), which concedes F19 and asks for nothing.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3063-f1 | da2ce7 | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3063-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3063-f3 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3063-f4 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3063-f5 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3063-f6 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3063-f7 | da2ce7 | Suggestion | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3063-f8 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3063-f9 | da2ce7 | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3063-f10 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3063-f11 | coderabbitai[bot] | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3063-f12 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3063-f13 | coderabbitai[bot] | Minor | security | ORIGINAL | NO_ACTION | SUPERSEDED |
| F14 | review-finding:pr-3063-f14 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3063-f15 | da2ce7 | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3063-f16 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3063-f17 | coderabbitai[bot] | Minor (inferred) | other | RE_RAISE_OF:F12 | NO_ACTION | SUPERSEDED |
| F18 | review-finding:pr-3063-f18 | coderabbitai[bot] | Minor (inferred) | security | RE_RAISE_OF:F13 | NO_ACTION | SUPERSEDED |
| F19 | review-finding:pr-3063-f19 | coderabbitai[bot] | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Rule 10 still holds the open duplicate-key status

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774545
- Concern: The ADR still contained the open decision on the status of a duplicate REST key and a sentence promising an update before merge; merged at that head it would record no status and the sentence would be false.
- Solution: Recorded that a duplicate key keeps its 500 in v1 with the new text, removed the open block and the sentence, updated the REST table lead-in, the duplicate-key row and the pull request description.
- Current-tree verification: Read rule 10 and the REST table of the ADR at the new head: no `Open for the maintainer` text remains (`grep -n 'Open for\|open decision'` prints nothing).
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709492415

### F2 - The duplicate-key text in the Description predates #2999

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774555
- Concern: The Description said a duplicate key answers `UNIQUE constraint failed: keys.key`, which was true only before #2999.
- Solution: The Description now gives that text as history and states the current reply and that the status is still 500.
- Current-tree verification: Read the Description's database bullet in the ADR: it names #2999 and the current text.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709492920

### F3 - Only the no-persistence container configuration binds the health-check API to 0.0.0.0:1313

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774564
- Concern: The ADR said the container configurations bind the health-check API to `0.0.0.0:1313`; only the no-persistence and the e2e configurations do.
- Solution: Rule 9 now names the default `127.0.0.1:1313`, the two configurations that bind `0.0.0.0:1313` and that an operator can bind it publicly.
- Current-tree verification: Read rule 9 of the ADR.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709493436

### F4 - `char::escape_default` escapes more than control characters

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774572
- Concern: `escape_default` also escapes quotes, backslash and all non-ASCII characters, and the ADR did not say whether the 64-character cut counts before or after escaping.
- Solution: Rule 5 uses `char::escape_debug`, counts the 64 characters on the value as sent and escapes after the cut.
- Current-tree verification: Read rule 5 of the ADR.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709493938

### F5 - The tracker release keeps no changelog

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774580
- Concern: Rule 4 promised a changelog entry for a changed text, but the tracker has no changelog.
- Solution: Rule 4 now says the pull request that changes a text lists old and new in its description and the notes of the GitHub release summarize them.
- Current-tree verification: Read rule 4 of the ADR.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709494490

### F6 - The v2 mapping lists 409 only under refused

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774589
- Concern: Rule 10 listed `409` only under refused while a duplicate key, a bad request, maps to it.
- Solution: Rule 10 lists `409` under bad request too and says a duplicate key is a `409` in v2.
- Current-tree verification: Read the v2 paragraph of rule 10.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709495285

### F7 - Mark the amendment in the errors ADR's index row

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774600
- Concern: The index row of the errors ADR still summarized rule 4 without marking the amendment.
- Solution: The row now ends with `(amended 2026-11-25)`, like the other amended ADRs.
- Current-tree verification: `grep -c 'amended 2026-11-25' docs/adrs/index.md` prints 1.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709495910

### F8 - No audit record exists at this head

- PR number: 3063
- Source review ID: 6075692867
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709637476
- Concern: The seven threads of the first review were answered and resolved, but `docs/pr-reviews/pr-3063-review/PR-REVIEW.md` did not exist, as `process-pr-review` requires before the first reply.
- Solution: Created the audit record with one row per finding of both rounds; it is a late start, logged in the Processing Log.
- Current-tree verification: Ran `validate-audit-record.py --pr-number 3063` at the new head.
- Resolution reference: docs(pr-reviews): [#3000] record the review rounds of pull request 3063
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709719639

### F9 - The branch conflicts with develop in docs/adrs/index.md

- PR number: 3063
- Source review ID: 6075692867
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709637487
- Concern: `develop` inserted the `20261124120307` row next to the edited row, so the branch conflicted and the pull request was not mergeable.
- Solution: Rebased the branch onto `develop` and resolved the conflict by keeping this ADR's rows and the new row, in order.
- Current-tree verification: `git merge-tree --write-tree torrust/develop HEAD` after the rebase exits 0, and the index lists the `20261124120307`, `20261124182709` and `20261125175726` rows in order.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709720083

### F10 - No record shows the decision the ADR attributes to the maintainer

- PR number: 3063
- Source review ID: 6075692867
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709637495
- Concern: The ADR sentence in rule 10 attributed the duplicate-key decision to the maintainer, but no record at that head showed it.
- Solution: The ADR sentence states the rule without naming who decided it, and the specification's progress log records the decision with its date and how it was given.
- Current-tree verification: Read the end of rule 10 and the progress-log entry of 2026-11-26 in the specification.
- Resolution reference: docs(adrs): [#3000] drop the decision attribution from rule 10 and record it in the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709720583

### F11 - Use stable issue references in the ADR metadata

- PR number: 3063
- Source review ID: 6075857869
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709773181
- Concern: The ADR's frontmatter links three issue specifications under `docs/issues/open/`, which move to `docs/issues/closed/` when the issues close; a long-lived artifact should name the issues by number.
- Solution: Removed the three paths from `related-artifacts` and named the issues by number in the Description and the References section.
- Current-tree verification: `grep -n 'docs/issues/open'` on the ADR prints nothing.
- Resolution reference: docs(adrs): [#3000] link the issues by number in the ADR metadata and references
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709888558

### F12 - Specify correlation for health-check internal failures

- PR number: 3063
- Source review ID: 6075857869
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709773192
- Concern: Rules 1, 2 and 7 require an operator-searchable reference for internal failures, but the fixed health-check result for a panicked probe job carries no request id and rule 9 named no other reference.
- Solution: Rule 9 now says the report keeps its shape, the reference is the `x-request-id` header the health-check API sets on every reply, and the rule 7 event carries the same id and the service binding.
- Current-tree verification: Read rule 9 of the ADR; `packages/axum-health-check-api-server/src/server.rs` sets `SetRequestIdLayer` and `PropagateHeaderLayer` for `x-request-id`. This first check named only the two layers and missed that the span does not declare `request_id`; F15 corrected it, and rule 9 now relies on the span field that rule 7 requires (`request_id = tracing::field::Empty` in `RedactedMakeSpan`, `packages/axum-server/src/request_log.rs`).
- Resolution reference: docs(adrs): [#3000] require the request span to declare request_id
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709888992

### F13 - The health-check report can return credentials from public_url

- PR number: 3063
- Source review ID: 6075857869
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709773200
- Concern: The `public_url` of a service is serialized unchanged into the health-check report and may hold userinfo, query or fragment; the ADR should state that it must not expose credentials.
- Solution: No change. The field is not an error text and appears in every report, so it is outside this policy; the EPIC #2993 inventory checked the health-check output and found it safe; the URL is the one clients use, so credentials in it are an operator misconfiguration. Rejecting such a URL in configuration validation would be a separate issue if the maintainer wants it.
- Current-tree verification: `packages/axum-health-check-api-server/src/resources.rs` holds `public_url: Option<String>` copied from the service metadata; the ADR's rule 9 changes only the failure results.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709889463
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709889463

### F14 - The audit record was still missing though a reply said a commit created it

- PR number: 3063
- Source review ID: 6075885313
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709795338
- Concern: Reply 4709719639 (14:26 UTC) named a commit that created the audit record, but the force-push of 14:25 UTC carried four commits and none of them did, so the reply described work the branch did not hold. The review asked that, when the record lands, the commit ids the reviews cite are mapped to their commit subjects.
- Solution: The record landed on the branch with the push of 14:45 UTC. The ids cited by the reviews map to these subjects: `8f16e324a` is `docs(issues): [#3000] record the ADR draft (T1) in the SI-B specification`, and `b2942ac31` is `docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings`. The reply was posted before the push, and the thread was resolved before the reply; the Processing Log records both.
- Current-tree verification: `git log torrust/develop..HEAD` lists the commit with the subject below and `validate-audit-record.py --pr-number 3063` passes with this row.
- Resolution reference: docs(pr-reviews): [#3000] record the review rounds of pull request 3063
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709958980

### F15 - The request span declares no request_id, so an event in it does not carry the id

- PR number: 3063
- Source review ID: 6076348650
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710122588
- Concern: All three axum servers build the span with `RedactedMakeSpan`, which declares only `method`, `uri` and `version`, so `span.record("request_id", ...)` records nothing; rules 2, 7 and 9 of the ADR claimed the event carries the id. The audit row of F12 verified only the header.
- Solution: Rule 7 now says the span must declare the field (`request_id = tracing::field::Empty` added to `RedactedMakeSpan` by the implementation) and that the event is emitted inside it; rule 2's option 3 and rule 9 follow, and "the other two servers" is "the HTTP tracker and the REST API". The F12 row cites this commit and verifies the span field.
- Current-tree verification: `packages/axum-server/src/request_log.rs` lines 126-135 declare only `method`, `uri` and `version`; the UDP handler (`packages/udp-server/src/handlers/mod.rs`) declares `request_id` in `#[instrument(fields(request_id))]`; read rules 2, 7 and 9 of the ADR.
- Resolution reference: docs(adrs): [#3000] require the request span to declare request_id
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710185694

### F16 - The log entry dates the fixes of review 6070914967 by their rebase

- PR number: 3063
- Source review ID: 6076348650
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710122605
- Concern: The entry of 14:21 UTC dates the fixes of F1 to F7 by the committer date of `docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings` after the rebase; its author date is 14:04:41Z, so the log puts the fixes after the replies of 14:07 that announced them.
- Solution: A correcting entry is appended to the Processing Log (the log is append-only and chronological) with the author date and the explanation; the earlier entry is not rewritten.
- Current-tree verification: `git log --format='%aI %cI %s'` shows author date 14:04:41Z and committer date 14:21:33Z for that commit.
- Resolution reference: docs(pr-reviews): [#3000] record the third review round of da2ce7 on pull request 3063
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710186181

### F17 - CodeRabbit asks for the audit validator to be run before the F12 thread is resolved

- PR number: 3063
- Source review ID: 6076107248
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#pullrequestreview-6076107248
- Concern: CodeRabbit could not run the repository's audit validator in its sandbox and asked for it to be run for PR 3063 after the reply URLs were recorded, before the thread of F12 is resolved. (The source is the reply `discussion_r4709952021` inside an existing thread, the only comment of its review; the row cites the review because the audit validator anchors a discussion row to the first comment of its thread.)
- Solution: No change. The validator was run before the thread was resolved (`status: ok`, 13 rows at 14:41 UTC) and passes at the current head; the reply says so. Replied with the `Superseded by F12` form; the thread was already resolved and stays resolved.
- Current-tree verification: `validate-audit-record.py --pr-number 3063` prints `status: ok` with 16 rows and 0 failures.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710375345
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710375345

### F18 - CodeRabbit withdraws its public_url request and says the inventory does not prove every URL secret-free

- PR number: 3063
- Source review ID: 6076112434
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#pullrequestreview-6076112434
- Concern: CodeRabbit withdrew the request to change the ADR, and added that the EPIC inventory's safety statement does not establish that every configured `public_url` is secret-free, because the value is checked for its scheme and serialized into the report. (The source is the reply `discussion_r4709955681` inside an existing thread, the only comment of its review; the row cites the review because the audit validator anchors a discussion row to the first comment of its thread.)
- Solution: No change; the decision of F13 stands. Whether to reject credentials in a configured `public_url` is a configuration-validation question left to the maintainer, who is told in the hand-over report. Replied with the `Superseded by F13` form; the thread stays resolved.
- Current-tree verification: `packages/axum-health-check-api-server/src/resources.rs` holds `public_url: Option<String>`; the ADR's rule 9 changes only the failure results.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710375832
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710375832

### F19 - CodeRabbit asks for issue entries in related-artifacts

- PR number: 3063
- Source review ID: 6076112623
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#pullrequestreview-6076112623
- Concern: After the stale paths were removed (F11), CodeRabbit asked to add `issue #3000`, `issue #2993` and `issue #144` to the ADR's `related-artifacts` list, saying the semantic-link convention specifies issue numbers in that list. (The source is the reply `discussion_r4709955819` inside an existing thread, the only comment of its review; the row cites the review because the audit validator anchors a discussion row to the first comment of its thread.)
- Solution: No change. The marker table of `docs/skills/semantic-skill-link-convention.md` gives `related-artifacts` the values `<repo-relative-path>` or `review-finding:pr-<number>-<id>`, and `issue` is a separate marker; no ADR puts `issue #N` strings in `related-artifacts`. The ADR's Description and References name the issues by number, which is what the stability warning asks for.
- Current-tree verification: read the marker table (lines 22-31 of the convention) and `grep -rn 'issue #' docs/adrs/*.md` frontmatter blocks: none lists an issue in `related-artifacts`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710376279
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4710376279

## Processing Log

- 2026-11-26 14:07 UTC - Late start: the seven replies to review 6070914967 were posted (14:07 UTC) before this record existed; the replies carry the dispositions, and this record was created after review 6075692867 noted its absence.
- 2026-11-26 14:21 UTC - Fixes for F1 to F7 committed in `docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings`; F9 resolved by the rebase onto `develop` in the same commit.
- 2026-11-26 14:22 UTC - Fix for F10 committed in `docs(adrs): [#3000] drop the decision attribution from rule 10 and record it in the specification`; the branch was pushed with `--force-with-lease` after the rebase.
- 2026-11-26 14:26 UTC - Replies to F8, F9 and F10 posted.
- 2026-11-26 14:29 UTC - Ran `validate-audit-record.py --pr-number 3063` (`status: ok`, 10 rows, 0 failures), resolved the threads of F8, F9 and F10, and refreshed GraphQL: all ten review threads are resolved.
- 2026-11-26 14:30 UTC - Review 6075857869 (`coderabbitai[bot]`, three inline findings F11 to F13) arrived; the poller reported only its first comment, and a listing of the review's comments showed all three.
- 2026-11-26 14:40 UTC - Fixes for F11 and F12 committed in `docs(adrs): [#3000] link the issues by number in the ADR metadata and references` and `docs(adrs): [#3000] say how a health-check internal failure is correlated with its event` and pushed; replies to F11, F12 and F13 posted (14:40 UTC). F13 is `NO_ACTION` with the `Superseded by F13` reply form.
- 2026-11-26 14:47 UTC - Recorded after the events. `da2ce7` commented at 14:32 UTC (4709795338, F14) that reply 4709719639 described an audit commit the branch did not hold: correct, the reply was posted at 14:26 UTC while the commit existed only locally, and the audit was first pushed at 14:45 UTC. At 14:43 UTC I resolved every open thread in one loop, including the F14 thread before it had a reply, which was out of order; the refreshed GraphQL list then showed all fourteen threads resolved. At 14:46 UTC I replied on the F14 thread (the reply states the mistake and maps the cited commit ids to subjects); the thread stays resolved.
- 2026-11-26 15:02 UTC - Review 6076348650 (`da2ce7`, F15 and F16) arrived at 15:00 UTC. Fix for F15 committed in `docs(adrs): [#3000] require the request span to declare request_id` and pushed; replies to F15 and F16 posted (15:06 UTC).
- 2026-11-26 15:07 UTC - Correction for F16: the fixes of F1 to F7 and F9 are in `docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings`, authored at 14:04 UTC (before the 14:07 replies); the 14:21 UTC stamp of the earlier entry is that commit's committer date after the rebase onto `develop`. The F12 row was updated in place to cite the F15 commit and to verify the span field.
- 2026-11-26 15:20 UTC - `da2ce7` approved at head `2819aae20` (review 6076660183) and noted that three follow-up comments of `coderabbitai[bot]` from 14:46 UTC (4709952021, 4709955681, 4709955819) had no reply and no row, and that the F16 row cited the wrong commit for the log correction.
- 2026-11-26 15:22 UTC - Read the three follow-ups, replied to each with the `Superseded by` form (15:22 UTC), recorded them as F17 to F19, and corrected the resolution reference of F16 to the commit that carries the log correction. The threads were already resolved and stay resolved. The poller did not report these comments because I restarted it with the time of my own reply (14:46:49 UTC) as its cursor, which is later than the three comments (14:46:15 to 14:46:34 UTC); the cursor should come from the last reported event, and all reviews should be listed before a round is recorded.

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
