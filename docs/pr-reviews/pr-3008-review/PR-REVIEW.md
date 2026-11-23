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

Round 1 covers review 6024919812 (da2ce7, CHANGES_REQUESTED, submitted 2026-11-23T16:03:08Z, of commit `076c87b02`) with seven inline comments, each with a `[Severity][FindingId]` first line; the IDs F1 to F7 are kept and the severities are the brackets. Its body summarizes the same seven findings and the checks the reviewer ran; it makes no row.

Round 1 also covers review 6024982355 (Copilot, COMMENTED, submitted 2026-11-23T16:07:10Z) with three inline comments without IDs or severity brackets, recorded as F8 to F10. F9 has the same text as F8 on another line and is a re-raise with its own row.

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

## Processing Log

- 2026-11-23 16:10 UTC - Started audit. Round 1: review 6024919812 (da2ce7, CHANGES_REQUESTED, submitted 16:03:08Z) with seven inline comments, F1 to F7. F1 needed a maintainer decision: he chose D4 (the `http scrape` entries under `files`) and D5 (the UDP field beside the tag). The fixes of F1 and F3 to F7 were committed and pushed as `josecelano-bot` (`docs(issues): [#3006] move http scrape entries under files and name all consumers`).
- 2026-11-23 16:19 UTC - Review 6024982355 (Copilot, COMMENTED, submitted 16:07:10Z) arrived with three inline comments, recorded as F8 to F10. The fix of F8 and F10 was committed and pushed (`docs(issues): [#3006] say that --format text is pretty-printed JSON`).
- 2026-11-23 16:26 UTC - Replied on the ten threads as `josecelano-bot` (reply `created_at` 16:25:58Z to 16:26:20Z), resolved the ten threads as `josecelano-bot` (each mutation returned `isResolved` true) and recorded the reply URLs.
- 2026-11-23 16:28 UTC - The audit validator, run with the bot token so its `gh` acts as `josecelano-bot`, exits 0 (10 rows, 0 failures). A GraphQL fetch shows 10 threads, 0 unresolved.

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
