---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2517"
---

<!-- skill-link: process-pr-review -->

# PR #2520 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2520>.

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

Copilot review 5514549824 (submitted 2026-10-14 07:37:11 UTC, before merge) left two inline
findings headed `[Major][F1]` and `[Minor][F2]`. The reviewer-provided IDs do not collide with any
earlier audit finding, so they are used unchanged. The review overview repeats those two findings
and summarizes the PR; it adds no other request, so it adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2520-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2520-f2` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The visibility-only change does not remove the lock type from the public API

- PR number: 2520
- Source review ID: 5514549824
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268014012>
- Concern: The spec said the public field "makes the lock type part of each crate's public API",
  implying that making it private removes the lock from the API. All five public read accessors
  (`get_stats()` / `get_metrics()`) still return `RwLockReadGuard<'_, Metrics>`, and the spec
  excluded changing method signatures. The goal and the semver rationale contradicted the planned
  change.
- Solution: The maintainer chose to narrow the spec rather than broaden it. The Goal now states a
  visibility-only change that keeps the read guards. The Background no longer claims the lock type
  leaves the API, and names callers outside the five crates that rely on the guards. The semver
  note is limited to the field. Out of Scope records replacing the guards (per-metric accessors or
  a cheap snapshot) as a possible separate #1669 sub-issue; cloning the whole `Metrics` per read is
  too costly on event-handler paths. The PR body and the #2517 issue body were corrected the same
  way.
- Current-tree verification: `grep -n "RwLockReadGuard" packages/*/src/statistics/repository.rs`
  shows the five read accessors returning the guard; the spec's Goal, Background, Semver impact,
  and Out of Scope sections now describe the change as visibility-only. Outside callers confirmed
  with a search for `.get_stats()` / `.get_metrics()` (for example
  `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs` and the `axum-http-server`
  contract tests).
- Resolution reference: `docs(issues): [#2517] narrow the spec to a visibility-only change`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268384235>

### F2 - Cite the EPIC by stable issue reference instead of its path

- PR number: 2520
- Source review ID: 5514549824
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268014067>
- Concern: The spec frontmatter listed `docs/issues/open/1669-overhaul-packages/EPIC.md` in
  `semantic-links.related-artifacts`. That path becomes stale when the EPIC spec moves to
  `docs/issues/closed/`; the semantic link convention requires a quoted `issue #NNNN` value.
- Solution: Replaced the path with `"issue #1669"`.
- Current-tree verification: the spec frontmatter's `related-artifacts` lists `"issue #1669"` and
  no `docs/issues/` path; the staged-frontmatter pre-commit check passed.
- Resolution reference: `docs(issues): [#2517] cite EPIC #1669 by issue number in spec frontmatter`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2520#discussion_r4268384509>

## Processing Log

- 2026-10-14 08:12 UTC - Started audit: fetched review 5514549824 and its two unresolved threads
  with the `github-review-threads` tool. Asked the maintainer whether to narrow or broaden the
  spec for F1; the maintainer chose to narrow it.
- 2026-10-14 08:17 UTC - Pushed the signed fix commits for F2 and F1; the pre-push checks passed.
  Corrected the same inaccurate claim in the PR body and the #2517 issue body. Replied on both threads
  with the disposition, verification, and resolution reference.
- 2026-10-14 08:19 UTC - Recorded this audit; the threads are resolved after it is committed and
  pushed.

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
