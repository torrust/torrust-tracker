---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - review-finding:pr-2232-f1
    - review-finding:pr-2232-f2
    - review-finding:pr-2232-f3
    - review-finding:pr-2232-f4
    - review-finding:pr-2232-f5
    - review-finding:pr-2232-f6
    - review-finding:pr-2232-f7
    - review-finding:pr-2232-f13
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:disable -->

# PR #2232 Review Audit

Source: pull-request reviews and inline review threads for
https://github.com/torrust/torrust-tracker/pull/2232.

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2232-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2232-f2 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2232-f3 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2232-f4 | Human | Blocker | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F5 | review-finding:pr-2232-f5 | Human | Blocker | link-integrity | ORIGINAL | FIXED | NON_RESOLVABLE |
| F6 | review-finding:pr-2232-f6 | Human | Blocker | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F7 | review-finding:pr-2232-f7 | Human | Major | testing | ORIGINAL | FIXED | NON_RESOLVABLE |
| F8 | review-finding:pr-2232-f8 | Human | Suggestion | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F9 | review-finding:pr-2232-f9 | Human | Suggestion | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F10 | review-finding:pr-2232-f10 | Human | Nit | formatting | ORIGINAL | FIXED | NON_RESOLVABLE |
| F11 | review-finding:pr-2232-f11 | Human | Nit | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F12 | review-finding:pr-2232-f12 | Human | Suggestion (inferred) | documentation | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F13 | review-finding:pr-2232-f13 | Human | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F14 | review-finding:pr-2232-f14 | Human | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F15 | review-finding:pr-2232-f15 | Human | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F16 | review-finding:pr-2232-f16 | Human | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F17 | review-finding:pr-2232-f17 | Human | Nit | formatting | ORIGINAL | FIXED | NON_RESOLVABLE |
| F18 | review-finding:pr-2232-f18 | Human | Nit | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - PR-template issue metadata could render in a PR body

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNsfloA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4018776876
- Concern: `.github/PULL_REQUEST_TEMPLATE/review-findings.md` used issue-template YAML frontmatter,
  which GitHub renders verbatim inside a pull-request body.
- Solution: removed the issue-form metadata and kept the advisory guidance as plain Markdown.
- Current-tree verification: `linter markdown` and the pre-commit gate passed after removal.
- Resolution reference: fix(pr-reviews): remove PR template metadata
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4019117053

### F2 - Nightly formatting ran after aggregate linting

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNsfloA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4018776945
- Concern: the pre-commit hook ran `linter all` before the nightly rustfmt check, so a common
  formatting failure surfaced only after the slower aggregate step.
- Solution: moved the nightly formatting step before aggregate linting so it fails fast; no check
  was removed.
- Current-tree verification: the pre-commit gate reports nightly formatting as step 5 and
  aggregate linting as step 6; all steps passed.
- Resolution reference: ci(hooks): run nightly formatting earlier
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4019117216

### F3 - Flow-style semantic-link YAML was less robust than a block list

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNsfloA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4018776990
- Concern: the canonical skill declared `semantic-links.related-artifacts` as a multi-line
  flow-style YAML sequence with a trailing comma, which is easier to break and less portable.
- Solution: converted the sequence to the repository's block-list style with identical values.
- Current-tree verification: YAML and Markdown linting, the focused contract test, and the
  pre-commit gate passed.
- Resolution reference: docs(pr-reviews): use portable semantic link YAML
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#discussion_r4019117462

### F4 - Historical audits referenced never-existing paths and restated history

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: the migration rewrote self-referential Path cells in 9 historical audits to
  `docs/pr-reviews/pr-<N>-copilot-suggestions.md` (21 occurrences), a path that never existed,
  and restated a historical narrative cell to name a marker that did not exist at the time,
  violating the workflow's own no-backfill rule.
- Solution: restored all nine files verbatim from the merge base and re-applied only the three
  intentional migration metadata lines (frontmatter skill-link, related artifact, marker).
- Current-tree verification: `git grep -c -E 'docs/pr-reviews/pr-[0-9]+-copilot-suggestions\.md'`
  returns no matches; each file's diff against its merge-base original is exactly the three
  metadata line pairs.
- Resolution reference: fix(pr-reviews): restore historical audit source paths
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F5 - docs/AGENTS.md routed agents to deleted directories

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: the agent-facing directory map still listed deleted `copilot-pr-reviews/` and
  `pr-review-feedback/` and omitted the replacement `pr-reviews/`.
- Solution: replaced both stale rows with one `pr-reviews/` row describing the unified archive.
- Current-tree verification: `grep` over docs/AGENTS.md matches only `pr-reviews/`.
- Resolution reference: docs(agents): fix review archive directory map
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F6 - Batch-resolve reply guard was dropped

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: the deleted legacy skills carried the only documented precondition that
  `check-thread-reply-status.sh` must exit 0 before batch resolution, while the batch resolver
  itself remained documented, weakening the reply-before-resolve rule.
- Solution: restored the guard in the `resolve-review-threads` Batch Pattern section, added the
  script to its related artifacts, and added a completion-checklist item.
- Current-tree verification: `git grep check-thread-reply-status` now matches the
  resolve-review-threads skill in addition to the script itself.
- Resolution reference: docs(pr-reviews): restore batch-resolve reply guard
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F7 - Contract test took an undeclared PyYAML dependency

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: `require_yaml_related_artifact` shelled out to `python3` with `import yaml`, failing on
  hosts without PyYAML and misreporting the missing module as a content violation.
- Solution: replaced the helper with a dependency-free awk parse of the block-style
  `related-artifacts` list, consistent with the sibling helpers.
- Current-tree verification: the contract test passes; `grep python3` in the test returns nothing.
- Resolution reference: test(checks): drop PyYAML dependency from contract test
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F8 - Reviewer guidance lived in an author-facing directory

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: `.github/PULL_REQUEST_TEMPLATE/review-findings.md` contained reviewer instructions but
  everything in that directory is offered to PR authors at creation time.
- Solution: moved the guidance to `docs/templates/REVIEW-FINDINGS.md` and repointed the skill
  link, frontmatter artifact, and contract-test paths.
- Current-tree verification: the contract test and Lychee pass; `.github/PULL_REQUEST_TEMPLATE/`
  no longer exists.
- Resolution reference: docs(templates): relocate reviewer finding guidance
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F9 - related-artifacts type was stale in the convention table

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: the convention table typed `related-artifacts` as `<repo-relative-path>` while prose
  and this audit already use `review-finding:` values.
- Solution: widened the table row to `<repo-relative-path>` or `review-finding:pr-<number>-<id>`.
- Current-tree verification: the marker-catalog row and the value-type row now agree; linters pass.
- Resolution reference: docs(skills): widen related-artifacts value type
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F10 - Templates README table order was broken

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: the consolidated `PR-REVIEW-TEMPLATE.md` row inherited the alphabetical slot of the
  removed row instead of its own.
- Solution: reordered the table alphabetically while adding the relocated `REVIEW-FINDINGS.md` row.
- Current-tree verification: this PR's template rows sit in their alphabetical slots (a
  pre-existing inversion elsewhere in the table is untouched); Markdown linting passes.
- Resolution reference: docs(templates): relocate reviewer finding guidance
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F11 - ISSUE.md timestamp lagged its own progress log

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: `last-updated-utc` said 17:50 UTC while the progress log recorded 19:05 and 19:10 UTC.
- Solution: refreshed `last-updated-utc` alongside this round's bookkeeping.
- Current-tree verification: frontmatter timestamp is now at or after the newest log entry.
- Resolution reference: docs(pr-reviews): record human review round
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F12 - Reviewer-side skill vocabulary diverges from the unified contract

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNtWO8Q
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5214932721
- Concern: this PR changes the advisory finding format and severity vocabulary without touching
  `review-pr/SKILL.md`, so reviewer-side and author-side skills describe findings in different
  terms; round-scoping for re-pushed heads (F48) and a checklist N/A convention (F49) also remain
  open on the reviewer side.
- Solution: deferred; aligning the reviewer-side skill is follow-up work outside this PR's
  author-side scope, tracked in issue #2233 together with the other first-use improvement
  candidates.
- Current-tree verification: `git diff` over `review-pr/SKILL.md` in this PR is empty, as reported.
- Resolution reference: https://github.com/torrust/torrust-tracker/issues/2233
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5687518457

### F13 - Relocated template missed the canonical-template contract (N1)

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNt9h6A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5215576552
- Concern: `docs/templates/REVIEW-FINDINGS.md` lacked frontmatter and was absent from the
  `docs/index.md` template catalog, both required by the `create-markdown-template` contract.
- Solution: added semantic-links frontmatter and the index catalog row.
- Current-tree verification: the file starts with closed YAML frontmatter and
  `grep REVIEW-FINDINGS docs/index.md` matches the catalog row; linters pass.
- Resolution reference: docs(templates): complete reviewer guidance relocation
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5693446035

### F14 - Issue spec still placed the guidance under .github (N2)

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNt9h6A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5215576552
- Concern: the #2219 spec's present-tense design statement and AC5 criterion/evidence cited the
  removed `.github/PULL_REQUEST_TEMPLATE/` path; the completed T5 row is valid history.
- Solution: updated the design statement, AC5 wording, and AC5 evidence to the relocated path,
  citing `review-finding:pr-2232-f8`; left the T5 historical record unchanged.
- Current-tree verification: the spec's only remaining `.github/PULL_REQUEST_TEMPLATE` reference
  is the completed T5 row.
- Resolution reference: docs(issues): update spec to relocated guidance
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5693446035

### F15 - Recorded F4 verification command did not discriminate (N3)

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNt9h6A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5215576552
- Concern: the audit recorded `git grep -c` without `-E`, a basic-regex form that also returns
  nothing at the broken head, so it was not evidence.
- Solution: recorded the `-E` form actually needed to discriminate.
- Current-tree verification: the `-E` command matches 21 occurrences at the pre-fix head `54ae5faa`
  and nothing at this head.
- Resolution reference: docs(pr-reviews): record approval round
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5693446035

### F16 - F10 verification claim was broader than the tree (N4)

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNt9h6A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5215576552
- Concern: the audit claimed the templates README table reads alphabetically while a pre-existing
  inversion this PR never touched remains.
- Solution: narrowed the claim to the rows this PR placed.
- Current-tree verification: audit F10 entry now states the pre-existing inversion is untouched.
- Resolution reference: docs(pr-reviews): record approval round
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5693446035

### F17 - Two docs/AGENTS.md rows lost column alignment (N5)

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNt9h6A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5215576552
- Concern: the `refactor-plans/` and `pr-reviews/` rows were 200 characters against the table's
  uniform 199.
- Solution: removed the extra padding space from both rows.
- Current-tree verification: every directory-map row measures 199 characters.
- Resolution reference: docs(agents): restore directory map alignment
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5693446035

### F18 - Processing-log entry dated after the commit that wrote it (N6)

- PR number: 2232
- Source review ID: PRR_kwDOGp2yqc8AAAABNt9h6A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2232#pullrequestreview-5215576552
- Concern: the entry stamped 20:40 UTC was authored at 20:28:55Z and records an action from
  20:22:42Z, so the permanent record carried a future timestamp.
- Solution: corrected the entry to 20:28 UTC, matching its commit author time.
- Current-tree verification: the corrected stamp is not later than the commit that introduced it.
- Resolution reference: docs(pr-reviews): record approval round
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2232#issuecomment-5693446035

## Processing Log

- 2026-09-15 18:47 UTC - Collected the authoritative GraphQL thread snapshot and the submitted
  Copilot review `PRR_kwDOGp2yqc8AAAABNsfloA`; normalized its three independently actionable
  inline findings as F1-F3 in source order.
- 2026-09-15 18:50 UTC - Applied, focused-validated, and separately GPG-signed fixes for F1-F3.
- 2026-09-15 18:51 UTC - Replied to and resolved all three Copilot threads. A refreshed GraphQL
  snapshot reported no unresolved threads. Future asynchronous reviews append new source rows to
  this audit without changing these immutable finding references.
- 2026-09-15 19:05 UTC - First real use showed the single 15-column findings table was hard to
  read; split the audit into a compact tracking table plus a Finding Details section, updating the
  canonical template, workflow, and contract test in the same change. All required fields remain
  recorded.
- 2026-09-15 19:10 UTC - Deferred automation candidate: use a high-capability model to assess
  findings against the current tree, decide their disposition, and specify a bounded solution;
  route that solution to a lower-cost implementation model for validation and evidence capture;
  then independently verify the result before resolving the finding. Defer custom-agent design
  until this manual workflow has completed its first asynchronous human-review round.
- 2026-09-15 19:36 UTC - Cameron (da2ce7) submitted changes-requested review
  `PRR_kwDOGp2yqc8AAAABNtWO8Q` with no inline threads; normalized its nine independently
  actionable assertions as F4-F12 in source order. He independently verified F1-F3 as genuinely
  fixed and this audit's identifiers as accurate.
- 2026-09-15 20:13 UTC - Fixed F4-F11 in separate signed commits: restored the nine historical
  audits verbatim plus intentional metadata, fixed the docs/AGENTS.md directory map, restored the
  batch-resolve reply guard, replaced the PyYAML helper with awk, relocated the reviewer guidance
  to docs/templates with alphabetical README ordering, widened the related-artifacts value type,
  and refreshed the ISSUE.md timestamp. F12 is recorded as FOLLOW_UP: align `review-pr/SKILL.md`
  vocabulary, round scoping for re-pushed heads, and the checklist N/A convention in a dedicated
  task. The review body is non-resolvable, so the disposition reply is the consolidated PR
  response whose URL is stored in each detail entry.
- 2026-09-15 20:28 UTC - Documented the first-use lessons in the #2219 implementation
  retrospective and opened follow-up issue #2233 covering F12, the code-span path guardrail
  (F4/F5), the retirement obligation inventory (F6), the rename-purity verification step (F4),
  and the tiered model-routing design. Re-requested Cameron's review on the remediated head;
  awaiting the next round.
- 2026-09-15 20:47 UTC - Cameron (da2ce7) approved in review `PRR_kwDOGp2yqc8AAAABNt9h6A`,
  recomputing every round-1 disposition from the bytes: F4-F11 verified FIXED, the F12 deferral
  accepted because issue #2233 tracks it. The approval body carried six new non-blocking
  assertions, normalized as F13-F18 in source order.
- 2026-09-16 06:53 UTC - Fixed F13-F18: completed the canonical-template contract for the
  relocated guidance, updated the #2219 spec statements and AC5 to the relocated path, corrected
  the two non-discriminating verification claims (F15, F16), restored the directory-map
  alignment, and corrected the early log timestamp. Cameron's condition on the F12 deferral -
  a real tracked task - was already satisfied by issue #2233.
