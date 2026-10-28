---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2681-2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2682 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2682>.

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

Three reviews of `copilot-pull-request-reviewer` were submitted so far. Review 5682548756 (COMMENTED, 2026-10-28 12:27 UTC, on the head whose subject is `docs(issues): [#2681] specify gating the tracker-core test modules and mocks`) carries four low-severity inline threads, F1 to F4, and no timeout warning. Review 5683278144 (COMMENTED, 13:24 UTC, on the head whose subject is `docs(pr-reviews): [#2681] record the review of PR 2682`) carries a "didn't start before the timeout" warning, so Copilot was re-requested once; it adds two inline threads, F5 and F6, both about table rows that start with `||`, which the files do not contain. The re-requested review 5683607616 (COMMENTED, 13:43 UTC, same head) adds one inline thread, F7, and no warning. Copilot was requested through the `request_copilot_review` MCP tool, which worked each time. da2ce7 was requested through the REST `requested_reviewers` endpoint (and again after the push) and has not reviewed. The maintainer asked on 2026-10-28 to address Copilot's review first.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2682-f1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2682-f2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2682-f3 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2682-f4 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2682-f5 | Copilot | Minor | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2682-f6 | Copilot | Minor | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2682-f7 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Move the feature-enabled build checkpoint from T3 to T4

- PR number: 2682
- Source review ID: 5682548756
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4400522242>
- Concern: the design-review checkpoint after "the first compiling slice (T3)" asks for a build with `--features testing`, but the `testing` feature is only added in T4, so the build fails with an unknown feature. The reviewer asks to move the checkpoint to T4.
- Solution: The finding was right: T3 only changes the three crate-private mocks to `cfg_attr(test, automock)`. The checkpoint now follows T4, and a sentence says T2 and T3 add no feature, so their checkpoint is the package's plain `cargo test` and clippy.
- Current-tree verification: read the Implementation Plan: T3 names no `[features]` change, T4 adds `testing = ["dep:mockall"]`; the spec's Design and Ownership Review now names T4.
- Resolution reference: docs(issues): [#2681] move the feature-enabled build checkpoint from T3 to T4
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4400567477>

### F2 - Count doc tests separately with matching baseline and post-change commands

- PR number: 2682
- Source review ID: 5682548756
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4400522306>
- Concern: AC3 requires separately counted doc tests, but `cargo test --all-targets --all-features` does not run doc tests, so the comparison with the baseline was not defined for them. The reviewer asks for matching commands, including `cargo test -p <package> --doc --all-features`, and aligned T1 and automatic checks.
- Solution: The finding was right (`--all-targets` excludes doc tests). T1, AC3 and the automatic checks now name four commands per package (plain, `--all-features`, `--all-targets --all-features`, `--doc --all-features`) and compare each count with the baseline recorded by the same command.
- Current-tree verification: the baseline run of this session used plain `cargo test -p`, whose log shows a `Doc-tests` section with its own count; no `--all-targets` log has one. The spec text now matches.
- Resolution reference: docs(issues): [#2681] compare doc tests with matching baseline and post-change commands
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4400567799>

### F3 - Verify mockall is absent from the consumer normal dependency tree

- PR number: 2682
- Source review ID: 5682548756
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4400522353>
- Concern: M3 expects the consumer's production build to have no `mockall`, but a successful `cargo build` does not prove it, especially after tests may have cached it. The reviewer asks for the consumer's normal dependency tree to be inspected.
- Solution: The finding was right. M3 now runs `cargo tree -p torrust-tracker-http-core -e normal -i mockall` (and the same for `udp-core`) before and after the change; `cargo tree` resolves the graph itself, so a cached build cannot hide the result.
- Current-tree verification: ran `cargo tree -p torrust-tracker-http-core -e normal -i mockall` on the current tree: it prints `mockall` reached through `torrust-tracker-core`, so the before state of the scenario is real.
- Resolution reference: docs(issues): [#2681] verify mockall is absent from the consumers' normal dependency graph
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4400568128>

### F4 - Correct M5 to inspect dependency edge kinds and optional mockall

- PR number: 2682
- Source review ID: 5682548756
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4400522400>
- Concern: M5 expects the coupling report to show `mockall` as `optional (testing)`, but the report lists only workspace dependencies, and the new consumer dev-dependencies add `dev` entries beside the existing `normal` ones. The reviewer asks to correct M5 to expect both kinds and to inspect Cargo metadata for the optional `mockall`.
- Solution: The finding was right. M5 and T5 now expect `torrust-tracker-core` under both `[normal]` and `[dev]` headings for `http-core` and `udp-core`, no self edge, and check the optional `mockall` in `cargo metadata` instead of the report.
- Current-tree verification: ran `workspace-coupling` into a scratch output file on the current tree: dependencies appear as `#### <package> [<kind>]` headings and only workspace packages are listed; read `workspace_dependencies` and `edge_kind` in the tool to confirm that a dev and a normal dependency on one package are separate entries.
- Resolution reference: docs(issues): [#2681] correct the coupling-report scenario to the edges it actually lists
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4400568411>

### F5 - Table rows that start with `||` in the issue spec

- PR number: 2682
- Source review ID: 5683278144
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4401079473>
- Concern: the table rows at the Pattern C table of `ISSUE.md` start with `||`, which renders an extra empty column; the reviewer asks for a single leading `|` in those rows and in the other tables of the document.
- Solution: no change. The claim does not match the file: no line of `ISSUE.md` starts with `||`, and every table row starts with a single `|`. The reviewer probably read the `+` that GitHub prefixes to each line of a new-file diff, followed by the row's own `|`.
- Current-tree verification: `grep -c '^||'` on `ISSUE.md` printed `0`, `grep -n '||'` found no occurrence, and `linter markdown` (markdownlint, which checks table pipe style) passed.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4401103911
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4401103911>

### F6 - Table rows that start with `||` in the audit record

- PR number: 2682
- Source review ID: 5683278144
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4401079526>
- Concern: the findings table of this audit record starts its header, separator and rows with `||`; the reviewer asks for a single leading `|`.
- Solution: no change, for the same reason as F5: the committed table starts every line with a single `|`.
- Current-tree verification: `grep -c '^||'` on `PR-REVIEW.md` printed `0`, and the committed text of the head under review (read with `git show`) shows single leading pipes; `linter markdown` passed.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4401104263
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4401104263>

### F7 - Record the actual -spec PR head branch

- PR number: 2682
- Source review ID: 5683607616
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4401301130>
- Concern: the frontmatter `branch:` of `ISSUE.md` records the implementation branch, but `create-issue` requires a spec-only PR's spec to record the `-spec` branch that the PR uses.
- Solution: the finding was right. Earlier rows recorded the base name, but the skill text governs. `branch:` now records the `-spec` branch; the implementation PR switches it to the base name.
- Current-tree verification: read `create-issue` Optional Step 6 (the frontmatter `branch:` value is the `-spec` branch name) and the PR head (`gh pr view 2682`); the field now equals the PR head branch.
- Resolution reference: docs(issues): [#2681] record the spec-only branch in the issue frontmatter
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2682#discussion_r4401328328>

## Processing Log

- 2026-10-28 12:33 UTC - The review of `copilot-pull-request-reviewer` (5682548756) arrived with four inline threads, F1 to F4. Each claim was checked against the spec and the tree and confirmed; each was fixed in its own signed commit, replied on, and resolved. This record is committed and pushed after the fixes; da2ce7 is re-requested after the push.
- 2026-10-28 13:45 UTC - Round 2: review 5683278144 arrived with a timeout warning and two inline threads, F5 and F6. The claim (rows starting with `||`) was checked against both files and found false; replied with the prescribed `Superseded by F5:` and `Superseded by F6:` forms and resolved both threads. Copilot was re-requested once because of the warning.
- 2026-10-28 13:45 UTC - Round 3: the re-requested review 5683607616 arrived with one inline thread, F7. The claim was confirmed against the `create-issue` skill, fixed in its own signed commit, replied on and resolved. This record is committed and pushed after the fix; da2ce7 is re-requested after the push.

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
