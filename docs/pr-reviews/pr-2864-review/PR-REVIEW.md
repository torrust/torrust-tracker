---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# PR #2864 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2864.

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

Round 1 is da2ce7's review `5910653714` (changes requested) at the head whose last commit is
`docs(issues): [#2824] record the S4 pull request number`. Its five inline findings keep the
reviewer's IDs F1 to F5; the review body only summarizes them.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2864-f1 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2864-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2864-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2864-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2864-f5 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - AC9 is checked, but F5 has no draft spec and no issue

- PR number: 2864
- Source review ID: 5910653714
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578333763
- Concern: AC9 requires a draft spec, a log entry and a `lessons.md` row for every finding that needs
  a production change. F5 (`peer::Id::new` and `peer::fixture` in a production crate) had none of a
  draft spec or an issue, yet AC9 was checked, and this PR closes #2824.
- Solution: The maintainer chose a draft spec written by a separate session on its own branch from
  `develop`, as the findings procedure of #2824 asks: PR #2865 adds
  `docs/issues/drafts/peer-id-test-helper/ISSUE.md`. The F5 register row, the AC9 evidence row, the
  `peer::fixture` risk, the F5 `lessons.md` row and the retrospective cite it; a progress-log entry
  (2026-11-15 12:47) records it. It has no GitHub issue yet: its open design questions go to the
  maintainer first. AC9 stays checked.
- Current-tree verification: `gh-bot pr view 2865` shows it open, by `josecelano-bot`, created
  2026-11-15T12:47:15Z, changing only `docs/issues/drafts/peer-id-test-helper/ISSUE.md`;
  `grep -n 'fill in its number\|to be filled in\|no GitHub issue exists'` over the issue folder prints
  nothing.
- Resolution reference: docs(issues): [#2824] cite the F5 draft spec pull request for AC9
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578461775

### F2 - T5 is DONE, but the Findings Register was not reconciled

- PR number: 2864
- Source review ID: 5910653714
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578333768
- Concern: The F5 register row kept the "fill in its number" placeholder, and the F4 row still said
  the `AddAssign` impl "is being deleted", although its removal is merged.
- Solution: The F4 row now says PR #2827 deleted the impl on 2026-11-11 (`gh-bot pr list --search
  2823 --state merged` lists #2827, merged 2026-11-11T13:43:48Z). The F5 placeholder is replaced
  in the F1 fix (`docs(issues): [#2824] cite the F5 draft spec pull request for AC9`).
- Current-tree verification: `grep -n 'is being deleted'` over the issue folder prints nothing; the F4
  row reads "was deleted by PR #2827 (merged 2026-11-11)".
- Resolution reference: docs(issues): [#2824] record the merged removal of the AddAssign impl in the F4 row
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578461942

### F3 - The cited audit records five review rounds, not six

- PR number: 2864
- Source review ID: 5910653714
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578333773
- Concern: `docs/pr-reviews/pr-2852-review/PR-REVIEW.md` names rounds 1 to 5, but the retrospective
  (twice) and the progress-log entry of 2026-11-14 12:09 say six.
- Solution: The retrospective says five in both places. The progress-log entry is not rewritten; an
  appended entry (2026-11-15 12:35) corrects it.
- Current-tree verification: `grep -rn 'six review'` over the issue folder prints only the
  2026-11-14 12:09 progress-log entry, which the appended entry corrects; `PR-REVIEW.md` of #2852
  names rounds 1 to 5, and its reviews list shows no later da2ce7 or Copilot review.
- Resolution reference: docs(issues): [#2824] count five review rounds of the S3 pull request
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578462094

### F4 - The description miscounts the test targets

- PR number: 2864
- Source review ID: 5910653714
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578333776
- Concern: The description said "the other two test targets pass, 2 + 1"; the package has no
  `tests/` directory, and the 2 + 1 are the three doctests of `peer.rs` reported in two groups.
- Solution: The description's Validation line now says "109 unit tests and the 3 doctests, 2 + 1".
  The plan records count `109 + 2 + 1` per cargo group, which is correct, so they are unchanged.
- Current-tree verification: `gh-bot pr view 2864 --json body` shows the new line and no
  "test targets".
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2864#issuecomment-6894593185
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578462209

### F5 - The new tests omit the AAA section comments

- PR number: 2864
- Source review ID: 5910653714
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578333781
- Concern: The four `SwarmMetadata` tests separated their sections with blank lines only, while the
  `write-unit-test` checklist asks for `// Arrange`, `// Act` and `// Assert` comments.
- Solution: Added the comments (the `zeroed` test has no arrange step, so it has `// Act` and
  `// Assert`); no assertion changed. `swarm_metadata.md` records the change in a separate commit,
  `docs(issues): [#2824] record the AAA comments added to the swarm metadata tests`.
- Current-tree verification: `cargo test -p torrust-tracker-primitives --lib swarm_metadata`: 4 passed;
  `cargo +nightly fmt --all -- --check` passed; the pre-commit gate passed at the commit.
- Resolution reference: test(primitives): [#2824] add the AAA section comments to the swarm metadata tests
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2864#discussion_r4578462329

## Processing Log

- 2026-11-15 12:37 UTC - Started this audit for da2ce7's review `5910653714` (submitted
  2026-11-15T12:19:00Z), before any reply. F2, F3 and F5 are committed locally; F1 waits on the F5
  draft spec the maintainer asked for (a separate session and PR); F4 is a description edit.
- 2026-11-15 12:41 UTC - Copilot's review `5910671180` (submitted 2026-11-15T12:22:14Z, same head)
  recommends approval with no findings and no inline comments; no row.
- 2026-11-15 12:52 UTC - Round 1 processed. The fixes were pushed by `git-bot-push` (head
  `docs(issues): [#2824] cite the F5 draft spec pull request for AC9`); the description was edited
  and comment https://github.com/torrust/torrust-tracker/pull/2864#issuecomment-6894593185 states
  the F4 and F1 description changes; each of the five threads has a reply and was resolved
  (`isResolved: true`). `validate-audit-record.py --pr-number 2864` returned `status: ok`, 5 rows,
  0 failures, before the resolution.

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
