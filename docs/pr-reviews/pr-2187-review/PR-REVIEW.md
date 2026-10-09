---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2187 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2187>.

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

Automated review 5155730377 raised two inline findings without reviewer finding IDs; they are recorded as F1 and F2. They were first audited in `docs/copilot-pr-reviews/pr-2187-copilot-suggestions.md` under the deprecated suggestions workflow and were migrated here when that file was removed (F3). They keep their source review, dispositions, thread states, and replies as recorded and acted on at the time, and the first eight Processing Log entries are carried from that file with its thread numbers mapped to F1 and F2 and its commit hashes replaced by the commit subject. The review body is an overview of the pull request with no request, so it has no row of its own.

Human review 5466878644 (josecelano, changes requested) numbered its findings F1-F8: F1 to F7 inline and F8 in the review body. Its F3 to F8 keep their IDs. Its F1 and F2 collide with the findings already recorded as F1 and F2, so they are recorded as F9 and F10, with the reviewer's IDs in their detail entries. The rest of the review body records the checks that produced no finding and the verdict, so it has no row of its own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2187-f1` | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | RESOLVED |
| F2 | `review-finding:pr-2187-f2` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2187-f9` | Human | Blocker | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2187-f10` | Human | Blocker | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2187-f3` | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2187-f4` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2187-f5` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2187-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2187-f7` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2187-f8` | Human | Nit | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The spec's tables were reported to start every row with a doubled pipe

- PR number: 2187
- Source review ID: 5155730377
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r3969598486>
- Concern: The tables in the #2183 spec were reported to start each row with `||`, which GitHub-flavored Markdown renders as an unintended empty first column. The reviewer gave no severity; Minor is inferred from the reported readability impact.
- Solution: No change; the premise did not hold. The file carried no doubled pipe, every table row began with a single pipe and a space, and GitHub's renderer produced no leading blank column for the flagged Computation table or the implementation-plan table. Migrated from the removed suggestions file with its original disposition and thread state.
- Current-tree verification: A search for `||` in `docs/issues/open/2183-adopt-calendar-msrv-policy/ISSUE.md` finds no match.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r3969769302>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r3969769302>

### F2 - The pin extraction command carried an escaped pipe

- PR number: 2187
- Source review ID: 5155730377
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r3969598582>
- Concern: The T3 row's pin extraction command carried `\|` before `head -1`, so copying it from the source into a shell would not pipe. The reviewer gave no severity; Minor is inferred.
- Solution: Moved the command out of the implementation-plan table into a fenced `bash` block under `### Job shape`, where the pipe needs no escape, and pointed T3 at it. The `&#124;` entity alternative was declined because a code span does not decode it. Migrated from the removed suggestions file with its original disposition and thread state.
- Current-tree verification: `### Job shape` carries `sed -n 's/^rust-version = "\(.*\)"/\1/p' Cargo.toml | head -1` in a fenced block, T3 reads "the extraction command given under Job shape", the spec contains no `\|`, and the command without `head -1` prints the single line `1.88` against the rebased `Cargo.toml`.
- Resolution reference: `docs(issues): make the MSRV pin extraction command copyable`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r3969767724>

### F9 - Links to the closed MSRV specs are broken against current develop

- PR number: 2187
- Source review ID: 5466878644
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r4227532659>
- Concern: The branch was cut before the closed specs moved to the folder-style layout, so on the merge result with `develop` the offline link check failed with four errors on the Background and References links, and the two `related-artifacts` entries named the old flat paths.
- Solution: Rebased onto `develop` and repointed the two `## Background` links, the two `## References` links, and both `related-artifacts` entries to `docs/issues/closed/1778-migrate-to-rust-edition-2024/ISSUE.md` and `docs/issues/closed/1787-evaluate-msrv-bump/ISSUE.md`.
- Current-tree verification: The spec no longer contains `1778-migrate-to-rust-edition-2024.md` or `1787-evaluate-msrv-bump.md`; every relative link target in the spec and every `related-artifacts` entry resolves to a tracked file.
- Resolution reference: `docs(issues): [#2183] repoint the closed MSRV spec links and migrate the frontmatter to v1`
- Follow-up PR URL: N/A
- Reply URL: {reply-2187-f1}

### F10 - The frontmatter fails the v1 frontmatter validator

- PR number: 2187
- Source review ID: 5466878644
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r4227532671>
- Concern: On the merge result, `frontmatter-validator` reported `legacy-shape`: draft and open issue specs must use the v1 frontmatter with `schema-version: 1`.
- Solution: Migrated the frontmatter to the shape of `docs/templates/ISSUE.md` following the validator's migration checklist: added `schema-version: 1`, replaced `status: open` with the lifecycle value `planned`, which fits an open issue whose implementation has not started, and quoted `last-updated-utc`. The stale `related-artifacts` paths were repaired under F9.
- Current-tree verification: The frontmatter has exactly the twelve fields the issue profile of `docs/schemas/frontmatter-v1.schema.json` requires, in the template's order; `spec-path` is the file's own path; `last-updated-utc` equals the newest progress-log entry.
- Resolution reference: `docs(issues): [#2183] repoint the closed MSRV spec links and migrate the frontmatter to v1`
- Follow-up PR URL: N/A
- Reply URL: {reply-2187-f2}

### F3 - The review audit uses the deprecated location and format

- PR number: 2187
- Source review ID: 5466878644
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r4227532682>
- Concern: `docs/copilot-pr-reviews/pr-2187-copilot-suggestions.md` followed the deprecated suggestions workflow instead of the single audit record that `process-pr-review` requires.
- Solution: Created this record from `docs/templates/PR-REVIEW-TEMPLATE.md`, migrated the two earlier findings as F1 and F2 with their source review, dispositions, thread states, and replies, recorded this review's findings, carried the old processing log, and removed the old file.
- Current-tree verification: `docs/copilot-pr-reviews/pr-2187-copilot-suggestions.md` is absent; `validate-audit-record.py --pr-number 2187` parses ten tracking rows with ten matching detail entries and a chronological Processing Log.
- Resolution reference: `docs(pr-reviews): [#2183] move the PR #2187 audit to the unified record`
- Follow-up PR URL: N/A
- Reply URL: {reply-2187-f3}

### F4 - Several implementation instructions are stale against current develop

- PR number: 2187
- Source review ID: 5466878644
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r4227532687>
- Concern: The spec said `actions/checkout@v7` was "already pinned elsewhere in this same file" while `testing.yaml` pins `actions/checkout@v7.0.1`; T4 named the flat PGO draft path, `Cargo.toml` line 67, and skill line 34; and Background counted "all 25 members".
- Solution: `### Job shape` and T3 now reuse "the exact `uses:` forms of the sibling jobs in `testing.yaml` at implementation time" instead of pins copied into the spec, and the sccache and rust-cache mentions drop their version suffixes for the same reason. T4 names its targets (the `AGENTS.md` **Language** bullet and its **MSRV policy** note, the skill's MSRV sentence, the PGO draft's T1 row) instead of line numbers and uses `docs/issues/drafts/1840-workflow-performance-pgo-optimization/ISSUE.md`. Background counts "the root package and all 30 workspace members".
- Current-tree verification: On `develop`, the workspace `members` list has 30 entries, the root and all 30 member manifests declare `rust-version.workspace = true`, and only the root `Cargo.toml` declares the literal `rust-version = "1.88"`; the PGO draft exists at its folder path; the spec contains no `@v` action pin and no line-number reference outside the dated 2026-09-09 progress-log entry.
- Resolution reference: `docs(issues): [#2183] re-verify the implementation values against current develop`
- Follow-up PR URL: N/A
- Reply URL: {reply-2187-f4}

### F5 - T4 misses the MSRV pointer in EPIC #1669

- PR number: 2187
- Source review ID: 5466878644
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r4227532694>
- Concern: `docs/issues/open/1669-overhaul-packages/EPIC.md` lists "MSRV changes (tracked under #1787)" as out of scope; this policy supersedes the follow-up #1787 deferred, so that pointer goes stale when it lands.
- Solution: T4 now repoints that out-of-scope item to #2183 and the new ADR, and the T4 commit point includes it.
- Current-tree verification: The `### Out of Scope` list of the EPIC still reads "MSRV changes (tracked under #1787)" on `develop`, and the T4 row of the spec's `## Implementation Plan` names that item.
- Resolution reference: `docs(issues): [#2183] re-verify the implementation values against current develop`
- Follow-up PR URL: N/A
- Reply URL: {reply-2187-f5}

### F6 - The AC5 sweep hits code comments that AC5 does not classify

- PR number: 2187
- Source review ID: 5466878644
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r4227532702>
- Concern: `git grep -n -i -e msrv -e rust-version` also matches the `derive_more::Constructor ... MSRV-compatible version` comments and `#[allow]` reasons in five packages, which fit none of the three categories AC5 allowed.
- Solution: AC5 and M5 name those comments and reasons as an expected category, M5 with the five packages and the count, together with the `MSRV` entry in `project-words.txt`, which the same sweep returns. `### Out of Scope` records upgrading `derive_more` and removing the allowances as a follow-up candidate.
- Current-tree verification: The sweep on `develop`, excluding `docs/issues/closed/`, returns 66 lines: 32 manifest lines, 26 `derive_more` comment and reason lines at 13 sites in `primitives`, `http-protocol`, `axum-http-server`, `axum-rest-api-server`, and `udp-server`, and 8 prose or dictionary lines, none of which states a floor other than `1.88`.
- Resolution reference: `docs(issues): [#2183] re-verify the implementation values against current develop`
- Follow-up PR URL: N/A
- Reply URL: {reply-2187-f6}

### F7 - The MSRV job should check with `--locked`

- PR number: 2187
- Source review ID: 5466878644
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#discussion_r4227532708>
- Concern: Without `--locked` the job could quietly re-resolve an out-of-date lockfile and check a dependency set other than the committed one.
- Solution: T3, M1, `### What the job runs, and why not more`, and the floor-toolchain check under `### Automatic Checks` now run `cargo check --locked --workspace --all-targets --all-features`.
- Current-tree verification: Every `cargo check` and `cargo +{pin} check` command in the spec carries `--locked`.
- Resolution reference: `docs(issues): [#2183] re-verify the implementation values against current develop`
- Follow-up PR URL: N/A
- Reply URL: {reply-2187-f7}

### F8 - The PR description is out of date

- PR number: 2187
- Source review ID: 5466878644
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2187#pullrequestreview-5466878644>
- Concern: The description said only `docs/issues/` was touched although the audit file was added later, and that the pre-commit gate has six steps where `develop` has nine.
- Solution: Replaced the description after the rebase: it names every file the branch touches, the rebase, this round's commits, and the validation at the new head with all nine pre-commit steps.
- Current-tree verification: `contrib/dev-tools/git/hooks/pre-commit.sh` on `develop` defines nine `STEPS` entries, and the branch's diff against `develop` adds the spec and this record only.
- Resolution reference: {reply-2187-f8}
- Follow-up PR URL: N/A
- Reply URL: {reply-2187-f8}

## Processing Log

- 2026-09-09 14:35 UTC - Started processing suggestions (two unresolved threads, both on `docs/issues/open/2183-adopt-calendar-msrv-policy/ISSUE.md`).
- 2026-09-09 14:37 UTC - Declined F1 after confirming the file carries no doubled pipe and that GitHub's own renderer produces no leading blank column for either flagged table.
- 2026-09-09 14:39 UTC - Accepted F2 and moved the pin extraction command out of the implementation-plan table into a fenced block under Job shape, in `docs(issues): make the MSRV pin extraction command copyable`.
- 2026-09-09 14:40 UTC - Gated the commit on the build server: `linter all` passes in 18.2 s, and the relocated command run verbatim against the branch's `Cargo.toml` prints the pin.
- 2026-09-09 14:46 UTC - Pushed that commit to the pull request branch as a fast-forward.
- 2026-09-09 14:46 UTC - Replied on F2 with the fix commit and its validation, and resolved it immediately after the reply.
- 2026-09-09 14:46 UTC - Replied on F1 with the evidence that the reported pattern is absent from the file, and resolved it immediately after the reply.
- 2026-09-09 14:47 UTC - Re-fetched the pull request's review threads and confirmed both are resolved and none remains open.
- 2026-10-09 10:23 UTC - Read josecelano review 5466878644 (changes requested), its seven inline threads, and its review body from the captured review data; recorded F9 (reviewer F1), F10 (reviewer F2), and F3 to F8.
- 2026-10-09 10:25 UTC - Rebased the branch onto `develop`; the five patches applied unchanged.
- 2026-10-09 10:29 UTC - Committed the F9 and F10 fixes.
- 2026-10-09 10:30 UTC - Committed the F4, F5, F6, and F7 fixes.
- 2026-10-09 10:33 UTC - Created this record from the template, migrated F1 and F2 from `docs/copilot-pr-reviews/pr-2187-copilot-suggestions.md`, carried its processing log, and removed that file (F3).
- 2026-10-09 10:49 UTC - The documentation gate failed on `cspell`: it does not know `repoint`, the word the reviewer's F5 uses and this record and the fix commit subject repeat. The texts stay as written, and the word's three forms go into the project dictionary.

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
