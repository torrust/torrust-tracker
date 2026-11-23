---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3008 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3008>.

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

Round 1 covers review 6024919812 (da2ce7, CHANGES_REQUESTED, submitted 2026-11-23T16:03:08Z, of the head whose last commit is `docs(issues): [#3006] link spec-only PR #3008`) with seven inline comments, each with a `[Severity][FindingId]` first line; the IDs F1 to F7 are kept and the severities are the brackets. Its body summarizes the same seven findings and the checks the reviewer ran; it makes no row.

Round 1 also covers review 6024982355 (Copilot, COMMENTED, submitted 2026-11-23T16:07:10Z) with three inline comments without IDs or severity brackets, recorded as F8 to F10. F9 has the same text as F8 on another line and is a re-raise with its own row.

Round 2 covers review 6025241076 (da2ce7, COMMENTED, submitted 2026-11-23T16:23:52Z, of the head whose last commit is `docs(issues): [#3006] move http scrape entries under files and name all consumers`) with three inline comments that carry the reviewer IDs F8, F9 and F10. Those IDs collide with the audit's F8 to F10 (Copilot's threads), so they are recorded as F11, F12 and F13, with the reviewer's ID in each detail entry; the severities are the brackets. Its body also says "F2 stands", a re-raise of F2 without a thread, recorded as F14 with the review URL as its source.

Round 3 covers review 6025333875 (da2ce7, COMMENTED, submitted 2026-11-23T16:29:55Z, of the head whose last commit is `docs(issues): [#3006] say that --format text is pretty-printed JSON`) with no inline comment. Its body accepts the F2 route, restates F11 to F13 (its F8 to F10) and adds no finding, so it makes no row.

Round 4 covers review 6025551279 (Copilot, COMMENTED, submitted 2026-11-23T16:44:16Z, of the head whose last commit is `docs(issues): [#3006] say that --format text is pretty-printed JSON`, a lite review) with one inline comment, recorded as F15. Its body is an overview that repeats it and lists three earlier threads as resolved; it makes no row. The overview marks it Low, recorded as `Nit (inferred)`.

Round 5 covers review 6025811357 (da2ce7, CHANGES_REQUESTED, submitted 2026-11-23T17:00:35Z, of the head whose last commit is `docs(issues): [#3006] say that agent-reply-validator uses a schema version`) with three inline comments that carry the reviewer IDs F12, F13 and F14. They collide with the audit's F12 to F14, so they are recorded in source order as F16 (their F13), F17 (their F12) and F18 (their F14), with the reviewer's ID in each detail entry. Its body summarizes them and makes no row.

Round 6 covers review 6025861610 (Copilot, COMMENTED, submitted 2026-11-23T17:04:02Z, of the head whose last commit is `docs(issues): [#3006] say that agent-reply-validator uses a schema version`) with three inline comments without IDs or severity brackets, recorded as F19, F20 and F21; severities are inferred. F21 asks for the same F15 entry as F18 and is a re-raise.

Round 7 covers review 6025926288 (da2ce7, submitted 2026-11-23T17:08:10Z, of the head whose last commit is `docs(pr-reviews): [#3006] record review round 4 of PR #3008`; GitHub shows its state as DISMISSED) with one inline comment carrying the reviewer ID F16, which collides with the audit's F16, so it is recorded as F22. Its body restates F12 (audit F17) as standing and makes no other row; da2ce7 posted an ACK of that head at 17:08:14Z.

Round 8 covers review 6026068748 (CodeRabbit, COMMENTED, submitted 2026-11-23T17:16:52Z, of the head whose last commit is `docs(pr-reviews): [#3006] record review round 5 of PR #3008 and drop branch commit ids`) with one inline comment marked Minor, recorded as F23 (author class `Unknown`, a bot that is not Copilot). Its body counts the one actionable comment and makes no row. It also covers review 6026144266 (da2ce7, COMMENTED, submitted 2026-11-23T17:20:32Z, same head) with two inline comments carrying the reviewer IDs F19 and F20, which collide with the audit's F19 and F20, so they are recorded as F24 and F25; F25 asks for the same T2 change as F23 and is a re-raise.

Round 9 covers review 6026332138 (da2ce7, submitted as APPROVED and now shown as DISMISSED, submitted 2026-11-23T17:31:06Z, of the head whose last commit is `docs(pr-reviews): [#3006] record review rounds 6 and 7 of PR #3008`) with one inline comment carrying the reviewer ID F23, which collides with the audit's F23, so it is recorded as F26.

Round 10 covers review 6026521422 (CodeRabbit, COMMENTED, submitted 2026-11-23T17:42:25Z, of the head whose last commit is `docs(issues): [#3006] make T2 red-green increments agree with the test loop`), a follow-up reply in the resolved F23 thread that asks the T2 row and the loop's first step to name the first-run-green exception, recorded as F27. It also covers review 6026616249 (da2ce7, submitted 2026-11-23T17:48:17Z, same head; now shown as DISMISSED) with two inline comments carrying the reviewer IDs F24 and F25, recorded as F28 (a re-raise of F27) and F29.

Round 11 covers review 6026749477 (da2ce7, APPROVED, submitted 2026-11-23T17:56:39Z, of the head whose last commit is `docs(pr-reviews): [#3006] record review rounds 8 and 9 of PR #3008`) with two inline comments carrying the reviewer IDs F27 and F28, recorded as F30 and F31.

Round 12 covers review 6027138634 (Copilot, COMMENTED, submitted 2026-11-23T18:22:27Z, of the head whose last commit is `docs(issues): [#3006] name the first-run-green exception in the T2 row and the loop`) with two inline comments without IDs or severity brackets, recorded as F32 and F33 with inferred severities. Both report table rows that start with `||`; neither file has one, so both are recorded as no-change findings, as PR 2237's audit records the same report.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3008-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3008-f2 | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | RESOLVED |
| F3 | review-finding:pr-3008-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3008-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3008-f5 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3008-f6 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3008-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3008-f8 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3008-f9 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F8 | NO_ACTION | SUPERSEDED |
| F10 | review-finding:pr-3008-f10 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3008-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3008-f12 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3008-f13 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3008-f14 | Human | Minor | documentation | RE_RAISE_OF:F2 | FOLLOW_UP | NON_RESOLVABLE |
| F15 | review-finding:pr-3008-f15 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3008-f16 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3008-f17 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3008-f18 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-3008-f19 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-3008-f20 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-3008-f21 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F18 | NO_ACTION | SUPERSEDED |
| F22 | review-finding:pr-3008-f22 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-3008-f23 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-3008-f24 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-3008-f25 | Human | Minor | testing | RE_RAISE_OF:F23 | NO_ACTION | SUPERSEDED |
| F26 | review-finding:pr-3008-f26 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-3008-f27 | Unknown | Minor | testing | RE_RAISE_OF:F23 | FIXED | RESOLVED |
| F28 | review-finding:pr-3008-f28 | Human | Minor | testing | RE_RAISE_OF:F27 | NO_ACTION | SUPERSEDED |
| F29 | review-finding:pr-3008-f29 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F30 | review-finding:pr-3008-f30 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F31 | review-finding:pr-3008-f31 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F32 | review-finding:pr-3008-f32 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | RESOLVED |
| F33 | review-finding:pr-3008-f33 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | RESOLVED |

## Finding Details

### F1 - The HTTP scrape result is a map keyed by info-hash, so a top-level `schema_version` does not fit it as the spec claimed

- PR number: 3008
- Source review ID: 6024919812
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668362839
- Concern: `http scrape` serializes the scrape response as a top-level map keyed by 40-hex info-hashes, so a `schema_version` key would sit among the data keys and break consumers that read each key as an info-hash, such as the package test helper `returned_files`. The UDP result's single top-level key is the variant tag. da2ce7 asked to name both shapes, record the maintainer's decision and fit T2 to T4.
- Solution: Background names both shapes. The maintainer decided D4 (the scrape entries move under `files`, and the ADR rule adds that a result never uses data as top-level keys) and D5 (the UDP field goes beside the variant tag). D3 now names both changes that `1` marks. Scope, T1 to T5, AC2, a new AC5 and Risks follow.
- Current-tree verification: `grep -c '^- D4, the .http scrape. shape' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c '^- D5, the UDP shape' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c 'AC5' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 2; `sed -n 63,74p packages/http-protocol/src/v1/responses/scrape/deserialization.rs` shows the map serializer with hex keys.
- Resolution reference: `docs(issues): [#3006] move http scrape entries under files and name all consumers`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668650441

### F2 - The EPIC does not list this subissue or its order before #2770

- PR number: 3008
- Source review ID: 6024919812
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668362857
- Concern: `EPIC.md` of #2758 has no row for #3006 and its delivery strategy keeps #2770 last with no ordering against this issue; da2ce7 asked to add the row, its wave and the ordering in this PR.
- Solution: No change in this PR: the EPIC coordinator adds the #3006 row, its wave and the order before #2770 in PR #3007, which edits the EPIC and the #2770 spec. Keeping the EPIC out of this PR avoids two PRs editing the same file. The spec's progress log points to #3007.
- Current-tree verification: `gh-bot pr view 3007 --repo torrust/torrust-tracker --json title,files` shows the title "docs(issues): [#2758] add #3004 and #3006 to the EPIC and the #2770 scope" and the files `EPIC.md` of #2758 and `ISSUE.md` of #2770.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668650939
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/3007
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668650939

### F3 - The consumer list and the file set miss skills this change makes stale

- PR number: 3008
- Source review ID: 6024919812
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668362870
- Concern: `use-rest-api` and `public-trackers-for-testing` also pipe the result to `jq`, `use-tracker-client` shows whole samples, and the file set named only `console/tracker-client/**` and the ADR, omitting `.claude/skills/**` and the e2e helper in #2769's set.
- Solution: Scope lists the package tests, six skills by name and the README; the file set adds the six skill paths; the e2e helper is stated as needing no change because it only logs the stdout, and stays in #2769's set.
- Current-tree verification: `grep -c 'use-rest-api' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 2; `grep -c 'public-trackers-for-testing' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 2; `grep -c 'needs no change: it only logs' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] move http scrape entries under files and name all consumers`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668651485

### F4 - The verification policy does not ask for toolchain-qualified evidence

- PR number: 3008
- Source review ID: 6024919812
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668362885
- Concern: `create-issue` requires toolchain-qualified evidence for every recorded command result where a toolchain can affect behaviour; the plan did not ask for it.
- Solution: The Verification Plan asks to record the commands, output and the toolchain of each result in `manual-verification-evidence.md`, from the template.
- Current-tree verification: `grep -c 'the toolchain that produced each result' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] move http scrape entries under files and name all consumers`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668651934

### F5 - The checkpoints record the issue as created while the spec is not yet approved

- PR number: 3008
- Source review ID: 6024919812
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668362903
- Concern: The "GitHub issue created" box was ticked while the "Spec reviewed and approved" box was not; da2ce7 asked to tick it if the review approved the spec, or record which approval is pending.
- Solution: The Progress Log records that the maintainer answered D1 to D3 before the issue was created and that the approval of the whole spec, now with D4 and D5, is the review and merge of this PR, so the checkpoint stays open until then.
- Current-tree verification: `grep -c 'so that checkpoint stays open until then' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c '^- \[ \] Spec reviewed and approved' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] move http scrape entries under files and name all consumers`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668652430

### F6 - `tracker_client` result keys are not all snake_case

- PR number: 3008
- Source review ID: 6024919812
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668362913
- Concern: The HTTP announce result carries `min interval` and `peer id`, the UDP tag is PascalCase and the scrape keys are info-hashes, so "snake_case like every other key" was wrong.
- Solution: D2 compares only with the keys `tracker_client` defines itself (`results`, `udp_trackers`).
- Current-tree verification: `grep -c 'like every other' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 0; `grep -c 'defines itself' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] move http scrape entries under files and name all consumers`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668652931

### F7 - D3 reads as a fact about the contract doc at this head

- PR number: 3008
- Source review ID: 6024919812
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668362930
- Concern: D3 said the contract doc names the change that `1` marks, in the present tense, but the contract doc names no version today.
- Solution: D3 reads "The contract doc will name the two changes that `1` marks".
- Current-tree verification: `grep -c 'The contract doc will name' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] move http scrape entries under files and name all consumers`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668653358

### F8 - `--format text` was not stated to be JSON

- PR number: 3008
- Source review ID: 6024982355
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668413134
- Concern: The spec asks for `schema_version` in both `--format` values and checks it with `jq`; Copilot asked whether `text` is JSON, since non-JSON output would make the requirement ambiguous.
- Solution: Background and Scope state that `text` is the same JSON pretty-printed (`OutputFormat::is_pretty`), so both formats are one JSON value that `jq` reads.
- Current-tree verification: `grep -c 'pretty-printed (.OutputFormat::is_pretty' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `sed -n 13,18p console/tracker-client/src/console/clients/unified/app.rs` shows `is_pretty` true for `Text`.
- Resolution reference: `docs(issues): [#3006] say that --format text is pretty-printed JSON`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668653883

### F9 - Same `--format text` comment on the verification scenario

- PR number: 3008
- Source review ID: 6024982355
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668413220
- Concern: Same text as F8, anchored on the manual verification scenario.
- Solution: No separate change: F8's fix covers it.
- Current-tree verification: Same as F8.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668654463
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668654463

### F10 - "da2ce7 raised it" is ambiguous

- PR number: 3008
- Source review ID: 6024982355
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668413291
- Concern: Copilot read `da2ce7` as possibly a short SHA and asked for an unambiguous reference.
- Solution: The Background reads "The reviewer da2ce7 (GitHub user `da2ce7`) raised it".
- Current-tree verification: `grep -c 'GitHub user .da2ce7.' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] say that --format text is pretty-printed JSON`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668655015

### F11 - The head processed the round-1 review without an audit record, and no thread had a reply

- PR number: 3008
- Source review ID: 6025241076
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668624060
- Concern: At the head whose last commit is `docs(issues): [#3006] move http scrape entries under files and name all consumers`, the round-1 fixes were committed but `docs/pr-reviews/pr-3008-review/PR-REVIEW.md` did not exist, and none of the ten threads had a reply; `process-pr-review` asks for the record before the first reply.
- Solution: The record was created after the replies (a late start, logged below) with a row and a detail entry for each of the ten round-1 threads; the replies and resolutions were done at 16:26 UTC.
- Current-tree verification: `ls docs/pr-reviews/pr-3008-review/PR-REVIEW.md` lists the file; the audit validator run with the bot token prints `"rows": 10, "failures": 0` for the round-1 rows.
- Resolution reference: `docs(pr-reviews): [#3006] audit record of the first review round of PR #3008`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668834063

### F12 - D4 and D5 cite an answer the PR's record does not hold

- PR number: 3008
- Source review ID: 6025241076
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668624070
- Concern: D4 and D5 said "answered in the review of PR #3008, finding F1", but the F1 thread held only the finding, and the PR's only maintainer comment is an ACK of an earlier head.
- Solution: D4 says the maintainer answered the spec author's question in its agent session on 2026-11-23 and links the reply on the F1 thread that records the answer; D5 refers to the same record. The Decisions intro says when each group of decisions was answered.
- Current-tree verification: `grep -c 'discussion_r4668650441' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c 'same question, answer and record as D4' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] trace the D4 and D5 answers and keep decisions and ACs in order`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668834565

### F13 - D4, D5 and AC5 were out of reading order

- PR number: 3008
- Source review ID: 6025241076
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668624084
- Concern: The lists read D1, D2, D4, D5, D3 and AC1, AC5, AC2, AC3, AC4.
- Solution: The lists read D1 to D5 and AC1 to AC5.
- Current-tree verification: `grep -oE '^- (D[0-9],|\[ \] AC[0-9])' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md | tr -d '[] ,-'` prints D1 D2 D3 D4 D5 AC1 AC2 AC3 AC4 AC5 in that order.
- Resolution reference: `docs(issues): [#3006] trace the D4 and D5 answers and keep decisions and ACs in order`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668835084

### F14 - F2 stands: the EPIC still has no row for #3006

- PR number: 3008
- Source review ID: 6025241076
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#pullrequestreview-6025241076
- Concern: The body of review 6025241076 says F2 stands because the EPIC at that head and on `develop` has no row, and the log defers it to a PR the reviewer could not read.
- Solution: No change in this PR, as for F2: the coordinator's PR #3007 adds the row and the order. Review 6025333875 accepts that route.
- Current-tree verification: `gh-bot pr view 3007 --repo torrust/torrust-tracker --json title,files` shows the EPIC and the #2770 spec among its files.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#issuecomment-7107505007
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/3007
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#issuecomment-7107505007

### F15 - "already fixes a schema version" reads like a bug fix

- PR number: 3008
- Source review ID: 6025551279
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668878114
- Concern: In the Background, "`agent-reply-validator` already fixes a schema version of 1" can read as correcting a bug; Copilot suggested "uses" or "sets".
- Solution: The sentence reads "already uses a schema version of 1".
- Current-tree verification: `grep -c 'already uses a schema version of 1' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c 'fixes a schema version' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 0.
- Resolution reference: `docs(issues): [#3006] say that agent-reply-validator uses a schema version`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669009621

### F16 - The PR description predated D4, D5 and the audit record

- PR number: 3008
- Source review ID: 6025811357
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669084154
- Concern: The description listed only D1 to D3, gave D3 its old meaning (marking only the #2768 change) and named only the spec among the files, and the merge tool copies the description into the merge commit.
- Solution: The description now lists D1 to D5, says D3's `1` marks both the #2768 `check` change and D4, names both files (the spec and this record), points to PR #3007 for the EPIC row, and names the audit validator under Validation. It was edited with `gh-bot pr edit`, so no commit carries it.
- Current-tree verification: `gh-bot pr view 3008 --repo torrust/torrust-tracker --json body -q .body | grep -c 'D5 (decided'` prints 1; the same with `PR-REVIEW.md` prints 1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669109330
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669109330

### F17 - The record cited PR-branch commit ids

- PR number: 3008
- Source review ID: 6025811357
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669084167
- Concern: The Findings paragraphs and the F11 concern cited three commits by ids that are not on `develop`, which the `open-pull-request` commit-id check forbids in review audits.
- Solution: Each id is replaced by the subject of the head's last commit, and the commit-id check from `open-pull-request` prints nothing for the files this PR changes.
- Current-tree verification: The `open-pull-request` commit-id check (`git diff -z --name-only --diff-filter=d torrust/develop...HEAD | xargs -0 -r grep -IohE ...`) prints nothing.
- Resolution reference: `docs(pr-reviews): [#3006] record review round 5 of PR #3008 and drop branch commit ids`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669121066

### F18 - The reply to Copilot's round-4 thread cited audit F15 before the record held it

- PR number: 3008
- Source review ID: 6025811357
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669084176
- Concern: Reply 4669009621 answered the round-4 thread as "FIXED (audit F15)", but the record at the reviewed head ended at F14.
- Solution: The F15 row and detail entry, with that reply URL, were added in `docs(pr-reviews): [#3006] record review round 4 of PR #3008`, pushed at 16:55 UTC, before this review was read.
- Current-tree verification: `grep -c '^| F15 |' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 1; `grep -c '^- Reply URL: .*r4669009621' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#3006] record review round 4 of PR #3008`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669109801

### F19 - The test loop did not state when the design review is recorded or when the maintainer reviews

- PR number: 3008
- Source review ID: 6025861610
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669125450
- Concern: The plan asked for a design review after each passing increment but did not require recording it before the next test-producing subtask, nor a maintainer review after the final increment, as `create-issue` requires.
- Solution: The commit-points section states the `write-unit-test` loop for T2: the smallest increment, the prose-first Arrange-Act-Assert design review recorded in the task evidence before the next test-producing increment, and a stop for maintainer review after the final increment.
- Current-tree verification: `grep -c 'Record the result in the task evidence before the next test-producing increment' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c 'stop for maintainer review before final verification' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] state the test-loop gates and the completion-review rule`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669252579

### F20 - The Implementation Completion Review had only a placeholder

- PR number: 3008
- Source review ID: 6025861610
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669125529
- Concern: The section did not say when to create the retrospective or how to close the review without one.
- Solution: The section says to compare the result with the spec, create `implementation-retrospective.md` from the template when there are material discoveries, and otherwise add a progress-log entry explaining why there were none.
- Current-tree verification: `grep -c 'Otherwise add a progress-log entry explaining why the work had no material discovery' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] state the test-loop gates and the completion-review rule`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669253063

### F21 - The audit ended at F14 while a reply cited audit F15

- PR number: 3008
- Source review ID: 6025861610
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669125616
- Concern: Same request as F18: add the F15 row and detail entry that reply discussion_r4669009621 cites.
- Solution: No separate change: F18's fix covers it.
- Current-tree verification: Same as F18.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669253697
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669253697

### F22 - The round-5 threads awaited rows, one reply and resolutions

- PR number: 3008
- Source review ID: 6025926288
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669179002
- Concern: At the reviewed head the record had no rows for the three round-5 threads, the F12 thread had no reply and none was resolved.
- Solution: The round-5 audit commit added F16 to F18 with their reply URLs; the F12 thread was answered and the three threads resolved.
- Current-tree verification: `grep -cE '^\| F1[678] \|' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 3.
- Resolution reference: `docs(pr-reviews): [#3006] record review round 5 of PR #3008 and drop branch commit ids`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669259608

### F23 - The red-first T2 tests conflicted with the loop's pass gate

- PR number: 3008
- Source review ID: 6026068748
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669288470
- Concern: T2 asked for tests that fail before T3, while the `write-unit-test` loop moves on only after an increment passes focused validation, so an implementer could not satisfy both.
- Solution: T2 and T3 run as red-green increments: each test runs red first (recorded in the task evidence, not committed), the T3 change in the same increment turns it green, and the gate and design review apply to the increment as a whole. The T2 row and the commit points agree.
- Current-tree verification: `grep -c 'red-green increment at a time' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c 'The loop.s gate applies to the increment as a whole' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c '^| T2 and T3 |' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] make T2 red-green increments agree with the test loop`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669613832

### F24 - Three round-6 threads had no rows, and a log entry gave a wrong reply time

- PR number: 3008
- Source review ID: 6026144266
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669343526
- Concern: The round-6 threads were answered and resolved but had no rows at the reviewed head, and the 17:03 log entry gave the first round-5 reply as 17:01Z, while it was 17:02:40Z.
- Solution: The rounds 6 and 7 audit commit added F19 to F22; an appended log entry corrects the reply time.
- Current-tree verification: `grep -cE '^\| F(19|2[012]) \|' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 4; `grep -c '^- 2026.* - Correction of the 17:03 entry' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#3006] record review rounds 8 and 9 of PR #3008`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669614458

### F25 - The CodeRabbit thread on the T2 loop awaited a reply, a row and a resolution

- PR number: 3008
- Source review ID: 6026144266
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669343558
- Concern: Same T2 change as F23, and a reply, row and resolution for that thread.
- Solution: No separate change: F23's fix and reply cover it.
- Current-tree verification: Same as F23.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669614985
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669614985

### F26 - The round-8 threads awaited replies, rows and resolutions

- PR number: 3008
- Source review ID: 6026332138
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669481576
- Concern: The two round-8 threads of da2ce7 and the CodeRabbit thread had no reply, row or resolution; the T2 gate still conflicted; the 17:01Z time still stood.
- Solution: The T2 gate is fixed (F23), the four threads have replies and are resolved, and this record holds F23 to F26 and the time correction.
- Current-tree verification: `grep -cE '^\| F2[3-6] \|' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 4.
- Resolution reference: `docs(pr-reviews): [#3006] record review rounds 8 and 9 of PR #3008`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669615728

### F27 - The T2 row and the loop's first step did not name the first-run-green exception

- PR number: 3008
- Source review ID: 6026068748
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669288470
- Concern: In its follow-up reply r4669629323 (review 6026521422) in the F23 thread (the source review and URL above are that thread's root, as the validator ties a row to its thread), after the F23 fix, the T2 row still said each test is red first and the loop's first step said the test must fail, while the second step lets a later `schema_version` test pass on its first run.
- Solution: The T2 row and the loop's first step name the exception: a later `schema_version` test that the shared writer already covers passes on its first run and records that green run.
- Current-tree verification: `grep -c 'except a later .schema_version. test that the shared writer already covers' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1; `grep -c 'The one exception is in the next step' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 1.
- Resolution reference: `docs(issues): [#3006] name the first-run-green exception in the T2 row and the loop`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669902384

### F28 - The T2 row and the loop's first step said every test fails first

- PR number: 3008
- Source review ID: 6026616249
- Reviewer finding ID: F24
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669702667
- Concern: Same request as F27.
- Solution: No separate change: F27's fix covers it.
- Current-tree verification: Same as F27.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669902930
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669902930

### F29 - The round-8 and round-9 rows, the promised commit and the reply-time correction were not at the reviewed head

- PR number: 3008
- Source review ID: 6026616249
- Reviewer finding ID: F25
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669702679
- Concern: At the reviewed head the record ended at F22, the replies cited audit F23 to F26 and a commit the branch did not hold, and the 17:01Z reply time stood uncorrected.
- Solution: The rounds 8 and 9 audit commit, pushed after the reviewed head, added F23 to F26 and the correction entry.
- Current-tree verification: `grep -cE '^\| F2[3-6] \|' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 4; `grep -c '^- 2026.* - Correction of the 17:03 entry' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#3006] record review rounds 8 and 9 of PR #3008`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669903429

### F30 - The round-9 note called review 6026332138 APPROVED while the forge shows it DISMISSED

- PR number: 3008
- Source review ID: 6026749477
- Reviewer finding ID: F27
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669807605
- Concern: The round-9 note and the 17:41 log entry said APPROVED, while the round-7 note reports the DISMISSED state of its review.
- Solution: The round-9 note says the review was submitted as APPROVED and is now shown as DISMISSED, and an appended log entry corrects the 17:41 entry the same way.
- Current-tree verification: `grep -c '^Round 9 .*submitted as APPROVED and now shown as DISMISSED' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 1; `grep -c '^- 2026.* - Correction of the 17:41 entry' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 1.
- Resolution reference: `docs(pr-reviews): [#3006] record review rounds 10 and 11 of PR #3008`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669903888

### F31 - The round-10 threads awaited replies, rows and resolutions

- PR number: 3008
- Source review ID: 6026749477
- Reviewer finding ID: F28
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669807614
- Concern: Review 6026616249's two threads had no reply, row or resolution, and the T2 exception was still unnamed.
- Solution: The T2 exception is fixed (F27); the two threads have replies and are resolved; this record holds F28 and F29.
- Current-tree verification: `grep -cE '^\| F(28|29) \|' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 2.
- Resolution reference: `docs(pr-reviews): [#3006] record review rounds 10 and 11 of PR #3008`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4669904297

### F32 - The spec's tables were reported to start rows with `||`

- PR number: 3008
- Source review ID: 6027138634
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4670109460
- Concern: Copilot reported that table rows in `docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` start with `||`, which would add an empty column, and suggested single leading pipes.
- Solution: No change: every table row in the file starts with a single `|`, at the reviewed head and at the current head.
- Current-tree verification: `grep -c '^||' docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md` prints 0, and `git show <head>:docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md | grep -c '^||'` prints 0 for the reviewed head and the current head; `linter markdown` passes.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4670117002
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4670117002

### F33 - The audit's findings table was reported to start rows with `||`

- PR number: 3008
- Source review ID: 6027138634
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4670109602
- Concern: Copilot reported that table rows in `docs/pr-reviews/pr-3008-review/PR-REVIEW.md` start with `||`, which would add an empty column, and suggested single leading pipes.
- Solution: No change: every table row in the file starts with a single `|`, at the reviewed head and at the current head.
- Current-tree verification: `grep -c '^||' docs/pr-reviews/pr-3008-review/PR-REVIEW.md` prints 0, and `git show <head>:docs/pr-reviews/pr-3008-review/PR-REVIEW.md | grep -c '^||'` prints 0 for the reviewed head and the current head; `linter markdown` passes.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4670117477
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4670117477

## Processing Log

- 2026-11-23 16:10 UTC - Started audit. Round 1: review 6024919812 (da2ce7, CHANGES_REQUESTED, submitted 16:03:08Z) with seven inline comments, F1 to F7. F1 needed a maintainer decision: he chose D4 (the `http scrape` entries under `files`) and D5 (the UDP field beside the tag). The fixes of F1 and F3 to F7 were committed and pushed as `josecelano-bot` (`docs(issues): [#3006] move http scrape entries under files and name all consumers`).
- 2026-11-23 16:19 UTC - Review 6024982355 (Copilot, COMMENTED, submitted 16:07:10Z) arrived with three inline comments, recorded as F8 to F10. The fix of F8 and F10 was committed and pushed (`docs(issues): [#3006] say that --format text is pretty-printed JSON`).
- 2026-11-23 16:26 UTC - Replied on the ten threads as `josecelano-bot` (reply `created_at` 16:25:58Z to 16:26:20Z), resolved the ten threads as `josecelano-bot` (each mutation returned `isResolved` true) and recorded the reply URLs.
- 2026-11-23 16:28 UTC - The audit validator, run with the bot token so its `gh` acts as `josecelano-bot`, exits 0 (10 rows, 0 failures). A GraphQL fetch shows 10 threads, 0 unresolved.
- 2026-11-23 16:40 UTC - Correction of the 16:28 entry: its "A GraphQL fetch shows 10 threads, 0 unresolved" was written before the fetch ran. The fetch that ran after it showed 13 threads, 3 unresolved: review 6025241076 (da2ce7, submitted 16:23:52Z) had added three threads before the replies of 16:26. The audit record was also created after those replies, not before them (late start).
- 2026-11-23 16:40 UTC - Rounds 2 and 3: review 6025241076 recorded as F11 to F14 and review 6025333875 (16:29:55Z) as context without a row. The fix of F12 and F13 was committed and pushed as `josecelano-bot` (`docs(issues): [#3006] trace the D4 and D5 answers and keep decisions and ACs in order`). Replied on the three threads (reply `created_at` 16:40:37Z to 16:40:42Z), answered F14 in a PR comment (16:40:45Z), and resolved the three threads (each mutation returned `isResolved` true).
- 2026-11-23 16:41 UTC - The audit validator, run with the bot token, exits 0 (14 rows, 0 failures). A GraphQL fetch that ran before this entry shows 13 threads, 0 unresolved.
- 2026-11-23 16:54 UTC - Round 4: review 6025551279 (Copilot, 16:44:16Z) recorded as F15. The fix was committed and pushed as `josecelano-bot` (`docs(issues): [#3006] say that agent-reply-validator uses a schema version`). Replied on the thread (16:54:30Z) and resolved it (the mutation returned `isResolved` true).
- 2026-11-23 16:55 UTC - The audit validator, run with the bot token, exits 0 (15 rows, 0 failures). A GraphQL fetch that ran before this entry shows 14 threads, 0 unresolved.
- 2026-11-23 17:03 UTC - Round 5: review 6025811357 (da2ce7, CHANGES_REQUESTED, 17:00:35Z) recorded as F16 to F18. F16: edited the PR description with `gh-bot pr edit`. F17: replaced the three branch commit ids in this record with commit subjects; the `open-pull-request` commit-id check prints nothing. F18 was already fixed by the round-4 audit commit. Replied on the three threads (17:01Z to 17:03:39Z) and resolved them (each mutation returned `isResolved` true). `linter all` exits 0.
- 2026-11-23 17:15 UTC - Rounds 6 and 7: review 6025861610 (Copilot, 17:04:02Z) recorded as F19 to F21 and review 6025926288 (da2ce7, 17:08:10Z) as F22. The fix of F19 and F20 and the round-5 audit commit were pushed as `josecelano-bot` (`git-bot-push` exit 0). Replied on the four threads (17:14:17Z to 17:14:49Z) and resolved them (each mutation returned `isResolved` true).
- 2026-11-23 17:16 UTC - The audit validator, run with the bot token, exits 0 (22 rows, 0 failures), and the `open-pull-request` commit-id check prints nothing. A GraphQL fetch that ran before this entry shows 21 threads, 0 unresolved.
- 2026-11-23 17:41 UTC - Correction of the 17:03 entry: it gives the first round-5 reply as 17:01Z; that reply (to the F16 thread) was created at 17:02:40Z.
- 2026-11-23 17:41 UTC - Rounds 8 and 9: reviews 6026068748 (CodeRabbit, 17:16:52Z), 6026144266 (da2ce7, 17:20:32Z) and 6026332138 (da2ce7, APPROVED, 17:31:06Z) recorded as F23 to F26. The fix of F23 was committed and pushed as `josecelano-bot` (`git-bot-push` exit 0). Replied on the four threads (17:41:15Z to 17:41:23Z) and resolved them (each mutation returned `isResolved` true).
- 2026-11-23 18:05 UTC - Correction of the 17:41 entry: review 6026332138 was submitted as APPROVED and is now shown as DISMISSED.
- 2026-11-23 18:05 UTC - Rounds 10 and 11: reviews 6026521422 (CodeRabbit, 17:42:25Z), 6026616249 (da2ce7, 17:48:17Z) and 6026749477 (da2ce7, APPROVED, 17:56:39Z) recorded as F27 to F31. The fix of F27 was committed and pushed as `josecelano-bot` (`git-bot-push` exit 0). Replied on the five threads (18:04:34Z to 18:04:44Z) and resolved the four open ones (each mutation returned `isResolved` true; the F27 reply is in the already resolved F23 thread).
- 2026-11-23 18:23 UTC - Round 12: review 6027138634 (Copilot, 18:22:27Z) recorded as F32 and F33, both no-change. Replied on the two threads (18:23:07Z and 18:23:09Z) and resolved them (each mutation returned `isResolved` true).

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
