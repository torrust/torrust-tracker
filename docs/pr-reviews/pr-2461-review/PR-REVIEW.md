---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2458-inject-udp-cookie-cipher/ISSUE.md
    - docs/issues/open/2458-inject-udp-cookie-cipher/manual-verification-evidence.md
---

<!-- skill-link: process-pr-review -->

# PR #2461 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2461>,
and for the closed PR it replaced, <https://github.com/torrust/torrust-tracker/pull/2460>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

PR #2460 was closed without merging at 2026-10-06 16:26 UTC. Its source branch had been renamed to
add the required `-spec` suffix, and GitHub kept the PR on the old branch name, so PR #2461 replaced
it with the same commits. Copilot review 5431658413 arrived on the closed PR at 16:36 UTC. Its
findings apply to the same files in PR #2461. Following the process-pr-review skill's
closed-and-superseded section, they are fixed and audited here, with `PR number` 2460 and
`review-finding:pr-2460-...` references.

Review 5431658413 supplied the finding IDs `F8` and `F1` with `[Minor]` and `[Major]` severities;
both IDs are kept. Its body summarizes those two threads and adds no other assertion. Copilot review
5431782940 on PR #2461 (16:48 UTC) supplied no finding IDs, so its four comments take `F2` to `F5`
in source order, skipping the IDs already used. Their severities are inferred from its overview
badges: Medium as Minor, and Low as Suggestion for the "Consider quoting" comment. No finding
re-raises another.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F8 | `review-finding:pr-2460-f8` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2460-f1` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2461-f2` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2461-f3` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2461-f4` | Copilot | Minor (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2461-f5` | Copilot | Suggestion (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F8 - The spec misstated when `check_seed()` was added

- PR number: 2460
- Source review ID: 5431658413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2460#discussion_r4197957445>
- Concern: The spec said commit `e3562f069` added the cipher and "kept" an earlier seed check that
  had "made sense" before. In fact that commit added the cipher and `check_seed()` together, so the
  defect is that the new safeguard checked the old seed instead of the new cipher.
- Solution: Rewrote Background item 3 and the References entry. Before `e3562f069`, cookies hashed
  the seed and nothing checked it at startup. That commit replaced the hash with the cipher and, in
  the same patch, added `check_seed()`, which checks the now-unused seed. The other lines the
  reviewer cited only mention `check_seed()` and make no history claim, so they are unchanged.
- Current-tree verification: `git show e3562f069` adds `fn check_seed()` and
  `RANDOM_CIPHER_BLOWFISH`; `git log -S check_seed` finds no earlier commit;
  `git show e3562f069^:src/servers/udp/connection_cookie.rs` shows the seed-hash cookie. The spec
  no longer contains "made sense" or "kept the".
- Resolution reference: `docs(issues): correct #2458 cookie-cipher history`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2460#discussion_r4198706123>

### F1 - The disposable reproducer source was not preserved

- PR number: 2460
- Source review ID: 5431658413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2460#discussion_r4197957531>
- Concern: Evidence V1 relied on the removed `forge_zeroed_cookie` example but kept only its
  command and output. The fix-bug skill requires temporary reproduction code to be recorded
  verbatim, otherwise reviewers cannot audit or repeat the forgery.
- Solution: Added a "Temporary Code (Verbatim)" section to V1 with the alias mutation diff and the
  full example. Recreated that exact code and ran it again. Added a control run with the alias
  restored to show that the acceptance comes from the mutation. Reverted both temporary changes.
- Current-tree verification: The 2026-10-06 17:06 UTC rerun printed
  `forged cookie accepted for issue time 1728000000`. The control panicked with `ValueExpired`.
  `git status --short` was empty afterwards. Pre-commit `linter all`, including the lychee fragment
  check of `#temporary-code-verbatim`, passed.
- Resolution reference: `docs(issues): record #2458 reproducer source verbatim`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2460#discussion_r4198706415>

### F2 - Scenario M2 was marked done after only its pre-fix half

- PR number: 2461
- Source review ID: 5431782940
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198063765>
- Concern: M2's expected result covered both the pre-fix reproduction and the post-fix recheck,
  but its status was already `DONE (local reproduction)`.
- Solution: Split M2 into M2a (the pre-fix reproduction, `DONE`, described as actually performed)
  and M2b (the post-fix recheck, `TODO`). AC5 cites both, and the V1 evidence heading names M2a.
- Current-tree verification: `ISSUE.md` scenario rows M2a and M2b exist; AC5 reads
  `M2a (before), M2b (after)`; the evidence heading is `V1 - M2a: ...`.
- Resolution reference: `docs(issues): split #2458 forged-cookie scenario into before and after`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198706733>

### F3 - Scenario evidence links pointed at wrong or missing sections

- PR number: 2461
- Source review ID: 5431782940
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198063825>
- Concern: M1 pointed at V1, which records the forgery, and M3 pointed at V3, which does not exist.
- Solution: The pending scenarios M1, M2b and M3 now say "Not yet recorded". Only M2a links to the
  existing V1.
- Current-tree verification: In the manual-verification table, only the M2a row references
  `manual-verification-evidence.md`.
- Resolution reference: `docs(issues): split #2458 forged-cookie scenario into before and after`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198707015>

### F4 - The evidence file had a duplicate `Failures and Follow-up` section

- PR number: 2461
- Source review ID: 5431782940
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198063884>
- Concern: A second, template-text copy of `## Failures and Follow-up` followed the real one.
- Solution: Removed the leftover template copy.
- Current-tree verification: `grep -c '^## Failures and Follow-up'` on the evidence file returns 1.
- Resolution reference: `docs(issues): tidy #2458 verification evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198707234>

### F5 - `last-updated-utc` was unquoted in the evidence file

- PR number: 2461
- Source review ID: 5431782940
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198063931>
- Concern: Quote the value to avoid YAML timestamp coercion and to match `ISSUE.md`.
- Solution: Quoted it. An unquoted value without seconds is not a YAML timestamp, so there was no
  coercion bug, but quoting keeps the issue folder consistent.
- Current-tree verification: The evidence frontmatter reads `last-updated-utc: "2026-10-06 17:06"`.
- Resolution reference: `docs(issues): tidy #2458 verification evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198707504>

## Processing Log

- 2026-10-06 16:54 UTC - Started audit at the maintainer's request. Fetched all threads of PR #2460
  (two, unresolved) and PR #2461 (four, unresolved) with `github-review-threads`, and both Copilot
  reviews. Confirmed that review 5431658413 was submitted after PR #2460 closed. After read-only
  triage, the maintainer approved handling the PR #2460 findings on PR #2461 and documenting this
  case in the process-pr-review skill.
- 2026-10-06 17:05 UTC - Committed the F8 fix.
- 2026-10-06 17:06 UTC - Recreated and reran the reproducer for F1, with a control run.
- 2026-10-06 17:09 UTC - Committed the F1, F2/F3, and F4/F5 fixes.
- 2026-10-06 17:51 UTC - Committed the process-pr-review skill update.
- 2026-10-06 17:52 UTC - Rebased onto the latest `develop` (two new commits) and pushed after the
  pre-push checks passed.
- 2026-10-06 17:54 UTC - Replied to all six threads; recorded reply URLs.

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
